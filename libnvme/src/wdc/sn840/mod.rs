// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Log pages of the WDC Ultrastar DC SN840, with layouts from illumos
//! `sys/nvme/wdc_sn840.h`.

mod devmgmt;
mod eol;

pub use devmgmt::{
    DevMgmtError, DevMgmtLog, ThermalThrottleStatus, UnknownEntry, UnknownValue,
};
pub use eol::{EolError, EolLog, EolState};
