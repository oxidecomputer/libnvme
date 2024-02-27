// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use thiserror::Error;

pub(crate) trait LibraryError {
    type Error;

    fn get_errmsg(&self) -> String;
    fn get_syserr(&self) -> i32;
    fn current_error(&self, internal: InternalError) -> Self::Error;
    fn fatal_context<C: Into<String>>(&self, context: C) -> Self::Error {
        let errmsg = self.get_errmsg();
        let syserr = self.get_syserr();
        let syserr = std::io::Error::from_raw_os_error(syserr);
        self.current_error(InternalError {
            context: context.into(),
            syserr,
            errmsg,
        })
    }

    fn check_result<C, F>(
        &self,
        result: bool,
        context: F,
    ) -> Result<(), Self::Error>
    where
        C: Into<String>,
        F: FnOnce() -> C,
    {
        if result {
            Ok(())
        } else {
            Err(self.fatal_context(context()))
        }
    }
}

#[derive(Debug, Error)]
#[error("{context}: {errmsg} [{syserr}]")]
pub struct InternalError {
    context: String,
    syserr: std::io::Error,
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

    fn current_error(&self, internal: InternalError) -> Self::Error {
        (*self).current_error(internal)
    }
}
