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

//! Error umbrella type generation macro.

//Because Rust macros are a peace of shit.
/// This macro is internal and called by another macro.
#[macro_export]
macro_rules! typed_ident {
    ($t: ty, $name: ident) => {
        $name
    };
}

//Because Rust macros are a peace of shit.
/// This macro is internal and called by another macro.
#[macro_export]
macro_rules! hack_rust_buggy_macros {
    ($name: ident, $ty: ident, $e: ident, $data: ty) => {
        impl $e<$data> for $name {
            fn from(value: $data) -> Self {
                Self::$ty(value)
            }
        }
    };
    ($name: ident, $ty: ty, $($e: ident)?, $($data: ty)?) => {};
}

/// Generates a simple enum which maps multiple error types and implements [Error](std::error::Error) and
/// [Display](std::fmt::Display) automatically. This optionally can generate [From](From) implementations
/// on demand.
///
/// # Example
///
/// ```
/// use bp3d_util::simple_error;
/// simple_error!(
///     /// Doc.
///     TestError {
///         /// This is a doc comment which is recorded by the macro.
///         Untyped => "untyped variant",
///         /// Another doc comment.
///         (impl From) Io(std::io::Error) => "io error {}",
///         Other(u8) => "other u8 error {}"
///     }
/// );
/// println!("{}", TestError::Untyped);
/// ```
#[macro_export]
macro_rules! simple_error {
    (
        $(#[$meta: meta])*
        $vis: vis $name: ident {
            $(
                $(#[$field_meta: meta])*
                $((impl $e: ident))? $ty: ident $(($data: ty))? => $desc: literal
            ),*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug)]
        $vis enum $name {
            $(
                $(#[$field_meta])*
                $ty $(($data))?
            ),*
        }

        $(
            $crate::hack_rust_buggy_macros!($name, $ty, $($e)?, $($data)?);
        )*

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$ty $(($crate::typed_ident!($data, e)))? => write!(f, $desc $(, $crate::typed_ident!($data, e))?) ),*
                }
            }
        }

        impl std::error::Error for $name {}
    };
}
