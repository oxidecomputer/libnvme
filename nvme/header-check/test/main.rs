// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Only constants are checked, which leaves a generated helper unused.
#![allow(dead_code)]

extern crate nvme;

use nvme::*;

include!(concat!(env!("OUT_DIR"), "/main.rs"));
