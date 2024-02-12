// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use thiserror::Error;

pub(crate) trait LibraryError {
    type Error;

    fn get_errmsg(&self) -> String;
    fn get_syserr(&self) -> i32;
    fn to_error(&self, internal: InternalError) -> Self::Error;
    fn fatal_context<C: Into<String>>(&self, context: C) -> Self::Error {
        let errmsg = self.get_errmsg();
        let syserr = self.get_syserr();
        let syserr = if syserr == 0 {
            "no system errno".to_string()
        } else {
            std::io::Error::from_raw_os_error(syserr).to_string()
        };
        self.to_error(InternalError { context: context.into(), syserr, errmsg })
    }
    fn with_fatal_context<C: Into<String>, F: FnOnce() -> C>(
        &self,
        f: F,
    ) -> Self::Error {
        self.fatal_context(f())
    }
}

#[derive(Debug, Error)]
#[error("{context}: {errmsg} ({syserr})")]
pub struct InternalError {
    context: String,
    syserr: String,
    errmsg: String,
}

// Add a blanket implementation that works on references as well
impl<T: LibraryError> LibraryError for &T {
    type Error = T::Error;

    fn get_errmsg(&self) -> String {
        (*self).get_errmsg()
    }

    fn get_syserr(&self) -> i32 {
        (*self).get_syserr()
    }

    fn to_error(&self, internal: InternalError) -> Self::Error {
        (*self).to_error(internal)
    }
}
