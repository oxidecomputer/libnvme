// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{ffi::CStr, marker::PhantomData};

use libnvme_sys::nvme::*;

use crate::{
    controller::{Controller, NvmeControllerError},
    error::LibraryError,
};

pub(crate) struct NvmeLogReq<'a> {
    pub(crate) inner: *mut nvme_log_req_t,
    // The log page being requested
    pub(crate) page_name: LogPageName,
    // This is the `Controller` the log request was created from.
    _phantom: PhantomData<&'a Controller<'a>>,
}

impl<'a> Drop for NvmeLogReq<'a> {
    fn drop(&mut self) {
        unsafe { nvme_log_req_fini(self.inner) }
    }
}

pub(crate) struct NvmeLogDisc<'a> {
    pub(crate) inner: *mut nvme_log_disc_t,
    // This is the `Controller` the log disc was created from.
    _phantom: PhantomData<&'a Controller<'a>>,
}

impl<'a> Drop for NvmeLogDisc<'a> {
    fn drop(&mut self) {
        unsafe { nvme_log_disc_free(self.inner) }
    }
}

/// The largest log page we will fetch: the illumos nvme driver's limit on the
/// size of a log page (`nvme_log_page_max_size` in
/// `uts/common/io/nvme/nvme_validate.c`, 1 MiB). A larger size reported by a
/// device can not be read, so return an error before allocating a buffer for
/// it.
pub(crate) const LOG_PAGE_MAX_SIZE: usize = 1024 * 1024;

/// Errors from [`Controller::get_logpage_bytes`].
pub(crate) enum LogPageBytesError {
    Controller(NvmeControllerError),
    TooLarge { size: usize, max: usize },
}

impl From<NvmeControllerError> for LogPageBytesError {
    fn from(e: NvmeControllerError) -> Self {
        LogPageBytesError::Controller(e)
    }
}

pub(crate) struct LogPageInfo<'a> {
    pub(crate) size: usize,
    pub(crate) req: NvmeLogReq<'a>,
}

#[derive(Debug, Clone, Copy)]
pub enum LogPageName {
    Firmware,
    Health,
    WdcDevMgmt,
    WdcEol,
}

impl LogPageName {
    fn as_cstr(&self) -> &CStr {
        match self {
            LogPageName::Firmware => c"firmware",
            LogPageName::Health => c"health",
            LogPageName::WdcDevMgmt => c"wdc/devmgmt",
            LogPageName::WdcEol => c"wdc/eol",
        }
    }
}

/// Determine the actual size of an NVMe logpage, returning the original
/// `NvmeLogReq` and the number of bytes needed to store the logpage data.
fn get_logpage_size<'a>(
    controller: &Controller<'_>,
    disc: &NvmeLogDisc<'_>,
    req: NvmeLogReq<'a>,
) -> Result<(NvmeLogReq<'a>, usize), NvmeControllerError> {
    let mut len = 0;
    match unsafe { nvme_log_disc_size(disc.inner, &mut len) } {
        NVME_LOG_SIZE_K_VAR => {
            // We have a log page with variable length. We need to determine the
            // actual size.
            let mut actual_size_needed = 0;
            let len = len.try_into().expect("32-bit systems unsupported");
            let mut buf = vec![0; len];

            controller.check_result(
                unsafe {
                    nvme_log_req_set_output(
                        req.inner,
                        buf.as_mut_ptr().cast(),
                        len,
                    )
                },
                || format!("failed to set output parameters to determine log length for {:?}", req.page_name),
            )?;
            controller.check_result(
                unsafe { nvme_log_req_exec(req.inner) },
                || format!("failed to execute log request to determine log length for {:?}", req.page_name),
            )?;
            controller.check_result(
                unsafe {
                    nvme_log_disc_calc_size(
                        disc.inner,
                        &mut actual_size_needed,
                        buf.as_mut_ptr().cast(),
                        len,
                    )
                },
                || {
                    format!(
                        "failed to determine full log page length for {:?}",
                        req.page_name
                    )
                },
            )?;

            // Clean up the temporary req output so that it's not left with a
            // dangling pointer after this function returns.
            controller.check_result(
                unsafe { nvme_log_req_clear_output(req.inner) },
                || format!("failed to clear req log output while determining the full log page length for {:?}", req.page_name),
            )?;

            Ok((
                req,
                actual_size_needed
                    .try_into()
                    .expect("32-bit systems unsupported"),
            ))
        }
        _ => Ok((req, len.try_into().expect("32-bit systems unsupported"))),
    }
}

impl<'a> Controller<'a> {
    pub(crate) fn get_logpage(
        &self,
        name: LogPageName,
    ) -> Result<LogPageInfo<'a>, NvmeControllerError> {
        let mut disc_ptr = std::ptr::null_mut();
        let mut req_ptr = std::ptr::null_mut();

        self.check_result(
            unsafe {
                nvme_log_req_init_by_name(
                    self.inner,
                    name.as_cstr().as_ptr(),
                    0,
                    &mut disc_ptr,
                    &mut req_ptr,
                )
            },
            || format!("failed to get logpage {:?}", name),
        )?;

        let disc = NvmeLogDisc { inner: disc_ptr, _phantom: PhantomData };
        let req = NvmeLogReq {
            inner: req_ptr,
            page_name: name,
            _phantom: PhantomData,
        };
        let (req, size) = get_logpage_size(self, &disc, req)?;

        Ok(LogPageInfo { size, req })
    }

    /// Fetch a log page into a buffer of the size libnvme reports, which for
    /// variable length log pages is determined from the page itself.
    ///
    /// That size comes from the device, so it is checked against
    /// [`LOG_PAGE_MAX_SIZE`] before anything is allocated.
    pub(crate) fn get_logpage_bytes(
        &self,
        name: LogPageName,
    ) -> Result<Vec<u8>, LogPageBytesError> {
        let LogPageInfo { size, req } = self.get_logpage(name)?;
        if size > LOG_PAGE_MAX_SIZE {
            return Err(LogPageBytesError::TooLarge {
                size,
                max: LOG_PAGE_MAX_SIZE,
            });
        }
        let mut buf = vec![0u8; size];
        self.check_result(
            unsafe {
                nvme_log_req_set_output(
                    req.inner,
                    buf.as_mut_ptr().cast(),
                    buf.len(),
                )
            },
            || format!("failed to set logpage req size to {size} for {name:?}"),
        )?;
        self.check_result(unsafe { nvme_log_req_exec(req.inner) }, || {
            format!("failed to execute log request for {name:?}")
        })?;
        // Finish the request before handing back the buffer it points at.
        drop(req);
        Ok(buf)
    }
}
