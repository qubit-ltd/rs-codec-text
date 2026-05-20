# Qubit Unicode User Guide

Qubit Unicode is for parsers, codecs, and compatibility layers that need
explicit control over Unicode scalar values, UTF-8 bytes, UTF-16 code units, and
ASCII-only behavior.

## Choosing an API

Use standard Rust APIs first:

- `std::str::from_utf8` for validating a full UTF-8 byte slice;
- `String::from_utf16` and `char::decode_utf16` for ordinary UTF-16 decoding;
- `char::encode_utf8` and `char::encode_utf16` for one scalar value.

Use this crate when you need cursor-based partial decoding, precise malformed
or incomplete sequence reporting, caller-owned buffers, or Java-compatible
ASCII folding and Unicode escape behavior.

## Error Handling

Cursor and encoder APIs return `UnicodeResult<T>`. Errors carry a
`UnicodeErrorKind` and the index where the problem was detected. The kind maps
to the same core categories as the Java utilities:

- `BufferOverflow`;
- `MalformedUnicode`;
- `IncompleteUnicode`.

## ASCII Folding

`Ascii::fold` ports the Java `Ascii.fold` mapping table and writes up to
`Ascii::MAX_FOLDING` output characters. Unknown non-ASCII characters are kept
unchanged.
