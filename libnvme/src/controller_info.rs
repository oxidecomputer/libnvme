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
#[error("{error}")]
pub struct NvmeInfoError {
    code: NvmeInfoErrorCode,
    error: InternalError,
}

impl NvmeInfoError {
    pub(crate) fn from_code_and_error(
        code: NvmeInfoErrorCode,
        error: InternalError,
    ) -> Self {
        Self { code, error }
    }
    pub fn code(&self) -> NvmeInfoErrorCode {
        self.code
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvmeInfoErrorCode {
    Ok,
    Transport,
    Version,
    MissingCap,
    BadLbaFmt,
    PersistNvl,
    BadFmt,
    BadFmtData,
    NsInactive,
    NsNoBlkdev,
    Unknown(u32),
}

impl NvmeInfoErrorCode {
    pub(crate) fn from_raw(raw: u32) -> Self {
        match raw {
            NVME_INFO_ERR_OK => NvmeInfoErrorCode::Ok,
            NVME_INFO_ERR_TRANSPORT => NvmeInfoErrorCode::Transport,
            NVME_INFO_ERR_VERSION => NvmeInfoErrorCode::Version,
            NVME_INFO_ERR_MISSING_CAP => NvmeInfoErrorCode::MissingCap,
            NVME_INFO_ERR_BAD_LBA_FMT => NvmeInfoErrorCode::BadLbaFmt,
            NVME_INFO_ERR_PERSIST_NVL => NvmeInfoErrorCode::PersistNvl,
            NVME_INFO_ERR_BAD_FMT => NvmeInfoErrorCode::BadFmt,
            NVME_INFO_ERR_BAD_FMT_DATA => NvmeInfoErrorCode::BadFmtData,
            NVME_INFO_ERR_NS_INACTIVE => NvmeInfoErrorCode::NsInactive,
            NVME_INFO_ERR_NS_NO_BLKDEV => NvmeInfoErrorCode::NsNoBlkdev,
            code => NvmeInfoErrorCode::Unknown(code),
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
        self.check_result(
            unsafe { nvme_ctrl_info_pci_vid(self.ctrl_info, &mut vid) },
            || "failed to get pci vid",
        )
        .map(|_| vid)
    }

    fn nformats(&self) -> u32 {
        unsafe { nvme_ctrl_info_nformats(self.ctrl_info) }
    }

    fn nvm_lba_fmt(&self, index: u32) -> Result<LbaFormat<'_>, NvmeInfoError> {
        let mut lba: *const nvme_nvm_lba_fmt_t = std::ptr::null_mut();
        self.check_result(
            unsafe { nvme_ctrl_info_format(self.ctrl_info, index, &mut lba) },
            || format!("failed to get lba fmt for index {index}"),
        )
        .map(|_| unsafe { LbaFormat::from_raw(lba) })
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

    fn current_error(&self, internal: InternalError) -> Self::Error {
        let raw = unsafe { nvme_ctrl_info_err(self.ctrl_info) };
        NvmeInfoError {
            code: NvmeInfoErrorCode::from_raw(raw),
            error: internal,
        }
    }
}
