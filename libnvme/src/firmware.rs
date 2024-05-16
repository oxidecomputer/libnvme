use std::{ffi::CString, mem::MaybeUninit};

use libnvme_sys::nvme::*;
use thiserror::Error;

use crate::{
    controller::{Controller, WriteLockedController},
    controller_info::ControllerInfoIdentify,
    error::LibraryError,
    logpage::{LogPageInfo, LogPageName},
    NvmeError,
};

#[derive(Debug, Error)]
pub enum NvmeSlotError {
    #[error("NVMe slots must be between 1 and 7 but got {0}")]
    InvalidSlotNumber(u32),
    #[error("NVMe device does not have slot {0}")]
    DoesNotExisit(u32),
    #[error("libnvme error: {0}")]
    NvmeError(#[from] NvmeError),
}

// NOTE: we represent this is a u32 because `nvme_fw_commit_req_set_slot` takes
// a uint32_t as the spec may change in the future to add additional slots.
/// NVMe Slot Number.
pub struct NvmeSlot(u32);

// Rust's default interger type is `i32`.
impl TryFrom<i32> for NvmeSlot {
    type Error = NvmeSlotError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1..=7 => Ok(NvmeSlot(value as u32)),
            // XXX maybe we should map the error to something not lossy?
            invalid => Err(NvmeSlotError::InvalidSlotNumber(invalid as u32)),
        }
    }
}

// Setting a slot via `nvme_fw_commit_req_set_slot` uses a `u32`.
impl TryFrom<u32> for NvmeSlot {
    type Error = NvmeSlotError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1..=7 => Ok(NvmeSlot(value)),
            invalid => Err(NvmeSlotError::InvalidSlotNumber(invalid)),
        }
    }
}

// The NVMe spec currently uses a u8 for active and next active slots.
impl TryFrom<u8> for NvmeSlot {
    type Error = NvmeSlotError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1..=7 => Ok(NvmeSlot(value as _)),
            invalid => Err(NvmeSlotError::InvalidSlotNumber(invalid as _)),
        }
    }
}

#[derive(Debug)]
/// The Firmwware log page for an NVMe controller.
pub struct FirmwareLogPage {
    /// The currently active firmware slot.
    pub active_slot: u8,
    /// The next slot that will be active on a controller reset.
    pub next_active_slot: Option<u8>,
    /// Slot 1 is read-only.
    pub slot1_is_read_only: bool,
    /// The number of firmware slots the controller has.
    pub number_of_slots: usize,
    firmware_slots: Vec<Option<String>>,
}

impl FirmwareLogPage {
    fn init(
        identify: &ControllerInfoIdentify<'_>,
        logpage: nvme_fwslot_log_t,
    ) -> Self {
        let active_slot = logpage.bitfield1.fw_afi();
        let slot1_is_read_only =
            unsafe { (*identify.inner).id_frmw.fw_readonly() };

        // NVMe Spec: "If this field is 0h, then the controller does not
        // indicate the firmware slot that is going to be activated at the next
        // Controller Level Reset."
        let next_active_slot = match logpage.bitfield1.fw_next() {
            0 => None,
            slot => Some(slot.saturating_sub(1)),
        };

        let number_of_slots =
            unsafe { (*identify.inner).id_frmw.fw_nslot() } as usize;
        let mut firmware_slots = Vec::with_capacity(number_of_slots);
        for slot in &logpage.fw_frs[..number_of_slots] {
            // XXX there's probably a better way to do this? Basically the
            // strings are packed into an array without a nul byte unless the
            // slot itself is empty so we can't depend on `CStr::from_ptr()`.
            // Instead we have to take each slice of bytes and convert it into a
            // CString checking if the first byte is nul along the way.
            let bytes: Vec<u8> = slot.iter().map(|&x| x as u8).collect();
            if bytes[0] == b'\0' {
                firmware_slots.push(None);
                continue;
            }
            let cstring = unsafe { CString::from_vec_unchecked(bytes) };
            // NVMe Spec: "The firmware revision is indicated as an ASCII
            // string."
            let firmware = cstring.to_string_lossy().trim().to_string();
            firmware_slots.push(Some(firmware));
        }

        Self {
            active_slot,
            next_active_slot,
            slot1_is_read_only,
            number_of_slots,
            firmware_slots,
        }
    }

    /// Get the firmware version for a particular slot.
    ///
    /// Note that the NVMe spec allows for slots 1 - 7.
    pub fn get_slot<S>(&self, slot: S) -> Result<Option<&String>, NvmeSlotError>
    where
        S: TryInto<NvmeSlot>,
        S::Error: Into<NvmeSlotError>,
    {
        let slot = slot.try_into().map_err(Into::into)?;
        // We subtract 1 because our internal mapping is 0 indexed.
        match self.firmware_slots.get(slot.0 as usize - 1) {
            Some(slot) => Ok(slot.as_ref()),
            None => Err(NvmeSlotError::DoesNotExisit(slot.0)),
        }
    }

    /// Returns an iterator over the firmware slots.
    ///
    /// The iterator yields a `Some(String)` if the slot has a firmware version
    /// commited otherwise it yeilds `None`.
    pub fn slot_iter(&self) -> impl Iterator<Item = Option<&String>> {
        self.firmware_slots.iter().map(|o| o.as_ref())
    }
}

impl<'a> Controller<'a> {
    pub fn get_firmware_log_page(&self) -> Result<FirmwareLogPage, NvmeError> {
        let LogPageInfo { size, req, .. } =
            self.get_logpage(LogPageName::Firmware)?;
        // TODO the size of nvme_fwslot_log_t should always match the size
        // returned from get_logpage. We may want a custom error type here if
        // that's not true. Although for now libnvme should error if we give it
        // something too small.
        let mut buf = MaybeUninit::<nvme_fwslot_log_t>::zeroed();
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
    pub fn set_slot<S>(self, slot: S) -> Result<Self, NvmeSlotError>
    where
        S: TryInto<NvmeSlot>,
        S::Error: Into<NvmeSlotError>,
    {
        let slot = slot.try_into().map_err(|e| e.into())?;
        self.controller
            .check_result(
                unsafe { nvme_fw_commit_req_set_slot(self.req, slot.0) },
                || {
                    format!(
                        "failed to set firmware commit request slot to {}",
                        slot.0
                    )
                },
            )
            .map_err(|e| e.into())
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
