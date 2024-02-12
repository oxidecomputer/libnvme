// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{ffi::CStr, ops::Deref};

use crate::{
    controller_info::ControllerInfo,
    error::LibraryError,
    namespace::{NamespaceDiscovery, NamespaceDiscoveryLevel},
    util::FfiPtr,
    Nvme, NvmeError,
};

use nvme_sys::nvme::*;

enum ControllerLockLevel {
    Read = 1,
    Write,
}

enum ControllerLockFlags {
    Block = 0,
    DontBlock = 1 << 0,
}

pub enum TryLockResult<L, T, E> {
    Ok(L),
    Locked(T),
    Err(E),
}

pub struct Controller(pub(crate) *mut nvme_ctrl_t);

impl Controller {
    pub fn get_info(&self) -> Result<ControllerInfo, NvmeError> {
        let mut ctrl_info: *mut nvme_ctrl_info_t = std::ptr::null_mut();
        match { unsafe { nvme_ctrl_info_snap(self.0, &mut ctrl_info) } } {
            true => Ok(unsafe { ControllerInfo::from_raw(ctrl_info) }),
            false => {
                Err(self.fatal_context("failed to get controller snapshot"))
            }
        }
    }

    fn lock_impl(
        self,
        level: ControllerLockLevel,
        flags: ControllerLockFlags,
    ) -> Result<LockedController, (Controller, NvmeError)> {
        match unsafe { nvme_ctrl_lock(self.0, level as u32, flags as u32) } {
            true => Ok(LockedController { controller: Some(self) }),
            false => {
                let error =
                    self.fatal_context("failed to grab nvme controller lock");
                Err((self, error))
            }
        }
    }

    pub fn read_lock(self) -> Result<LockedController, NvmeError> {
        match self
            .lock_impl(ControllerLockLevel::Read, ControllerLockFlags::Block)
        {
            Ok(l) => Ok(l),
            Err((_, e)) => Err(e),
        }
    }

    pub fn write_lock(self) -> Result<LockedController, NvmeError> {
        match self
            .lock_impl(ControllerLockLevel::Write, ControllerLockFlags::Block)
        {
            Ok(l) => Ok(l),
            Err((_, e)) => Err(e),
        }
    }

    pub fn try_read_lock(
        self,
    ) -> TryLockResult<LockedController, Controller, NvmeError> {
        match self.lock_impl(
            ControllerLockLevel::Read,
            ControllerLockFlags::DontBlock,
        ) {
            Ok(lock) => TryLockResult::Ok(lock),
            Err((c, e)) => match e {
                NvmeError::LockWouldBlock(_) => TryLockResult::Locked(c),
                e => TryLockResult::Err(e),
            },
        }
    }

    pub fn try_write_lock(
        self,
    ) -> TryLockResult<LockedController, Controller, NvmeError> {
        match self.lock_impl(
            ControllerLockLevel::Write,
            ControllerLockFlags::DontBlock,
        ) {
            Ok(lock) => TryLockResult::Ok(lock),
            Err((c, e)) => match e {
                NvmeError::LockWouldBlock(_) => TryLockResult::Locked(c),
                e => TryLockResult::Err(e),
            },
        }
    }

    pub fn namespace_discovery(
        &self,
        level: NamespaceDiscoveryLevel,
    ) -> Result<NamespaceDiscovery, NvmeError> {
        NamespaceDiscovery::new(self, level)
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        unsafe { nvme_ctrl_fini(self.0) }
    }
}

pub struct ControllerDiscovery<'a> {
    nvme: &'a Nvme,
    iter: *mut nvme_ctrl_iter_t,
}

impl<'a> Drop for ControllerDiscovery<'a> {
    fn drop(&mut self) {
        unsafe { nvme_ctrl_discover_fini(self.iter) }
    }
}

impl<'a> ControllerDiscovery<'a> {
    pub(crate) fn new(nvme: &'a Nvme) -> Result<Self, NvmeError> {
        let mut iter = std::ptr::null_mut();
        match unsafe { nvme_ctrl_discover_init(nvme.0, &mut iter) } {
            true => Ok(ControllerDiscovery { nvme, iter }),
            false => {
                Err(nvme
                    .fatal_context("failed to init nvme controller discovery"))
            }
        }
    }

    fn internal_step(&self) -> Result<Option<Controller>, NvmeError> {
        let mut nvme_ctr_disc: *const nvme_ctrl_disc_t = std::ptr::null_mut();
        let state =
            unsafe { nvme_ctrl_discover_step(self.iter, &mut nvme_ctr_disc) };
        match state {
            NVME_ITER_VALID => {
                let di_node_t = unsafe { nvme_ctrl_disc_devi(nvme_ctr_disc) };
                let mut nvme_ctrl: *mut nvme_ctrl_t = std::ptr::null_mut();
                match unsafe {
                    nvme_ctrl_init(self.nvme.0, di_node_t, &mut nvme_ctrl)
                } {
                    true => Ok(Some(Controller(nvme_ctrl))),
                    false => Err(self
                        .nvme
                        .fatal_context("failed to init nvme controller")),
                }
            }
            NVME_ITER_DONE => Ok(None),
            NVME_ITER_ERROR => Err(self
                .nvme
                .fatal_context("failed to iterate nvme controllers")),
            invalid => unreachable!(
                "invalid nvme controller iteration state ({invalid})",
            ),
        }
    }
}

impl Iterator for ControllerDiscovery<'_> {
    type Item = Result<Controller, NvmeError>;

    fn next(&mut self) -> Option<Result<Controller, NvmeError>> {
        self.internal_step().transpose()
    }
}

impl LibraryError for Controller {
    type Error = NvmeError;

    fn get_errmsg(&self) -> String {
        let errmsg = unsafe { nvme_ctrl_errmsg(self.0) };
        unsafe { CStr::from_ptr(errmsg) }.to_string_lossy().to_string()
    }

    fn get_syserr(&self) -> i32 {
        unsafe { nvme_ctrl_syserr(self.0) }
    }

    fn to_error(&self, internal: crate::error::InternalError) -> Self::Error {
        NvmeError::from_raw_with_internal_error(
            unsafe { nvme_ctrl_err(self.0) },
            internal,
        )
    }
}

pub struct LockedController {
    pub(crate) controller: Option<Controller>,
}

impl Drop for LockedController {
    fn drop(&mut self) {
        if let Some(controller) = self.controller.take() {
            unsafe { nvme_ctrl_unlock(controller.0) }
        }
    }
}

impl LockedController {
    pub fn unlock(mut self) -> Controller {
        self.controller.take().expect("controller invariant violated")
    }

    pub fn format_request(&self) -> Result<FormatRequestBuilder, NvmeError> {
        let controller =
            self.controller.as_ref().expect("controller is locked");
        let mut req = std::ptr::null_mut();
        match unsafe { nvme_format_req_init(controller.0, &mut req) } {
            true => Ok(FormatRequestBuilder { req, controller: self }),
            false => {
                Err(controller.fatal_context("failed to create format request"))
            }
        }
    }
}

impl Deref for LockedController {
    type Target = Controller;

    fn deref(&self) -> &Self::Target {
        self.controller.as_ref().expect("controller is locked")
    }
}

pub struct FormatRequestBuilder<'ctrl> {
    req: *mut nvme_format_req_t,
    controller: &'ctrl LockedController,
}

impl<'ctrl> Drop for FormatRequestBuilder<'ctrl> {
    fn drop(&mut self) {
        unsafe { nvme_format_req_fini(self.req) }
    }
}

impl<'ctrl> FormatRequestBuilder<'ctrl> {
    pub fn set_lbaf(self, lbaf: u32) -> Result<Self, NvmeError> {
        match unsafe { nvme_format_req_set_lbaf(self.req, lbaf) } {
            true => Ok(self),
            false => Err(self.controller.with_fatal_context(|| {
                format!("failed to set LBA format {lbaf} on format request")
            })),
        }
    }

    pub fn set_nsid(self, nsid: u32) -> Result<Self, NvmeError> {
        match unsafe { nvme_format_req_set_nsid(self.req, nsid) } {
            true => Ok(self),
            false => Err(self.controller.with_fatal_context(|| {
                format!("failed to set nsid {nsid} on format request")
            })),
        }
    }

    pub fn set_ses(self, ses: u32) -> Result<Self, NvmeError> {
        match unsafe { nvme_format_req_set_ses(self.req, ses) } {
            true => Ok(self),
            false => Err(self.controller.with_fatal_context(|| {
                format!("failed to set ses {ses} on format request")
            })),
        }
    }

    pub fn execute(self) -> Result<(), NvmeError> {
        match unsafe { nvme_format_req_exec(self.req) } {
            true => Ok(()),
            false => Err(self
                .controller
                .fatal_context("failed to execute format request")),
        }
    }
}
