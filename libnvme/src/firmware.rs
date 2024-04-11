use std::{ffi::CString, mem::MaybeUninit};

use libnvme_sys::nvme::*;

use crate::{
    controller::{Controller, WriteLockedController},
    controller_info::ControllerInfoIdentify,
    error::LibraryError,
    logpage::{LogPageInfo, LogPageName},
    NvmeError,
};

#[derive(Debug, Clone)]
pub enum FirmwareSlot {
    Empty,
    Occupied(String),
    OccupiedReadOnly(String),
    NonExistent,
}

#[derive(Debug)]
pub struct FirmwareLogPage {
    /// The currently active firmware slot.
    pub active_slot: u8,
    /// The next slot that will be active on a controller reset.
    pub next_active_slot: Option<u8>,
    pub firmware_slots: [FirmwareSlot; 7],
}

impl FirmwareLogPage {
    fn init(
        identify: &ControllerInfoIdentify<'_>,
        logpage: nvme_fwslot_log_t,
    ) -> Self {
        let num_slots =
            unsafe { (*identify.inner).id_frmw.fw_nslot() } as usize;
        let readonly = unsafe { (*identify.inner).id_frmw.fw_readonly() };
        let next_active_slot = match logpage.bitfield1.fw_next() {
            0 => None,
            slot => Some(slot),
        };

        const INITIAL_FIRMWARE_SLOT_VALUE: FirmwareSlot =
            FirmwareSlot::NonExistent;
        let mut firmware_slots = [INITIAL_FIRMWARE_SLOT_VALUE; 7];

        for (index, slot) in logpage.fw_frs[..num_slots].iter().enumerate() {
            // XXX there's probably a better way to do this? Basically the
            // strings are packed into an array without a nul byte unless the
            // slot itself is empty so we can't depend on `CStr::from_ptr()`.
            // Instead we have to take each slice of bytes and convert it into a
            // CString checking if the first byte is nul along the way.
            let bytes: Vec<u8> = slot.iter().map(|&x| x as u8).collect();
            if bytes[0] == b'\0' {
                firmware_slots[index] = FirmwareSlot::Empty;
                continue;
            }
            let cstring = unsafe { CString::from_vec_unchecked(bytes) };
            let firmware = cstring.to_string_lossy().trim().to_string();

            if index == 0 && readonly {
                firmware_slots[index] =
                    FirmwareSlot::OccupiedReadOnly(firmware);
            } else {
                firmware_slots[index] = FirmwareSlot::Occupied(firmware);
            }
        }

        Self {
            active_slot: logpage.bitfield1.fw_afi(),
            next_active_slot,
            firmware_slots,
        }
    }
}

impl<'a> Controller<'a> {
    pub fn get_firmware(&self) -> Result<FirmwareLogPage, NvmeError> {
        let LogPageInfo { size, req, .. } =
            self.get_logpage(LogPageName::Firmware)?;
        assert_eq!(
            std::mem::size_of::<nvme_fwslot_log_t>(),
            size,
            "the requested log page is the same size as nvme_fwslot_log_t"
        );
        let mut buf = MaybeUninit::<nvme_fwslot_log_t>::uninit();
        self.check_result(
            unsafe {
                nvme_log_req_set_output(req.inner, buf.as_mut_ptr() as _, size)
            },
            || format!("failed to set logpage req size to {size}"),
        )?;

        self.check_result(unsafe { nvme_log_req_exec(req.inner) }, || {
            "failed to execute firmware log request"
        })?;

        // Our request was successful so we can assume this struct has been
        // populated now.
        let logpage = unsafe { buf.assume_init() };
        let controller_info = self.get_info()?;
        let identify = controller_info.get_controller_info_identify();
        Ok(FirmwareLogPage::init(&identify, logpage))
    }
}

impl<'ctrl> WriteLockedController<'ctrl> {
    pub fn firmware_load(&self, data: &[u8]) -> Result<(), NvmeError> {
        self.check_result(
            unsafe {
                nvme_fw_load(
                    self.inner,
                    data.as_ptr() as *const _,
                    data.len(),
                    // XXX do we need to provide the offset?
                    0,
                )
            },
            || "failed to load firmware",
        )
    }

    pub fn firmware_commit_request(
        &self,
    ) -> Result<FirmwareCommitRequestBuilder<'_>, NvmeError> {
        let mut req = std::ptr::null_mut();
        self.check_result(
            unsafe { nvme_fw_commit_req_init(self.inner, &mut req) },
            || "failed to create firmware commit request",
        )?;

        Ok(FirmwareCommitRequestBuilder { req, controller: self })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FirmwareCommitAction {
    ///  Save image only.
    Save,
    /// Save and activate at next reset.
    SaveActivate,
    /// Activate slot at next reset.
    Activate,
    /// Activate slot immediately.
    ActivateImmediately,
}

impl From<FirmwareCommitAction> for u32 {
    fn from(value: FirmwareCommitAction) -> Self {
        match value {
            FirmwareCommitAction::Save => NVME_FWC_SAVE,
            FirmwareCommitAction::SaveActivate => NVME_FWC_SAVE_ACTIVATE,
            FirmwareCommitAction::Activate => NVME_FWC_ACTIVATE,
            FirmwareCommitAction::ActivateImmediately => {
                NVME_FWC_ACTIVATE_IMMED
            }
        }
    }
}

pub struct FirmwareCommitRequestBuilder<'ctrl> {
    req: *mut nvme_fw_commit_req_t,
    controller: &'ctrl WriteLockedController<'ctrl>,
}

impl<'ctrl> Drop for FirmwareCommitRequestBuilder<'ctrl> {
    fn drop(&mut self) {
        unsafe { nvme_fw_commit_req_fini(self.req) }
    }
}

impl<'ctrl> FirmwareCommitRequestBuilder<'ctrl> {
    pub fn set_slot(self, slot: u32) -> Result<Self, NvmeError> {
        self.controller
            .check_result(
                unsafe { nvme_fw_commit_req_set_slot(self.req, slot) },
                || {
                    format!(
                        "failed to set firmware commit request slot to {slot}"
                    )
                },
            )
            .map(|_| self)
    }

    pub fn set_action(
        self,
        action: FirmwareCommitAction,
    ) -> Result<Self, NvmeError> {
        self.controller
            .check_result(
                unsafe {
                    nvme_fw_commit_req_set_action(self.req, action as u32)
                },
                || {
                    format!(
                    "failed to set firmware commit request action to {action:?}"
                )
                },
            )
            .map(|_| self)
    }

    pub fn execute(self) -> Result<(), NvmeError> {
        self.controller
            .check_result(unsafe { nvme_fw_commit_req_exec(self.req) }, || {
                "failed to execute firmware commit request"
            })
    }
}
