use qubit_text_codec::{
    ByteOrder,
    TextEncoding,
    UnicodeBom,
};

#[test]
fn test_unicode_bom_exposes_bytes_lengths_orders_and_encodings() {
    let boms = [
        (
            UnicodeBom::Utf8,
            &[0xef, 0xbb, 0xbf][..],
            TextEncoding::UTF_8,
            None,
        ),
        (
            UnicodeBom::Utf16BigEndian,
            &[0xfe, 0xff][..],
            TextEncoding::UTF_16,
            Some(ByteOrder::BigEndian),
        ),
        (
            UnicodeBom::Utf16LittleEndian,
            &[0xff, 0xfe][..],
            TextEncoding::UTF_16,
            Some(ByteOrder::LittleEndian),
        ),
        (
            UnicodeBom::Utf32BigEndian,
            &[0x00, 0x00, 0xfe, 0xff][..],
            TextEncoding::UTF_32,
            Some(ByteOrder::BigEndian),
        ),
        (
            UnicodeBom::Utf32LittleEndian,
            &[0xff, 0xfe, 0x00, 0x00][..],
            TextEncoding::UTF_32,
            Some(ByteOrder::LittleEndian),
        ),
    ];

    for (bom, bytes, encoding, byte_order) in boms {
        assert_eq!(bytes, bom.bytes());
        assert_eq!(bytes.len(), bom.byte_len());
        assert_eq!(encoding, bom.encoding());
        assert_eq!(byte_order, bom.byte_order());
        assert_eq!(Some(bom), UnicodeBom::detect(bytes));
    }
    assert_eq!(None, UnicodeBom::detect(&[0, 1, 2, 3]));
}
