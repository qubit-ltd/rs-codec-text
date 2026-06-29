// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::ByteOrder;

use crate::{BomDetectStatus, Charset};

/// Unicode byte order marks supported by this crate.
///
/// `detect` recognizes BOMs only from the bytes supplied to the call. Streaming
/// callers should buffer up to four bytes, or read until EOF, before deciding
/// that no longer BOM can be present.
///
/// # Examples
///
/// ```rust
/// use qubit_codec::ByteOrder;
/// use qubit_codec_text::{
///     Charset,
///     UnicodeBom,
/// };
///
/// let bom = UnicodeBom::detect(&[0xff, 0xfe, 0x00, 0x00]);
/// assert_eq!(Some(UnicodeBom::Utf32LittleEndian), bom);
///
/// let bom = bom.expect("UTF-32LE BOM");
/// assert_eq!(Charset::UTF_32LE, bom.charset());
/// assert_eq!(Some(ByteOrder::LittleEndian), bom.byte_order());
/// assert_eq!(4, bom.byte_len());
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnicodeBom {
    /// UTF-8 byte order mark.
    Utf8,

    /// UTF-16 big-endian byte order mark.
    Utf16BigEndian,

    /// UTF-16 little-endian byte order mark.
    Utf16LittleEndian,

    /// UTF-32 big-endian byte order mark.
    Utf32BigEndian,

    /// UTF-32 little-endian byte order mark.
    Utf32LittleEndian,
}

impl UnicodeBom {
    /// Supported BOMs in longest-prefix detection order.
    const ALL: &'static [Self] = &[
        Self::Utf32BigEndian,
        Self::Utf32LittleEndian,
        Self::Utf8,
        Self::Utf16BigEndian,
        Self::Utf16LittleEndian,
    ];

    /// Detects a Unicode byte order mark at the beginning of `bytes`.
    ///
    /// # Parameters
    ///
    /// - `bytes`: The byte buffer to inspect.
    ///
    /// # Returns
    ///
    /// Returns the detected BOM, or `None` if no supported BOM prefix is
    /// present.
    ///
    /// UTF-32 BOMs are checked before UTF-16 BOMs so that overlapping prefixes
    /// such as `FF FE 00 00` are classified as UTF-32 little-endian when all
    /// four bytes are available.
    pub fn detect(bytes: &[u8]) -> Option<Self> {
        match Self::detect_progress(bytes, true) {
            BomDetectStatus::Match(bom) => Some(bom),
            BomDetectStatus::Pending | BomDetectStatus::None => None,
        }
    }

    /// Detects a Unicode byte order mark with an explicit EOF signal.
    ///
    /// # Parameters
    ///
    /// - `bytes`: Bytes currently available from the beginning of the stream.
    /// - `eof`: Whether no more bytes can arrive.
    ///
    /// # Returns
    ///
    /// Returns [`BomDetectStatus::Pending`] when the bytes are a valid prefix
    /// of a longer BOM and EOF has not been reached, [`BomDetectStatus::Match`]
    /// when a supported BOM can be selected, or [`BomDetectStatus::None`] when
    /// no supported BOM can match.
    pub fn detect_progress(bytes: &[u8], eof: bool) -> BomDetectStatus {
        if let Some(bom) = detect_complete(bytes) {
            if !eof && has_longer_possible_match(bytes, bom) {
                BomDetectStatus::Pending
            } else {
                BomDetectStatus::Match(bom)
            }
        } else if !eof && is_possible_bom_prefix(bytes) {
            BomDetectStatus::Pending
        } else {
            BomDetectStatus::None
        }
    }

    /// Returns the bytes that represent this BOM.
    ///
    /// # Returns
    ///
    /// Returns a static byte slice containing the BOM bytes.
    #[inline]
    pub const fn bytes(self) -> &'static [u8] {
        match self {
            Self::Utf8 => &[0xef, 0xbb, 0xbf],
            Self::Utf16BigEndian => &[0xfe, 0xff],
            Self::Utf16LittleEndian => &[0xff, 0xfe],
            Self::Utf32BigEndian => &[0x00, 0x00, 0xfe, 0xff],
            Self::Utf32LittleEndian => &[0xff, 0xfe, 0x00, 0x00],
        }
    }

    /// Returns the byte length of this BOM.
    ///
    /// # Returns
    ///
    /// Returns the number of bytes in this BOM.
    #[inline]
    pub const fn byte_len(self) -> usize {
        match self {
            Self::Utf8 => 3,
            Self::Utf16BigEndian | Self::Utf16LittleEndian => 2,
            Self::Utf32BigEndian | Self::Utf32LittleEndian => 4,
        }
    }

    /// Returns the charset indicated by this BOM.
    ///
    /// # Returns
    ///
    /// Returns the corresponding [`Charset`], including fixed byte order for
    /// UTF-16 and UTF-32 BOMs.
    #[inline]
    pub const fn charset(self) -> Charset {
        match self {
            Self::Utf8 => Charset::UTF_8,
            Self::Utf16BigEndian => Charset::UTF_16BE,
            Self::Utf16LittleEndian => Charset::UTF_16LE,
            Self::Utf32BigEndian => Charset::UTF_32BE,
            Self::Utf32LittleEndian => Charset::UTF_32LE,
        }
    }

    /// Returns the byte order indicated by this BOM when applicable.
    ///
    /// # Returns
    ///
    /// Returns `Some(ByteOrder)` for UTF-16 and UTF-32 BOMs. Returns `None` for
    /// UTF-8 because byte order does not apply.
    #[inline]
    pub const fn byte_order(self) -> Option<ByteOrder> {
        match self {
            Self::Utf8 => None,
            Self::Utf16BigEndian | Self::Utf32BigEndian => Some(ByteOrder::BigEndian),
            Self::Utf16LittleEndian | Self::Utf32LittleEndian => Some(ByteOrder::LittleEndian),
        }
    }
}

/// Detects a complete BOM using longest-prefix order.
///
/// # Parameters
///
/// - `bytes`: Candidate stream prefix.
///
/// # Returns
///
/// Returns the longest supported BOM that is fully present in `bytes`.
#[inline]
fn detect_complete(bytes: &[u8]) -> Option<UnicodeBom> {
    UnicodeBom::ALL
        .iter()
        .copied()
        .find(|bom| bytes.starts_with(bom.bytes()))
}

/// Tests whether `bytes` may still become a supported BOM.
///
/// # Parameters
///
/// - `bytes`: Bytes currently available from the beginning of the stream.
///
/// # Returns
///
/// Returns `true` when `bytes` is a prefix of any supported BOM.
#[inline]
fn is_possible_bom_prefix(bytes: &[u8]) -> bool {
    UnicodeBom::ALL
        .iter()
        .any(|bom| bom.bytes().starts_with(bytes))
}

/// Tests whether a currently matched BOM is ambiguous with a longer BOM.
///
/// # Parameters
///
/// - `bytes`: Bytes currently available from the beginning of the stream.
/// - `matched`: BOM selected from the currently available bytes.
///
/// # Returns
///
/// Returns `true` when additional bytes could promote the match to a longer
/// BOM.
#[inline]
fn has_longer_possible_match(bytes: &[u8], matched: UnicodeBom) -> bool {
    UnicodeBom::ALL
        .iter()
        .any(|bom| bom.byte_len() > matched.byte_len() && bom.bytes().starts_with(bytes))
}
