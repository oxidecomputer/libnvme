// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The WDC Device Manageability log page (`wdc/devmgmt`, log identifier
//! c2h), supported by the Ultrastar DC SN840.
//!
//! The log is a sequence of variable length entries (`wdc_vsd_t` in illumos
//! `sys/nvme/wdc.h`), each a little-endian `u32` length that includes the
//! 8-byte entry header, a `u32` identifier, and data. The first entry is the
//! log header: its length is that of the whole log and its identifier is the
//! log version. Identifiers and their types come from `wdc_sn840_vsd_id_t` in
//! `sys/nvme/wdc_sn840.h`, whose names the fields below use.

use thiserror::Error;

use crate::{
    controller::{Controller, NvmeControllerError},
    logpage::{LogPageBytesError, LogPageName},
};

/// Size of an entry header (and of the log header).
const VSD_HDR_LEN: usize = 8;

/// The decoded Device Manageability log page. A field is `None` if the drive
/// didn't report that entry; if it reported one more than once, the last
/// value is kept.
///
/// While the log page is a list of identifier/value entries, in practice
/// SN840s report exactly one entry for each identifier. The fields are
/// `Option`s so callers can distinguish a missing value from `Some(0)`.
/// Repeated identifiers aren't expected, so keeping the last value is a
/// simple approach that makes the decoded data easier to use than if every
/// field were a `Vec`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DevMgmtLog {
    /// Log version (the identifier of the log header).
    pub version: u32,
    /// 0x01
    pub id: Option<u32>,
    /// 0x02
    pub uefi_ver: Option<String>,
    /// 0x03
    pub sbl_ver: Option<String>,
    /// 0x04
    pub def_user_cap: Option<u64>,
    /// 0x05
    pub max_user_cap: Option<u64>,
    /// 0x06
    pub min_user_cap: Option<u64>,
    /// 0x07
    pub name: Option<String>,
    /// 0x08: supported log page identifiers.
    pub log_sup: Option<Vec<u8>>,
    /// 0x09: supported feature identifiers.
    pub feat_sup: Option<Vec<u8>>,
    /// 0x0a
    pub form_factor: Option<u32>,
    /// 0x0b
    pub resize_gran: Option<u64>,
    /// 0x0c
    pub ns_alloc_size: Option<u64>,
    /// 0x0d
    pub ns_reg_avail: Option<u64>,
    /// 0x0e
    pub raw_nvm: Option<u64>,
    /// 0x0f
    pub port_cfg_sts: Option<u32>,
    /// 0x10
    pub mpn: Option<String>,
    /// 0x11
    pub sn: Option<String>,
    /// 0x12
    pub def_ns_attrs: Option<u32>,
    /// 0x13
    pub git_descr: Option<String>,
    /// 0x14
    pub smb_bl: Option<String>,
    /// 0x15
    pub cust_id: Option<u32>,
    /// 0x16
    pub prod_desc: Option<String>,
    /// 0x17
    pub tmm_ver: Option<String>,
    /// 0x18
    pub therm_throt_sts: Option<u32>,
    /// 0x19: non-zero if the drive has recorded an internal firmware assert.
    pub assert_dump: Option<u32>,
    /// 0x1a
    pub cust_eol_sts: Option<u32>,
    /// 0x1b
    pub ifs_eol_sts: Option<u32>,
    /// 0x1c
    pub cust_eol_state: Option<u32>,
    /// 0x1d
    pub ifs_eol_state: Option<u32>,
    /// 0x1e
    pub fcr: Option<u32>,
    /// 0x1f
    pub vca_bpc_rev: Option<u32>,
    /// 0x20
    pub vca_bpc_min_rev: Option<u32>,
    /// 0x21
    pub vca_bpc_rst_seq: Option<u32>,
    /// 0x22
    pub vca_tpc_rst_seq: Option<u32>,
    /// 0x23
    pub vca_tpc_fss_seq: Option<u32>,
    /// Entries with identifiers illumos doesn't define, in log order.
    pub unknown: Vec<UnknownEntry>,
}

/// An entry with an unknown identifier, kept as its raw data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownEntry {
    pub id: u32,
    pub data: Vec<u8>,
}

impl UnknownEntry {
    /// Return the unknown value as a `String` if it cleanly decodes to one.
    /// In practice the four unknown entries (0x24 through 0x27) contain the
    /// string "CUSTOMER_ID_GENERIC".
    pub fn as_text(&self) -> Option<String> {
        let text = cbs(&self.data)?;
        let padding = &self.data[4 + text.len()..];
        let text = text.strip_suffix(&[0]).unwrap_or(text);
        if text.is_empty()
            || padding.len() > 3
            || padding.iter().any(|&b| b != 0)
            || !text.iter().all(|&b| b.is_ascii_graphic() || b == b' ')
        {
            return None;
        }
        Some(String::from_utf8_lossy(text).into_owned())
    }
}

#[derive(Debug, Error)]
pub enum DevMgmtError {
    #[error("libnvme error: {0}")]
    ControllerError(#[from] NvmeControllerError),
    #[error("malformed device manageability log at offset {offset}: {reason}")]
    Malformed { offset: usize, reason: &'static str },
    #[error(
        "device manageability log is {size} bytes, more than the {max} \
         byte maximum"
    )]
    TooLarge { size: usize, max: usize },
}

fn u32_at(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

fn u64_at(b: &[u8], off: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(off..off + 8)?.try_into().ok()?))
}

/// The contents of a WDC counted byte string (`wdc_cbs_t`): a `u32` length
/// followed by that many bytes and any padding.
fn cbs(data: &[u8]) -> Option<&[u8]> {
    let len = usize::try_from(u32_at(data, 0)?).ok()?;
    data.get(4..4usize.checked_add(len)?)
}

fn cbs_str(data: &[u8]) -> Option<String> {
    Some(String::from_utf8_lossy(cbs(data)?).trim_end_matches('\0').to_string())
}

impl DevMgmtLog {
    /// Decode a raw Device Manageability log page.
    pub fn parse(buf: &[u8]) -> Result<Self, DevMgmtError> {
        let malformed =
            |offset, reason| DevMgmtError::Malformed { offset, reason };

        let (Some(total), Some(version)) = (u32_at(buf, 0), u32_at(buf, 4))
        else {
            return Err(malformed(0, "truncated log header"));
        };
        let end = usize::try_from(total)
            .map_err(|_| malformed(0, "log length too large"))?;
        if end < VSD_HDR_LEN {
            return Err(malformed(0, "log length shorter than its header"));
        }
        if end > buf.len() {
            return Err(malformed(0, "log length exceeds the data read"));
        }

        let mut log = DevMgmtLog { version, ..Default::default() };
        let mut off = VSD_HDR_LEN;
        while off < end {
            let (Some(len), Some(id)) =
                (u32_at(&buf[..end], off), u32_at(&buf[..end], off + 4))
            else {
                return Err(malformed(off, "truncated entry header"));
            };
            let len = usize::try_from(len)
                .map_err(|_| malformed(off, "entry length too large"))?;
            if len < VSD_HDR_LEN {
                return Err(malformed(off, "entry shorter than its header"));
            }
            let next = off
                .checked_add(len)
                .filter(|&next| next <= end)
                .ok_or_else(|| malformed(off, "entry extends past the log"))?;
            if !log.set(id, &buf[off + VSD_HDR_LEN..next]) {
                return Err(malformed(off, "entry too short for its type"));
            }
            off = next;
        }

        Ok(log)
    }

    /// Store an entry's value in its field, or in `unknown`. Returns false if
    /// the data is too short for the entry's type.
    fn set(&mut self, id: u32, data: &[u8]) -> bool {
        fn put<T>(field: &mut Option<T>, value: Option<T>) -> bool {
            *field = value;
            field.is_some()
        }
        let bytes = |data| cbs(data).map(<[u8]>::to_vec);
        match id {
            0x01 => put(&mut self.id, u32_at(data, 0)),
            0x02 => put(&mut self.uefi_ver, cbs_str(data)),
            0x03 => put(&mut self.sbl_ver, cbs_str(data)),
            0x04 => put(&mut self.def_user_cap, u64_at(data, 0)),
            0x05 => put(&mut self.max_user_cap, u64_at(data, 0)),
            0x06 => put(&mut self.min_user_cap, u64_at(data, 0)),
            0x07 => put(&mut self.name, cbs_str(data)),
            0x08 => put(&mut self.log_sup, bytes(data)),
            0x09 => put(&mut self.feat_sup, bytes(data)),
            0x0a => put(&mut self.form_factor, u32_at(data, 0)),
            0x0b => put(&mut self.resize_gran, u64_at(data, 0)),
            0x0c => put(&mut self.ns_alloc_size, u64_at(data, 0)),
            0x0d => put(&mut self.ns_reg_avail, u64_at(data, 0)),
            0x0e => put(&mut self.raw_nvm, u64_at(data, 0)),
            0x0f => put(&mut self.port_cfg_sts, u32_at(data, 0)),
            0x10 => put(&mut self.mpn, cbs_str(data)),
            0x11 => put(&mut self.sn, cbs_str(data)),
            0x12 => put(&mut self.def_ns_attrs, u32_at(data, 0)),
            0x13 => put(&mut self.git_descr, cbs_str(data)),
            0x14 => put(&mut self.smb_bl, cbs_str(data)),
            0x15 => put(&mut self.cust_id, u32_at(data, 0)),
            0x16 => put(&mut self.prod_desc, cbs_str(data)),
            0x17 => put(&mut self.tmm_ver, cbs_str(data)),
            0x18 => put(&mut self.therm_throt_sts, u32_at(data, 0)),
            0x19 => put(&mut self.assert_dump, u32_at(data, 0)),
            0x1a => put(&mut self.cust_eol_sts, u32_at(data, 0)),
            0x1b => put(&mut self.ifs_eol_sts, u32_at(data, 0)),
            0x1c => put(&mut self.cust_eol_state, u32_at(data, 0)),
            0x1d => put(&mut self.ifs_eol_state, u32_at(data, 0)),
            0x1e => put(&mut self.fcr, u32_at(data, 0)),
            0x1f => put(&mut self.vca_bpc_rev, u32_at(data, 0)),
            0x20 => put(&mut self.vca_bpc_min_rev, u32_at(data, 0)),
            0x21 => put(&mut self.vca_bpc_rst_seq, u32_at(data, 0)),
            0x22 => put(&mut self.vca_tpc_rst_seq, u32_at(data, 0)),
            0x23 => put(&mut self.vca_tpc_fss_seq, u32_at(data, 0)),
            _ => {
                self.unknown.push(UnknownEntry { id, data: data.to_vec() });
                true
            }
        }
    }
}

impl<'a> Controller<'a> {
    /// Get the controller's WDC Device Manageability log page. Only
    /// controllers that list `wdc/devmgmt` in
    /// [`Controller::log_page_discovery`] support it.
    pub fn get_wdc_devmgmt_log_page(&self) -> Result<DevMgmtLog, DevMgmtError> {
        let buf = self.get_logpage_bytes(LogPageName::WdcDevMgmt).map_err(
            |e| match e {
                LogPageBytesError::Controller(e) => e.into(),
                LogPageBytesError::TooLarge { size, max } => {
                    DevMgmtError::TooLarge { size, max }
                }
            },
        )?;
        DevMgmtLog::parse(&buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `nvmeadm get-logpage nvme4 wdc/devmgmt` from an SN840 that had
    /// recorded an internal assert.
    const NVME4: &[u8] = &[
        0x14, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x0c, 0x55, 0x03, 0x04, 0x10, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00, 0x00, 0x60, 0xe5, 0x3e, 0x7e, 0x03, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x60, 0xe5, 0x3e,
        0x7e, 0x03, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
        0x00, 0x20, 0x43, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
        0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa0, 0xac, 0xb9, 0x03, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0xca, 0x9a, 0x3b,
        0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00,
        0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
        0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x20, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00,
        0x55, 0x6c, 0x74, 0x72, 0x61, 0x73, 0x74, 0x61, 0x72, 0x20, 0x44, 0x43,
        0x20, 0x53, 0x4e, 0x38, 0x34, 0x30, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x57, 0x55, 0x53, 0x34,
        0x43, 0x36, 0x34, 0x33, 0x32, 0x44, 0x53, 0x50, 0x33, 0x58, 0x33, 0x00,
        0x14, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x41, 0x30, 0x38, 0x34, 0x41, 0x36, 0x37, 0x43, 0x30, 0x00, 0x00, 0x00,
        0x13, 0x00, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00, 0x6d, 0x61, 0x6c, 0x69,
        0x62, 0x75, 0x78, 0x2f, 0x52, 0x32, 0x32, 0x31, 0x30, 0x30, 0x31, 0x30,
        0x2e, 0x30, 0x30, 0x2d, 0x30, 0x2d, 0x67, 0x35, 0x31, 0x33, 0x61, 0x66,
        0x35, 0x66, 0x36, 0x61, 0x32, 0x64, 0x37, 0x00, 0x14, 0x00, 0x00, 0x00,
        0x14, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x52, 0x32, 0x32, 0x31,
        0x30, 0x33, 0x30, 0x32, 0x0c, 0x00, 0x00, 0x00, 0x15, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00,
        0x1f, 0x00, 0x00, 0x00, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x20,
        0x53, 0x46, 0x46, 0x20, 0x37, 0x6d, 0x6d, 0x20, 0x2c, 0x20, 0x49, 0x53,
        0x45, 0x20, 0x2c, 0x20, 0x47, 0x45, 0x4e, 0x45, 0x52, 0x49, 0x43, 0x00,
        0x1c, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00,
        0x31, 0x62, 0x39, 0x36, 0x2d, 0x32, 0x37, 0x32, 0x30, 0x2d, 0x33, 0x33,
        0x30, 0x63, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00,
        0x19, 0x06, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x00, 0x00, 0x30, 0x2e, 0x30, 0x2e, 0x30, 0x00, 0x00, 0x00,
        0x1c, 0x00, 0x00, 0x00, 0x17, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
        0x42, 0x34, 0x66, 0x5f, 0x43, 0x2e, 0x31, 0x2e, 0x30, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x1a, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x1b, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x1d, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
        0x44, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00,
        0x15, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x22, 0x00, 0x00, 0x00,
        0x0f, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00,
        0x2d, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00,
        0x13, 0x00, 0x00, 0x00, 0x43, 0x55, 0x53, 0x54, 0x4f, 0x4d, 0x45, 0x52,
        0x5f, 0x49, 0x44, 0x5f, 0x47, 0x45, 0x4e, 0x45, 0x52, 0x49, 0x43, 0x00,
        0x20, 0x00, 0x00, 0x00, 0x25, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00,
        0x43, 0x55, 0x53, 0x54, 0x4f, 0x4d, 0x45, 0x52, 0x5f, 0x49, 0x44, 0x5f,
        0x47, 0x45, 0x4e, 0x45, 0x52, 0x49, 0x43, 0x00, 0x20, 0x00, 0x00, 0x00,
        0x26, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00, 0x43, 0x55, 0x53, 0x54,
        0x4f, 0x4d, 0x45, 0x52, 0x5f, 0x49, 0x44, 0x5f, 0x47, 0x45, 0x4e, 0x45,
        0x52, 0x49, 0x43, 0x00, 0x20, 0x00, 0x00, 0x00, 0x27, 0x00, 0x00, 0x00,
        0x13, 0x00, 0x00, 0x00, 0x43, 0x55, 0x53, 0x54, 0x4f, 0x4d, 0x45, 0x52,
        0x5f, 0x49, 0x44, 0x5f, 0x47, 0x45, 0x4e, 0x45, 0x52, 0x49, 0x43, 0x00,
        0x20, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x80, 0x81, 0xc0, 0xc2,
        0xc4, 0xc5, 0xc6, 0xca, 0xcb, 0xfa, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00,
        0x09, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04,
        0x05, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0e, 0x17, 0x7e, 0x7f, 0x80, 0x81,
        0x82, 0x83, 0xc3, 0xcb, 0xce, 0xcf, 0xd2, 0xd3,
    ];

    #[test]
    fn parses_sn840_log() {
        let log = DevMgmtLog::parse(NVME4).unwrap();
        assert_eq!(log.version, 0);
        assert_eq!(log.assert_dump, Some(1));
        assert_eq!(log.sn.as_deref(), Some("A084A67C"));
        assert_eq!(log.name.as_deref(), Some("Ultrastar DC SN840"));
        assert_eq!(log.def_user_cap, Some(3_840_755_982_336));
        assert_eq!(log.cust_eol_sts, Some(8));
        assert_eq!(log.vca_tpc_fss_seq, Some(45));
        // Identifiers illumos doesn't name are kept, and read as text.
        let ids: Vec<u32> = log.unknown.iter().map(|e| e.id).collect();
        assert_eq!(ids, [0x24, 0x25, 0x26, 0x27]);
        for e in &log.unknown {
            assert_eq!(e.as_text().as_deref(), Some("CUSTOMER_ID_GENERIC"));
        }
    }

    #[test]
    fn last_value_wins() {
        // Log header plus two assert_dump entries, 0 then 1.
        let mut buf = Vec::new();
        for word in [32u32, 0, 12, 0x19, 0, 12, 0x19, 1] {
            buf.extend_from_slice(&word.to_le_bytes());
        }
        assert_eq!(DevMgmtLog::parse(&buf).unwrap().assert_dump, Some(1));
    }

    #[test]
    fn as_text_is_strict() {
        let entry =
            |data: &[u8]| UnknownEntry { id: 0x24, data: data.to_vec() };
        assert_eq!(entry(b"\x02\0\0\0hi\0\0").as_text().as_deref(), Some("hi"));
        // A bare u32 value.
        assert_eq!(entry(&7u32.to_le_bytes()).as_text(), None);
        // Zero length.
        assert_eq!(entry(b"\0\0\0\0\0\0\0\0").as_text(), None);
        // Length past the end.
        assert_eq!(entry(b"\x09\0\0\0hi\0\0").as_text(), None);
        // Non-zero padding.
        assert_eq!(entry(b"\x02\0\0\0hi\0x").as_text(), None);
        // Too much padding.
        assert_eq!(entry(b"\x02\0\0\0hi\0\0\0\0").as_text(), None);
        // Non-printable text.
        assert_eq!(entry(b"\x02\0\0\0h\x01\0\0").as_text(), None);
    }

    #[test]
    fn rejects_malformed() {
        // Too short for the header.
        assert!(DevMgmtLog::parse(&NVME4[..4]).is_err());
        // Header claims more data than was read.
        assert!(DevMgmtLog::parse(&NVME4[..100]).is_err());
        // An entry length shorter than its header must not loop forever.
        let mut bad = NVME4.to_vec();
        bad[8..12].copy_from_slice(&4u32.to_le_bytes());
        assert!(DevMgmtLog::parse(&bad).is_err());
        // An entry length that runs past the end of the log.
        let mut bad = NVME4.to_vec();
        bad[8..12].copy_from_slice(&0xffff_fff0u32.to_le_bytes());
        assert!(DevMgmtLog::parse(&bad).is_err());
    }
}
