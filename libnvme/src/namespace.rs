// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::CStr;

use crate::{
    controller::Controller,
    controller_info::NvmeInfoError,
    error::{InternalError, LibraryError},
    lba::LbaFormat,
    util::FfiPtr,
    NvmeError,
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
        controller: &'a Controller,
        level: NamespaceDiscoveryLevel,
    ) -> Result<Self, NvmeError> {
        let mut iter = std::ptr::null_mut();
        match unsafe {
            nvme_ns_discover_init(
                controller.inner,
                level.as_ns_disc_level(),
                &mut iter,
            )
        } {
            true => Ok(NamespaceDiscovery { controller, iter }),
            false => Err(controller
                .fatal_context("failed to init nvme namespace discovery")),
        }
    }

    fn internal_step(&self) -> Result<Option<Namespace<'a>>, NvmeError> {
        let mut nvme_ns_disc: *const nvme_ns_disc_t = std::ptr::null_mut();
        let state =
            unsafe { nvme_ns_discover_step(self.iter, &mut nvme_ns_disc) };
        match state {
            NVME_ITER_VALID => {
                let nsid = unsafe { nvme_ns_disc_nsid(nvme_ns_disc) };
                let mut ns: *mut nvme_ns_t = std::ptr::null_mut();
                match unsafe {
                    nvme_ns_init(self.controller.inner, nsid, &mut ns)
                } {
                    true => Ok(Some(Namespace {
                        inner: ns,
                        controller: self.controller,
                    })),
                    false => Err(self
                        .controller
                        .fatal_context("failed to init nvme namespace")),
                }
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
    type Item = Result<Namespace<'a>, NvmeError>;

    fn next(&mut self) -> Option<Result<Namespace<'a>, NvmeError>> {
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
    pub fn get_info(&self) -> Result<NamespaceInfo, NvmeError> {
        let mut nvme_ns_info: *mut nvme_ns_info_t = std::ptr::null_mut();
        match unsafe { nvme_ns_info_snap(self.inner, &mut nvme_ns_info) } {
            true => Ok(unsafe { NamespaceInfo::from_raw(nvme_ns_info) }),
            false => Err(self
                .controller
                .fatal_context("failed to get ns info snapshot")),
        }
    }

    pub fn blkdev_attach(&self) -> Result<(), NvmeError> {
        match unsafe { nvme_ns_bd_attach(self.inner) } {
            true => Ok(()),
            false => Err(self
                .controller
                .fatal_context("failed to attach blkdev to namespace")),
        }
    }

    pub fn blkdev_detach(&self) -> Result<(), NvmeError> {
        match unsafe { nvme_ns_bd_detach(self.inner) } {
            true => Ok(()),
            false => Err(self
                .controller
                .fatal_context("failed to detach blkdev to namespace")),
        }
    }
}

pub struct NamespaceInfo(*mut nvme_ns_info_t);

impl Drop for NamespaceInfo {
    fn drop(&mut self) {
        unsafe { nvme_ns_info_free(self.0) }
    }
}

impl NamespaceInfo {
    pub fn current_format(&self) -> Result<LbaFormat, NvmeInfoError> {
        let mut lba: *const nvme_nvm_lba_fmt_t = std::ptr::null_mut();
        match unsafe { nvme_ns_info_curformat(self.0, &mut lba) } {
            true => Ok(unsafe { LbaFormat::from_raw(lba) }),
            false => Err(self.fatal_context(
                "failed to get current format of NVMe namespace",
            )),
        }
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

    fn to_error(&self, internal: InternalError) -> Self::Error {
        NvmeInfoError::from_raw_with_internal_error(
            unsafe { nvme_ns_info_err(self.0) },
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
