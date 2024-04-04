// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::CString;

use libnvme_sys::nvme::*;

#[derive(Debug, Clone)]
pub enum FirmwareSlot {
    Empty,
    Revision(String),
}

pub struct FirmwareLogpage {
    pub active_slot: u8,
    pub next_active_slot: Option<u8>,
    pub firmware_slots: Vec<FirmwareSlot>,
}

impl FirmwareLogpage {
    pub(crate) fn init_from_raw(logpage: nvme_fwslot_log_t) -> Self {
        let next_active_slot = match logpage.bitfield1.fw_next() {
            0 => None,
            slot => Some(slot),
        };

        let mut firmware_slots = Vec::new();
        for slot in logpage.fw_frs {
            // XXX is there a more efficent way?
            let bytes: Vec<u8> = slot.iter().map(|&x| x as u8).collect();
            let cstring = unsafe { CString::from_vec_unchecked(bytes) };

            let firmware = cstring.to_string_lossy().trim().to_string();
            if firmware.is_empty() {
                firmware_slots.push(FirmwareSlot::Empty);
            } else {
                firmware_slots.push(FirmwareSlot::Revision(firmware));
            }
        }

        Self {
            active_slot: logpage.bitfield1.fw_afi(),
            next_active_slot,
            firmware_slots,
        }
    }
}

pub(crate) fn get_logpage_size(
    disc: *mut nvme_log_disc_t,
    _req: *mut nvme_log_req_t,
) -> usize {
    let mut len = 0;
    match unsafe { nvme_log_disc_size(disc, &mut len) } {
        NVME_LOG_SIZE_K_FIXED => len as usize,
        _ => todo!("variable length log page found"),
    }
}
