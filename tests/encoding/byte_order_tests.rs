use qubit_text_codec::ByteOrder;

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
    assert_eq!(
        [0x00, 0x01, 0xf6, 0x00],
        ByteOrder::BigEndian.u32_bytes(0x0001f600),
    );
    assert_eq!(
        [0x00, 0xf6, 0x01, 0x00],
        ByteOrder::LittleEndian.u32_bytes(0x0001f600),
    );
}
