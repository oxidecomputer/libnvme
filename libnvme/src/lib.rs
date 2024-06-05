// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![deny(elided_lifetimes_in_paths)]

use std::ffi::CStr;

use error::{InternalError, LibraryError};
use thiserror::Error;

pub mod controller;
pub mod controller_info;
mod error;
pub mod firmware;
mod lba;
mod logpage;
pub mod namespace;
mod util;
mod wdc;

use libnvme_sys::nvme::*;

pub use ::nvme as nvmespec;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Unknown(u32),
}

impl NvmeErrorCode {
    fn from_raw(raw: u32) -> Self {
        match raw {
            NVME_ERR_OK => NvmeErrorCode::Ok,
            NVME_ERR_CONTROLLER => NvmeErrorCode::Controller,
            NVME_ERR_NO_MEM => NvmeErrorCode::NoMem,
            NVME_ERR_NO_DMA_MEM => NvmeErrorCode::NoDmaMem,
            NVME_ERR_LIBDEVINFO => NvmeErrorCode::Libdevinfo,
            NVME_ERR_INTERNAL => NvmeErrorCode::Internal,
            NVME_ERR_BAD_PTR => NvmeErrorCode::BadPtr,
            NVME_ERR_BAD_FLAG => NvmeErrorCode::BadFlag,
            NVME_ERR_BAD_DEVI => NvmeErrorCode::BadDevi,
            NVME_ERR_BAD_DEVI_PROP => NvmeErrorCode::BadDeviProp,
            NVME_ERR_ILLEGAL_INSTANCE => NvmeErrorCode::IllegalInstance,
            NVME_ERR_BAD_CONTROLLER => NvmeErrorCode::BadController,
            NVME_ERR_PRIVS => NvmeErrorCode::Privs,
            NVME_ERR_OPEN_DEV => NvmeErrorCode::OpenDev,
            NVME_ERR_BAD_RESTORE => NvmeErrorCode::BadRestore,
            NVME_ERR_NS_RANGE => NvmeErrorCode::NsRange,
            NVME_ERR_NS_UNUSE => NvmeErrorCode::NsUnuse,
            NVME_ERR_LOG_CSI_RANGE => NvmeErrorCode::LogCsiRange,
            NVME_ERR_LOG_LID_RANGE => NvmeErrorCode::LogLidRange,
            NVME_ERR_LOG_LSP_RANGE => NvmeErrorCode::LogLspRange,
            NVME_ERR_LOG_LSI_RANGE => NvmeErrorCode::LogLsiRange,
            NVME_ERR_LOG_RAE_RANGE => NvmeErrorCode::LogRaeRange,
            NVME_ERR_LOG_SIZE_RANGE => NvmeErrorCode::LogSizeRange,
            NVME_ERR_LOG_OFFSET_RANGE => NvmeErrorCode::LogOffsetRange,
            NVME_ERR_LOG_CSI_UNSUP => NvmeErrorCode::LogCsiUnsup,
            NVME_ERR_LOG_LSP_UNSUP => NvmeErrorCode::LogLspUnsup,
            NVME_ERR_LOG_LSI_UNSUP => NvmeErrorCode::LogLsiUnsup,
            NVME_ERR_LOG_RAE_UNSUP => NvmeErrorCode::LogRaeUnsup,
            NVME_ERR_LOG_OFFSET_UNSUP => NvmeErrorCode::LogOffsetUnsup,
            NVME_ERR_LOG_LSP_UNUSE => NvmeErrorCode::LogLspUnuse,
            NVME_ERR_LOG_LSI_UNUSE => NvmeErrorCode::LogLsiUnuse,
            NVME_ERR_LOG_RAE_UNUSE => NvmeErrorCode::LogRaeUnuse,
            NVME_ERR_LOG_SCOPE_MISMATCH => NvmeErrorCode::LogScopeMismatch,
            NVME_ERR_LOG_REQ_MISSING_FIELDS => {
                NvmeErrorCode::LogReqMissingFields
            }
            NVME_ERR_LOG_NAME_UNKNOWN => NvmeErrorCode::LogNameUnknown,
            NVME_ERR_LOG_UNSUP_BY_DEV => NvmeErrorCode::LogUnsupByDev,
            NVME_ERR_IDENTIFY_UNKNOWN => NvmeErrorCode::IdentifyUnknown,
            NVME_ERR_IDENTIFY_UNSUP_BY_DEV => NvmeErrorCode::IdentifyUnsupByDev,
            NVME_ERR_IDENTIFY_CTRLID_RANGE => {
                NvmeErrorCode::IdentifyCtrlidRange
            }
            NVME_ERR_IDENTIFY_OUTPUT_RANGE => {
                NvmeErrorCode::IdentifyOutputRange
            }
            NVME_ERR_IDENTIFY_CTRLID_UNSUP => {
                NvmeErrorCode::IdentifyCtrlidUnsup
            }
            NVME_ERR_IDENTIFY_CTRLID_UNUSE => {
                NvmeErrorCode::IdentifyCtrlidUnuse
            }
            NVME_ERR_IDENTIFY_REQ_MISSING_FIELDS => {
                NvmeErrorCode::IdentifyReqMissingFields
            }
            NVME_ERR_VUC_UNSUP_BY_DEV => NvmeErrorCode::VucUnsupByDev,
            NVME_ERR_VUC_TIMEOUT_RANGE => NvmeErrorCode::VucTimeoutRange,
            NVME_ERR_VUC_OPCODE_RANGE => NvmeErrorCode::VucOpcodeRange,
            NVME_ERR_VUC_IMPACT_RANGE => NvmeErrorCode::VucImpactRange,
            NVME_ERR_VUC_NDT_RANGE => NvmeErrorCode::VucNdtRange,
            NVME_ERR_VUC_CANNOT_RW => NvmeErrorCode::VucCannotRw,
            NVME_ERR_VUC_NO_RESULTS => NvmeErrorCode::VucNoResults,
            NVME_ERR_VUC_UNKNOWN => NvmeErrorCode::VucUnknown,
            NVME_ERR_VUC_REQ_MISSING_FIELDS => {
                NvmeErrorCode::VucReqMissingFields
            }
            NVME_ERR_VU_FUNC_UNSUP_BY_DEV => NvmeErrorCode::VuFuncUnsupByDev,
            NVME_ERR_WDC_E6_OFFSET_RANGE => NvmeErrorCode::WdcE6OffsetRange,
            NVME_ERR_FW_UNSUP_BY_DEV => NvmeErrorCode::FwUnsupByDev,
            NVME_ERR_KERN_FW_IMPOS => NvmeErrorCode::KernFwImpos,
            NVME_ERR_FW_LOAD_LEN_RANGE => NvmeErrorCode::FwLoadLenRange,
            NVME_ERR_FW_LOAD_OFFSET_RANGE => NvmeErrorCode::FwLoadOffsetRange,
            NVME_ERR_FW_COMMIT_SLOT_RANGE => NvmeErrorCode::FwCommitSlotRange,
            NVME_ERR_FW_COMMIT_ACTION_RANGE => {
                NvmeErrorCode::FwCommitActionRange
            }
            NVME_ERR_FW_COMMIT_REQ_MISSING_FIELDS => {
                NvmeErrorCode::FwCommitReqMissingFields
            }
            NVME_ERR_FW_SLOT_RO => NvmeErrorCode::FwSlotRo,
            NVME_ERR_FORMAT_UNSUP_BY_DEV => NvmeErrorCode::FormatUnsupByDev,
            NVME_ERR_CRYPTO_SE_UNSUP_BY_DEV => {
                NvmeErrorCode::CryptoSeUnsupByDev
            }
            NVME_ERR_NS_FORMAT_UNSUP_BY_DEV => {
                NvmeErrorCode::NsFormatUnsupByDev
            }
            NVME_ERR_KERN_FORMAT_UNSUP => NvmeErrorCode::KernFormatUnsup,
            NVME_ERR_FORMAT_LBAF_RANGE => NvmeErrorCode::FormatLbafRange,
            NVME_ERR_FORMAT_SES_RANGE => NvmeErrorCode::FormatSesRange,
            NVME_ERR_FORMAT_PARAM_UNSUP => NvmeErrorCode::FormatParamUnsup,
            NVME_ERR_FORMAT_REQ_MISSING_FIELDS => {
                NvmeErrorCode::FormatReqMissingFields
            }
            NVME_ERR_WDC_E6_REQ_MISSING_FIELDS => {
                NvmeErrorCode::WdcE6ReqMissingFields
            }
            NVME_ERR_FEAT_NAME_UNKNOWN => NvmeErrorCode::FeatNameUnknown,
            NVME_ERR_FEAT_UNSUP_BY_DEV => NvmeErrorCode::FeatUnsupByDev,
            NVME_ERR_FEAT_FID_RANGE => NvmeErrorCode::FeatFidRange,
            NVME_ERR_FEAT_SEL_RANGE => NvmeErrorCode::FeatSelRange,
            NVME_ERR_FEAT_CDW11_RANGE => NvmeErrorCode::FeatCdw11Range,
            NVME_ERR_FEAT_DATA_RANGE => NvmeErrorCode::FeatDataRange,
            NVME_ERR_FEAT_SEL_UNSUP => NvmeErrorCode::FeatSelUnsup,
            NVME_ERR_FEAT_CDW11_UNUSE => NvmeErrorCode::FeatCdw11Unuse,
            NVME_ERR_FEAT_DATA_UNUSE => NvmeErrorCode::FeatDataUnuse,
            NVME_ERR_FEAT_NO_RESULTS => NvmeErrorCode::FeatNoResults,
            NVME_ERR_GET_FEAT_REQ_MISSING_FIELDS => {
                NvmeErrorCode::GetFeatReqMissingFields
            }
            NVME_ERR_NEED_CTRL_WRLOCK => NvmeErrorCode::NeedCtrlWrlock,
            NVME_ERR_NEED_NS_WRLOCK => NvmeErrorCode::NeedNsWrlock,
            NVME_ERR_CTRL_LOCKED => NvmeErrorCode::CtrlLocked,
            NVME_ERR_NS_LOCKED => NvmeErrorCode::NsLocked,
            NVME_ERR_LOCK_PROG => NvmeErrorCode::LockProg,
            NVME_ERR_LOCK_ORDER => NvmeErrorCode::LockOrder,
            NVME_ERR_LOCK_WAIT_INTR => NvmeErrorCode::LockWaitIntr,
            NVME_ERR_LOCK_WOULD_BLOCK => NvmeErrorCode::LockWouldBlock,
            NVME_ERR_DETACH_KERN => NvmeErrorCode::DetachKern,
            NVME_ERR_ATTACH_KERN => NvmeErrorCode::AttachKern,
            NVME_ERR_ATTACH_UNSUP_KERN => NvmeErrorCode::AttachUnsupKern,
            NVME_ERR_NS_BLKDEV_ATTACH => NvmeErrorCode::NsBlkdevAttach,
            NVME_ERR_NO_KERN_MEM => NvmeErrorCode::NoKernMem,
            NVME_ERR_CTRL_DEAD => NvmeErrorCode::CtrlDead,
            NVME_ERR_CTRL_GONE => NvmeErrorCode::CtrlGone,
            code => NvmeErrorCode::Unknown(code),
        }
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
