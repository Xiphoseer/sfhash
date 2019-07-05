// The "SuperFastHash" byte sequence hash function
// Copyright (C) 2003  Paul Hsieh
// Copyright (C) 2019  Xiphoseer
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

//! This crate contains the `Hsieh Hash` or `SuperFastHash` function
//! created by Paul Hsieh and presented at <http://www.azillionmonkeys.com/qed/hash.html>
//!
//! ```rust
//! use hsieh_hash::digest;
//!
//! let hash = digest("Hello World!".as_bytes());
//! assert_eq!(hash, 1774740540);
//! ```
//!
//! The hash value is initialized with the lenght of the input, so
//! the algorithm cannot be used incrementally.

/// Calculate the Hash of a byte slice
pub fn digest(mut data: &[u8]) -> u32 {
    let len: u32 = data.len() as u32;
    let mut hash: u32 = len;

    if len == 0 {
        return 0;
    }

    let remainder = len & 3;
    let block_count = len >> 2;

    /* Main loop */
    for _i in 0..block_count {
        hash += u16::from_le_bytes([data[0], data[1]]) as u32;
        let temp = (u16::from_le_bytes([data[2], data[3]]) as u32) << 11 ^ hash;
        hash = hash << 16 ^ temp;
        data = &data[4..];
        hash += hash >> 11;
    }

    /* Handle end cases */
    match remainder {
        0 => {
            // Do nothing
        }
        1 => {
            hash += data[0] as u32;
            hash ^= hash << 10;
            hash += hash >> 1;
        },
        2 => {
            hash += u16::from_le_bytes([data[0], data[1]]) as u32;
            hash ^= hash << 11;
            hash += hash >> 17;
        },
        3 => {
            hash += u16::from_le_bytes([data[0], data[1]]) as u32;
            hash ^= hash << 16;
            hash ^= (data[2] as u32) << 18;
            hash += hash >> 11;
        },
        _ => {
            panic!("Please investigate");
        }
    }

    /* Force "avalanching" of final 127 bits */
    hash ^= hash << 3;
    hash += hash >> 5;
    hash ^= hash << 4;
    hash += hash >> 17;
    hash ^= hash << 25;
    hash += hash >> 6;

    return hash;
}

#[cfg(test)]
mod hiesh_tests {
    use super::digest;

    #[test]
    fn test_digest() {
        assert_eq!(digest("Hello World!".as_bytes()), 1774740540);
        assert_eq!(digest("Hsieh Hash".as_bytes()), 1552477933);
        assert_eq!(digest("SuperFastHash".as_bytes()), 2245601745);
    }
}