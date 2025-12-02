// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Functions to convert a byte-string to and from hexadecimal form.

use std::fmt::Write;

/// Represent a sequence of bytes as an hexadecimal string.
pub fn to_hex(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);
    bytes.iter().for_each(|b| {
        write!(result, "{b:02x}").unwrap();
    });
    result
}

/// Convert a hexadecimal string into a byte array.
/// For use in constants. Panics on invalid data.
pub const fn into_bytes<const N: usize>(s: &str) -> [u8; N] {
    const fn nibble(nibble: u8) -> u8 {
        match nibble {
            b'0'..=b'9' => nibble - b'0',
            b'a'..=b'f' => nibble - b'a' + 10,
            b'A'..=b'F' => nibble - b'A' + 10,
            _ => panic!("invalid hex data")
        }
    }

    if s.len() % 2 != 0 {
        panic!("invalid hex data: odd string length");
    }

    let mut arr = [0; N];

    let mut i = 0;
    while i < arr.len() / 2 {
        arr[i] = nibble(s.as_bytes()[i * 2]) << 4 + nibble(s.as_bytes()[i * 2 + 1]);

        i += 1;
    }

    arr
}
