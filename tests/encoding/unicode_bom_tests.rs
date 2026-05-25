use qubit_text_codec::{
    ByteOrder,
    Charset,
    UnicodeBom,
};

#[test]
fn test_unicode_bom_exposes_bytes_lengths_orders_and_charsets() {
    let boms = [
        (UnicodeBom::Utf8, &[0xef, 0xbb, 0xbf][..], Charset::UTF_8, None),
        (
            UnicodeBom::Utf16BigEndian,
            &[0xfe, 0xff][..],
            Charset::UTF_16BE,
            Some(ByteOrder::BigEndian),
        ),
        (
            UnicodeBom::Utf16LittleEndian,
            &[0xff, 0xfe][..],
            Charset::UTF_16LE,
            Some(ByteOrder::LittleEndian),
        ),
        (
            UnicodeBom::Utf32BigEndian,
            &[0x00, 0x00, 0xfe, 0xff][..],
            Charset::UTF_32BE,
            Some(ByteOrder::BigEndian),
        ),
        (
            UnicodeBom::Utf32LittleEndian,
            &[0xff, 0xfe, 0x00, 0x00][..],
            Charset::UTF_32LE,
            Some(ByteOrder::LittleEndian),
        ),
    ];

    for (bom, bytes, encoding, byte_order) in boms {
        assert_eq!(bytes, bom.bytes());
        assert_eq!(bytes.len(), bom.byte_len());
        assert_eq!(encoding, bom.charset());
        assert_eq!(byte_order, bom.byte_order());
        assert_eq!(Some(bom), UnicodeBom::detect(bytes));
    }
    assert_eq!(None, UnicodeBom::detect(&[0, 1, 2, 3]));
}
