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

/// Identifies the text encoding associated with a codec or error.
///
/// A text encoding is represented by a stable normalized identifier, a display
/// name, and accepted aliases. Equality and hashing use only the identifier, so
/// display names and alias lists can evolve without changing identity.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::TextEncoding;
///
/// const GBK: TextEncoding = TextEncoding::new("gbk", "GBK", &["cp936"]);
///
/// assert!(GBK.matches_label("CP936"));
/// assert_eq!(GBK, TextEncoding::new("gbk", "Chinese GBK", &[]));
/// assert_eq!("GBK", GBK.to_string());
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TextEncoding {
    id: &'static str,
    name: &'static str,
    aliases: &'static [&'static str],
}

impl TextEncoding {
    /// US-ASCII text.
    pub const ASCII: Self = Self::new("ascii", "ASCII", &["us-ascii"]);

    /// UTF-8 text.
    pub const UTF_8: Self = Self::new("utf-8", "UTF-8", &["utf8"]);

    /// UTF-16 text.
    pub const UTF_16: Self = Self::new("utf-16", "UTF-16", &["utf16"]);

    /// UTF-32 text.
    pub const UTF_32: Self = Self::new("utf-32", "UTF-32", &["utf32"]);

    /// Creates a text encoding descriptor.
    ///
    /// # Parameters
    ///
    /// - `id`: Stable normalized identifier used for equality and hashing.
    /// - `name`: Human-readable display name.
    /// - `aliases`: Additional labels accepted for this encoding.
    ///
    /// # Returns
    ///
    /// Returns an encoding descriptor carrying the supplied metadata.
    #[must_use]
    pub const fn new(
        id: &'static str,
        name: &'static str,
        aliases: &'static [&'static str],
    ) -> Self {
        Self { id, name, aliases }
    }

    /// Returns the stable normalized encoding identifier.
    ///
    /// # Returns
    ///
    /// Returns the identifier used for equality and hashing.
    #[must_use]
    pub const fn id(self) -> &'static str {
        self.id
    }

    /// Returns a human-readable encoding label.
    ///
    /// # Returns
    ///
    /// Returns the display name stored in this descriptor.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Returns accepted aliases for this encoding.
    ///
    /// # Returns
    ///
    /// Returns the static alias list stored in this descriptor.
    #[must_use]
    pub const fn aliases(self) -> &'static [&'static str] {
        self.aliases
    }

    /// Tests whether a label names this encoding.
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
    #[must_use]
    pub fn matches_label(self, label: &str) -> bool {
        if label.eq_ignore_ascii_case(self.id) || label.eq_ignore_ascii_case(self.name) {
            return true;
        }
        self.aliases
            .iter()
            .any(|alias| label.eq_ignore_ascii_case(alias))
    }
}

impl PartialEq for TextEncoding {
    /// Compares text encodings by stable identifier.
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

impl Eq for TextEncoding {}

impl Hash for TextEncoding {
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

impl fmt::Display for TextEncoding {
    /// Formats this encoding label.
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
