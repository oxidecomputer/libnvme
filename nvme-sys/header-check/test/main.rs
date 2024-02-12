// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// This is to disable a warning about the unused offset_of! macro that
// gets defined for us. Our build.rs file elides a bunch of the opaque
// types so there is nothing to check for currently.
#![allow(unused_macros)]

extern crate nvme_sys;

use nvme_sys::nvme::*;

include!(concat!(env!("OUT_DIR"), "/main.rs"));
