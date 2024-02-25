// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{controller::LockedController, error::LibraryError, NvmeError};

use libnvme_sys::nvme::{nvme_wdc_resize_get, nvme_wdc_resize_set};

impl<'a> LockedController<'a> {
    pub fn wdc_resize_set(&self, size: u32) -> Result<(), NvmeError> {
        let controller =
            self.controller.as_ref().expect("controller is locked");
        controller.check_result(
            unsafe { nvme_wdc_resize_set(controller.inner, size) },
            || "failed to resize wdc device",
        )
    }

    pub fn wdc_resize_get(&self) -> Result<u32, NvmeError> {
        let mut size = 0;
        let controller =
            self.controller.as_ref().expect("controller is locked");
        controller
            .check_result(
                unsafe { nvme_wdc_resize_get(controller.inner, &mut size) },
                || "failed to get size of wdc device",
            )
            .map(|_| size)
    }
}
