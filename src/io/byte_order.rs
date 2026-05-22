/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
/// Byte order used when serializing multi-byte integer values.
///
/// `ByteOrder` exposes three read API families:
///
/// - `read_u32_from_array` accepts a fixed-width byte array. Use it when the
///   caller already has `[u8; N]`; no slice bounds checks are involved.
/// - `read_u32_at` accepts a byte slice and an absolute offset. Use it at public
///   or untrusted-input boundaries where short buffers should return `None`.
/// - `read_u32_at_unchecked` skips bounds checks. Use it only in hot paths after
///   the caller has already validated the full byte range.
///
/// The write API mirrors the same split: `u32_bytes` produces a fixed byte array,
/// `write_u32_at` checks slice bounds, and `write_u32_at_unchecked` is for
/// validated internal paths.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::ByteOrder;
///
/// let value = ByteOrder::BigEndian.read_u32_from_array([0x00, 0x01, 0xf6, 0x00]);
/// assert_eq!(0x0001_f600, value);
///
/// let packet = [0xaa, 0x12, 0x34, 0xbb];
/// assert_eq!(Some(0x1234), ByteOrder::BigEndian.read_u16_at(&packet, 1));
/// assert_eq!(None, ByteOrder::BigEndian.read_u32_at(&packet, 1));
///
/// let mut output = [0_u8; 4];
/// assert_eq!(Some(()), ByteOrder::LittleEndian.write_u32_at(&mut output, 0, value));
/// assert_eq!([0x00, 0xf6, 0x01, 0x00], output);
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ByteOrder {
    /// Most significant byte first.
    BigEndian,

    /// Least significant byte first.
    LittleEndian,
}

impl ByteOrder {
    /// Reads a `u16` value from a fixed-width byte array.
    ///
    /// Use this API when the caller already has exactly two bytes, for example
    /// after array pattern matching, `first_chunk`, or a parser that stores fixed
    /// fields as arrays. The array length is known at compile time, so converting
    /// it through `u16::from_be_bytes` or `u16::from_le_bytes` does not need slice
    /// bounds checks.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Exactly two bytes in this byte order.
    ///
    /// # Returns
    ///
    /// Returns the decoded `u16` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// assert_eq!(0x1234, ByteOrder::BigEndian.read_u16_from_array([0x12, 0x34]));
    /// assert_eq!(0x1234, ByteOrder::LittleEndian.read_u16_from_array([0x34, 0x12]));
    /// ```
    #[inline]
    pub const fn read_u16_from_array(self, bytes: [u8; 2]) -> u16 {
        match self {
            Self::BigEndian => u16::from_be_bytes(bytes),
            Self::LittleEndian => u16::from_le_bytes(bytes),
        }
    }

    /// Reads a `u32` value from a fixed-width byte array.
    ///
    /// Use this API when the caller already has exactly four bytes. It is the
    /// lowest-friction safe API for fixed protocol fields and avoids runtime
    /// slice length checks at this layer.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Exactly four bytes in this byte order.
    ///
    /// # Returns
    ///
    /// Returns the decoded `u32` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let bytes = [0x00, 0x01, 0xf6, 0x00];
    /// assert_eq!(0x0001_f600, ByteOrder::BigEndian.read_u32_from_array(bytes));
    /// ```
    #[inline]
    pub const fn read_u32_from_array(self, bytes: [u8; 4]) -> u32 {
        match self {
            Self::BigEndian => u32::from_be_bytes(bytes),
            Self::LittleEndian => u32::from_le_bytes(bytes),
        }
    }

    /// Reads a `u64` value from a fixed-width byte array.
    ///
    /// Use this API when the caller already has exactly eight bytes. Like the
    /// smaller fixed-array readers, it keeps bounds responsibility outside this
    /// method.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Exactly eight bytes in this byte order.
    ///
    /// # Returns
    ///
    /// Returns the decoded `u64` value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let bytes = [1, 2, 3, 4, 5, 6, 7, 8];
    /// assert_eq!(0x0102_0304_0506_0708, ByteOrder::BigEndian.read_u64_from_array(bytes));
    /// ```
    #[inline]
    pub const fn read_u64_from_array(self, bytes: [u8; 8]) -> u64 {
        match self {
            Self::BigEndian => u64::from_be_bytes(bytes),
            Self::LittleEndian => u64::from_le_bytes(bytes),
        }
    }

    /// Reads a `u16` value at `index` from a byte slice.
    ///
    /// Use this safe API at public boundaries, while parsing untrusted input, or
    /// whenever short buffers should be represented as absence instead of panic.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Source byte slice.
    /// - `index`: Absolute byte offset where the two-byte value starts.
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` when `index..index + 2` is in bounds. Returns `None`
    /// when `index` is out of bounds or fewer than two bytes remain.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let bytes = [0xaa, 0x12, 0x34];
    /// assert_eq!(Some(0x1234), ByteOrder::BigEndian.read_u16_at(&bytes, 1));
    /// assert_eq!(None, ByteOrder::BigEndian.read_u16_at(&bytes, 2));
    /// ```
    #[inline]
    pub fn read_u16_at(self, bytes: &[u8], index: usize) -> Option<u16> {
        let chunk = bytes.get(index..)?.first_chunk::<2>()?;
        Some(self.read_u16_from_array(*chunk))
    }

    /// Reads a `u32` value at `index` from a byte slice.
    ///
    /// Use this safe API when the slice length has not already been validated by
    /// a higher layer. It performs the necessary bounds check and returns `None`
    /// for incomplete values.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Source byte slice.
    /// - `index`: Absolute byte offset where the four-byte value starts.
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` when `index..index + 4` is in bounds. Returns `None`
    /// when `index` is out of bounds or fewer than four bytes remain.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let bytes = [0xaa, 0x00, 0x01, 0xf6, 0x00];
    /// assert_eq!(Some(0x0001_f600), ByteOrder::BigEndian.read_u32_at(&bytes, 1));
    /// assert_eq!(None, ByteOrder::BigEndian.read_u32_at(&bytes, 2));
    /// ```
    #[inline]
    pub fn read_u32_at(self, bytes: &[u8], index: usize) -> Option<u32> {
        let chunk = bytes.get(index..)?.first_chunk::<4>()?;
        Some(self.read_u32_from_array(*chunk))
    }

    /// Reads a `u64` value at `index` from a byte slice.
    ///
    /// Use this safe API when parsing length-checked external buffers where an
    /// incomplete field should be handled by the caller.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Source byte slice.
    /// - `index`: Absolute byte offset where the eight-byte value starts.
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` when `index..index + 8` is in bounds. Returns `None`
    /// when `index` is out of bounds or fewer than eight bytes remain.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let bytes = [0xaa, 1, 2, 3, 4, 5, 6, 7, 8];
    /// assert_eq!(
    ///     Some(0x0102_0304_0506_0708),
    ///     ByteOrder::BigEndian.read_u64_at(&bytes, 1),
    /// );
    /// ```
    #[inline]
    pub fn read_u64_at(self, bytes: &[u8], index: usize) -> Option<u64> {
        let chunk = bytes.get(index..)?.first_chunk::<8>()?;
        Some(self.read_u64_from_array(*chunk))
    }

    /// Reads a `u16` value at `index` without checking slice bounds.
    ///
    /// Use this API only in internal hot paths where a higher layer has already
    /// checked that the full field is available. Prefer [`Self::read_u16_at`] at
    /// public or untrusted-input boundaries.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Source byte slice.
    /// - `index`: Absolute byte offset where the two-byte value starts.
    ///
    /// # Returns
    ///
    /// Returns the decoded `u16` value.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 2` is in bounds for
    /// `bytes`. Violating this requirement is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let bytes = [0xaa, 0x12, 0x34];
    /// assert!(1 + 2 <= bytes.len());
    /// // SAFETY: The assertion above proves the requested byte range is in bounds.
    /// let value = unsafe { ByteOrder::BigEndian.read_u16_at_unchecked(&bytes, 1) };
    /// assert_eq!(0x1234, value);
    /// ```
    #[inline(always)]
    pub unsafe fn read_u16_at_unchecked(self, bytes: &[u8], index: usize) -> u16 {
        // SAFETY: The caller guarantees that `index` starts an in-bounds two-byte range.
        let ptr = unsafe { bytes.as_ptr().add(index).cast::<[u8; 2]>() };
        // SAFETY: `[u8; 2]` has alignment 1, and the caller guarantees the range is valid.
        let chunk = unsafe { *ptr };
        self.read_u16_from_array(chunk)
    }

    /// Reads a `u32` value at `index` without checking slice bounds.
    ///
    /// Use this API inside codecs and parsers after an outer bounds check has
    /// already established that at least four bytes are available.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Source byte slice.
    /// - `index`: Absolute byte offset where the four-byte value starts.
    ///
    /// # Returns
    ///
    /// Returns the decoded `u32` value.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 4` is in bounds for
    /// `bytes`. Violating this requirement is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let bytes = [0xaa, 0x00, 0x01, 0xf6, 0x00];
    /// assert!(1 + 4 <= bytes.len());
    /// // SAFETY: The assertion above proves the requested byte range is in bounds.
    /// let value = unsafe { ByteOrder::BigEndian.read_u32_at_unchecked(&bytes, 1) };
    /// assert_eq!(0x0001_f600, value);
    /// ```
    #[inline(always)]
    pub unsafe fn read_u32_at_unchecked(self, bytes: &[u8], index: usize) -> u32 {
        // SAFETY: The caller guarantees that `index` starts an in-bounds four-byte range.
        let ptr = unsafe { bytes.as_ptr().add(index).cast::<[u8; 4]>() };
        // SAFETY: `[u8; 4]` has alignment 1, and the caller guarantees the range is valid.
        let chunk = unsafe { *ptr };
        self.read_u32_from_array(chunk)
    }

    /// Reads a `u64` value at `index` without checking slice bounds.
    ///
    /// Use this API for validated internal binary parsers that repeatedly read
    /// fixed-width fields from the same checked buffer.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Source byte slice.
    /// - `index`: Absolute byte offset where the eight-byte value starts.
    ///
    /// # Returns
    ///
    /// Returns the decoded `u64` value.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 8` is in bounds for
    /// `bytes`. Violating this requirement is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let bytes = [0xaa, 1, 2, 3, 4, 5, 6, 7, 8];
    /// assert!(1 + 8 <= bytes.len());
    /// // SAFETY: The assertion above proves the requested byte range is in bounds.
    /// let value = unsafe { ByteOrder::BigEndian.read_u64_at_unchecked(&bytes, 1) };
    /// assert_eq!(0x0102_0304_0506_0708, value);
    /// ```
    #[inline]
    pub unsafe fn read_u64_at_unchecked(self, bytes: &[u8], index: usize) -> u64 {
        // SAFETY: The caller guarantees that `index` starts an in-bounds eight-byte range.
        let ptr = unsafe { bytes.as_ptr().add(index).cast::<[u8; 8]>() };
        // SAFETY: `[u8; 8]` has alignment 1, and the caller guarantees the range is valid.
        let chunk = unsafe { *ptr };
        self.read_u64_from_array(chunk)
    }

    /// Converts a `u16` value to bytes using this byte order.
    ///
    /// Use this API when the caller wants a fixed-width array, for example before
    /// appending to a buffer or writing through an already validated destination.
    ///
    /// # Parameters
    ///
    /// - `value`: The value to serialize.
    ///
    /// # Returns
    ///
    /// Returns two bytes in this byte order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// assert_eq!([0x12, 0x34], ByteOrder::BigEndian.u16_bytes(0x1234));
    /// assert_eq!([0x34, 0x12], ByteOrder::LittleEndian.u16_bytes(0x1234));
    /// ```
    #[inline]
    pub const fn u16_bytes(self, value: u16) -> [u8; 2] {
        match self {
            Self::BigEndian => value.to_be_bytes(),
            Self::LittleEndian => value.to_le_bytes(),
        }
    }

    /// Converts a `u32` value to bytes using this byte order.
    ///
    /// Use this API when the caller wants a fixed-width array, for example before
    /// appending to a buffer or writing through an already validated destination.
    ///
    /// # Parameters
    ///
    /// - `value`: The value to serialize.
    ///
    /// # Returns
    ///
    /// Returns four bytes in this byte order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// assert_eq!(
    ///     [0x00, 0x01, 0xf6, 0x00],
    ///     ByteOrder::BigEndian.u32_bytes(0x0001_f600),
    /// );
    /// ```
    #[inline]
    pub const fn u32_bytes(self, value: u32) -> [u8; 4] {
        match self {
            Self::BigEndian => value.to_be_bytes(),
            Self::LittleEndian => value.to_le_bytes(),
        }
    }

    /// Converts a `u64` value to bytes using this byte order.
    ///
    /// Use this API when the caller wants a fixed-width array for an eight-byte
    /// integer field.
    ///
    /// # Parameters
    ///
    /// - `value`: The value to serialize.
    ///
    /// # Returns
    ///
    /// Returns eight bytes in this byte order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// assert_eq!(
    ///     [1, 2, 3, 4, 5, 6, 7, 8],
    ///     ByteOrder::BigEndian.u64_bytes(0x0102_0304_0506_0708),
    /// );
    /// ```
    #[inline]
    pub const fn u64_bytes(self, value: u64) -> [u8; 8] {
        match self {
            Self::BigEndian => value.to_be_bytes(),
            Self::LittleEndian => value.to_le_bytes(),
        }
    }

    /// Writes a `u16` value at `index` into a byte slice.
    ///
    /// Use this safe API at public boundaries or when destination capacity has
    /// not already been checked.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Destination byte slice.
    /// - `index`: Absolute byte offset where the two-byte value starts.
    /// - `value`: The value to serialize.
    ///
    /// # Returns
    ///
    /// Returns `Some(())` when `index..index + 2` is in bounds. Returns `None`
    /// when the destination does not have enough space.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let mut bytes = [0_u8; 3];
    /// assert_eq!(Some(()), ByteOrder::BigEndian.write_u16_at(&mut bytes, 1, 0x1234));
    /// assert_eq!([0, 0x12, 0x34], bytes);
    /// assert_eq!(None, ByteOrder::BigEndian.write_u16_at(&mut bytes, 2, 0x1234));
    /// ```
    #[inline]
    pub fn write_u16_at(self, bytes: &mut [u8], index: usize, value: u16) -> Option<()> {
        let chunk = bytes.get_mut(index..)?.first_chunk_mut::<2>()?;
        *chunk = self.u16_bytes(value);
        Some(())
    }

    /// Writes a `u32` value at `index` into a byte slice.
    ///
    /// Use this safe API when a short destination should be reported to the caller
    /// instead of causing a panic.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Destination byte slice.
    /// - `index`: Absolute byte offset where the four-byte value starts.
    /// - `value`: The value to serialize.
    ///
    /// # Returns
    ///
    /// Returns `Some(())` when `index..index + 4` is in bounds. Returns `None`
    /// when the destination does not have enough space.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let mut bytes = [0_u8; 4];
    /// assert_eq!(Some(()), ByteOrder::BigEndian.write_u32_at(&mut bytes, 0, 0x0001_f600));
    /// assert_eq!([0x00, 0x01, 0xf6, 0x00], bytes);
    /// ```
    #[inline]
    pub fn write_u32_at(self, bytes: &mut [u8], index: usize, value: u32) -> Option<()> {
        let chunk = bytes.get_mut(index..)?.first_chunk_mut::<4>()?;
        *chunk = self.u32_bytes(value);
        Some(())
    }

    /// Writes a `u64` value at `index` into a byte slice.
    ///
    /// Use this safe API for externally supplied or dynamically sized
    /// destinations where capacity should be checked locally.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Destination byte slice.
    /// - `index`: Absolute byte offset where the eight-byte value starts.
    /// - `value`: The value to serialize.
    ///
    /// # Returns
    ///
    /// Returns `Some(())` when `index..index + 8` is in bounds. Returns `None`
    /// when the destination does not have enough space.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let mut bytes = [0_u8; 8];
    /// assert_eq!(
    ///     Some(()),
    ///     ByteOrder::BigEndian.write_u64_at(&mut bytes, 0, 0x0102_0304_0506_0708),
    /// );
    /// assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], bytes);
    /// ```
    #[inline]
    pub fn write_u64_at(self, bytes: &mut [u8], index: usize, value: u64) -> Option<()> {
        let chunk = bytes.get_mut(index..)?.first_chunk_mut::<8>()?;
        *chunk = self.u64_bytes(value);
        Some(())
    }

    /// Writes a `u16` value at `index` without checking slice bounds.
    ///
    /// Use this API only after an outer layer has checked that two bytes are
    /// available from `index`. Prefer [`Self::write_u16_at`] at public boundaries.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Destination byte slice.
    /// - `index`: Absolute byte offset where the two-byte value starts.
    /// - `value`: The value to serialize.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 2` is in bounds for
    /// `bytes`. Violating this requirement is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let mut bytes = [0_u8; 2];
    /// assert!(2 <= bytes.len());
    /// // SAFETY: The assertion above proves the requested byte range is in bounds.
    /// unsafe { ByteOrder::BigEndian.write_u16_at_unchecked(&mut bytes, 0, 0x1234) };
    /// assert_eq!([0x12, 0x34], bytes);
    /// ```
    #[inline(always)]
    pub unsafe fn write_u16_at_unchecked(self, bytes: &mut [u8], index: usize, value: u16) {
        let value = self.u16_bytes(value);
        // SAFETY: The caller guarantees that `index` starts an in-bounds two-byte range.
        let ptr = unsafe { bytes.as_mut_ptr().add(index) };
        // SAFETY: `value` is a distinct stack array and the destination range is valid.
        unsafe { ptr.copy_from_nonoverlapping(value.as_ptr(), 2) };
    }

    /// Writes a `u32` value at `index` without checking slice bounds.
    ///
    /// Use this API in validated internal paths where the destination range has
    /// already been checked once by the caller.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Destination byte slice.
    /// - `index`: Absolute byte offset where the four-byte value starts.
    /// - `value`: The value to serialize.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 4` is in bounds for
    /// `bytes`. Violating this requirement is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let mut bytes = [0_u8; 4];
    /// assert!(4 <= bytes.len());
    /// // SAFETY: The assertion above proves the requested byte range is in bounds.
    /// unsafe { ByteOrder::BigEndian.write_u32_at_unchecked(&mut bytes, 0, 0x0001_f600) };
    /// assert_eq!([0x00, 0x01, 0xf6, 0x00], bytes);
    /// ```
    #[inline(always)]
    pub unsafe fn write_u32_at_unchecked(self, bytes: &mut [u8], index: usize, value: u32) {
        let value = self.u32_bytes(value);
        // SAFETY: The caller guarantees that `index` starts an in-bounds four-byte range.
        let ptr = unsafe { bytes.as_mut_ptr().add(index) };
        // SAFETY: `value` is a distinct stack array and the destination range is valid.
        unsafe { ptr.copy_from_nonoverlapping(value.as_ptr(), 4) };
    }

    /// Writes a `u64` value at `index` without checking slice bounds.
    ///
    /// Use this API in hot binary codecs after the destination buffer has already
    /// been validated by an outer loop.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Destination byte slice.
    /// - `index`: Absolute byte offset where the eight-byte value starts.
    /// - `value`: The value to serialize.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `index..index + 8` is in bounds for
    /// `bytes`. Violating this requirement is undefined behavior.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_text_codec::ByteOrder;
    ///
    /// let mut bytes = [0_u8; 8];
    /// assert!(8 <= bytes.len());
    /// // SAFETY: The assertion above proves the requested byte range is in bounds.
    /// unsafe {
    ///     ByteOrder::BigEndian.write_u64_at_unchecked(&mut bytes, 0, 0x0102_0304_0506_0708);
    /// }
    /// assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], bytes);
    /// ```
    #[inline]
    pub unsafe fn write_u64_at_unchecked(self, bytes: &mut [u8], index: usize, value: u64) {
        let value = self.u64_bytes(value);
        // SAFETY: The caller guarantees that `index` starts an in-bounds eight-byte range.
        let ptr = unsafe { bytes.as_mut_ptr().add(index) };
        // SAFETY: `value` is a distinct stack array and the destination range is valid.
        unsafe { ptr.copy_from_nonoverlapping(value.as_ptr(), 8) };
    }
}
