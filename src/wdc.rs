// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{
    controller::LockedController,
    error::LibraryError,
    ffi::nvme::{nvme_wdc_resize_get, nvme_wdc_resize_set},
    NvmeError,
};

impl LockedController {
    pub fn wdc_resize_set(&self, size: u32) -> Result<(), NvmeError> {
        let controller =
            self.controller.as_ref().expect("controller is locked");
        match unsafe { nvme_wdc_resize_set(controller.0, size) } {
            true => Ok(()),
            false => {
                Err(controller.fatal_context("failed to resize wdc device"))
            }
        }
    }

    pub fn wdc_resize_get(&self) -> Result<u32, NvmeError> {
        let mut size = 0;
        let controller =
            self.controller.as_ref().expect("controller is locked");
        match unsafe { nvme_wdc_resize_get(controller.0, &mut size) } {
            true => Ok(size),
            false => {
                Err(controller
                    .fatal_context("failed to get size of wdc device"))
            }
        }
    }
}
