// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;

use error::{InternalError, LibraryError};
use thiserror::Error;

pub mod controller;
pub mod controller_info;
mod error;
mod ffi;
mod lba;
pub mod namespace;
mod util;
mod wdc;

use ffi::nvme::*;

use crate::controller::ControllerDiscovery;

#[derive(Debug, Error)]
pub enum NvmeError {
    #[error("failed to initialize libnvme")]
    FailedInit,
    #[error(transparent)]
    Controller(InternalError),
    #[error(transparent)]
    NoMem(InternalError),
    #[error(transparent)]
    NoDmaMem(InternalError),
    #[error(transparent)]
    Libdevinfo(InternalError),
    #[error(transparent)]
    Internal(InternalError),
    #[error(transparent)]
    BadPtr(InternalError),
    #[error(transparent)]
    BadFlag(InternalError),
    #[error(transparent)]
    BadDevi(InternalError),
    #[error(transparent)]
    BadDeviProp(InternalError),
    #[error(transparent)]
    IllegalInstance(InternalError),
    #[error(transparent)]
    BadController(InternalError),
    #[error(transparent)]
    Privs(InternalError),
    #[error(transparent)]
    OpenDev(InternalError),
    #[error(transparent)]
    BadRestore(InternalError),
    #[error(transparent)]
    NsRange(InternalError),
    #[error(transparent)]
    NsUnuse(InternalError),
    #[error(transparent)]
    LogCsiRange(InternalError),
    #[error(transparent)]
    LogLidRange(InternalError),
    #[error(transparent)]
    LogLspRange(InternalError),
    #[error(transparent)]
    LogLsiRange(InternalError),
    #[error(transparent)]
    LogRaeRange(InternalError),
    #[error(transparent)]
    LogSizeRange(InternalError),
    #[error(transparent)]
    LogOffsetRange(InternalError),
    #[error(transparent)]
    LogCsiUnsup(InternalError),
    #[error(transparent)]
    LogLspUnsup(InternalError),
    #[error(transparent)]
    LogLsiUnsup(InternalError),
    #[error(transparent)]
    LogRaeUnsup(InternalError),
    #[error(transparent)]
    LogOffsetUnsup(InternalError),
    #[error(transparent)]
    LogLspUnuse(InternalError),
    #[error(transparent)]
    LogLsiUnuse(InternalError),
    #[error(transparent)]
    LogRaeUnuse(InternalError),
    #[error(transparent)]
    LogScopeMismatch(InternalError),
    #[error(transparent)]
    LogReqMissingFields(InternalError),
    #[error(transparent)]
    LogNameUnknown(InternalError),
    #[error(transparent)]
    LogUnsupByDev(InternalError),
    #[error(transparent)]
    IdentifyUnknown(InternalError),
    #[error(transparent)]
    IdentifyUnsupByDev(InternalError),
    #[error(transparent)]
    IdentifyCtrlidRange(InternalError),
    #[error(transparent)]
    IdentifyOutputRange(InternalError),
    #[error(transparent)]
    IdentifyCtrlidUnsup(InternalError),
    #[error(transparent)]
    IdentifyCtrlidUnuse(InternalError),
    #[error(transparent)]
    IdentifyReqMissingFields(InternalError),
    #[error(transparent)]
    VucUnsupByDev(InternalError),
    #[error(transparent)]
    VucTimeoutRange(InternalError),
    #[error(transparent)]
    VucOpcodeRange(InternalError),
    #[error(transparent)]
    VucImpactRange(InternalError),
    #[error(transparent)]
    VucNdtRange(InternalError),
    #[error(transparent)]
    VucCannotRw(InternalError),
    #[error(transparent)]
    VucNoResults(InternalError),
    #[error(transparent)]
    VucUnknown(InternalError),
    #[error(transparent)]
    VucReqMissingFields(InternalError),
    #[error(transparent)]
    VuFuncUnsupByDev(InternalError),
    #[error(transparent)]
    WdcE6OffsetRange(InternalError),
    #[error(transparent)]
    FwUnsupByDev(InternalError),
    #[error(transparent)]
    KernFwImpos(InternalError),
    #[error(transparent)]
    FwLoadLenRange(InternalError),
    #[error(transparent)]
    FwLoadOffsetRange(InternalError),
    #[error(transparent)]
    FwCommitSlotRange(InternalError),
    #[error(transparent)]
    FwCommitActionRange(InternalError),
    #[error(transparent)]
    FwCommitReqMissingFields(InternalError),
    #[error(transparent)]
    FwSlotRo(InternalError),
    #[error(transparent)]
    FormatUnsupByDev(InternalError),
    #[error(transparent)]
    CryptoSeUnsupByDev(InternalError),
    #[error(transparent)]
    NsFormatUnsupByDev(InternalError),
    #[error(transparent)]
    KernFormatUnsup(InternalError),
    #[error(transparent)]
    FormatLbafRange(InternalError),
    #[error(transparent)]
    FormatSesRange(InternalError),
    #[error(transparent)]
    FormatParamUnsup(InternalError),
    #[error(transparent)]
    FormatReqMissingFields(InternalError),
    #[error(transparent)]
    WdcE6ReqMissingFields(InternalError),
    #[error(transparent)]
    FeatNameUnknown(InternalError),
    #[error(transparent)]
    FeatUnsupByDev(InternalError),
    #[error(transparent)]
    FeatFidRange(InternalError),
    #[error(transparent)]
    FeatSelRange(InternalError),
    #[error(transparent)]
    FeatCdw11Range(InternalError),
    #[error(transparent)]
    FeatDataRange(InternalError),
    #[error(transparent)]
    FeatSelUnsup(InternalError),
    #[error(transparent)]
    FeatCdw11Unuse(InternalError),
    #[error(transparent)]
    FeatDataUnuse(InternalError),
    #[error(transparent)]
    FeatNoResults(InternalError),
    #[error(transparent)]
    GetFeatReqMissingFields(InternalError),
    #[error(transparent)]
    NeedCtrlWrlock(InternalError),
    #[error(transparent)]
    NeedNsWrlock(InternalError),
    #[error(transparent)]
    CtrlLocked(InternalError),
    #[error(transparent)]
    NsLocked(InternalError),
    #[error(transparent)]
    LockProg(InternalError),
    #[error(transparent)]
    LockOrder(InternalError),
    #[error(transparent)]
    LockWaitIntr(InternalError),
    #[error(transparent)]
    LockWouldBlock(InternalError),
    #[error(transparent)]
    DetachKern(InternalError),
    #[error(transparent)]
    AttachKern(InternalError),
    #[error(transparent)]
    AttachUnsupKern(InternalError),
    #[error(transparent)]
    NsBlkdevAttach(InternalError),
}

impl NvmeError {
    fn from_raw_with_internal_error(raw: u32, internal: InternalError) -> Self {
        match raw {
            NVME_ERR_OK => panic!("called fatal on NVME_ERR_OK"),
            NVME_ERR_CONTROLLER => NvmeError::Controller(internal),
            NVME_ERR_NO_MEM => NvmeError::NoMem(internal),
            NVME_ERR_NO_DMA_MEM => NvmeError::NoDmaMem(internal),
            NVME_ERR_LIBDEVINFO => NvmeError::Libdevinfo(internal),
            NVME_ERR_INTERNAL => NvmeError::Internal(internal),
            NVME_ERR_BAD_PTR => NvmeError::BadPtr(internal),
            NVME_ERR_BAD_FLAG => NvmeError::BadFlag(internal),
            NVME_ERR_BAD_DEVI => NvmeError::BadDevi(internal),
            NVME_ERR_BAD_DEVI_PROP => NvmeError::BadDeviProp(internal),
            NVME_ERR_ILLEGAL_INSTANCE => NvmeError::IllegalInstance(internal),
            NVME_ERR_BAD_CONTROLLER => NvmeError::BadController(internal),
            NVME_ERR_PRIVS => NvmeError::Privs(internal),
            NVME_ERR_OPEN_DEV => NvmeError::OpenDev(internal),
            NVME_ERR_BAD_RESTORE => NvmeError::BadRestore(internal),
            NVME_ERR_NS_RANGE => NvmeError::NsRange(internal),
            NVME_ERR_NS_UNUSE => NvmeError::NsUnuse(internal),
            NVME_ERR_LOG_CSI_RANGE => NvmeError::LogCsiRange(internal),
            NVME_ERR_LOG_LID_RANGE => NvmeError::LogLidRange(internal),
            NVME_ERR_LOG_LSP_RANGE => NvmeError::LogLspRange(internal),
            NVME_ERR_LOG_LSI_RANGE => NvmeError::LogLsiRange(internal),
            NVME_ERR_LOG_RAE_RANGE => NvmeError::LogRaeRange(internal),
            NVME_ERR_LOG_SIZE_RANGE => NvmeError::LogSizeRange(internal),
            NVME_ERR_LOG_OFFSET_RANGE => NvmeError::LogOffsetRange(internal),
            NVME_ERR_LOG_CSI_UNSUP => NvmeError::LogCsiUnsup(internal),
            NVME_ERR_LOG_LSP_UNSUP => NvmeError::LogLspUnsup(internal),
            NVME_ERR_LOG_LSI_UNSUP => NvmeError::LogLsiUnsup(internal),
            NVME_ERR_LOG_RAE_UNSUP => NvmeError::LogRaeUnsup(internal),
            NVME_ERR_LOG_OFFSET_UNSUP => NvmeError::LogOffsetUnsup(internal),
            NVME_ERR_LOG_LSP_UNUSE => NvmeError::LogLspUnuse(internal),
            NVME_ERR_LOG_LSI_UNUSE => NvmeError::LogLsiUnuse(internal),
            NVME_ERR_LOG_RAE_UNUSE => NvmeError::LogRaeUnuse(internal),
            NVME_ERR_LOG_SCOPE_MISMATCH => {
                NvmeError::LogScopeMismatch(internal)
            }
            NVME_ERR_LOG_REQ_MISSING_FIELDS => {
                NvmeError::LogReqMissingFields(internal)
            }
            NVME_ERR_LOG_NAME_UNKNOWN => NvmeError::LogNameUnknown(internal),
            NVME_ERR_LOG_UNSUP_BY_DEV => NvmeError::LogUnsupByDev(internal),
            NVME_ERR_IDENTIFY_UNKNOWN => NvmeError::IdentifyUnknown(internal),
            NVME_ERR_IDENTIFY_UNSUP_BY_DEV => {
                NvmeError::IdentifyUnsupByDev(internal)
            }
            NVME_ERR_IDENTIFY_CTRLID_RANGE => {
                NvmeError::IdentifyCtrlidRange(internal)
            }
            NVME_ERR_IDENTIFY_OUTPUT_RANGE => {
                NvmeError::IdentifyOutputRange(internal)
            }
            NVME_ERR_IDENTIFY_CTRLID_UNSUP => {
                NvmeError::IdentifyCtrlidUnsup(internal)
            }
            NVME_ERR_IDENTIFY_CTRLID_UNUSE => {
                NvmeError::IdentifyCtrlidUnuse(internal)
            }
            NVME_ERR_IDENTIFY_REQ_MISSING_FIELDS => {
                NvmeError::IdentifyReqMissingFields(internal)
            }
            NVME_ERR_VUC_UNSUP_BY_DEV => NvmeError::VucUnsupByDev(internal),
            NVME_ERR_VUC_TIMEOUT_RANGE => NvmeError::VucTimeoutRange(internal),
            NVME_ERR_VUC_OPCODE_RANGE => NvmeError::VucOpcodeRange(internal),
            NVME_ERR_VUC_IMPACT_RANGE => NvmeError::VucImpactRange(internal),
            NVME_ERR_VUC_NDT_RANGE => NvmeError::VucNdtRange(internal),
            NVME_ERR_VUC_CANNOT_RW => NvmeError::VucCannotRw(internal),
            NVME_ERR_VUC_NO_RESULTS => NvmeError::VucNoResults(internal),
            NVME_ERR_VUC_UNKNOWN => NvmeError::VucUnknown(internal),
            NVME_ERR_VUC_REQ_MISSING_FIELDS => {
                NvmeError::VucReqMissingFields(internal)
            }
            NVME_ERR_VU_FUNC_UNSUP_BY_DEV => {
                NvmeError::VuFuncUnsupByDev(internal)
            }
            NVME_ERR_WDC_E6_OFFSET_RANGE => {
                NvmeError::WdcE6OffsetRange(internal)
            }
            NVME_ERR_FW_UNSUP_BY_DEV => NvmeError::FwUnsupByDev(internal),
            NVME_ERR_KERN_FW_IMPOS => NvmeError::KernFwImpos(internal),
            NVME_ERR_FW_LOAD_LEN_RANGE => NvmeError::FwLoadLenRange(internal),
            NVME_ERR_FW_LOAD_OFFSET_RANGE => {
                NvmeError::FwLoadOffsetRange(internal)
            }
            NVME_ERR_FW_COMMIT_SLOT_RANGE => {
                NvmeError::FwCommitSlotRange(internal)
            }
            NVME_ERR_FW_COMMIT_ACTION_RANGE => {
                NvmeError::FwCommitActionRange(internal)
            }
            NVME_ERR_FW_COMMIT_REQ_MISSING_FIELDS => {
                NvmeError::FwCommitReqMissingFields(internal)
            }
            NVME_ERR_FW_SLOT_RO => NvmeError::FwSlotRo(internal),
            NVME_ERR_FORMAT_UNSUP_BY_DEV => {
                NvmeError::FormatUnsupByDev(internal)
            }
            NVME_ERR_CRYPTO_SE_UNSUP_BY_DEV => {
                NvmeError::CryptoSeUnsupByDev(internal)
            }
            NVME_ERR_NS_FORMAT_UNSUP_BY_DEV => {
                NvmeError::NsFormatUnsupByDev(internal)
            }
            NVME_ERR_KERN_FORMAT_UNSUP => NvmeError::KernFormatUnsup(internal),
            NVME_ERR_FORMAT_LBAF_RANGE => NvmeError::FormatLbafRange(internal),
            NVME_ERR_FORMAT_SES_RANGE => NvmeError::FormatSesRange(internal),
            NVME_ERR_FORMAT_PARAM_UNSUP => {
                NvmeError::FormatParamUnsup(internal)
            }
            NVME_ERR_FORMAT_REQ_MISSING_FIELDS => {
                NvmeError::FormatReqMissingFields(internal)
            }
            NVME_ERR_WDC_E6_REQ_MISSING_FIELDS => {
                NvmeError::WdcE6ReqMissingFields(internal)
            }
            NVME_ERR_FEAT_NAME_UNKNOWN => NvmeError::FeatNameUnknown(internal),
            NVME_ERR_FEAT_UNSUP_BY_DEV => NvmeError::FeatUnsupByDev(internal),
            NVME_ERR_FEAT_FID_RANGE => NvmeError::FeatFidRange(internal),
            NVME_ERR_FEAT_SEL_RANGE => NvmeError::FeatSelRange(internal),
            NVME_ERR_FEAT_CDW11_RANGE => NvmeError::FeatCdw11Range(internal),
            NVME_ERR_FEAT_DATA_RANGE => NvmeError::FeatDataRange(internal),
            NVME_ERR_FEAT_SEL_UNSUP => NvmeError::FeatSelUnsup(internal),
            NVME_ERR_FEAT_CDW11_UNUSE => NvmeError::FeatCdw11Unuse(internal),
            NVME_ERR_FEAT_DATA_UNUSE => NvmeError::FeatDataUnuse(internal),
            NVME_ERR_FEAT_NO_RESULTS => NvmeError::FeatNoResults(internal),
            NVME_ERR_GET_FEAT_REQ_MISSING_FIELDS => {
                NvmeError::GetFeatReqMissingFields(internal)
            }
            NVME_ERR_NEED_CTRL_WRLOCK => NvmeError::NeedCtrlWrlock(internal),
            NVME_ERR_NEED_NS_WRLOCK => NvmeError::NeedNsWrlock(internal),
            NVME_ERR_CTRL_LOCKED => NvmeError::CtrlLocked(internal),
            NVME_ERR_NS_LOCKED => NvmeError::NsLocked(internal),
            NVME_ERR_LOCK_PROG => NvmeError::LockProg(internal),
            NVME_ERR_LOCK_ORDER => NvmeError::LockOrder(internal),
            NVME_ERR_LOCK_WAIT_INTR => NvmeError::LockWaitIntr(internal),
            NVME_ERR_LOCK_WOULD_BLOCK => NvmeError::LockWouldBlock(internal),
            NVME_ERR_DETACH_KERN => NvmeError::DetachKern(internal),
            NVME_ERR_ATTACH_KERN => NvmeError::AttachKern(internal),
            NVME_ERR_ATTACH_UNSUP_KERN => NvmeError::AttachUnsupKern(internal),
            NVME_ERR_NS_BLKDEV_ATTACH => NvmeError::NsBlkdevAttach(internal),
            // TODO map this to an error type so we don't crash someones program
            _ => unreachable!("Unknown Error"),
        }
    }
}

#[derive(Debug)]
pub struct Nvme(*mut nvme_t);

impl Drop for Nvme {
    fn drop(&mut self) {
        unsafe { nvme_fini(self.0) }
    }
}

impl Nvme {
    pub fn new() -> Result<Self, NvmeError> {
        let ptr = unsafe { nvme_init() };
        if ptr.is_null() {
            return Err(NvmeError::FailedInit);
        }
        Ok(Self(ptr))
    }

    pub fn controller_discovery(
        &self,
    ) -> Result<ControllerDiscovery, NvmeError> {
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

    fn to_error(&self, internal: error::InternalError) -> Self::Error {
        NvmeError::from_raw_with_internal_error(
            unsafe { nvme_err(self.0) },
            internal,
        )
    }
}
