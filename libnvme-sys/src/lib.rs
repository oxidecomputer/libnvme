// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod devinfo;
pub mod nvme;

/// Generate an opaque type for C FFI.
/// These cannot be constructed manually due to the marker and can only be
/// obtained through foreign C functions.
macro_rules! opaque_type {
    ($name:ident, $alias:ident) => {
        #[repr(C)]
        pub struct $name {
            _data: [u8; 0],
            _marker: core::marker::PhantomData<(
                *mut u8,
                core::marker::PhantomPinned,
            )>,
        }

        pub type $alias = $name;
    };
}

// Export this macro within this crate only
pub(crate) use opaque_type;
