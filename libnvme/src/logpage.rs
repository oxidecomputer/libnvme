// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{ffi::CStr, marker::PhantomData};

use libnvme_sys::nvme::*;

use crate::{controller::Controller, error::LibraryError, NvmeError};

pub(crate) struct NvmeLogReq<'a> {
    pub(crate) inner: *mut nvme_log_req_t,
    // The log page being requested
    pub(crate) page_name: LogPageName,
    // When determining the actual size of a `NVME_LOG_SIZE_K_VAR` log page
    // we need to supply libnvme with a temporary buffer. To prevent a dangling
    // pointer we are holding onto this buffer even though it won't be used
    // directly by any future consumers as the library itself should be calling
    // `nvme_log_req_set_output` again with the appropriately sized buffer.
    buf: Vec<u8>,
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

pub(crate) struct LogPageInfo<'a> {
    pub(crate) size: usize,
    pub(crate) req: NvmeLogReq<'a>,
}

#[derive(Debug, Clone, Copy)]
pub enum LogPageName {
    Firmware,
}

impl LogPageName {
    fn as_cstr(&self) -> &CStr {
        match self {
            LogPageName::Firmware => c"firmware",
        }
    }
}

fn get_logpage_size(
    controller: &Controller<'_>,
    disc: &NvmeLogDisc<'_>,
    req: &mut NvmeLogReq<'_>,
) -> Result<usize, NvmeError> {
    let mut len = 0;
    match unsafe { nvme_log_disc_size(disc.inner, &mut len) } {
        NVME_LOG_SIZE_K_VAR => {
            // We have a log page with variable length. We need to determine the
            // actual size.
            let mut actual_size_needed = 0;
            let len = len.try_into().expect("32-bit systems unsupported");
            req.buf.resize(len, 0);

            controller.check_result(
                unsafe {
                    nvme_log_req_set_output(
                        req.inner,
                        req.buf.as_mut_ptr().cast(),
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
                        req.buf.as_mut_ptr().cast(),
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

            Ok(actual_size_needed
                .try_into()
                .expect("32-bit systems unsupported"))
        }
        _ => Ok(len.try_into().expect("32-bit systems unsupported")),
    }
}

impl<'a> Controller<'a> {
    pub(crate) fn get_logpage(
        &self,
        name: LogPageName,
    ) -> Result<LogPageInfo<'a>, NvmeError> {
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
        let mut req = NvmeLogReq {
            inner: req_ptr,
            page_name: name,
            buf: Vec::new(),
            _phantom: PhantomData,
        };
        let size = get_logpage_size(self, &disc, &mut req)?;

        Ok(LogPageInfo { size, req })
    }
}
