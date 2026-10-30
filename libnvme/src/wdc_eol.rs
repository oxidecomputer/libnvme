// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The WDC EOL log page (`wdc/eol`, log identifier c0h), supported by the
//! Ultrastar DC SN840.
//!
//! The layout is `wdc_vul_sn840_eol_t` from illumos `sys/nvme/wdc_sn840.h`,
//! a packed 118-byte structure. libnvme rounds the log size up to a multiple
//! of 4 bytes, so 120 bytes are read.

use std::mem;

use libnvme_sys::nvme::wdc_vul_sn840_eol_t;
use thiserror::Error;
use zerocopy::FromBytes;

use crate::{
    controller::{Controller, NvmeControllerError},
    logpage::{LogPageBytesError, LogPageName},
};

/// Size of `wdc_vul_sn840_eol_t`.
const EOL_LOG_LEN: usize = mem::size_of::<wdc_vul_sn840_eol_t>();

/// The decoded EOL log page. Field names follow nvmeadm's descriptions; the
/// illumos field name is given for each.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EolLog {
    /// Reallocated block count (`eol_rbc`).
    pub reallocated_block_count: u32,
    /// Write amplification factor (`eol_waf`).
    pub write_amplification_factor: u32,
    /// Percentage of rated life remaining (`eol_plr`).
    ///
    /// This is signed: once a drive passes its rated endurance it reports
    /// how far past it is as a negative value, e.g. -57 for a drive at 157%
    /// of its rated life.
    pub percentage_life_remaining: i32,
    /// Program fail count (`eol_pfc`).
    pub program_fail_count: u32,
    /// Erase fail count (`eol_efc`).
    pub erase_fail_count: u32,
    /// Vendor data (`eol_vendor`).
    pub vendor_data: u32,
    /// Customer EOL status, the percentage of rated life used
    /// (`eol_cust_sts`).
    pub customer_eol_status: u16,
    /// System area EOL status, the percentage of rated life used
    /// (`eol_sys_sts`).
    pub system_area_eol_status: u16,
    /// Customer EOL state (`eol_cust_state`).
    pub customer_eol_state: EolState,
    /// System area EOL state (`eol_sys_state`).
    pub system_area_eol_state: EolState,
}

/// An EOL state field of the EOL log page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum EolState {
    /// 00h: normal mode.
    Normal,
    /// 01h: end of life (EOL status is over 100%).
    EndOfLife,
    /// 02h: read only mode.
    ReadOnly,
    Unknown(u8),
}

impl From<u8> for EolState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::EndOfLife,
            2 => Self::ReadOnly,
            state => Self::Unknown(state),
        }
    }
}

#[derive(Debug, Error)]
pub enum EolError {
    #[error("libnvme error: {0}")]
    ControllerError(#[from] NvmeControllerError),
    #[error("EOL log is {len} bytes, shorter than the {EOL_LOG_LEN} expected")]
    TooShort { len: usize },
    #[error("EOL log is {size} bytes, more than the {max} byte maximum")]
    TooLarge { size: usize, max: usize },
}

impl EolLog {
    /// Decode a raw EOL log page.
    pub fn parse(buf: &[u8]) -> Result<Self, EolError> {
        let Ok((eol, _)) = wdc_vul_sn840_eol_t::read_from_prefix(buf) else {
            return Err(EolError::TooShort { len: buf.len() });
        };
        Ok(EolLog {
            reallocated_block_count: eol.eol_rbc,
            write_amplification_factor: eol.eol_waf,
            // Defined as unsigned, but SN840s report a signed value once past
            // their rated endurance; it matches 100 - `eol_cust_sts`.
            percentage_life_remaining: eol.eol_plr.cast_signed(),
            program_fail_count: eol.eol_pfc,
            erase_fail_count: eol.eol_efc,
            vendor_data: eol.eol_vendor,
            customer_eol_status: eol.eol_cust_sts,
            system_area_eol_status: eol.eol_sys_sts,
            customer_eol_state: eol.eol_cust_state.into(),
            system_area_eol_state: eol.eol_sys_state.into(),
        })
    }
}

impl<'a> Controller<'a> {
    /// Get the controller's WDC EOL log page. Only controllers that list
    /// `wdc/eol` in [`Controller::log_page_discovery`] support it.
    pub fn get_wdc_eol_log_page(&self) -> Result<EolLog, EolError> {
        let buf =
            self.get_logpage_bytes(LogPageName::WdcEol).map_err(
                |e| match e {
                    LogPageBytesError::Controller(e) => e.into(),
                    LogPageBytesError::TooLarge { size, max } => {
                        EolError::TooLarge { size, max }
                    }
                },
            )?;
        EolLog::parse(&buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `nvmeadm get-logpage nvme11 wdc/eol` from an SN840 at 157% of its
    /// rated life, as the 120 bytes libnvme reads.
    const NVME11: &[u8] = &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x8c, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x67, 0x00, 0x00, 0x00, 0xc7, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x35, 0x01, 0x00, 0x00, 0x9d, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn parses_sn840_log() {
        let log = EolLog::parse(NVME11).unwrap();
        assert_eq!(
            log,
            EolLog {
                reallocated_block_count: 0x328c,
                write_amplification_factor: 0x67,
                percentage_life_remaining: -57,
                program_fail_count: 0,
                erase_fail_count: 0,
                vendor_data: 0x135,
                customer_eol_status: 157,
                system_area_eol_status: 0,
                customer_eol_state: EolState::EndOfLife,
                system_area_eol_state: EolState::Normal,
            }
        );
    }

    #[test]
    fn eol_state_values() {
        assert_eq!(EolState::from(0), EolState::Normal);
        assert_eq!(EolState::from(1), EolState::EndOfLife);
        assert_eq!(EolState::from(2), EolState::ReadOnly);
        assert_eq!(EolState::from(3), EolState::Unknown(3));
    }

    #[test]
    fn rejects_short_log() {
        assert!(matches!(
            EolLog::parse(&NVME11[..117]),
            Err(EolError::TooShort { len: 117 })
        ));
        // The unpadded structure is enough.
        assert!(EolLog::parse(&NVME11[..EOL_LOG_LEN]).is_ok());
    }
}
