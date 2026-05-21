use std::fs;
use std::path::Path;

/// Reads a crate source file for implementation-level regression checks.
fn read_source_file(file_name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(file_name);
    fs::read_to_string(&path).expect("source file should be readable")
}

/// Removes whitespace so formatting changes do not break source-pattern checks.
fn compact_source(source: &str) -> String {
    source.chars().filter(|ch| !ch.is_whitespace()).collect()
}

#[test]
fn test_utf8_put_capacity_guard_cannot_overflow() {
    let source = compact_source(&read_source_file("utf8.rs"));

    assert!(
        source.contains("ifend_index-index<count{")
            || source.contains("index.checked_add(count)"),
        "Utf8::put should use a non-overflowing capacity guard"
    );
    assert!(
        !source.contains("ifindex+count>end_index{"),
        "Utf8::put must not use an overflowing index + count capacity guard"
    );
}

#[test]
fn test_utf16_put_capacity_guard_cannot_overflow() {
    let source = compact_source(&read_source_file("utf16.rs"));

    assert!(
        source.contains("ifend_index-index<count{")
            || source.contains("index.checked_add(count)"),
        "Utf16::put should use a non-overflowing capacity guard"
    );
    assert!(
        !source.contains("ifindex+count>end_index{"),
        "Utf16::put must not use an overflowing index + count capacity guard"
    );
}
