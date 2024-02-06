// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod devinfo;
pub mod nvme;

/// Generate an opaque type for C FFI.
/// These cannot be constructed manually due to the marker and can only be
/// obtained through foreign C functions.
macro_rules! opaque_type {
    ($t:tt) => {
        #[repr(C)]
        pub(crate) struct $t {
            _data: [u8; 0],
            _marker: core::marker::PhantomData<(
                *mut u8,
                core::marker::PhantomPinned,
            )>,
        }
    };
}

// Export this macro within this crate only
pub(crate) use opaque_type;
