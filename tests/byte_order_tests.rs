use qubit_unicode::{
    ByteOrder,
    UnicodeBom,
};

#[test]
fn test_byte_order_reads_and_writes_integers() {
    assert_eq!(0x1234, ByteOrder::BigEndian.read_u16(&[0x12, 0x34]));
    assert_eq!(0x1234, ByteOrder::LittleEndian.read_u16(&[0x34, 0x12]));
    assert_eq!(
        0x0001f600,
        ByteOrder::BigEndian.read_u32(&[0x00, 0x01, 0xf6, 0x00])
    );
    assert_eq!(
        0x0001f600,
        ByteOrder::LittleEndian.read_u32(&[0x00, 0xf6, 0x01, 0x00])
    );
    assert_eq!([0x12, 0x34], ByteOrder::BigEndian.u16_bytes(0x1234));
    assert_eq!([0x34, 0x12], ByteOrder::LittleEndian.u16_bytes(0x1234));
}

#[test]
fn test_unicode_bom_exposes_bytes_lengths_and_orders() {
    assert_eq!(&[0xef, 0xbb, 0xbf], UnicodeBom::Utf8.bytes());
    assert_eq!(3, UnicodeBom::Utf8.byte_len());
    assert_eq!(&[0xfe, 0xff], UnicodeBom::Utf16BigEndian.bytes());
    assert_eq!(2, UnicodeBom::Utf16BigEndian.byte_len());
    assert_eq!(
        Some(ByteOrder::BigEndian),
        UnicodeBom::Utf16BigEndian.byte_order()
    );
    assert_eq!(
        Some(ByteOrder::LittleEndian),
        UnicodeBom::Utf32LittleEndian.byte_order()
    );
}
