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

//! String utilities.

use crate::extension;
use std::borrow::Cow;

/// The range trait that represents all supported range types for sub_nearest method.
pub trait Range {
    /// The actual implementation of nearest substring, see [sub_nearest](StrTools::sub_nearest) for
    /// more information.
    fn sub_nearest<'a>(&self, obj: &'a str) -> &'a str;
}

extension! {
    /// The main StrTools extension trait.
    pub extension StrTools: str {
        /// A substring method which truncates strings at the nearest UTF-8 code rather than
        /// panicking.
        ///
        /// # Panics
        ///
        /// This function still panics if the given range is out of bounds. It however does not panic
        /// if the passed range falls withing a UTF-8 code.
        fn sub_nearest(&self, range: impl Range) -> &str;

        /// A string capitalize function which operates on UTF-8 strings.
        fn capitalise(&self) -> Cow<str>;

        /// A string decapitalize function which operates on UTF-8 strings. This essentially does
        /// the inverse of the [capitalise](StrTools::capitalise) function.
        fn decapitalise(&self) -> Cow<str>;
    }

    /// The main string tools operating on raw byte slices.
    pub extension BufTools: [u8] {
        /// A string capitalize function which operates on ASCII only strings.
        fn capitalise_ascii(&self) -> Cow<[u8]>;

        /// A string decapitalize function which operates on ASCII only strings. This essentially does
        /// the inverse of the [capitalise](BufTools::capitalise_ascii) function.
        fn decapitalise_ascii(&self) -> Cow<[u8]>;
    }
}

fn utf8_max(buf: &[u8], max: usize) -> &[u8] {
    if unsafe { buf.get_unchecked(max.unchecked_sub(1)) } & 0x80 == 0x00 {
        &buf[..max]
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
                &buf[..max]
            } else {
                &buf[..i]
            }
        }
    }
}

fn utf8_min(buf: &[u8], start: usize) -> &[u8] {
    if unsafe { buf.get_unchecked(start) } & 0x80 == 0x00 {
        &buf[start..]
    } else {
        let mut i = start;
        unsafe {
            while i < buf.len() && buf.get_unchecked(i) & 0xC0 == 0x80 {
                i = i.unchecked_add(1);
            }
            &buf[i..]
        }
    }
}

impl Range for std::ops::Range<usize> {
    fn sub_nearest<'a>(&self, obj: &'a str) -> &'a str {
        let bytes = obj.as_bytes();
        let bytes = utf8_max(bytes, self.end);
        if bytes.is_empty() {
            return "";
        }
        let bytes = utf8_min(bytes, self.start);
        unsafe { std::str::from_utf8(bytes).unwrap_unchecked() }
    }
}

impl Range for std::ops::RangeTo<usize> {
    fn sub_nearest<'a>(&self, obj: &'a str) -> &'a str {
        let bytes = obj.as_bytes();
        let bytes = utf8_max(bytes, self.end);
        unsafe { std::str::from_utf8(bytes).unwrap_unchecked() }
    }
}

impl Range for std::ops::RangeFrom<usize> {
    fn sub_nearest<'a>(&self, obj: &'a str) -> &'a str {
        let bytes = obj.as_bytes();
        let bytes = utf8_min(bytes, self.start);
        unsafe { std::str::from_utf8(bytes).unwrap_unchecked() }
    }
}

impl StrTools for str {
    fn sub_nearest(&self, range: impl Range) -> &str {
        range.sub_nearest(self)
    }

    fn capitalise(&self) -> Cow<str> {
        if self.is_empty() {
            return self.into();
        }
        let first = unsafe { self.chars().next().unwrap_unchecked() };
        if first.is_uppercase() {
            self.into()
        } else {
            (self.sub_nearest(..1).to_uppercase() + self.sub_nearest(1..)).into()
        }
    }

    fn decapitalise(&self) -> Cow<str> {
        if self.is_empty() {
            return self.into();
        }
        let first = unsafe { self.chars().next().unwrap_unchecked() };
        if first.is_uppercase() {
            (self.sub_nearest(..1).to_lowercase() + self.sub_nearest(1..)).into()
        } else {
            self.into()
        }
    }
}

impl BufTools for [u8] {
    fn capitalise_ascii(&self) -> Cow<[u8]> {
        if self.is_empty() {
            return self.into();
        }
        if self[0] >= b'A' && self[0] <= b'Z' {
            self.into()
        } else {
            let mut v: Vec<u8> = self.into();
            v[0] = v[0].to_ascii_uppercase();
            v.into()
        }
    }

    fn decapitalise_ascii(&self) -> Cow<[u8]> {
        if self.is_empty() {
            return self.into();
        }
        if self[0] >= b'A' && self[0] <= b'Z' {
            let mut v: Vec<u8> = self.into();
            v[0] = v[0].to_ascii_lowercase();
            v.into()
        } else {
            self.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::string::{BufTools, StrTools};
    use std::borrow::Cow;

    #[test]
    fn sub_basic() {
        let str = "Hello";
        assert_eq!(str.sub_nearest(..1), "H");
        assert_eq!(str.sub_nearest(1..), "ello");
    }

    #[test]
    fn truncate_ascii() {
        let s = "this is a test";
        assert_eq!(s.sub_nearest(..4), "this");
        assert_eq!(&s[4..7], " is");
        assert_eq!(s.sub_nearest(4..7), " is");
    }

    #[test]
    fn truncate_utf8() {
        let msg = "我";
        assert_eq!(msg.sub_nearest(..3), "我");
        assert_eq!(msg.sub_nearest(..1), "");
        assert_eq!(msg.sub_nearest(1..), "");
    }

    #[test]
    fn truncate_utf82() {
        let msg = "我是";
        assert_eq!(msg.sub_nearest(..6), "我是");
        assert_eq!(msg.sub_nearest(..5), "我");
        assert_eq!(msg.sub_nearest(1..), "是");
    }

    #[test]
    fn truncate_utf83() {
        let msg = "我abcd";
        assert_eq!(msg.sub_nearest(..6), "我abc");
        assert_eq!(msg.sub_nearest(1..), "abcd");
        assert_eq!(msg.sub_nearest(1..2), "");
        assert_eq!(msg.sub_nearest(1..4), "a");
        assert_eq!(msg.sub_nearest(1..5), "ab");
        assert_eq!(msg.sub_nearest(1..msg.len()), "abcd");
        assert_eq!(msg.sub_nearest(1..msg.len() - 1), "abc");
    }

    #[test]
    fn basic_capitalize() {
        let msg = "abc";
        let msg1 = "Abc";
        assert_eq!(msg.capitalise(), "Abc");
        assert_eq!(msg1.capitalise(), "Abc");
        assert!(matches!(msg1.capitalise(), Cow::Borrowed(_)));
        assert_eq!(msg1.decapitalise(), "abc");
    }

    #[test]
    fn ascii_capitalize() {
        let msg = "abc";
        let msg1 = "Abc";
        assert_eq!(&*msg.as_bytes().capitalise_ascii(), b"Abc");
        assert_eq!(&*msg1.as_bytes().capitalise_ascii(), b"Abc");
        assert!(matches!(
            msg1.as_bytes().capitalise_ascii(),
            Cow::Borrowed(_)
        ));
        assert_eq!(&*msg1.as_bytes().decapitalise_ascii(), b"abc");
    }
}
