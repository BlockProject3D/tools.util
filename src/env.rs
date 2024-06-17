// Copyright (c) 2024, BlockProject 3D
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

//! This module contains tools to simplify parsing environment variables.

use std::ffi::{OsStr, OsString};

/// Gets the content of an environment variable.
///
/// Returns None if the variable does not exist.
pub fn get_os<T: AsRef<OsStr>>(name: T) -> Option<OsString> {
    std::env::var_os(name)
}

/// Gets the content of an environment variable.
///
/// Returns None if the variable does not exist or is not valid UTF-8.
pub fn get<T: AsRef<OsStr>>(name: T) -> Option<String> {
    get_os(name).and_then(|v| v.into_string().ok())
}

/// Gets a boolean environment variable.
///
/// Returns None if the variable does not exist or the format is unrecognized.
pub fn get_bool<T: AsRef<OsStr>>(name: T) -> Option<bool> {
    match &*get(name)? {
        "off" | "OFF" | "FALSE" | "false" | "0" => Some(false),
        "on" | "ON" | "TRUE" | "true" | "1" => Some(true),
        _ => None,
    }
}
