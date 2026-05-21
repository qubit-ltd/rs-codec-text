/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
/// Byte order used when serializing multi-byte Unicode code units.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::ByteOrder;
///
/// let bytes = ByteOrder::LittleEndian.u16_bytes(0x0041);
/// assert_eq!([0x41, 0x00], bytes);
/// assert_eq!(0x0041, ByteOrder::LittleEndian.read_u16(&bytes));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ByteOrder {
    /// Most significant byte first.
    BigEndian,

    /// Least significant byte first.
    LittleEndian,
}

impl ByteOrder {
    /// Reads a `u16` value from two bytes using this byte order.
    ///
    /// # Parameters
    ///
    /// - `bytes`: The source byte slice. The first two bytes are read.
    ///
    /// # Returns
    ///
    /// Returns the decoded `u16` value.
    ///
    /// # Panics
    ///
    /// Panics if `bytes` has fewer than two bytes.
    #[inline]
    pub fn read_u16(self, bytes: &[u8]) -> u16 {
        match self {
            Self::BigEndian => u16::from_be_bytes([bytes[0], bytes[1]]),
            Self::LittleEndian => u16::from_le_bytes([bytes[0], bytes[1]]),
        }
    }

    /// Reads a `u32` value from four bytes using this byte order.
    ///
    /// # Parameters
    ///
    /// - `bytes`: The source byte slice. The first four bytes are read.
    ///
    /// # Returns
    ///
    /// Returns the decoded `u32` value.
    ///
    /// # Panics
    ///
    /// Panics if `bytes` has fewer than four bytes.
    #[inline]
    pub fn read_u32(self, bytes: &[u8]) -> u32 {
        match self {
            Self::BigEndian => u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            Self::LittleEndian => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        }
    }

    /// Converts a `u16` value to bytes using this byte order.
    ///
    /// # Parameters
    ///
    /// - `value`: The value to serialize.
    ///
    /// # Returns
    ///
    /// Returns two bytes in this byte order.
    #[inline]
    pub const fn u16_bytes(self, value: u16) -> [u8; 2] {
        match self {
            Self::BigEndian => value.to_be_bytes(),
            Self::LittleEndian => value.to_le_bytes(),
        }
    }

    /// Converts a `u32` value to bytes using this byte order.
    ///
    /// # Parameters
    ///
    /// - `value`: The value to serialize.
    ///
    /// # Returns
    ///
    /// Returns four bytes in this byte order.
    #[inline]
    pub const fn u32_bytes(self, value: u32) -> [u8; 4] {
        match self {
            Self::BigEndian => value.to_be_bytes(),
            Self::LittleEndian => value.to_le_bytes(),
        }
    }
}
