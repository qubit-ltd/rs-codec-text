/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use crate::{
    Leb128Codec,
    Leb128DecodeError,
};

/// Buffer-level codec for ZigZag signed integers.
///
/// ZigZag maps signed integers to unsigned integers before writing the payload
/// as unsigned LEB128. This keeps small negative values compact.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ZigZagCodec {
    leb128: Leb128Codec,
}

impl ZigZagCodec {
    /// Creates a non-strict ZigZag codec.
    #[inline]
    pub const fn new() -> Self {
        Self {
            leb128: Leb128Codec::new(),
        }
    }

    /// Creates a ZigZag codec with an explicit LEB128 canonical decoding policy.
    ///
    /// # Parameters
    ///
    /// - `strict`: Whether to reject non-canonical LEB128 payloads.
    #[inline]
    pub const fn with_strict(strict: bool) -> Self {
        Self {
            leb128: Leb128Codec::with_strict(strict),
        }
    }

    /// Reports whether strict canonical decoding is enabled.
    #[must_use]
    #[inline]
    pub const fn strict(self) -> bool {
        self.leb128.strict()
    }

    /// Updates the LEB128 canonical decoding policy.
    #[inline]
    pub fn set_strict(&mut self, strict: bool) {
        self.leb128.set_strict(strict);
    }

    /// Encodes an `i16` value into its ZigZag `u16` representation.
    #[must_use]
    #[inline]
    pub const fn encode_i16(value: i16) -> u16 {
        ((value << 1) ^ (value >> 15)) as u16
    }

    /// Decodes a ZigZag `u16` value into `i16`.
    #[must_use]
    #[inline]
    pub const fn decode_u16(value: u16) -> i16 {
        ((value >> 1) as i16) ^ (-((value & 1) as i16))
    }

    /// Encodes an `i32` value into its ZigZag `u32` representation.
    #[must_use]
    #[inline]
    pub const fn encode_i32(value: i32) -> u32 {
        ((value << 1) ^ (value >> 31)) as u32
    }

    /// Decodes a ZigZag `u32` value into `i32`.
    #[must_use]
    #[inline]
    pub const fn decode_u32(value: u32) -> i32 {
        ((value >> 1) as i32) ^ (-((value & 1) as i32))
    }

    /// Encodes an `i64` value into its ZigZag `u64` representation.
    #[must_use]
    #[inline]
    pub const fn encode_i64(value: i64) -> u64 {
        ((value << 1) ^ (value >> 63)) as u64
    }

    /// Decodes a ZigZag `u64` value into `i64`.
    #[must_use]
    #[inline]
    pub const fn decode_u64(value: u64) -> i64 {
        ((value >> 1) as i64) ^ (-((value & 1) as i64))
    }

    /// Encodes an `i128` value into its ZigZag `u128` representation.
    #[must_use]
    #[inline]
    pub const fn encode_i128(value: i128) -> u128 {
        ((value << 1) ^ (value >> 127)) as u128
    }

    /// Decodes a ZigZag `u128` value into `i128`.
    #[must_use]
    #[inline]
    pub const fn decode_u128(value: u128) -> i128 {
        ((value >> 1) as i128) ^ (-((value & 1) as i128))
    }

    /// Reads an `i16` value from a three-byte maximum-width array.
    ///
    /// # Returns
    ///
    /// Returns the decoded value and the number of bytes consumed.
    #[inline]
    pub fn read_i16_from_array(self, input: [u8; 3]) -> Result<(i16, usize), Leb128DecodeError> {
        self.leb128
            .read_u16_from_array(input)
            .map(|(value, consumed)| (Self::decode_u16(value), consumed))
    }

    /// Reads an `i32` value from a five-byte maximum-width array.
    #[inline]
    pub fn read_i32_from_array(self, input: [u8; 5]) -> Result<(i32, usize), Leb128DecodeError> {
        self.leb128
            .read_u32_from_array(input)
            .map(|(value, consumed)| (Self::decode_u32(value), consumed))
    }

    /// Reads an `i64` value from a ten-byte maximum-width array.
    #[inline]
    pub fn read_i64_from_array(self, input: [u8; 10]) -> Result<(i64, usize), Leb128DecodeError> {
        self.leb128
            .read_u64_from_array(input)
            .map(|(value, consumed)| (Self::decode_u64(value), consumed))
    }

    /// Reads an `i128` value from a nineteen-byte maximum-width array.
    #[inline]
    pub fn read_i128_from_array(self, input: [u8; 19]) -> Result<(i128, usize), Leb128DecodeError> {
        self.leb128
            .read_u128_from_array(input)
            .map(|(value, consumed)| (Self::decode_u128(value), consumed))
    }

    /// Reads a ZigZag encoded `i16` at `index`.
    #[inline]
    pub fn read_i16_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i16, usize)>, Leb128DecodeError> {
        self.leb128
            .read_u16_at(input, index)
            .map(|decoded| decoded.map(|(value, consumed)| (Self::decode_u16(value), consumed)))
    }

    /// Reads a ZigZag encoded `i32` at `index`.
    #[inline]
    pub fn read_i32_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i32, usize)>, Leb128DecodeError> {
        self.leb128
            .read_u32_at(input, index)
            .map(|decoded| decoded.map(|(value, consumed)| (Self::decode_u32(value), consumed)))
    }

    /// Reads a ZigZag encoded `i64` at `index`.
    #[inline]
    pub fn read_i64_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i64, usize)>, Leb128DecodeError> {
        self.leb128
            .read_u64_at(input, index)
            .map(|decoded| decoded.map(|(value, consumed)| (Self::decode_u64(value), consumed)))
    }

    /// Reads a ZigZag encoded `i128` at `index`.
    #[inline]
    pub fn read_i128_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i128, usize)>, Leb128DecodeError> {
        self.leb128
            .read_u128_at(input, index)
            .map(|decoded| decoded.map(|(value, consumed)| (Self::decode_u128(value), consumed)))
    }

    /// Reads an `i16` value at `index` without checking slice bounds.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 3` is in bounds for
    /// `input`.
    #[inline]
    pub unsafe fn read_i16_at_unchecked(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<(i16, usize), Leb128DecodeError> {
        // SAFETY: The caller guarantees the full i16 ZigZag range is in bounds.
        unsafe { self.leb128.read_u16_at_unchecked(input, index) }
            .map(|(value, consumed)| (Self::decode_u16(value), consumed))
    }

    /// Reads an `i32` value at `index` without checking slice bounds.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 5` is in bounds for
    /// `input`.
    #[inline]
    pub unsafe fn read_i32_at_unchecked(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<(i32, usize), Leb128DecodeError> {
        // SAFETY: The caller guarantees the full i32 ZigZag range is in bounds.
        unsafe { self.leb128.read_u32_at_unchecked(input, index) }
            .map(|(value, consumed)| (Self::decode_u32(value), consumed))
    }

    /// Reads an `i64` value at `index` without checking slice bounds.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 10` is in bounds for
    /// `input`.
    #[inline]
    pub unsafe fn read_i64_at_unchecked(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<(i64, usize), Leb128DecodeError> {
        // SAFETY: The caller guarantees the full i64 ZigZag range is in bounds.
        unsafe { self.leb128.read_u64_at_unchecked(input, index) }
            .map(|(value, consumed)| (Self::decode_u64(value), consumed))
    }

    /// Reads an `i128` value at `index` without checking slice bounds.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 19` is in bounds for
    /// `input`.
    #[inline]
    pub unsafe fn read_i128_at_unchecked(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<(i128, usize), Leb128DecodeError> {
        // SAFETY: The caller guarantees the full i128 ZigZag range is in bounds.
        unsafe { self.leb128.read_u128_at_unchecked(input, index) }
            .map(|(value, consumed)| (Self::decode_u128(value), consumed))
    }

    /// Encodes an `i16` value into a three-byte maximum-width array.
    #[must_use]
    #[inline]
    pub fn i16_bytes(self, value: i16) -> ([u8; 3], usize) {
        self.leb128.u16_bytes(Self::encode_i16(value))
    }

    /// Encodes an `i32` value into a five-byte maximum-width array.
    #[must_use]
    #[inline]
    pub fn i32_bytes(self, value: i32) -> ([u8; 5], usize) {
        self.leb128.u32_bytes(Self::encode_i32(value))
    }

    /// Encodes an `i64` value into a ten-byte maximum-width array.
    #[must_use]
    #[inline]
    pub fn i64_bytes(self, value: i64) -> ([u8; 10], usize) {
        self.leb128.u64_bytes(Self::encode_i64(value))
    }

    /// Encodes an `i128` value into a nineteen-byte maximum-width array.
    #[must_use]
    #[inline]
    pub fn i128_bytes(self, value: i128) -> ([u8; 19], usize) {
        self.leb128.u128_bytes(Self::encode_i128(value))
    }

    /// Writes a ZigZag encoded `i16` at `index`.
    #[inline]
    pub fn write_i16_at(self, output: &mut [u8], index: usize, value: i16) -> Option<usize> {
        self.leb128
            .write_u16_at(output, index, Self::encode_i16(value))
    }

    /// Writes a ZigZag encoded `i32` at `index`.
    #[inline]
    pub fn write_i32_at(self, output: &mut [u8], index: usize, value: i32) -> Option<usize> {
        self.leb128
            .write_u32_at(output, index, Self::encode_i32(value))
    }

    /// Writes a ZigZag encoded `i64` at `index`.
    #[inline]
    pub fn write_i64_at(self, output: &mut [u8], index: usize, value: i64) -> Option<usize> {
        self.leb128
            .write_u64_at(output, index, Self::encode_i64(value))
    }

    /// Writes a ZigZag encoded `i128` at `index`.
    #[inline]
    pub fn write_i128_at(self, output: &mut [u8], index: usize, value: i128) -> Option<usize> {
        self.leb128
            .write_u128_at(output, index, Self::encode_i128(value))
    }

    /// Writes an `i16` value at `index` without checking destination bounds.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 3` is in bounds for
    /// `output`.
    #[inline]
    pub unsafe fn write_i16_at_unchecked(
        self,
        output: &mut [u8],
        index: usize,
        value: i16,
    ) -> usize {
        // SAFETY: The caller guarantees the full i16 ZigZag destination range is valid.
        unsafe {
            self.leb128
                .write_u16_at_unchecked(output, index, Self::encode_i16(value))
        }
    }

    /// Writes an `i32` value at `index` without checking destination bounds.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 5` is in bounds for
    /// `output`.
    #[inline]
    pub unsafe fn write_i32_at_unchecked(
        self,
        output: &mut [u8],
        index: usize,
        value: i32,
    ) -> usize {
        // SAFETY: The caller guarantees the full i32 ZigZag destination range is valid.
        unsafe {
            self.leb128
                .write_u32_at_unchecked(output, index, Self::encode_i32(value))
        }
    }

    /// Writes an `i64` value at `index` without checking destination bounds.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 10` is in bounds for
    /// `output`.
    #[inline]
    pub unsafe fn write_i64_at_unchecked(
        self,
        output: &mut [u8],
        index: usize,
        value: i64,
    ) -> usize {
        // SAFETY: The caller guarantees the full i64 ZigZag destination range is valid.
        unsafe {
            self.leb128
                .write_u64_at_unchecked(output, index, Self::encode_i64(value))
        }
    }

    /// Writes an `i128` value at `index` without checking destination bounds.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 19` is in bounds for
    /// `output`.
    #[inline]
    pub unsafe fn write_i128_at_unchecked(
        self,
        output: &mut [u8],
        index: usize,
        value: i128,
    ) -> usize {
        // SAFETY: The caller guarantees the full i128 ZigZag destination range is valid.
        unsafe {
            self.leb128
                .write_u128_at_unchecked(output, index, Self::encode_i128(value))
        }
    }
}

impl Default for ZigZagCodec {
    /// Creates the default non-strict ZigZag codec.
    fn default() -> Self {
        Self::new()
    }
}
