// Copyright (c) 2025, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! Result utilities.

use crate::extension;
use std::error::Error;

extension! {
    /// Result extensions designed to simplify console based tools.
    pub extension ResultExt<T, E>: Result<T, E> {
        /// Expects a given result to unwrap without issues, in case the result is an error,
        /// this function exits the program.
        ///
        /// # Arguments
        ///
        /// * `msg`: a failure context message.
        /// * `code`: the exit code to exit the program with in case of error.
        ///
        /// returns: T the value if no errors have occurred.
        fn expect_exit(self, msg: &str, code: i32) -> T;
    }
}

impl<T, E: Error> ResultExt<T, E> for Result<T, E> {
    fn expect_exit(self, msg: &str, code: i32) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {}", msg, e);
                std::process::exit(code);
            }
        }
    }
}

/// Generates a match block which returns errors to circumvent rust borrow checker defects.
#[macro_export]
macro_rules! try_res {
    ($value: expr => |$e: ident| $err: expr) => {
        match $value {
            Ok(v) => v,
            Err($e) => return Err($err),
        }
    };
}

/// Generates a match block which returns errors to circumvent rust borrow checker defects.
#[macro_export]
macro_rules! try_opt {
    ($value: expr => $err: expr) => {
        match $value {
            Some(v) => v,
            None => return Err($err),
        }
    };
}
