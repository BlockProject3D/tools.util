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

//! Formatting utilities.

use std::mem::MaybeUninit;

/// A structure which acts similar to a [FixedBufStr] but borrows from a buffer instead of owning a
/// stack allocation.
pub struct MemBufStr<'a> {
    len: &'a mut usize,
    buffer: &'a mut [MaybeUninit<u8>],
}

impl<'a> MemBufStr<'a> {
    /// Wraps a memory buffer with its length in a new string buffer.
    ///
    /// # Safety
    ///
    /// It is UB to construct a [MemBufStr] if `len` is not a valid position in the buffer `buffer`.
    /// It is also UB to construct a [MemBufStr] from a `buffer` which does not contain only UTF-8
    /// bytes. If `len` points to uninitialized memory in `buffer` constructing [MemBufStr] is UB.
    pub unsafe fn wrap_uninit(
        len: &'a mut usize,
        buffer: &'a mut [MaybeUninit<u8>],
    ) -> MemBufStr<'a> {
        MemBufStr { buffer, len }
    }

    /// Wraps a memory buffer with its length in a new string buffer.
    ///
    /// # Safety
    ///
    /// It is UB to construct a [MemBufStr] if `len` is not a valid position in the buffer `buffer`.
    /// It is also UB to construct a [MemBufStr] from a `buffer` which does not contain only UTF-8
    /// bytes.
    pub unsafe fn wrap(len: &'a mut usize, buffer: &'a mut [u8]) -> MemBufStr<'a> {
        MemBufStr {
            buffer: std::mem::transmute(buffer),
            len,
        }
    }

    /// Extracts the string from this buffer.
    //type inference works so why should the code look awfully more complex?
    #[allow(clippy::missing_transmute_annotations)]
    pub fn str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(std::mem::transmute(&self.buffer[..*self.len])) }
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
    /// * [MemBufStr](MemBufStr) contains only valid UTF-8 strings so buf must contain only valid UTF-8
    ///   bytes.
    /// * If buf contains invalid UTF-8 bytes, further operations on the log message buffer may
    ///   result in UB.
    //type inference works so why should the code look awfully more complex?
    #[allow(clippy::missing_transmute_annotations)]
    pub unsafe fn write(&mut self, buf: &[u8]) -> usize {
        let len = utf8_max(buf, self.buffer.len() - *self.len);
        unsafe {
            std::ptr::copy_nonoverlapping(
                buf.as_ptr(),
                std::mem::transmute(self.buffer.as_mut_ptr().add(*self.len)),
                len,
            );
        }
        *self.len += len;
        len
    }
}

impl<'a> std::fmt::Write for MemBufStr<'a> {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        unsafe { self.write(value.as_bytes()) };
        Ok(())
    }
}

/// Fixed length string buffer.
#[derive(Clone, Debug)]
pub struct FixedBufStr<const N: usize> {
    len: usize,
    buffer: [MaybeUninit<u8>; N],
}

impl<const N: usize> Default for FixedBufStr<N> {
    fn default() -> Self {
        Self::new()
    }
}

// This function is full of unsafe because it ran slower than expected.
// It appears that even a single subtraction has a HUGE impact on performance in Rust.
// It also appears that having this as a function instead of being inlined multiplies by 2 running
// time.
// Unfortunately that thing is in a hot path within debug.tracing.
#[inline(always)]
fn utf8_max(buf: &[u8], max: usize) -> usize {
    let buf_len = buf.len();
    if buf_len <= max {
        buf_len
    } else if max == 0 {
        0
    } else if unsafe { buf.get_unchecked(max.unchecked_sub(1)) } & 0x80 == 0x00 {
        max
    } else {
        let start = unsafe { max.unchecked_sub(1) };
        let mut i = start;
        unsafe {
            while buf.get_unchecked(i) & 0xC0 == 0x80 {
                i = i.unchecked_sub(1);
            }
            let n = start.unchecked_sub(i);
            if (buf.get_unchecked(i) & 0xF0 == 0xF0 && n == 4)
                || (buf.get_unchecked(i) & 0xE0 == 0xE0 && n == 3)
                || (buf.get_unchecked(i) & 0xC0 == 0xC0 && n == 2)
            {
                max
            } else {
                i
            }
        }
    }
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
    //type inference works so why should the code look awfully more complex?
    #[allow(clippy::missing_transmute_annotations)]
    pub fn str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(std::mem::transmute(&self.buffer[..self.len as _])) }
    }

    /// Constructs this buffer from an existing string.
    //type inference works so why should the code look awfully more complex?
    #[allow(clippy::missing_transmute_annotations)]
    //I believe this is a false-positive, FromStr returns a Result not Self.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Self {
        let mut buffer = FixedBufStr::new();
        let len = utf8_max(value.as_bytes(), N);
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
    ///   bytes.
    /// * If buf contains invalid UTF-8 bytes, further operations on the log message buffer may
    ///   result in UB.
    //type inference works so why should the code look awfully more complex?
    #[allow(clippy::missing_transmute_annotations)]
    pub unsafe fn write(&mut self, buf: &[u8]) -> usize {
        let len = utf8_max(buf, N - self.len);
        unsafe {
            std::ptr::copy_nonoverlapping(
                buf.as_ptr(),
                std::mem::transmute(self.buffer.as_mut_ptr().add(self.len)),
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

#[cfg(test)]
mod tests {
    use crate::format::{FixedBufStr, MemBufStr};
    use std::fmt::Write;
    use std::mem::MaybeUninit;

    #[test]
    fn basic() {
        let mut msg: FixedBufStr<64> = FixedBufStr::new();
        let _ = write!(msg, "this");
        let _ = write!(msg, " is");
        let _ = write!(msg, " a");
        let _ = write!(msg, " test");
        assert_eq!(msg.str(), "this is a test");
    }

    #[test]
    fn basic_mem() {
        let mut buf: [MaybeUninit<u8>; 64] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut len = 0;
        let mut msg = unsafe { MemBufStr::wrap_uninit(&mut len, &mut buf) };
        let _ = write!(msg, "this");
        let _ = write!(msg, " is");
        let _ = write!(msg, " a");
        let _ = write!(msg, " test");
        assert_eq!(msg.str(), "this is a test");
    }

    #[test]
    fn truncate_ascii() {
        let mut msg: FixedBufStr<4> = FixedBufStr::new();
        let _ = write!(msg, "this");
        let _ = write!(msg, " is");
        let _ = write!(msg, " a");
        let _ = write!(msg, " test");
        assert_eq!(msg.str().len(), 4);
        assert_eq!(msg.str(), "this");
    }

    #[test]
    fn truncate_ascii_mem() {
        let mut buf = [0; 4];
        let mut len = 0;
        let mut msg = unsafe { MemBufStr::wrap(&mut len, &mut buf) };
        let _ = write!(msg, "this");
        let _ = write!(msg, " is");
        let _ = write!(msg, " a");
        let _ = write!(msg, " test");
        assert_eq!(msg.str().len(), 4);
        assert_eq!(msg.str(), "this");
    }

    #[test]
    fn truncate_utf8_exact() {
        let mut msg: FixedBufStr<3> = FixedBufStr::new();
        let _ = write!(msg, "我");
        assert_eq!(msg.str().len(), 3);
        assert_eq!(msg.str(), "我");
    }

    #[test]
    fn truncate_utf8_exact_mem() {
        let mut buf = [0; 3];
        let mut len = 0;
        let mut msg = unsafe { MemBufStr::wrap(&mut len, &mut buf) };
        let _ = write!(msg, "我");
        assert_eq!(msg.str().len(), 3);
        assert_eq!(msg.str(), "我");
    }

    #[test]
    fn truncate_utf8_exact2() {
        let mut msg: FixedBufStr<6> = FixedBufStr::new();
        let _ = write!(msg, "我是");
        assert_eq!(msg.str().len(), 6);
        assert_eq!(msg.str(), "我是");
    }

    #[test]
    fn truncate_utf8_exact2_mem() {
        let mut buf = [0; 6];
        let mut len = 0;
        let mut msg = unsafe { MemBufStr::wrap(&mut len, &mut buf) };
        let _ = write!(msg, "我是");
        assert_eq!(msg.str().len(), 6);
        assert_eq!(msg.str(), "我是");
    }

    #[test]
    fn truncate_utf8_exact3() {
        let mut msg: FixedBufStr<6> = FixedBufStr::new();
        let _ = write!(msg, "我abcd");
        assert_eq!(msg.str().len(), 6);
        assert_eq!(msg.str(), "我abc");
    }

    #[test]
    fn truncate_utf8_exact3_mem() {
        let mut buf = [0; 6];
        let mut len = 0;
        let mut msg = unsafe { MemBufStr::wrap(&mut len, &mut buf) };
        let _ = write!(msg, "我abcd");
        assert_eq!(msg.str().len(), 6);
        assert_eq!(msg.str(), "我abc");
    }

    #[test]
    fn truncate_utf8() {
        let mut msg: FixedBufStr<4> = FixedBufStr::new();
        let _ = write!(msg, "我是");
        assert_eq!(msg.str().len(), 3);
        assert_eq!(msg.str(), "我");
    }

    #[test]
    fn truncate_utf8_mem() {
        let mut buf = [0; 4];
        let mut len = 0;
        let mut msg = unsafe { MemBufStr::wrap(&mut len, &mut buf) };
        let _ = write!(msg, "我是");
        assert_eq!(msg.str().len(), 3);
        assert_eq!(msg.str(), "我");
    }
}
