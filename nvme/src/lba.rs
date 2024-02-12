// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::marker::PhantomData;

use crate::{controller_info::ControllerInfo, util::FfiPtr};

use nvme_sys::nvme::*;

pub enum Performance {
    Best,
    Better,
    Good,
    Degraded,
    Unknown,
}

impl From<u32> for Performance {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Best,
            1 => Self::Better,
            2 => Self::Good,
            3 => Self::Degraded,
            _ => Self::Unknown,
        }
    }
}

pub struct LbaFormat<'info> {
    lba: *const nvme_nvm_lba_fmt_t,
    _phantom: PhantomData<&'info ControllerInfo<'info>>,
}

impl<'info> LbaFormat<'info> {
    pub fn id(&self) -> u32 {
        unsafe { nvme_nvm_lba_fmt_id(self.lba) }
    }

    pub fn meta_size(&self) -> u32 {
        unsafe { nvme_nvm_lba_fmt_meta_size(self.lba) }
    }

    pub fn data_size(&self) -> u64 {
        unsafe { nvme_nvm_lba_fmt_data_size(self.lba) }
    }

    pub fn rel_perf(&self) -> Performance {
        unsafe { nvme_nvm_lba_fmt_rel_perf(self.lba) }.into()
    }
}

impl<'info> FfiPtr for LbaFormat<'info> {
    type Ptr = *const nvme_nvm_lba_fmt_t;

    unsafe fn from_raw(ptr: Self::Ptr) -> Self {
        Self { lba: ptr, _phantom: PhantomData }
    }
}
