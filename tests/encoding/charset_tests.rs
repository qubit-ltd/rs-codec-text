use std::{
    collections::hash_map::DefaultHasher,
    hash::{
        Hash,
        Hasher,
    },
};

use qubit_codec_text::{
    ByteOrder,
    Charset,
};

#[test]
fn test_charset_exposes_identity_metadata() {
    const GBK: Charset = Charset::new("gbk", "GBK", &["cp936", "windows-936"]);

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
    const GBK: Charset = Charset::new("gbk", "GBK", &["cp936", "windows-936"]);

    assert_eq!(Charset::new("utf-8", "Unicode UTF-8", &[]), Charset::UTF_8);

    let mut left_hasher = DefaultHasher::new();
    Charset::new("gbk", "Chinese GBK", &["cp936"]).hash(&mut left_hasher);
    let mut right_hasher = DefaultHasher::new();
    GBK.hash(&mut right_hasher);
    assert_eq!(left_hasher.finish(), right_hasher.finish());
}

#[test]
fn test_charset_matches_labels() {
    const GBK: Charset = Charset::new("gbk", "GBK", &["cp936", "windows-936"]);

    assert!(Charset::UTF_8.matches_label("utf8"));
    assert!(Charset::UTF_8.matches_label("UTF-8"));
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

    let display_named = Charset::new("example-encoding", "Example Encoding", &["example"]);
    assert!(display_named.matches_label("example-encoding"));
    assert!(display_named.matches_label("Example Encoding"));
    assert!(display_named.matches_label("EXAMPLE"));
}

#[test]
fn test_charset_exposes_fixed_byte_order_helpers() {
    assert_eq!(
        Charset::UTF_16LE,
        Charset::from_utf16_byte_order(ByteOrder::LittleEndian)
    );
    assert_eq!(Charset::UTF_16BE, Charset::from_utf16_byte_order(ByteOrder::BigEndian));
    assert_eq!(
        Charset::UTF_32LE,
        Charset::from_utf32_byte_order(ByteOrder::LittleEndian)
    );
    assert_eq!(Charset::UTF_32BE, Charset::from_utf32_byte_order(ByteOrder::BigEndian));
    assert_eq!(Some(ByteOrder::LittleEndian), Charset::UTF_16LE.byte_order());
    assert_eq!(Some(ByteOrder::BigEndian), Charset::UTF_32BE.byte_order());
    assert_eq!(None, Charset::UTF_16.byte_order());
    assert_eq!(None, Charset::UTF_8.byte_order());
}
