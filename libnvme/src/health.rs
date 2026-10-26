// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem::{self, MaybeUninit};

use libnvme_sys::{identify::nvme_uint128_t, nvme::*};
use thiserror::Error;

use crate::{
    controller::{Controller, NvmeControllerError},
    error::LibraryError,
    logpage::{LogPageInfo, LogPageName},
};

/// The Critical Warning field of the SMART / Health log page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CriticalWarnings {
    /// Available spare capacity has fallen below the threshold.
    pub available_spare: bool,
    /// Temperature is outside the threshold.
    pub temperature: bool,
    /// Reliability is degraded due to media errors.
    pub reliability_degraded: bool,
    /// Media has been placed in read-only mode.
    pub read_only: bool,
    /// Volatile memory backup device has failed.
    pub volatile_memory_backup: bool,
    /// Persistent Memory Region has become read-only or unreliable (1.4).
    pub persistent_memory_region_read_only: bool,
    /// A change to a personality's settings did not complete and could not be
    /// reverted, leaving them in an indeterminate state (2.3).
    pub indeterminate_personality_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The SMART / Health log page for an NVMe controller.
///
/// Temperatures are in Kelvin; a temperature sensor reporting 0 is not
/// implemented.
pub struct HealthLogPage {
    pub critical_warnings: CriticalWarnings,
    /// Composite temperature.
    pub temperature: u16,
    /// Normalized percentage of remaining spare capacity.
    pub available_spare: u8,
    /// Normalized percentage threshold for `available_spare`.
    pub available_spare_threshold: u8,
    /// Vendor-specific estimate of the percentage of life used.
    pub percentage_used: u8,
    /// Data read in units of 1000 512-byte blocks.
    pub data_units_read: u128,
    /// Data written in units of 1000 512-byte blocks.
    pub data_units_written: u128,
    pub host_read_commands: u128,
    pub host_write_commands: u128,
    /// Minutes the controller was busy with I/O commands.
    pub controller_busy_time: u128,
    pub power_cycles: u128,
    pub power_on_hours: u128,
    pub unsafe_shutdowns: u128,
    pub media_errors: u128,
    pub error_log_entries: u128,
    /// Minutes above the warning composite temperature threshold (1.2).
    pub warning_temperature_time: u32,
    /// Minutes above the critical composite temperature threshold (1.2).
    pub critical_temperature_time: u32,
    /// Temperature sensors 1-8 (1.2).
    pub temperature_sensors: [u16; 8],
    /// Transitions into thermal management temperature 1 (1.3).
    pub thermal_mgmt_temp1_transitions: u32,
    /// Transitions into thermal management temperature 2 (1.3).
    pub thermal_mgmt_temp2_transitions: u32,
    /// Seconds in thermal management temperature 1 (1.3).
    pub thermal_mgmt_temp1_time: u32,
    /// Seconds in thermal management temperature 2 (1.3).
    pub thermal_mgmt_temp2_time: u32,
}

fn u128_from(v: nvme_uint128_t) -> u128 {
    (u128::from(v.hi) << 64) | u128::from(v.lo)
}

impl HealthLogPage {
    fn init(logpage: nvme_health_log_t) -> Self {
        let cw = logpage.hl_crit_warn;
        Self {
            critical_warnings: CriticalWarnings {
                available_spare: cw.cw_avail() != 0,
                temperature: cw.cw_temp() != 0,
                reliability_degraded: cw.cw_reliab() != 0,
                read_only: cw.cw_readonly() != 0,
                volatile_memory_backup: cw.cw_volatile() != 0,
                persistent_memory_region_read_only: cw.cw_pmrro() != 0,
                indeterminate_personality_state: cw.cw_ips() != 0,
            },
            temperature: logpage.hl_temp,
            available_spare: logpage.hl_avail_spare,
            available_spare_threshold: logpage.hl_avail_spare_thr,
            percentage_used: logpage.hl_used,
            data_units_read: u128_from(logpage.hl_data_read),
            data_units_written: u128_from(logpage.hl_data_write),
            host_read_commands: u128_from(logpage.hl_host_read),
            host_write_commands: u128_from(logpage.hl_host_write),
            controller_busy_time: u128_from(logpage.hl_ctrl_busy),
            power_cycles: u128_from(logpage.hl_power_cycles),
            power_on_hours: u128_from(logpage.hl_power_on_hours),
            unsafe_shutdowns: u128_from(logpage.hl_unsafe_shutdn),
            media_errors: u128_from(logpage.hl_media_errors),
            error_log_entries: u128_from(logpage.hl_errors_logged),
            warning_temperature_time: logpage.hl_warn_temp_time,
            critical_temperature_time: logpage.hl_crit_temp_time,
            temperature_sensors: [
                logpage.hl_temp_sensor_1,
                logpage.hl_temp_sensor_2,
                logpage.hl_temp_sensor_3,
                logpage.hl_temp_sensor_4,
                logpage.hl_temp_sensor_5,
                logpage.hl_temp_sensor_6,
                logpage.hl_temp_sensor_7,
                logpage.hl_temp_sensor_8,
            ],
            thermal_mgmt_temp1_transitions: logpage.hl_tmtemp_1_tc,
            thermal_mgmt_temp2_transitions: logpage.hl_tmtemp_2_tc,
            thermal_mgmt_temp1_time: logpage.hl_tmtemp_1_time,
            thermal_mgmt_temp2_time: logpage.hl_tmtemp_2_time,
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
        let expected_size = mem::size_of::<nvme_health_log_t>();
        let LogPageInfo { size, req, .. } =
            self.get_logpage(LogPageName::Health)?;
        if size != expected_size {
            return Err(HealthLogPageError::UnexpectedSize {
                size,
                expected_size,
            });
        }
        let mut buf = MaybeUninit::<nvme_health_log_t>::zeroed();
        self.check_result(
            unsafe {
                nvme_log_req_set_output(
                    req.inner,
                    buf.as_mut_ptr().cast(),
                    size,
                )
            },
            || format!("failed to set logpage req size to {size}"),
        )?;

        self.check_result(unsafe { nvme_log_req_exec(req.inner) }, || {
            "failed to execute health log request"
        })?;

        // Our request was successful so we can assume this struct has been
        // populated now.
        let logpage = unsafe { buf.assume_init() };
        Ok(HealthLogPage::init(logpage))
    }
}
