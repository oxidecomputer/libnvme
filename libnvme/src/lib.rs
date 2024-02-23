// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![deny(elided_lifetimes_in_paths)]

use std::ffi::CStr;

use error::{InternalError, LibraryError};
use strum_macros::FromRepr;
use thiserror::Error;

pub mod controller;
pub mod controller_info;
mod error;
mod lba;
pub mod namespace;
mod util;
mod wdc;

use libnvme_sys::nvme::*;

use crate::controller::ControllerDiscovery;

#[derive(Debug, Error)]
#[error("{error}")]
pub struct NvmeError {
    code: NvmeErrorCode,
    error: InternalError,
}

impl NvmeError {
    pub fn code(&self) -> NvmeErrorCode {
        self.code
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum NvmeErrorCode {
    Ok,
    Controller,
    NoMem,
    NoDmaMem,
    Libdevinfo,
    Internal,
    BadPtr,
    BadFlag,
    BadDevi,
    BadDeviProp,
    IllegalInstance,
    BadController,
    Privs,
    OpenDev,
    BadRestore,
    NsRange,
    NsUnuse,
    LogCsiRange,
    LogLidRange,
    LogLspRange,
    LogLsiRange,
    LogRaeRange,
    LogSizeRange,
    LogOffsetRange,
    LogCsiUnsup,
    LogLspUnsup,
    LogLsiUnsup,
    LogRaeUnsup,
    LogOffsetUnsup,
    LogLspUnuse,
    LogLsiUnuse,
    LogRaeUnuse,
    LogScopeMismatch,
    LogReqMissingFields,
    LogNameUnknown,
    LogUnsupByDev,
    IdentifyUnknown,
    IdentifyUnsupByDev,
    IdentifyCtrlidRange,
    IdentifyOutputRange,
    IdentifyCtrlidUnsup,
    IdentifyCtrlidUnuse,
    IdentifyReqMissingFields,
    VucUnsupByDev,
    VucTimeoutRange,
    VucOpcodeRange,
    VucImpactRange,
    VucNdtRange,
    VucCannotRw,
    VucNoResults,
    VucUnknown,
    VucReqMissingFields,
    VuFuncUnsupByDev,
    WdcE6OffsetRange,
    FwUnsupByDev,
    KernFwImpos,
    FwLoadLenRange,
    FwLoadOffsetRange,
    FwCommitSlotRange,
    FwCommitActionRange,
    FwCommitReqMissingFields,
    FwSlotRo,
    FormatUnsupByDev,
    CryptoSeUnsupByDev,
    NsFormatUnsupByDev,
    KernFormatUnsup,
    FormatLbafRange,
    FormatSesRange,
    FormatParamUnsup,
    FormatReqMissingFields,
    WdcE6ReqMissingFields,
    FeatNameUnknown,
    FeatUnsupByDev,
    FeatFidRange,
    FeatSelRange,
    FeatCdw11Range,
    FeatDataRange,
    FeatSelUnsup,
    FeatCdw11Unuse,
    FeatDataUnuse,
    FeatNoResults,
    GetFeatReqMissingFields,
    NeedCtrlWrlock,
    NeedNsWrlock,
    CtrlLocked,
    NsLocked,
    LockProg,
    LockOrder,
    LockWaitIntr,
    LockWouldBlock,
    DetachKern,
    AttachKern,
    AttachUnsupKern,
    NsBlkdevAttach,
    NoKernMem,
    CtrlDead,
    CtrlGone,
    // The following is a catchall if we fail to translate the error code,
    // therefore we want to make sure it's not a valid option when calling
    // from_repr()
    #[strum(disabled)]
    Unknown(u32),
}

impl NvmeErrorCode {
    fn from_raw(raw: u32) -> Self {
        NvmeErrorCode::from_repr(raw as usize)
            .unwrap_or(NvmeErrorCode::Unknown(raw))
    }
}

#[derive(Debug, Error)]
#[error("Failed to initialize nvme handle")]
pub struct NvmeInitError;

#[derive(Debug)]
pub struct Nvme(*mut nvme_t);

impl Drop for Nvme {
    fn drop(&mut self) {
        unsafe { nvme_fini(self.0) }
    }
}

impl Nvme {
    pub fn new() -> Result<Self, NvmeInitError> {
        let ptr = unsafe { nvme_init() };
        if ptr.is_null() {
            return Err(NvmeInitError);
        }
        Ok(Self(ptr))
    }

    pub fn controller_discovery(
        &self,
    ) -> Result<ControllerDiscovery<'_>, NvmeError> {
        ControllerDiscovery::new(self)
    }
}

impl LibraryError for Nvme {
    type Error = NvmeError;

    fn get_errmsg(&self) -> String {
        let errmsg = unsafe { nvme_errmsg(self.0) };
        unsafe { CStr::from_ptr(errmsg) }.to_string_lossy().to_string()
    }

    fn get_syserr(&self) -> i32 {
        unsafe { nvme_syserr(self.0) }
    }

    fn current_error(&self, internal: error::InternalError) -> Self::Error {
        let code = NvmeErrorCode::from_raw(unsafe { nvme_err(self.0) });
        assert_ne!(
            code,
            NvmeErrorCode::Ok,
            "attempted to get current_error for a successful response"
        );
        NvmeError { code, error: internal }
    }
}
