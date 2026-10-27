// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{c_char, CStr};

use libnvme_sys::nvme::*;

use crate::{
    controller::{Controller, NvmeControllerError},
    error::LibraryError,
};

/// A log page known to libnvme for a controller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogPageDisc {
    /// The libnvme short name, e.g. `health` or `wdc/devmgmt`.
    pub name: String,
    /// A human readable description.
    pub desc: String,
    /// Whether the controller implements this log page.
    pub implemented: bool,
    /// The scopes (`NVME_LOG_SCOPE_*`) the log page applies to.
    pub scopes: u32,
}

fn cstr(p: *const c_char) -> String {
    if p.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned()
}

/// Iterates the log pages libnvme knows about for a controller, including
/// vendor-specific ones for devices it recognizes.
pub struct LogDiscovery<'a> {
    controller: &'a Controller<'a>,
    iter: *mut nvme_log_iter_t,
}

impl<'a> Drop for LogDiscovery<'a> {
    fn drop(&mut self) {
        unsafe { nvme_log_discover_fini(self.iter) }
    }
}

impl<'a> LogDiscovery<'a> {
    fn new(
        controller: &'a Controller<'_>,
        scope: nvme_log_disc_scope_t,
    ) -> Result<Self, NvmeControllerError> {
        let mut iter = std::ptr::null_mut();
        controller
            .check_result(
                unsafe {
                    nvme_log_discover_init(
                        controller.inner,
                        scope,
                        0,
                        &mut iter,
                    )
                },
                || "failed to init nvme log page discovery",
            )
            .map(|_| LogDiscovery { controller, iter })
    }

    fn internal_step(
        &self,
    ) -> Result<Option<LogPageDisc>, NvmeControllerError> {
        // The returned disc is owned by the iterator and is only valid until
        // the next step, so copy out what we need.
        let mut disc: *const nvme_log_disc_t = std::ptr::null();
        match unsafe { nvme_log_discover_step(self.iter, &mut disc) } {
            NVME_ITER_VALID => Ok(Some(LogPageDisc {
                name: cstr(unsafe { nvme_log_disc_name(disc) }),
                desc: cstr(unsafe { nvme_log_disc_desc(disc) }),
                implemented: unsafe { nvme_log_disc_impl(disc) },
                scopes: unsafe { nvme_log_disc_scopes(disc) },
            })),
            NVME_ITER_DONE => Ok(None),
            NVME_ITER_ERROR => Err(self
                .controller
                .fatal_context("failed to iterate nvme log pages")),
            invalid => {
                unreachable!("invalid nvme log iteration state ({invalid})")
            }
        }
    }
}

impl<'a> Iterator for LogDiscovery<'a> {
    type Item = Result<LogPageDisc, NvmeControllerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal_step().transpose()
    }
}

impl<'a> Controller<'a> {
    /// Discover the controller's log pages, as `nvmeadm list-logpages` does:
    /// controller and NVM subsystem scoped pages. Use
    /// [`LogPageDisc::implemented`] to find the ones the controller supports.
    pub fn log_page_discovery(
        &self,
    ) -> Result<LogDiscovery<'_>, NvmeControllerError> {
        LogDiscovery::new(self, NVME_LOG_SCOPE_CTRL | NVME_LOG_SCOPE_NVM)
    }
}
