// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem;

use libnvme_sys::{identify::nvme_uint128_t, nvme::*};
use thiserror::Error;
use zerocopy::FromBytes;

use crate::{
    controller::{Controller, NvmeControllerError},
    controller_info::ControllerInfo,
    logpage::{LogPageBytesError, LogPageName},
};

/// The Critical Warning field of the SMART / Health log page. Each is `true`
/// if that warning is raised.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CriticalWarnings {
    /// Available spare capacity has fallen below the threshold.
    pub available_spare_low: bool,
    /// Temperature is outside the threshold.
    pub temperature: bool,
    /// Reliability is degraded due to media errors.
    pub reliability_degraded: bool,
    /// Media has been placed in read-only mode.
    pub read_only: bool,
    /// Volatile memory backup device has failed. `None` if the controller
    /// has no volatile write cache.
    pub volatile_memory_backup_failed: Option<bool>,
    /// Persistent Memory Region has become read-only or unreliable (1.4).
    pub persistent_memory_region_read_only: Option<bool>,
    /// A change to a personality's settings did not complete and could not be
    /// reverted, leaving them in an indeterminate state (2.3).
    pub indeterminate_personality_state: Option<bool>,
}

/// The Endurance Group Critical Warning Summary field of the SMART / Health
/// log page (1.4). Each is `true` if that warning is raised for one or more
/// Endurance Groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnduranceGroupWarnings {
    /// Available spare capacity has fallen below the threshold.
    pub available_spare_low: bool,
    /// Reliability is degraded due to media or internal errors.
    pub reliability_degraded: bool,
    /// Namespaces have been placed in read-only mode, other than by a change
    /// in their write protection state.
    pub read_only: bool,
}

/// The SMART / Health log page for an NVMe controller.
///
/// Temperatures are converted to Celsius and data units to bytes. A field
/// that isn't meaningful for this controller, by its NVMe version or Identify
/// data, is `None`, following `nvme_print_health_log()` in illumos
/// `cmd/nvmeadm/nvmeadm_print.c`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HealthLogPage {
    pub critical_warnings: CriticalWarnings,
    /// Endurance Group critical warnings (1.4).
    pub endurance_group_warnings: Option<EnduranceGroupWarnings>,
    /// Composite temperature. `None` if the controller reports 0 K.
    pub temperature_c: Option<i32>,
    /// Normalized percentage of remaining spare capacity.
    pub available_spare_percent: u8,
    /// Normalized percentage threshold for `available_spare_percent`.
    pub available_spare_threshold_percent: u8,
    /// Vendor estimate of the percentage of rated life used; may exceed 100.
    pub life_used_percent: u8,
    pub data_read_bytes: u128,
    pub data_written_bytes: u128,
    pub host_read_commands: u128,
    pub host_write_commands: u128,
    /// Minutes the controller was busy with I/O commands.
    pub controller_busy_minutes: u128,
    pub power_cycles: u128,
    pub power_on_hours: u128,
    pub unsafe_shutdowns: u128,
    pub media_errors: u128,
    pub error_log_entries: u128,
    /// Minutes above the warning composite temperature threshold (1.2), if
    /// the controller reports one.
    pub warning_temperature_minutes: Option<u32>,
    /// Minutes above the critical composite temperature threshold (1.2), if
    /// the controller reports one.
    pub critical_temperature_minutes: Option<u32>,
    /// The implemented temperature sensors (1.2).
    pub temperature_sensors: Vec<TemperatureSensor>,
    /// Thermal management statistics (1.3).
    pub thermal_management: Option<ThermalManagement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemperatureSensor {
    /// Sensor number, 1 through 8.
    pub sensor: u8,
    pub temperature_c: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThermalManagement {
    /// Transitions into thermal management temperature 1.
    pub temp1_transitions: u32,
    /// Transitions into thermal management temperature 2.
    pub temp2_transitions: u32,
    /// Seconds in thermal management temperature 1.
    pub temp1_seconds: u32,
    /// Seconds in thermal management temperature 2.
    pub temp2_seconds: u32,
}

/// Controller properties that determine which health fields are meaningful.
#[derive(Debug, Clone, Copy)]
struct HealthContext {
    /// NVMe version as `(major, minor)`.
    version: (u16, u16),
    /// Volatile write cache present.
    vwc_present: bool,
    /// Warning Composite Temperature Threshold in Kelvin, 0 if unreported.
    wctemp: u16,
    /// Critical Composite Temperature Threshold in Kelvin, 0 if unreported.
    cctemp: u16,
}

impl HealthContext {
    fn new(info: &ControllerInfo) -> Self {
        Self {
            version: info.version(),
            vwc_present: info.volatile_write_cache_present(),
            wctemp: info.warning_composite_temp(),
            cctemp: info.critical_composite_temp(),
        }
    }

    fn at_least(&self, major: u16, minor: u16) -> bool {
        self.version >= (major, minor)
    }
}

fn u128_from(v: nvme_uint128_t) -> u128 {
    (u128::from(v.hi) << 64) | u128::from(v.lo)
}

/// Convert a temperature in Kelvin to Celsius. 0 K means the temperature
/// isn't reported.
fn kelvin_to_c(k: u16) -> Option<i32> {
    (k != 0).then(|| i32::from(k) - 273)
}

/// Data units are 1000 512-byte blocks.
fn data_units_to_bytes(units: nvme_uint128_t) -> u128 {
    u128_from(units).saturating_mul(1000 * 512)
}

impl HealthLogPage {
    fn init(logpage: nvme_health_log_t, ctx: &HealthContext) -> Self {
        let cw = logpage.hl_crit_warn;
        let v1_2 = ctx.at_least(1, 2);
        let sensors = [
            logpage.hl_temp_sensor_1,
            logpage.hl_temp_sensor_2,
            logpage.hl_temp_sensor_3,
            logpage.hl_temp_sensor_4,
            logpage.hl_temp_sensor_5,
            logpage.hl_temp_sensor_6,
            logpage.hl_temp_sensor_7,
            logpage.hl_temp_sensor_8,
        ];
        Self {
            critical_warnings: CriticalWarnings {
                available_spare_low: cw.cw_avail() != 0,
                temperature: cw.cw_temp() != 0,
                reliability_degraded: cw.cw_reliab() != 0,
                read_only: cw.cw_readonly() != 0,
                volatile_memory_backup_failed: ctx
                    .vwc_present
                    .then_some(cw.cw_volatile() != 0),
                persistent_memory_region_read_only: ctx
                    .at_least(1, 4)
                    .then_some(cw.cw_pmrro() != 0),
                indeterminate_personality_state: ctx
                    .at_least(2, 3)
                    .then_some(cw.cw_ips() != 0),
            },
            endurance_group_warnings: ctx.at_least(1, 4).then(|| {
                let eg = logpage.hl_ecgcws;
                EnduranceGroupWarnings {
                    available_spare_low: eg.ecgws_egascbt() != 0,
                    reliability_degraded: eg.ecgws_egdr() != 0,
                    read_only: eg.ecgws_egro() != 0,
                }
            }),
            temperature_c: kelvin_to_c(logpage.hl_temp),
            available_spare_percent: logpage.hl_avail_spare,
            available_spare_threshold_percent: logpage.hl_avail_spare_thr,
            life_used_percent: logpage.hl_used,
            data_read_bytes: data_units_to_bytes(logpage.hl_data_read),
            data_written_bytes: data_units_to_bytes(logpage.hl_data_write),
            host_read_commands: u128_from(logpage.hl_host_read),
            host_write_commands: u128_from(logpage.hl_host_write),
            controller_busy_minutes: u128_from(logpage.hl_ctrl_busy),
            power_cycles: u128_from(logpage.hl_power_cycles),
            power_on_hours: u128_from(logpage.hl_power_on_hours),
            unsafe_shutdowns: u128_from(logpage.hl_unsafe_shutdn),
            media_errors: u128_from(logpage.hl_media_errors),
            error_log_entries: u128_from(logpage.hl_errors_logged),
            warning_temperature_minutes: (v1_2 && ctx.wctemp != 0)
                .then_some(logpage.hl_warn_temp_time),
            critical_temperature_minutes: (v1_2 && ctx.cctemp != 0)
                .then_some(logpage.hl_crit_temp_time),
            temperature_sensors: if v1_2 {
                (1u8..)
                    .zip(sensors)
                    .filter_map(|(sensor, k)| {
                        Some(TemperatureSensor {
                            sensor,
                            temperature_c: kelvin_to_c(k)?,
                        })
                    })
                    .collect()
            } else {
                Vec::new()
            },
            thermal_management: ctx.at_least(1, 3).then_some(
                ThermalManagement {
                    temp1_transitions: logpage.hl_tmtemp_1_tc,
                    temp2_transitions: logpage.hl_tmtemp_2_tc,
                    temp1_seconds: logpage.hl_tmtemp_1_time,
                    temp2_seconds: logpage.hl_tmtemp_2_time,
                },
            ),
        }
    }
}

#[derive(Debug, Error)]
pub enum HealthLogPageError {
    #[error("libnvme error: {0}")]
    ControllerError(#[from] NvmeControllerError),
    #[error(
        "libnvme says the log page is {} bytes but it should be {} bytes",
        size,
        expected_size
    )]
    UnexpectedSize { size: usize, expected_size: usize },
}

impl<'a> Controller<'a> {
    /// Get the controller's SMART / Health log page.
    pub fn get_health_log_page(
        &self,
    ) -> Result<HealthLogPage, HealthLogPageError> {
        let ctx = HealthContext::new(&self.get_info()?);
        let expected_size = mem::size_of::<nvme_health_log_t>();
        let buf =
            self.get_logpage_bytes(LogPageName::Health).map_err(
                |e| match e {
                    LogPageBytesError::Controller(e) => e.into(),
                    LogPageBytesError::TooLarge { size, .. } => {
                        HealthLogPageError::UnexpectedSize {
                            size,
                            expected_size,
                        }
                    }
                },
            )?;
        let logpage =
            nvme_health_log_t::read_from_bytes(&buf).map_err(|_| {
                HealthLogPageError::UnexpectedSize {
                    size: buf.len(),
                    expected_size,
                }
            })?;
        Ok(HealthLogPage::init(logpage, &ctx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zerocopy::FromZeros;

    const CTX: HealthContext = HealthContext {
        version: (1, 4),
        vwc_present: false,
        wctemp: 343,
        cctemp: 353,
    };

    fn u128(v: u64) -> nvme_uint128_t {
        nvme_uint128_t { lo: v, hi: 0 }
    }

    /// Values from a Micron 7450.
    fn micron_sample() -> nvme_health_log_t {
        let mut log = nvme_health_log_t::new_zeroed();
        log.hl_temp = 47 + 273;
        log.hl_avail_spare = 100;
        log.hl_avail_spare_thr = 10;
        log.hl_data_read = u128(27_300);
        log.hl_data_write = u128(33_600);
        log.hl_host_read = u128(1_957_633);
        log.hl_host_write = u128(1_526_967);
        log.hl_ctrl_busy = u128(2);
        log.hl_power_cycles = u128(15);
        log.hl_power_on_hours = u128(234);
        log.hl_unsafe_shutdn = u128(14);
        log.hl_temp_sensor_1 = 61 + 273;
        log.hl_temp_sensor_2 = 53 + 273;
        log.hl_temp_sensor_3 = 43 + 273;
        log
    }

    #[test]
    fn converts_units() {
        let h = HealthLogPage::init(micron_sample(), &CTX);
        assert_eq!(h.temperature_c, Some(47));
        assert_eq!(h.data_read_bytes, 27_300 * 512_000);
        assert_eq!(h.data_written_bytes, 33_600 * 512_000);
        assert_eq!(h.power_on_hours, 234);
        let sensors: Vec<(u8, i32)> = h
            .temperature_sensors
            .iter()
            .map(|s| (s.sensor, s.temperature_c))
            .collect();
        assert_eq!(sensors, [(1, 61), (2, 53), (3, 43)]);
        assert_eq!(h.warning_temperature_minutes, Some(0));
        assert!(h.thermal_management.is_some());
        // No volatile write cache, so no volatile memory backup warning.
        assert_eq!(h.critical_warnings.volatile_memory_backup_failed, None);
    }

    #[test]
    fn zero_kelvin_is_unreported() {
        let mut log = micron_sample();
        log.hl_temp = 0;
        assert_eq!(HealthLogPage::init(log, &CTX).temperature_c, None);
    }

    #[test]
    fn critical_warnings() {
        let mut log = micron_sample();
        log.hl_crit_warn = nvme_health_log_t_crit_warn::from_bits(0x7f);
        let ctx = HealthContext { version: (2, 3), vwc_present: true, ..CTX };
        let cw = HealthLogPage::init(log, &ctx).critical_warnings;
        assert!(cw.available_spare_low && cw.temperature && cw.read_only);
        assert!(cw.reliability_degraded);
        assert_eq!(cw.volatile_memory_backup_failed, Some(true));
        assert_eq!(cw.persistent_memory_region_read_only, Some(true));
        assert_eq!(cw.indeterminate_personality_state, Some(true));
    }

    #[test]
    fn endurance_group_warnings() {
        let eg = |bits| {
            let mut log = micron_sample();
            log.hl_ecgcws = nvme_health_log_t_ecgcws::from_bits(bits);
            HealthLogPage::init(log, &CTX).endurance_group_warnings
        };
        // Bit 1 is reserved, and ignored.
        assert_eq!(
            eg(0b1111),
            Some(EnduranceGroupWarnings {
                available_spare_low: true,
                reliability_degraded: true,
                read_only: true,
            })
        );
        assert_eq!(
            eg(0b0100),
            Some(EnduranceGroupWarnings {
                available_spare_low: false,
                reliability_degraded: true,
                read_only: false,
            })
        );
    }

    #[test]
    fn version_gating() {
        let h = HealthLogPage::init(micron_sample(), &CTX);
        assert_eq!(
            h.critical_warnings.persistent_memory_region_read_only,
            Some(false)
        );
        assert_eq!(h.critical_warnings.indeterminate_personality_state, None);

        let ctx = HealthContext { version: (1, 1), ..CTX };
        let h = HealthLogPage::init(micron_sample(), &ctx);
        assert_eq!(
            h.critical_warnings.persistent_memory_region_read_only,
            None
        );
        assert_eq!(h.endurance_group_warnings, None);
        assert!(h.temperature_sensors.is_empty());
        assert_eq!(h.warning_temperature_minutes, None);
        assert!(h.thermal_management.is_none());

        let ctx = HealthContext { version: (1, 2), wctemp: 0, ..CTX };
        let h = HealthLogPage::init(micron_sample(), &ctx);
        assert_eq!(h.temperature_sensors.len(), 3);
        assert_eq!(h.warning_temperature_minutes, None);
        assert_eq!(h.critical_temperature_minutes, Some(0));
        assert!(h.thermal_management.is_none());
    }

    #[test]
    fn data_units() {
        assert_eq!(data_units_to_bytes(u128(0)), 0);
        assert_eq!(data_units_to_bytes(u128(1)), 512_000);
        let max = nvme_uint128_t { lo: u64::MAX, hi: u64::MAX };
        assert_eq!(data_units_to_bytes(max), u128::MAX);
    }
}
