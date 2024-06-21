// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::CStr;

use crate::{
    controller::{Controller, NvmeControllerError},
    controller_info::{NvmeInfoError, NvmeInfoErrorCode},
    error::{InternalError, LibraryError},
    lba::LbaFormat,
    util::FfiPtr,
};

use libnvme_sys::nvme::*;

pub enum NamespaceDiscoveryLevel {
    All,
    Allocated,
    Active,
    NotIgnored,
    BlkDev,
}

impl NamespaceDiscoveryLevel {
    fn as_ns_disc_level(&self) -> nvme_ns_disc_level_t {
        match self {
            NamespaceDiscoveryLevel::All => NVME_NS_DISC_F_ALL,
            NamespaceDiscoveryLevel::Allocated => NVME_NS_DISC_F_ALLOCATED,
            NamespaceDiscoveryLevel::Active => NVME_NS_DISC_F_ACTIVE,
            NamespaceDiscoveryLevel::NotIgnored => NVME_NS_DISC_F_NOT_IGNORED,
            NamespaceDiscoveryLevel::BlkDev => NVME_NS_DISC_F_BLKDEV,
        }
    }
}

pub struct NamespaceDiscovery<'a> {
    controller: &'a Controller<'a>,
    iter: *mut nvme_ns_iter_t,
}

impl<'a> Drop for NamespaceDiscovery<'a> {
    fn drop(&mut self) {
        unsafe { nvme_ns_discover_fini(self.iter) }
    }
}

impl<'a> NamespaceDiscovery<'a> {
    pub(crate) fn new(
        controller: &'a Controller<'_>,
        level: NamespaceDiscoveryLevel,
    ) -> Result<Self, NvmeControllerError> {
        let mut iter = std::ptr::null_mut();
        controller
            .check_result(
                unsafe {
                    nvme_ns_discover_init(
                        controller.inner,
                        level.as_ns_disc_level(),
                        &mut iter,
                    )
                },
                || "failed to init nvme namespace discovery",
            )
            .map(|_| NamespaceDiscovery { controller, iter })
    }

    fn internal_step(
        &self,
    ) -> Result<Option<Namespace<'a>>, NvmeControllerError> {
        let mut nvme_ns_disc: *const nvme_ns_disc_t = std::ptr::null_mut();
        let state =
            unsafe { nvme_ns_discover_step(self.iter, &mut nvme_ns_disc) };
        match state {
            NVME_ITER_VALID => {
                let nsid = unsafe { nvme_ns_disc_nsid(nvme_ns_disc) };
                let mut ns: *mut nvme_ns_t = std::ptr::null_mut();
                self.controller
                    .check_result(
                        unsafe {
                            nvme_ns_init(self.controller.inner, nsid, &mut ns)
                        },
                        || "failed to init nvme namespace",
                    )
                    .map(|_| {
                        Some(Namespace {
                            inner: ns,
                            controller: self.controller,
                        })
                    })
            }
            NVME_ITER_DONE => Ok(None),
            NVME_ITER_ERROR => Err(self
                .controller
                .fatal_context("failed to iterate nvme namespaces")),
            invalid => unreachable!(
                "invalid nvme controller iteration state ({invalid})",
            ),
        }
    }
}

impl<'a> Iterator for NamespaceDiscovery<'a> {
    type Item = Result<Namespace<'a>, NvmeControllerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal_step().transpose()
    }
}

pub struct Namespace<'a> {
    inner: *mut nvme_ns_t,
    controller: &'a Controller<'a>,
}

impl<'a> Drop for Namespace<'a> {
    fn drop(&mut self) {
        unsafe { nvme_ns_fini(self.inner) }
    }
}

impl<'a> Namespace<'a> {
    pub fn get_info(&self) -> Result<NamespaceInfo, NvmeControllerError> {
        let mut nvme_ns_info: *mut nvme_ns_info_t = std::ptr::null_mut();
        self.controller
            .check_result(
                unsafe { nvme_ns_info_snap(self.inner, &mut nvme_ns_info) },
                || "failed to get ns info snapshot",
            )
            .map(|_| unsafe { NamespaceInfo::from_raw(nvme_ns_info) })
    }

    pub fn blkdev_attach(&self) -> Result<(), NvmeControllerError> {
        self.controller
            .check_result(unsafe { nvme_ns_bd_attach(self.inner) }, || {
                "failed to attach blkdev to namespace"
            })
    }

    pub fn blkdev_detach(&self) -> Result<(), NvmeControllerError> {
        self.controller
            .check_result(unsafe { nvme_ns_bd_detach(self.inner) }, || {
                "failed to detach blkdev to namespace"
            })
    }
}

pub struct NamespaceInfo(*mut nvme_ns_info_t);

impl Drop for NamespaceInfo {
    fn drop(&mut self) {
        unsafe { nvme_ns_info_free(self.0) }
    }
}

impl NamespaceInfo {
    pub fn current_format(&self) -> Result<LbaFormat<'_>, NvmeInfoError> {
        let mut lba: *const nvme_nvm_lba_fmt_t = std::ptr::null_mut();
        self.check_result(
            unsafe { nvme_ns_info_curformat(self.0, &mut lba) },
            || "failed to get current format of NVMe namespace",
        )
        .map(|_| unsafe { LbaFormat::from_raw(lba) })
    }
}

impl LibraryError for NamespaceInfo {
    type Error = NvmeInfoError;

    fn get_errmsg(&self) -> String {
        let errmsg = unsafe { nvme_ns_info_errmsg(self.0) };
        unsafe { CStr::from_ptr(errmsg) }.to_string_lossy().to_string()
    }

    fn get_syserr(&self) -> i32 {
        unsafe { nvme_ns_info_syserr(self.0) }
    }

    fn current_error(&self, internal: InternalError) -> Self::Error {
        let raw = unsafe { nvme_ns_info_err(self.0) };
        NvmeInfoError::from_code_and_error(
            NvmeInfoErrorCode::from_raw(raw),
            internal,
        )
    }
}

impl FfiPtr for NamespaceInfo {
    type Ptr = *mut nvme_ns_info_t;

    unsafe fn from_raw(ptr: Self::Ptr) -> Self {
        NamespaceInfo(ptr)
    }
}
