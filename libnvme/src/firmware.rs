// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::slice;
use std::io::Read;
use std::mem::{self, MaybeUninit};

use libnvme_sys::nvme::*;
use thiserror::Error;

use crate::{
    controller::{Controller, NvmeControllerError, WriteLockedController},
    controller_info::ControllerInfoIdentify,
    error::LibraryError,
    logpage::{LogPageInfo, LogPageName},
    NvmeError,
};

#[derive(Debug, Error)]
pub enum NvmeSlotError {
    #[error("NVMe slots must be between 1 and 7 but got {0}")]
    InvalidSlotNumber(u8),
    #[error("NVMe device does not have slot {0}")]
    DoesNotExisit(u8),
    #[error("libnvme error: {0}")]
    NvmeError(#[from] NvmeError),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct NvmeSlot(u8);

// Setting a slot via `nvme_fw_commit_req_set_slot` uses a `u32`.
impl TryFrom<u8> for NvmeSlot {
    type Error = NvmeSlotError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1..=7 => Ok(NvmeSlot(value)),
            invalid => Err(NvmeSlotError::InvalidSlotNumber(invalid)),
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
    pub number_of_slots: u8,
    firmware_slot_versions: Vec<Option<String>>,
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
            slot => Some(slot),
        };

        let number_of_slots = unsafe { (*identify.inner).id_frmw.fw_nslot() };
        let nslots = usize::from(number_of_slots);
        let mut firmware_slots = Vec::with_capacity(nslots);
        for slot in &logpage.fw_frs[..nslots] {
            // The version strings are packed into an array without a nul
            // byte unless the slot itself is empty so we can't depend on
            // `CStr::from_ptr()`. Instead we have to take each slice of bytes
            // and check if the first byte is nul along the way.
            let u8slot = unsafe {
                slice::from_raw_parts(slot.as_ptr().cast::<u8>(), slot.len())
            };
            if u8slot[0] == b'\0' {
                firmware_slots.push(None);
                continue;
            }
            // NVMe Spec: "The firmware revision is indicated as an ASCII
            // string."
            let firmware = String::from_utf8_lossy(u8slot).trim().to_string();
            firmware_slots.push(Some(firmware));
        }

        Self {
            active_slot,
            next_active_slot,
            slot1_is_read_only,
            number_of_slots,
            firmware_slot_versions: firmware_slots,
        }
    }

    /// Get the firmware version for a particular slot.
    ///
    /// Note that the NVMe spec allows for slots 1 - 7.
    pub fn get_slot_version(
        &self,
        slot: NvmeSlot,
    ) -> Result<Option<&str>, NvmeSlotError> {
        // We subtract 1 because our internal mapping is 0 indexed.
        match self.firmware_slot_versions.get(usize::from(slot.0) - 1) {
            Some(slot) => Ok(slot.as_deref()),
            None => Err(NvmeSlotError::DoesNotExisit(slot.0)),
        }
    }

    /// Returns an iterator over the firmware slots.
    ///
    /// The iterator yields a `Some(String)` if the slot has a firmware version
    /// commited otherwise it yeilds `None`.
    pub fn slot_iter(&self) -> ControllerFirmwareSlotIter<'_> {
        ControllerFirmwareSlotIter { iter: self.firmware_slot_versions.iter() }
    }
}

pub struct ControllerFirmwareSlotIter<'a> {
    iter: <&'a Vec<Option<String>> as IntoIterator>::IntoIter,
}

impl<'a> Iterator for ControllerFirmwareSlotIter<'a> {
    type Item = Option<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|o| o.as_deref())
    }
}

#[derive(Debug, Error)]
pub enum FirmwareLogPageError {
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
    /// Get the controller's firmware log page.
    pub fn get_firmware_log_page(
        &self,
    ) -> Result<FirmwareLogPage, FirmwareLogPageError> {
        let expected_size = mem::size_of::<nvme_fwslot_log_t>();
        let LogPageInfo { size, req, .. } =
            self.get_logpage(LogPageName::Firmware)?;
        if size != expected_size {
            return Err(FirmwareLogPageError::UnexpectedSize {
                size,
                expected_size,
            });
        }
        let mut buf = MaybeUninit::<nvme_fwslot_log_t>::zeroed();
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

#[derive(Error, Debug)]
pub enum FirmwareLoadError {
    #[error("{0}")]
    Nvme(#[from] NvmeError),
    #[error("{0}")]
    NvmeController(#[from] NvmeControllerError),
    #[error("Failed to upload firmware: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Supplied firmware is too large")]
    FirmwareImageTooLarge,
}

impl<'ctrl> WriteLockedController<'ctrl> {
    /// Load a single chunk of firmware at offset
    fn firmware_load_chunk(
        &self,
        data: &[u8],
        offset: u64,
    ) -> Result<(), NvmeControllerError> {
        self.check_result(
            unsafe {
                nvme_fw_load(
                    self.inner,
                    data.as_ptr().cast(),
                    data.len(),
                    offset,
                )
            },
            || "failed to load firmware",
        )
    }

    /// Upload new firmware to the NVMe controller.
    ///
    /// Note this firmware needs to be commited to a slot via
    /// `FirmwareCommitRequestBuilder`.
    pub fn firmware_load<R: Read>(
        &self,
        mut data: R,
    ) -> Result<(), FirmwareLoadError> {
        // TODO swap to the libnvme granularity function Andy is adding
        const CHUNK_SIZE: u64 = 0x1000;

        let size = CHUNK_SIZE.try_into().expect("32-bit systems unsupported");
        let mut offset = 0u64;
        let mut buf = Vec::new();

        // Firmware blobs tend to be a few MB in size. For simplicity we are
        // going to read everything passed to us via the reader into a Vec so
        // that we split it into proper chunk sizes.
        data.read_to_end(&mut buf)?;
        let mut chunks = buf.chunks_exact(size);
        for chunk in &mut chunks {
            self.firmware_load_chunk(chunk, offset)?;
            // Safety: we expect libnvme to not allow us to upload a binary blob
            // this large, but let's catch it if it does happen.
            offset = offset
                .checked_add(CHUNK_SIZE)
                .ok_or(FirmwareLoadError::FirmwareImageTooLarge)?;
        }
        let remainder = chunks.remainder();
        if !remainder.is_empty() {
            // Take the remainder and pad it with zeros.
            let mut chunk = remainder.to_vec();
            chunk.resize(size, 0);
            self.firmware_load_chunk(&chunk, offset)?;
        }

        Ok(())
    }

    /// Returns a new `FirmwareCommitRequestBuilder` that can be used to commit
    /// uploaded firmware to a particular NVMe slot.
    pub fn firmware_commit_request(
        &self,
    ) -> Result<FirmwareCommitRequestBuilder<'_>, NvmeControllerError> {
        let mut req = std::ptr::null_mut();
        self.check_result(
            unsafe { nvme_fw_commit_req_init(self.inner, &mut req) },
            || "failed to create firmware commit request",
        )?;

        Ok(FirmwareCommitRequestBuilder { req, controller: self })
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum FirmwareCommitAction {
    ///  Save image only.
    Save = NVME_FWC_SAVE,
    /// Save and activate at next reset.
    SaveActivate = NVME_FWC_SAVE_ACTIVATE,
    /// Activate slot at next reset.
    Activate = NVME_FWC_ACTIVATE,
    /// Activate slot immediately.
    ///
    /// Note: illumos does not support this today.
    ActivateImmediately = NVME_FWC_ACTIVATE_IMMED,
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
    /// Set the NVMe slot the firmware is going to be commited to.
    pub fn set_slot(self, slot: NvmeSlot) -> Result<Self, NvmeControllerError> {
        self.controller
            .check_result(
                unsafe {
                    nvme_fw_commit_req_set_slot(self.req, u32::from(slot.0))
                },
                || {
                    format!(
                        "failed to set firmware commit request slot to {}",
                        slot.0
                    )
                },
            )
            .map(|_| self)
    }

    /// Set the commit action.
    pub fn set_action(
        self,
        action: FirmwareCommitAction,
    ) -> Result<Self, NvmeControllerError> {
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

    /// Execute a firmware commit request.
    pub fn execute(self) -> Result<(), NvmeControllerError> {
        self.controller
            .check_result(unsafe { nvme_fw_commit_req_exec(self.req) }, || {
                "failed to execute firmware commit request"
            })
    }
}
