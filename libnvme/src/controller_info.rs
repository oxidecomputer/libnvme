// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{borrow::Cow, ffi::CStr, marker::PhantomData};

use libnvme_sys::nvme::*;
use thiserror::Error;

use crate::{
    controller::Controller,
    error::{InternalError, LibraryError},
    lba::LbaFormat,
    util::FfiPtr,
};

#[derive(Debug, Error)]
pub enum NvmeInfoError {
    #[error(transparent)]
    Ok(InternalError),
    #[error(transparent)]
    Transport(InternalError),
    #[error(transparent)]
    Version(InternalError),
    #[error(transparent)]
    MissingCap(InternalError),
    #[error(transparent)]
    BadLbaFmt(InternalError),
    #[error(transparent)]
    PersistNvl(InternalError),
    #[error(transparent)]
    BadFmt(InternalError),
    #[error(transparent)]
    BadFmtData(InternalError),
    #[error(transparent)]
    NsInactive(InternalError),
    #[error(transparent)]
    NsNoBlkdev(InternalError),
}

impl NvmeInfoError {
    pub(crate) fn from_raw_with_internal_error(
        raw: u32,
        internal: InternalError,
    ) -> Self {
        match raw {
            NVME_INFO_ERR_OK => NvmeInfoError::Ok(internal),
            NVME_INFO_ERR_TRANSPORT => NvmeInfoError::Transport(internal),
            NVME_INFO_ERR_VERSION => NvmeInfoError::Version(internal),
            NVME_INFO_ERR_MISSING_CAP => NvmeInfoError::MissingCap(internal),
            NVME_INFO_ERR_BAD_LBA_FMT => NvmeInfoError::BadLbaFmt(internal),
            NVME_INFO_ERR_PERSIST_NVL => NvmeInfoError::PersistNvl(internal),
            NVME_INFO_ERR_BAD_FMT => NvmeInfoError::BadFmt(internal),
            NVME_INFO_ERR_BAD_FMT_DATA => NvmeInfoError::BadFmtData(internal),
            NVME_INFO_ERR_NS_INACTIVE => NvmeInfoError::NsInactive(internal),
            NVME_INFO_ERR_NS_NO_BLKDEV => NvmeInfoError::NsNoBlkdev(internal),
            // TODO map this to an error type so we don't crash someones program
            _ => unreachable!("Unknown Error"),
        }
    }
}
pub struct ControllerInfo<'ctrl> {
    ctrl_info: *mut nvme_ctrl_info_t,
    _phantom: PhantomData<&'ctrl Controller<'ctrl>>,
}

impl<'ctrl> Drop for ControllerInfo<'ctrl> {
    fn drop(&mut self) {
        unsafe { nvme_ctrl_info_free(self.ctrl_info) }
    }
}

impl<'ctrl> FfiPtr for ControllerInfo<'ctrl> {
    type Ptr = *mut nvme_ctrl_info_t;

    unsafe fn from_raw(ptr: Self::Ptr) -> Self {
        Self { ctrl_info: ptr, _phantom: PhantomData }
    }
}

impl<'ctrl> ControllerInfo<'ctrl> {
    pub fn model(&self) -> Cow<'_, str> {
        unsafe {
            CStr::from_ptr(nvme_ctrl_info_model(self.ctrl_info))
                .to_string_lossy()
        }
    }

    pub fn serial(&self) -> Cow<'_, str> {
        unsafe {
            CStr::from_ptr(nvme_ctrl_info_serial(self.ctrl_info))
                .to_string_lossy()
        }
    }

    pub fn fwrev(&self) -> Cow<'_, str> {
        unsafe {
            CStr::from_ptr(nvme_ctrl_info_fwrev(self.ctrl_info))
                .to_string_lossy()
        }
    }

    pub fn number_of_ns(&self) -> u32 {
        unsafe { nvme_ctrl_info_nns(self.ctrl_info) }
    }

    pub fn pci_vid(&self) -> Result<u16, NvmeInfoError> {
        let mut vid = 0;
        match unsafe { nvme_ctrl_info_pci_vid(self.ctrl_info, &mut vid) } {
            true => Ok(vid),
            false => Err(self.fatal_context("failed to get pci vid")),
        }
    }

    fn nformats(&self) -> u32 {
        unsafe { nvme_ctrl_info_nformats(self.ctrl_info) }
    }

    fn nvm_lba_fmt(&self, index: u32) -> Result<LbaFormat<'_>, NvmeInfoError> {
        let mut lba: *const nvme_nvm_lba_fmt_t = std::ptr::null_mut();
        if !unsafe { nvme_ctrl_info_format(self.ctrl_info, index, &mut lba) } {
            return Err(self.fatal_context(format!(
                "failed to get lba fmt for index {index}"
            )));
        }
        Ok(unsafe { LbaFormat::from_raw(lba) })
    }

    pub fn lba_formats(&self) -> Vec<Result<LbaFormat<'_>, NvmeInfoError>> {
        (0..self.nformats()).map(|i| self.nvm_lba_fmt(i)).collect()
    }
}

impl<'ctrl> LibraryError for ControllerInfo<'ctrl> {
    type Error = NvmeInfoError;

    fn get_errmsg(&self) -> String {
        let errmsg = unsafe { nvme_ctrl_info_errmsg(self.ctrl_info) };
        unsafe { CStr::from_ptr(errmsg) }.to_string_lossy().to_string()
    }

    fn get_syserr(&self) -> i32 {
        unsafe { nvme_ctrl_info_syserr(self.ctrl_info) }
    }

    fn to_error(&self, internal: InternalError) -> Self::Error {
        NvmeInfoError::from_raw_with_internal_error(
            unsafe { nvme_ctrl_info_err(self.ctrl_info) },
            internal,
        )
    }
}
