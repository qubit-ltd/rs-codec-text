use std::{
    collections::hash_map::DefaultHasher,
    hash::{
        Hash,
        Hasher,
    },
};

use qubit_text_codec::TextEncoding;

#[test]
fn test_text_encoding_exposes_identity_metadata() {
    const GBK: TextEncoding = TextEncoding::new("gbk", "GBK", &["cp936", "windows-936"]);

    assert_eq!("ascii", TextEncoding::ASCII.id());
    assert_eq!("ASCII", TextEncoding::ASCII.name());
    assert_eq!("UTF-8", TextEncoding::UTF_8.to_string());
    assert_eq!("UTF-16", TextEncoding::UTF_16.name());
    assert_eq!("UTF-32", TextEncoding::UTF_32.name());
    assert_eq!("GBK", GBK.to_string());
    assert_eq!(&["cp936", "windows-936"], GBK.aliases());
}

#[test]
fn test_text_encoding_identity_uses_id_only() {
    const GBK: TextEncoding = TextEncoding::new("gbk", "GBK", &["cp936", "windows-936"]);

    assert_eq!(
        TextEncoding::new("utf-8", "Unicode UTF-8", &[]),
        TextEncoding::UTF_8
    );

    let mut left_hasher = DefaultHasher::new();
    TextEncoding::new("gbk", "Chinese GBK", &["cp936"]).hash(&mut left_hasher);
    let mut right_hasher = DefaultHasher::new();
    GBK.hash(&mut right_hasher);
    assert_eq!(left_hasher.finish(), right_hasher.finish());
}

#[test]
fn test_text_encoding_matches_labels() {
    const GBK: TextEncoding = TextEncoding::new("gbk", "GBK", &["cp936", "windows-936"]);

    assert!(TextEncoding::UTF_8.matches_label("utf8"));
    assert!(TextEncoding::UTF_8.matches_label("UTF-8"));
    assert!(GBK.matches_label("CP936"));
    assert!(GBK.matches_label("windows-936"));
    assert!(!GBK.matches_label("big5"));

    let display_named = TextEncoding::new("example-encoding", "Example Encoding", &["example"]);
    assert!(display_named.matches_label("example-encoding"));
    assert!(display_named.matches_label("Example Encoding"));
    assert!(display_named.matches_label("EXAMPLE"));
}
