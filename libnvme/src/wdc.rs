// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::Deref;

use crate::{
    controller::{NvmeControllerError, WriteLockedController},
    error::LibraryError,
};

use libnvme_sys::nvme::{nvme_wdc_resize_get, nvme_wdc_resize_set};

impl<'a> WriteLockedController<'a> {
    pub fn wdc_resize_set(&self, size: u32) -> Result<(), NvmeControllerError> {
        let controller = self.deref();
        controller.check_result(
            unsafe { nvme_wdc_resize_set(controller.inner, size) },
            || "failed to resize wdc device",
        )
    }

    pub fn wdc_resize_get(&self) -> Result<u32, NvmeControllerError> {
        let mut size = 0;
        let controller = self.deref();
        controller
            .check_result(
                unsafe { nvme_wdc_resize_get(controller.inner, &mut size) },
                || "failed to get size of wdc device",
            )
            .map(|_| size)
    }
}
