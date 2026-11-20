// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate libnvme_sys;

use libnvme_sys::identify::*;
use libnvme_sys::nvme::*;

include!(concat!(env!("OUT_DIR"), "/main.rs"));
