// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub(crate) trait FfiPtr: Sized {
    type Ptr;

    unsafe fn from_raw(ptr: Self::Ptr) -> Self;
}
