/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use core::{
    fmt,
    hash::{
        Hash,
        Hasher,
    },
};

use crate::ByteOrder;

/// Identifies the charset associated with a codec or error.
///
/// A charset is represented by a stable normalized identifier, a display
/// name, and accepted aliases. Equality and hashing use only the identifier, so
/// display names and alias lists can evolve without changing identity.
///
/// # Examples
///
/// ```rust
/// use qubit_codec_text::Charset;
///
/// const GBK: Charset = Charset::new("gbk", "GBK", &["cp936"]);
///
/// assert!(GBK.matches_label("CP936"));
/// assert_eq!(GBK, Charset::new("gbk", "Chinese GBK", &[]));
/// assert_eq!("GBK", GBK.to_string());
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Charset {
    /// Stable normalized identifier used for identity comparison.
    id: &'static str,
    /// Human-friendly display name for logs and errors.
    name: &'static str,
    /// Static alias list accepted in label matching.
    aliases: &'static [&'static str],
}

impl Charset {
    /// US-ASCII text.
    pub const ASCII: Self = Self::new("ascii", "ASCII", &["us-ascii"]);

    /// ISO-8859-1 / Latin-1 text.
    pub const ISO_8859_1: Self = Self::new(
        "iso-8859-1",
        "ISO-8859-1",
        &["latin1", "latin-1", "iso8859-1", "csisolatin1", "iso_8859-1"],
    );

    /// UTF-8 text.
    pub const UTF_8: Self = Self::new("utf-8", "UTF-8", &["utf8"]);

    /// UTF-16 text.
    pub const UTF_16: Self = Self::new("utf-16", "UTF-16", &["utf16"]);

    /// UTF-16 text serialized in little-endian byte order.
    pub const UTF_16LE: Self = Self::new("utf-16le", "UTF-16LE", &["utf16le", "utf16_le", "utf_16_le"]);

    /// UTF-16 text serialized in big-endian byte order.
    pub const UTF_16BE: Self = Self::new("utf-16be", "UTF-16BE", &["utf16be", "utf16_be", "utf_16_be"]);

    /// UTF-32 text.
    pub const UTF_32: Self = Self::new("utf-32", "UTF-32", &["utf32"]);

    /// UTF-32 text serialized in little-endian byte order.
    pub const UTF_32LE: Self = Self::new("utf-32le", "UTF-32LE", &["utf32le", "utf32_le", "utf_32_le"]);

    /// UTF-32 text serialized in big-endian byte order.
    pub const UTF_32BE: Self = Self::new("utf-32be", "UTF-32BE", &["utf32be", "utf32_be", "utf_32_be"]);

    /// Built-in charsets known by this crate.
    pub const BUILTINS: &'static [Self] = &[
        Self::ASCII,
        Self::ISO_8859_1,
        Self::UTF_8,
        Self::UTF_16,
        Self::UTF_16LE,
        Self::UTF_16BE,
        Self::UTF_32,
        Self::UTF_32LE,
        Self::UTF_32BE,
    ];

    /// Creates a charset descriptor.
    ///
    /// # Parameters
    ///
    /// - `id`: Stable normalized identifier used for equality and hashing.
    /// - `name`: Human-readable display name.
    /// - `aliases`: Additional labels accepted for this charset.
    ///
    /// # Returns
    ///
    /// Returns a charset descriptor carrying the supplied metadata.
    pub const fn new(id: &'static str, name: &'static str, aliases: &'static [&'static str]) -> Self {
        Self { id, name, aliases }
    }

    /// Finds a built-in charset by label.
    ///
    /// # Parameters
    ///
    /// - `label`: Charset label to match against built-in identifiers, names, and
    ///   aliases.
    ///
    /// # Returns
    ///
    /// Returns `Some(Charset)` when `label` names one of this crate's built-in
    /// charsets, or `None` when no built-in charset matches. Custom descriptors
    /// created with [`Self::new`] are not part of this lookup. Leading and
    /// trailing ASCII whitespace is ignored before matching.
    pub fn from_label(label: &str) -> Option<Self> {
        let label = label.trim_ascii();
        Self::BUILTINS
            .iter()
            .copied()
            .find(|charset| charset.matches_label(label))
    }

    /// Returns the stable normalized charset identifier.
    ///
    /// # Returns
    ///
    /// Returns the identifier used for equality and hashing.
    #[inline(always)]
    pub const fn id(self) -> &'static str {
        self.id
    }

    /// Returns a human-readable charset label.
    ///
    /// # Returns
    ///
    /// Returns the display name stored in this descriptor.
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Returns accepted aliases for this charset.
    ///
    /// # Returns
    ///
    /// Returns the static alias list stored in this descriptor.
    pub const fn aliases(self) -> &'static [&'static str] {
        self.aliases
    }

    /// Returns the UTF-16 charset with a fixed byte order.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used by the byte stream.
    ///
    /// # Returns
    ///
    /// Returns [`Self::UTF_16LE`] for little-endian byte order and
    /// [`Self::UTF_16BE`] for big-endian byte order.
    pub const fn from_utf16_byte_order(byte_order: ByteOrder) -> Self {
        match byte_order {
            ByteOrder::LittleEndian => Self::UTF_16LE,
            ByteOrder::BigEndian => Self::UTF_16BE,
        }
    }

    /// Returns the UTF-32 charset with a fixed byte order.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used by the byte stream.
    ///
    /// # Returns
    ///
    /// Returns [`Self::UTF_32LE`] for little-endian byte order and
    /// [`Self::UTF_32BE`] for big-endian byte order.
    pub const fn from_utf32_byte_order(byte_order: ByteOrder) -> Self {
        match byte_order {
            ByteOrder::LittleEndian => Self::UTF_32LE,
            ByteOrder::BigEndian => Self::UTF_32BE,
        }
    }

    /// Returns the fixed byte order represented by this charset.
    ///
    /// # Returns
    ///
    /// Returns `Some(ByteOrder)` for fixed-endian UTF-16 and UTF-32 charsets.
    /// Returns `None` for UTF-8 and generic UTF-16/UTF-32 charsets.
    pub fn byte_order(self) -> Option<ByteOrder> {
        if self == Self::UTF_16LE || self == Self::UTF_32LE {
            Some(ByteOrder::LittleEndian)
        } else if self == Self::UTF_16BE || self == Self::UTF_32BE {
            Some(ByteOrder::BigEndian)
        } else {
            None
        }
    }

    /// Tests whether a label names this charset.
    ///
    /// # Parameters
    ///
    /// - `label`: The label to compare with this descriptor's identifier, display
    ///   name, and aliases.
    ///
    /// # Returns
    ///
    /// Returns `true` when `label` matches the identifier, display name, or one of
    /// the aliases using ASCII case-insensitive comparison.
    pub fn matches_label(self, label: &str) -> bool {
        if label.eq_ignore_ascii_case(self.id) || label.eq_ignore_ascii_case(self.name) {
            return true;
        }
        self.aliases.iter().any(|alias| label.eq_ignore_ascii_case(alias))
    }
}

impl PartialEq for Charset {
    /// Compares charsets by stable identifier.
    ///
    /// # Parameters
    ///
    /// - `other`: The descriptor to compare against.
    ///
    /// # Returns
    ///
    /// Returns `true` when both descriptors have the same identifier.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Charset {}

impl Hash for Charset {
    /// Hashes the stable encoding identifier.
    ///
    /// # Parameters
    ///
    /// - `state`: The hasher receiving this encoding's identity.
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Charset {
    /// Formats this charset label.
    ///
    /// # Parameters
    ///
    /// - `formatter`: The formatter receiving the label.
    ///
    /// # Errors
    ///
    /// Returns any formatting error reported by `formatter`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}
