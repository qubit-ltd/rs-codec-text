// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::{
    fmt,
    hash::{
        Hash,
        Hasher,
    },
};
use std::sync::{
    OnceLock,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
};

use qubit_codec::ByteOrder;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
    de,
};

use crate::{
    normalize_label_loose,
    normalize_label_whatwg,
};

use super::charset_registration_error::CharsetRegistrationError;

/// Global runtime charset registry.
static CHARSET_REGISTRY: OnceLock<RwLock<Vec<Charset>>> = OnceLock::new();

/// Identifies the charset associated with a codec or error.
///
/// A charset is represented by a stable normalized identifier, a display
/// name, and accepted aliases. Equality and hashing use only the identifier, so
/// display names and alias lists can evolve without changing identity.
/// [`fmt::Display`] formats the human-readable name; diagnostics that need a
/// stable cross-tool token should use [`Charset::id`].
///
/// # Examples
///
/// ```rust
/// use qubit_codec_text::Charset;
///
/// const GBK: Charset = Charset::new_static("gbk", "GBK", &["cp936"]);
///
/// assert!(GBK.matches_label("CP936"));
/// assert_eq!(GBK, Charset::new_static("gbk", "Chinese GBK", &[]));
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

/// Label normalization flavor used by charset lookup.
#[derive(Clone, Copy)]
enum LabelNormalization {
    /// Loose matching trims ASCII whitespace, folds ASCII case, and ignores
    /// `-` / `_` separators.
    Loose,
    /// WHATWG-style preprocessing trims ASCII whitespace and folds ASCII case.
    Whatwg,
}

impl Charset {
    /// US-ASCII text.
    pub const ASCII: Self = Self::new_static("ascii", "ASCII", &["us-ascii"]);

    /// ISO-8859-1 / Latin-1 text.
    pub const ISO_8859_1: Self = Self::new_static(
        "iso-8859-1",
        "ISO-8859-1",
        &[
            "latin1",
            "latin-1",
            "iso8859-1",
            "csisolatin1",
            "iso_8859-1",
        ],
    );

    /// UTF-8 text.
    pub const UTF_8: Self = Self::new_static("utf-8", "UTF-8", &["utf8"]);

    /// UTF-16 text.
    pub const UTF_16: Self = Self::new_static("utf-16", "UTF-16", &["utf16"]);

    /// UTF-16 text serialized in little-endian byte order.
    pub const UTF_16LE: Self = Self::new_static(
        "utf-16le",
        "UTF-16LE",
        &["utf16le", "utf16_le", "utf_16_le"],
    );

    /// UTF-16 text serialized in big-endian byte order.
    pub const UTF_16BE: Self = Self::new_static(
        "utf-16be",
        "UTF-16BE",
        &["utf16be", "utf16_be", "utf_16_be"],
    );

    /// UTF-32 text.
    pub const UTF_32: Self = Self::new_static("utf-32", "UTF-32", &["utf32"]);

    /// UTF-32 text serialized in little-endian byte order.
    pub const UTF_32LE: Self = Self::new_static(
        "utf-32le",
        "UTF-32LE",
        &["utf32le", "utf32_le", "utf_32_le"],
    );

    /// UTF-32 text serialized in big-endian byte order.
    pub const UTF_32BE: Self = Self::new_static(
        "utf-32be",
        "UTF-32BE",
        &["utf32be", "utf32_be", "utf_32_be"],
    );

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

    /// Creates an unchecked static charset descriptor.
    ///
    /// This constructor is intended for constant initialization of built-in or
    /// otherwise statically audited charset metadata. It performs no
    /// validation: identifiers, display names, and aliases may be empty,
    /// duplicate existing labels, or normalize to unusable lookup keys. The
    /// caller must ensure every supplied label is valid for the intended use.
    ///
    /// Use [`Self::try_new`] when descriptor labels should be validated before
    /// a value is used at runtime, or [`Self::register_new`] when the
    /// descriptor should also be inserted into the runtime registry.
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
    #[inline]
    pub const fn new_static(
        id: &'static str,
        name: &'static str,
        aliases: &'static [&'static str],
    ) -> Self {
        Self { id, name, aliases }
    }

    /// Creates a validated static charset descriptor without registering it.
    ///
    /// Validation uses the same labels and loose conflict checks as
    /// [`Self::register`], but does not mutate the global runtime registry.
    ///
    /// # Parameters
    ///
    /// - `id`: Stable normalized identifier used for equality and hashing.
    /// - `name`: Human-readable display name.
    /// - `aliases`: Additional labels accepted for this charset.
    ///
    /// # Returns
    ///
    /// Returns a charset descriptor when every descriptor label is usable and
    /// does not conflict with a different built-in or registered charset.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetRegistrationError`] when any candidate label normalizes
    /// to an empty lookup key or conflicts with an existing charset.
    pub fn try_new(
        id: &'static str,
        name: &'static str,
        aliases: &'static [&'static str],
    ) -> Result<Self, CharsetRegistrationError> {
        let candidate = Self::new_static(id, name, aliases);
        validate_descriptor(candidate, &read_registry())
    }

    /// Creates and registers a charset descriptor.
    ///
    /// # Parameters
    ///
    /// - `id`: Stable normalized identifier used for equality and hashing.
    /// - `name`: Human-readable display name.
    /// - `aliases`: Additional labels accepted for this charset.
    ///
    /// # Returns
    ///
    /// Returns the registered descriptor when no label conflicts exist.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetRegistrationError`] when the descriptor's identifier,
    /// display name, or aliases normalize to an empty label or conflict with a
    /// built-in or previously registered charset under loose label
    /// normalization.
    pub fn register_new(
        id: &'static str,
        name: &'static str,
        aliases: &'static [&'static str],
    ) -> Result<Self, CharsetRegistrationError> {
        Self::try_new(id, name, aliases)?.register()
    }

    /// Registers this descriptor in the global charset registry.
    ///
    /// # Returns
    ///
    /// Returns this descriptor when registration succeeds. Registering the
    /// exact same descriptor more than once is idempotent and returns the
    /// already known descriptor. After registration, [`Self::from_label`] and
    /// [`Self::from_whatwg_label`] can discover the charset by identifier,
    /// display name, or alias.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetRegistrationError`] when any candidate label normalizes
    /// to an empty label or conflicts with a different built-in or registered
    /// charset under loose label normalization.
    pub fn register(self) -> Result<Self, CharsetRegistrationError> {
        let mut registry = write_registry();
        let descriptor = validate_descriptor(self, &registry)?;
        if descriptor_exists(descriptor, &registry) {
            return Ok(descriptor);
        }
        registry.push(descriptor);
        Ok(descriptor)
    }

    /// Returns a snapshot of runtime-registered charsets.
    ///
    /// # Returns
    ///
    /// Returns the charsets inserted through [`Self::register`] or
    /// [`Self::register_new`]. Built-in charsets are available through
    /// [`Self::BUILTINS`] and are not included in this snapshot.
    ///
    /// # Panics
    ///
    /// Panics if the global registry lock is poisoned.
    #[must_use]
    pub fn registered() -> Vec<Self> {
        read_registry().clone()
    }

    /// Finds a built-in or registered charset by loose label matching.
    ///
    /// # Parameters
    ///
    /// - `label`: Charset label to match against identifiers, names, and
    ///   aliases.
    ///
    /// # Returns
    ///
    /// Returns `Some(Charset)` when `label` names a built-in or registered
    /// charset, or `None` when no charset matches. The label is normalized with
    /// [`crate::normalize_label_loose`], which trims ASCII whitespace, folds
    /// ASCII case, and ignores `-` / `_` separators.
    pub fn from_label(label: &str) -> Option<Self> {
        Self::from_normalized_label(
            &normalize_label_loose(label),
            LabelNormalization::Loose,
        )
    }

    /// Finds a built-in or registered charset by WHATWG-style label matching.
    ///
    /// # Parameters
    ///
    /// - `label`: Charset label to match against identifiers, names, and
    ///   aliases.
    ///
    /// # Returns
    ///
    /// Returns `Some(Charset)` when `label` names a built-in or registered
    /// charset, or `None` when no charset matches. The label is normalized with
    /// [`crate::normalize_label_whatwg`], which trims ASCII whitespace and
    /// folds ASCII case while preserving punctuation and separators.
    ///
    /// This applies WHATWG-style preprocessing only to this crate's own
    /// charset descriptor table. It is not the full WHATWG Encoding Standard
    /// label table, and it does not remap charset semantics such as treating
    /// `iso-8859-1` as Windows-1252.
    pub fn from_whatwg_label(label: &str) -> Option<Self> {
        Self::from_normalized_label(
            &normalize_label_whatwg(label),
            LabelNormalization::Whatwg,
        )
    }

    /// Returns the stable normalized charset identifier.
    ///
    /// # Returns
    ///
    /// Returns the identifier used for equality, hashing, and stable
    /// diagnostic output.
    #[inline]
    pub const fn id(self) -> &'static str {
        self.id
    }

    /// Returns a human-readable charset label.
    ///
    /// # Returns
    ///
    /// Returns the display name stored in this descriptor.
    #[inline]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Returns accepted aliases for this charset.
    ///
    /// # Returns
    ///
    /// Returns the static alias list stored in this descriptor.
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn byte_order(self) -> Option<ByteOrder> {
        match self.id {
            "utf-16le" | "utf-32le" => Some(ByteOrder::LittleEndian),
            "utf-16be" | "utf-32be" => Some(ByteOrder::BigEndian),
            _ => None,
        }
    }

    /// Tests whether a label names this charset.
    ///
    /// # Parameters
    ///
    /// - `label`: The label to compare with this descriptor's identifier,
    ///   display name, and aliases.
    ///
    /// # Returns
    ///
    /// Returns `true` when `label` matches the identifier, display name, or one
    /// of the aliases using loose label normalization.
    pub fn matches_label(self, label: &str) -> bool {
        let label = normalize_label_loose(label);
        self.matches_normalized_label(&label, LabelNormalization::Loose)
    }

    /// Finds a charset by a pre-normalized label.
    ///
    /// # Parameters
    ///
    /// - `label`: Normalized label to search for.
    /// - `normalize`: Normalization function used for charset-owned labels.
    ///
    /// # Returns
    ///
    /// Returns the first built-in or registered charset whose labels normalize
    /// to `label`, or `None` when no charset matches.
    #[inline]
    fn from_normalized_label(
        label: &str,
        normalization: LabelNormalization,
    ) -> Option<Self> {
        if label.is_empty() {
            return None;
        }
        Self::BUILTINS
            .iter()
            .copied()
            .find(|charset| {
                charset.matches_normalized_label(label, normalization)
            })
            .or_else(|| {
                let registry = read_registry();
                registry.iter().copied().find(|charset| {
                    charset.matches_normalized_label(label, normalization)
                })
            })
    }

    /// Tests this descriptor against a pre-normalized label.
    ///
    /// # Parameters
    ///
    /// - `label`: Normalized caller-provided label.
    /// - `normalize`: Normalization function applied to this descriptor's
    ///   labels.
    ///
    /// # Returns
    ///
    /// Returns `true` when any descriptor label normalizes to `label`.
    #[inline]
    fn matches_normalized_label(
        self,
        label: &str,
        normalization: LabelNormalization,
    ) -> bool {
        charset_labels(self).any(|candidate| {
            label_matches_normalized(candidate, label, normalization)
        })
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
    #[inline]
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
    #[inline]
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
    /// The display form is the human-readable [`Charset::name`]. Use
    /// [`Charset::id`] for stable diagnostics or cross-language comparison.
    ///
    /// # Parameters
    ///
    /// - `formatter`: The formatter receiving the label.
    ///
    /// # Errors
    ///
    /// Returns any formatting error reported by `formatter`.
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

#[cfg(feature = "serde")]
impl Serialize for Charset {
    /// Serializes this charset as its stable identifier.
    ///
    /// # Parameters
    ///
    /// - `serializer`: Serde serializer receiving the identifier string.
    ///
    /// # Errors
    ///
    /// Returns any serialization error reported by `serializer`.
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.id())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Charset {
    /// Deserializes a charset from its stable identifier.
    ///
    /// The identifier must already name a built-in or runtime-registered
    /// charset in this process.
    ///
    /// # Parameters
    ///
    /// - `deserializer`: Serde deserializer providing the identifier string.
    ///
    /// # Errors
    ///
    /// Returns a serde error when the string is not a known charset label.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = <&str>::deserialize(deserializer)?;
        Self::from_label(id)
            .ok_or_else(|| de::Error::custom(format!("unknown charset `{id}`")))
    }
}

/// Returns the global charset registry.
///
/// # Returns
///
/// Returns the lazily initialized registry used by [`Charset::register`].
#[inline]
fn registry() -> &'static RwLock<Vec<Charset>> {
    CHARSET_REGISTRY.get_or_init(|| RwLock::new(Vec::new()))
}

/// Reads the global charset registry.
///
/// # Returns
///
/// Returns a read guard for the registry.
///
/// # Panics
///
/// Panics if the registry lock is poisoned.
#[inline]
fn read_registry() -> RwLockReadGuard<'static, Vec<Charset>> {
    registry()
        .read()
        .expect("charset registry lock should not be poisoned")
}

/// Writes the global charset registry.
///
/// # Returns
///
/// Returns a write guard for the registry.
///
/// # Panics
///
/// Panics if the registry lock is poisoned.
#[inline]
fn write_registry() -> RwLockWriteGuard<'static, Vec<Charset>> {
    registry()
        .write()
        .expect("charset registry lock should not be poisoned")
}

/// Iterates over every label owned by `charset`.
///
/// # Parameters
///
/// - `charset`: Charset descriptor whose labels are inspected.
///
/// # Returns
///
/// Returns an iterator over the identifier, display name, and aliases.
#[inline]
fn charset_labels(charset: Charset) -> impl Iterator<Item = &'static str> {
    std::iter::once(charset.id)
        .chain(std::iter::once(charset.name))
        .chain(charset.aliases.iter().copied())
}

/// Finds the first invalid label in `candidate`.
///
/// # Parameters
///
/// - `candidate`: Charset descriptor being validated.
///
/// # Returns
///
/// Returns the first descriptor label whose loose-normalized form is empty, or
/// `None` when all labels can be used for lookup.
#[inline]
fn invalid_label_for(candidate: Charset) -> Option<&'static str> {
    charset_labels(candidate)
        .find(|label| normalize_label_loose(label).is_empty())
}

/// Validates a descriptor against built-ins and a registry snapshot.
///
/// # Parameters
///
/// - `candidate`: Charset descriptor to validate.
/// - `registered`: Runtime registry contents to compare against.
///
/// # Returns
///
/// Returns `candidate` when all labels are valid and unambiguous. If the only
/// conflict is an identical existing descriptor, returns that existing
/// descriptor so repeated registration stays idempotent.
fn validate_descriptor(
    candidate: Charset,
    registered: &[Charset],
) -> Result<Charset, CharsetRegistrationError> {
    if let Some(label) = invalid_label_for(candidate) {
        return Err(CharsetRegistrationError::invalid_label(label, candidate));
    }
    if let Some((label, existing)) = conflict_for(candidate, registered) {
        if same_descriptor(existing, candidate) {
            return Ok(existing);
        }
        return Err(CharsetRegistrationError::conflicting_label(
            label, existing, candidate,
        ));
    }
    Ok(candidate)
}

/// Tests whether a descriptor already exists in built-ins or the registry.
///
/// # Parameters
///
/// - `candidate`: Charset descriptor to search for.
/// - `registered`: Runtime registry contents to compare against.
///
/// # Returns
///
/// Returns `true` when an identical descriptor is already known.
#[inline]
fn descriptor_exists(candidate: Charset, registered: &[Charset]) -> bool {
    Charset::BUILTINS
        .iter()
        .copied()
        .chain(registered.iter().copied())
        .any(|existing| same_descriptor(existing, candidate))
}

/// Tests whether two descriptors carry exactly the same public metadata.
///
/// # Parameters
///
/// - `left`: First descriptor.
/// - `right`: Second descriptor.
///
/// # Returns
///
/// Returns `true` when id, display name, and alias list are identical.
#[inline]
fn same_descriptor(left: Charset, right: Charset) -> bool {
    left.id == right.id
        && left.name == right.name
        && left.aliases == right.aliases
}

/// Finds a label conflict for `candidate`.
///
/// # Parameters
///
/// - `candidate`: Charset descriptor being registered.
/// - `registered`: Runtime registry contents to compare against.
///
/// # Returns
///
/// Returns the conflicting candidate label and existing charset, or `None`
/// when all candidate labels are available.
fn conflict_for(
    candidate: Charset,
    registered: &[Charset],
) -> Option<(&'static str, Charset)> {
    for label in charset_labels(candidate) {
        let normalized = normalize_label_loose(label);
        if let Some(existing) = Charset::BUILTINS
            .iter()
            .copied()
            .chain(registered.iter().copied())
            .find(|existing| {
                existing.matches_normalized_label(
                    &normalized,
                    LabelNormalization::Loose,
                )
            })
        {
            return Some((label, existing));
        }
    }
    None
}

/// Tests whether `candidate` equals a pre-normalized label.
///
/// # Parameters
///
/// - `candidate`: Charset-owned label to normalize lazily.
/// - `normalized`: Caller-provided normalized label.
/// - `normalization`: Normalization semantics to apply.
///
/// # Returns
///
/// Returns `true` when `candidate` normalizes to `normalized`.
#[inline]
fn label_matches_normalized(
    candidate: &str,
    normalized: &str,
    normalization: LabelNormalization,
) -> bool {
    match normalization {
        LabelNormalization::Loose => {
            loose_label_matches_normalized(candidate, normalized)
        }
        LabelNormalization::Whatwg => {
            whatwg_label_matches_normalized(candidate, normalized)
        }
    }
}

/// Tests loose-normalized equality without allocating a temporary `String`.
#[inline]
fn loose_label_matches_normalized(candidate: &str, normalized: &str) -> bool {
    let mut expected = normalized.chars();
    for ch in candidate.trim_ascii().chars() {
        if matches!(ch, '-' | '_') {
            continue;
        }
        match expected.next() {
            Some(expected_ch) if expected_ch == ch.to_ascii_lowercase() => {}
            _ => return false,
        }
    }
    expected.next().is_none()
}

/// Tests WHATWG-preprocessed equality without allocating a temporary `String`.
#[inline]
fn whatwg_label_matches_normalized(candidate: &str, normalized: &str) -> bool {
    let mut expected = normalized.chars();
    for ch in candidate.trim_ascii().chars() {
        match expected.next() {
            Some(expected_ch) if expected_ch == ch.to_ascii_lowercase() => {}
            _ => return false,
        }
    }
    expected.next().is_none()
}
