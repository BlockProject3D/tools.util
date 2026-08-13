// Copyright (c) 2026, BlockProject 3D
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

//! Simple memory tools.

use std::process::abort;

struct AbortUnwind;

impl Drop for AbortUnwind {
    fn drop(&mut self) {
        eprintln!("Attempt to replace memory with panic!");
        abort()
    }
}

/// Replaces content of `dest` by value mapped with `f`.
/// This function aborts if `f` panics.
///
/// # Arguments
///
/// * `dest`: the destination to replace.
/// * `f`: the replace function.
///
/// returns: ()
pub fn replace_with<T, F: FnOnce(T) -> T>(dest: &mut T, f: F) {
    unsafe {
        let old = std::ptr::read(dest);
        if cfg!(panic = "abort") {
            std::ptr::write(dest, f(old));
        } else {
            let p = AbortUnwind;
            let new = f(old);
            std::mem::forget(p);
            std::ptr::write(dest, new);
        }
    }
}

/// Replaces content of `dest` by value mapped with `f`.
///
/// # Arguments
///
/// * `dest`: the destination to replace.
/// * `f`: the replace function.
///
/// returns: ()
///
/// # Safety
///
/// If `f` panics, then this function is UB. Use this variant when absolutely sure `f` cannot panic.
#[inline]
pub unsafe fn replace_with_unchecked<T, F: FnOnce(T) -> T>(dest: &mut T, f: F) {
    let old = std::ptr::read(dest);
    std::ptr::write(dest, f(old));
}

#[cfg(test)]
mod tests {
    use crate::mem::replace_with;

    #[test]
    fn basic() {
        let mut value: Vec<String> = Vec::with_capacity(12);
        assert_eq!(value.len(), 0);
        assert_eq!(value.capacity(), 12);
        replace_with(&mut value, |mut value| {
            value.push("test".into());
            value
        });
        assert_eq!(value.len(), 1);
        assert_eq!(value.capacity(), 12);
    }
}
