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

//! Formatting utilities.

use std::mem::MaybeUninit;

/// Fixed length string buffer.
#[derive(Clone, Debug)]
pub struct FixedBufStr<const N: usize> {
    len: usize,
    buffer: [MaybeUninit<u8>; N],
}

impl<const N: usize> FixedBufStr<N> {
    /// Creates a new fixed length string buffer.
    pub fn new() -> FixedBufStr<N> {
        FixedBufStr {
            buffer: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Extracts the string from this buffer.
    pub fn str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(std::mem::transmute(&self.buffer[..self.len as _])) }
    }

    /// Constructs this buffer from an existing string.
    pub fn from_str(value: &str) -> Self {
        let mut buffer = FixedBufStr::new();
        let len = std::cmp::min(value.len(), N);
        unsafe {
            std::ptr::copy_nonoverlapping(
                value.as_ptr(),
                std::mem::transmute(buffer.buffer.as_mut_ptr()),
                len,
            );
        }
        buffer.len = len as _;
        buffer
    }

    /// Appends a raw byte buffer at the end of this string buffer.
    ///
    /// Returns the number of bytes written.
    ///
    /// # Arguments
    ///
    /// * `buf`: the raw byte buffer to append.
    ///
    /// returns: usize
    ///
    /// # Safety
    ///
    /// * [FixedBufStr](FixedBufStr) contains only valid UTF-8 strings so buf must contain only valid UTF-8
    /// bytes.
    /// * If buf contains invalid UTF-8 bytes, further operations on the log message buffer may
    /// result in UB.
    pub unsafe fn write(&mut self, buf: &[u8]) -> usize {
        let len = std::cmp::min(buf.len(), N - self.len);
        unsafe {
            std::ptr::copy_nonoverlapping(
                buf.as_ptr(),
                std::mem::transmute(self.buffer.as_mut_ptr().offset(self.len as _)),
                len,
            );
        }
        self.len += len;
        len
    }
}

impl<const N: usize> std::fmt::Write for FixedBufStr<N> {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        unsafe { self.write(value.as_bytes()) };
        Ok(())
    }
}

/// An io [Write](std::io::Write) to fmt [Write](std::fmt::Write).
///
/// This may look like a hack but is a requirement for pathological APIs such as presented by the
/// time crate.
pub struct IoToFmt<W: std::fmt::Write>(W);

impl<W: std::fmt::Write> IoToFmt<W> {
    /// Create a new [IoToFmt](IoToFmt) wrapper.
    ///
    /// # Arguments
    ///
    /// * `w`: target fmt [Write](std::fmt::Write) to write into.
    ///
    /// returns: IoToFmt<W>
    pub fn new(w: W) -> Self {
        Self(w)
    }

    /// Extracts the underlying [Write](std::fmt::Write).
    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<W: std::fmt::Write> std::io::Write for IoToFmt<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let str = std::str::from_utf8(buf)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        self.0
            .write_str(str)
            .map(|_| str.len())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
