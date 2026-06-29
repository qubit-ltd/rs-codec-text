use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use qubit_codec::ByteOrder;
use qubit_codec_text::{
    Charset, CharsetRegistrationErrorKind, normalize_label_loose, normalize_label_whatwg,
};

#[test]
fn test_charset_exposes_identity_metadata() {
    const GBK: Charset = Charset::new_static("gbk", "GBK", &["cp936", "windows-936"]);

    assert_eq!("ascii", Charset::ASCII.id());
    assert_eq!("ASCII", Charset::ASCII.name());
    assert_eq!("iso-8859-1", Charset::ISO_8859_1.id());
    assert_eq!("ISO-8859-1", Charset::ISO_8859_1.name());
    assert_eq!("UTF-8", Charset::UTF_8.to_string());
    assert_eq!("UTF-16", Charset::UTF_16.name());
    assert_eq!("utf-16le", Charset::UTF_16LE.id());
    assert_eq!("UTF-16BE", Charset::UTF_16BE.name());
    assert_eq!("UTF-32", Charset::UTF_32.name());
    assert_eq!("utf-32le", Charset::UTF_32LE.id());
    assert_eq!("UTF-32BE", Charset::UTF_32BE.name());
    assert_eq!("GBK", GBK.to_string());
    assert_eq!(&["cp936", "windows-936"], GBK.aliases());
}

#[test]
fn test_charset_identity_uses_id_only() {
    const GBK: Charset = Charset::new_static("gbk", "GBK", &["cp936", "windows-936"]);

    assert_eq!(
        Charset::new_static("utf-8", "Unicode UTF-8", &[]),
        Charset::UTF_8
    );

    let mut left_hasher = DefaultHasher::new();
    Charset::new_static("gbk", "Chinese GBK", &["cp936"]).hash(&mut left_hasher);
    let mut right_hasher = DefaultHasher::new();
    GBK.hash(&mut right_hasher);
    assert_eq!(left_hasher.finish(), right_hasher.finish());
}

#[test]
fn test_charset_matches_labels() {
    const GBK: Charset = Charset::new_static("gbk", "GBK", &["cp936", "windows-936"]);

    assert!(Charset::UTF_8.matches_label("utf_8"));
    assert!(Charset::UTF_8.matches_label("utf8"));
    assert!(Charset::UTF_8.matches_label("UTF-8"));
    assert!(Charset::UTF_16LE.matches_label("utf-16_le"));
    assert!(Charset::UTF_16LE.matches_label("utf16_le"));
    assert!(Charset::UTF_16BE.matches_label("UTF-16BE"));
    assert!(Charset::ISO_8859_1.matches_label("latin1"));
    assert!(Charset::ISO_8859_1.matches_label("ISO_8859-1"));
    assert!(Charset::ISO_8859_1.matches_label("csisolatin1"));
    assert!(Charset::UTF_32LE.matches_label("utf32le"));
    assert!(Charset::UTF_32BE.matches_label("UTF_32_BE"));
    assert!(GBK.matches_label("CP936"));
    assert!(GBK.matches_label("windows-936"));
    assert!(!GBK.matches_label("big5"));

    let display_named = Charset::new_static("example-encoding", "Example Encoding", &["example"]);
    assert!(display_named.matches_label("example-encoding"));
    assert!(display_named.matches_label("Example Encoding"));
    assert!(display_named.matches_label("EXAMPLE"));
}

#[test]
fn test_charset_from_label_finds_builtin_charsets() {
    assert_eq!(Some(Charset::ASCII), Charset::from_label("US-ASCII"));
    assert_eq!(Some(Charset::ISO_8859_1), Charset::from_label("latin1"));
    assert_eq!(Some(Charset::UTF_8), Charset::from_label("utf_8"));
    assert_eq!(Some(Charset::UTF_8), Charset::from_label("utf8"));
    assert_eq!(Some(Charset::UTF_16LE), Charset::from_label("utf-16_le"));
    assert_eq!(Some(Charset::UTF_16LE), Charset::from_label("utf16_le"));
    assert_eq!(Some(Charset::UTF_16BE), Charset::from_label("UTF-16BE"));
    assert_eq!(Some(Charset::UTF_32LE), Charset::from_label("utf32le"));
    assert_eq!(Some(Charset::UTF_32BE), Charset::from_label("UTF_32_BE"));
    assert_eq!(None, Charset::from_label("gbk"));
    assert_eq!(None, Charset::from_label("-_"));
}

#[test]
fn test_charset_from_label_trims_ascii_whitespace() {
    assert_eq!(Some(Charset::UTF_8), Charset::from_label(" \tUTF-8\r\n"));
    assert_eq!(Some(Charset::ASCII), Charset::from_label("\nUS-ASCII "));
}

#[test]
fn test_charset_from_whatwg_label_finds_builtin_charsets() {
    assert_eq!(Some(Charset::UTF_8), Charset::from_whatwg_label(" UTF-8 "));
    assert_eq!(Some(Charset::UTF_8), Charset::from_whatwg_label("utf8"));
    assert_eq!(
        Some(Charset::UTF_16LE),
        Charset::from_whatwg_label("UTF16_LE")
    );
    assert_eq!(None, Charset::from_whatwg_label("utf_8"));
    assert_eq!(None, Charset::from_whatwg_label("utf-16_le"));
}

#[test]
fn test_charset_new_static_does_not_validate_or_register_descriptor() {
    let charset = Charset::new_static(
        "x-qubit-unregistered",
        "Qubit Unregistered",
        &["x-qubit-unregistered-alias"],
    );

    assert_eq!(None, Charset::from_label(charset.id()));
    assert_eq!(None, Charset::from_label("x-qubit-unregistered-alias"));

    let unchecked = Charset::new_static("", "", &["-_"]);
    assert_eq!("", unchecked.id());
    assert_eq!("", unchecked.name());
    assert_eq!(&["-_"], unchecked.aliases());
}

#[test]
fn test_charset_try_new_validates_static_descriptor_labels() {
    let charset = Charset::try_new(
        "x-qubit-try-new",
        "Qubit Try New",
        &["x-qubit-try-new-alias"],
    )
    .expect("valid static descriptor should be accepted");

    assert_eq!("x-qubit-try-new", charset.id());
    assert_eq!("Qubit Try New", charset.name());
    assert_eq!(&["x-qubit-try-new-alias"], charset.aliases());

    let error = Charset::try_new("-_", "Qubit Invalid Id", &[])
        .expect_err("empty normalized id should be rejected");
    assert_eq!("-_", error.label());
    assert_eq!(CharsetRegistrationErrorKind::InvalidLabel, error.kind());
}

#[test]
fn test_charset_register_makes_descriptor_discoverable() {
    let charset = Charset::new_static(
        "x-qubit-registered",
        "Qubit Registered",
        &["x-qubit-registered-alias"],
    );
    let registered = Charset::register(charset).expect("register test charset");

    assert_eq!(charset, registered);
    assert_eq!(Some(charset), Charset::from_label("x_qubit_registered"));
    assert_eq!(
        Some(charset),
        Charset::from_label("x-qubit-registered-alias")
    );
    assert_eq!(
        Some(charset),
        Charset::from_whatwg_label("x-qubit-registered-alias")
    );
}

#[test]
fn test_charset_register_new_constructs_and_registers_descriptor() {
    let charset = Charset::register_new(
        "x-qubit-register-new",
        "Qubit Register New",
        &["x-qubit-register-new-alias"],
    )
    .expect("construct and register test charset");

    assert_eq!(Some(charset), Charset::from_label("x_qubit_register_new"));
    assert_eq!(
        Some(charset),
        Charset::from_whatwg_label("X-QUBIT-REGISTER-NEW-ALIAS")
    );
}

#[test]
fn test_charset_register_new_rejects_invalid_descriptor() {
    let error = Charset::register_new("-_", "Qubit Invalid Register New", &[])
        .expect_err("register_new should validate before registration");

    assert_eq!("-_", error.label());
    assert_eq!(CharsetRegistrationErrorKind::InvalidLabel, error.kind());
}

#[test]
fn test_charset_registered_returns_runtime_registry_snapshot() {
    let charset = Charset::register_new(
        "x-qubit-registered-snapshot",
        "Qubit Registered Snapshot",
        &["x-qubit-registered-snapshot-alias"],
    )
    .expect("register snapshot test charset");

    let registered = Charset::registered();
    assert!(registered.contains(&charset));
    assert!(!registered.contains(&Charset::UTF_8));
}

#[cfg(feature = "serde")]
#[test]
fn test_charset_serde_serializes_as_id_and_deserializes_known_label() {
    assert_eq!(
        "\"utf-8\"",
        serde_json::to_string(&Charset::UTF_8).expect("charset should serialize as string id"),
    );

    let builtin: Charset = serde_json::from_str("\"utf8\"").expect("known alias should parse");
    assert_eq!(Charset::UTF_8, builtin);

    let custom = Charset::register_new("x-qubit-serde", "Qubit Serde", &["x-qubit-serde-alias"])
        .expect("register serde test charset");
    let decoded: Charset =
        serde_json::from_str("\"x-qubit-serde-alias\"").expect("registered alias should parse");
    assert_eq!(custom, decoded);

    let error = serde_json::from_str::<Charset>("\"x-qubit-serde-missing\"")
        .expect_err("unknown charset labels should be rejected");
    assert!(error.to_string().contains("unknown charset"));

    serde_json::from_str::<Charset>("123")
        .expect_err("charset serde representation must be a string");
}

#[test]
fn test_charset_register_rejects_conflicting_labels() {
    let candidate = Charset::new_static(
        "x-qubit-conflicting-utf8",
        "Qubit Conflicting UTF8",
        &["utf8"],
    );
    let error =
        Charset::register(candidate).expect_err("builtin alias conflict should be rejected");

    assert_eq!("utf8", error.label());
    assert_eq!(candidate, error.candidate());
    assert_eq!(
        CharsetRegistrationErrorKind::ConflictingLabel {
            existing: Charset::UTF_8,
        },
        error.kind(),
    );
    assert_eq!(Some(Charset::UTF_8), error.existing());
    assert_eq!(
        "charset label \"utf8\" for Qubit Conflicting UTF8 conflicts with UTF-8",
        error.to_string(),
    );
}

#[test]
fn test_charset_register_is_idempotent_for_same_descriptor() {
    let charset = Charset::new_static(
        "x-qubit-idempotent",
        "Qubit Idempotent",
        &["x-qubit-idempotent-alias"],
    );

    assert_eq!(
        charset,
        Charset::register(charset).expect("first registration")
    );
    assert_eq!(
        charset,
        Charset::register(charset).expect("same descriptor can register twice")
    );
    assert_eq!(Some(charset), Charset::from_label("x_qubit_idempotent"));
}

#[test]
fn test_charset_register_rejects_empty_normalized_labels() {
    let error = Charset::register(Charset::new_static("-_", "Qubit Invalid Id", &[]))
        .expect_err("empty normalized id should be rejected");

    assert_eq!("-_", error.label());
    assert_eq!(CharsetRegistrationErrorKind::InvalidLabel, error.kind());
    assert_eq!(None, error.existing());
    assert_eq!(
        Charset::new_static("-_", "Qubit Invalid Id", &[]),
        error.candidate()
    );
    assert_eq!(
        "charset label \"-_\" for Qubit Invalid Id is invalid",
        error.to_string(),
    );

    let error = Charset::register(Charset::new_static(
        "x-qubit-invalid-alias",
        "Qubit Invalid Alias",
        &[" \t-_ "],
    ))
    .expect_err("empty normalized alias should be rejected");

    assert_eq!(" \t-_ ", error.label());
    assert_eq!(CharsetRegistrationErrorKind::InvalidLabel, error.kind());
    assert_eq!(None, error.existing());
}

#[test]
fn test_charset_registry_supports_concurrent_access() {
    let charsets = [
        Charset::new_static(
            "x-qubit-threaded-a",
            "Qubit Threaded A",
            &["x-qubit-threaded-a-alias"],
        ),
        Charset::new_static(
            "x-qubit-threaded-b",
            "Qubit Threaded B",
            &["x-qubit-threaded-b-alias"],
        ),
        Charset::new_static(
            "x-qubit-threaded-c",
            "Qubit Threaded C",
            &["x-qubit-threaded-c-alias"],
        ),
    ];
    let handles = charsets
        .into_iter()
        .map(|charset| std::thread::spawn(move || Charset::register(charset)))
        .collect::<Vec<_>>();

    for handle in handles {
        handle
            .join()
            .expect("registration thread should finish")
            .expect("threaded charset should register");
    }

    assert_eq!(
        Some(Charset::new_static(
            "x-qubit-threaded-a",
            "Qubit Threaded A",
            &["x-qubit-threaded-a-alias"],
        )),
        Charset::from_label("x_qubit_threaded_a")
    );
    assert_eq!(
        Some(Charset::new_static(
            "x-qubit-threaded-c",
            "Qubit Threaded C",
            &["x-qubit-threaded-c-alias"],
        )),
        Charset::from_whatwg_label("x-qubit-threaded-c-alias")
    );
}

#[test]
fn test_label_normalization_helpers() {
    assert_eq!("utf8", normalize_label_loose(" UTF_8 "));
    assert_eq!("utf16le", normalize_label_loose("UTF-16_LE"));
    assert_eq!("utf_8", normalize_label_whatwg(" UTF_8 "));
    assert_eq!("utf-16_le", normalize_label_whatwg("UTF-16_LE"));
}

#[test]
fn test_charset_exposes_fixed_byte_order_helpers() {
    assert_eq!(
        Charset::UTF_16LE,
        Charset::from_utf16_byte_order(ByteOrder::LittleEndian)
    );
    assert_eq!(
        Charset::UTF_16BE,
        Charset::from_utf16_byte_order(ByteOrder::BigEndian)
    );
    assert_eq!(
        Charset::UTF_32LE,
        Charset::from_utf32_byte_order(ByteOrder::LittleEndian)
    );
    assert_eq!(
        Charset::UTF_32BE,
        Charset::from_utf32_byte_order(ByteOrder::BigEndian)
    );
    assert_eq!(
        Some(ByteOrder::LittleEndian),
        Charset::UTF_16LE.byte_order()
    );
    assert_eq!(Some(ByteOrder::BigEndian), Charset::UTF_32BE.byte_order());
    assert_eq!(None, Charset::UTF_16.byte_order());
    assert_eq!(None, Charset::UTF_8.byte_order());
}
