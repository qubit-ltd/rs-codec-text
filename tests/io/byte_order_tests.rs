use qubit_text_codec::ByteOrder;
use std::hint::black_box;

#[test]
fn test_read_from_array_decodes_fixed_width_values() {
    assert_eq!(
        0x1234,
        ByteOrder::BigEndian.read_u16_from_array([0x12, 0x34]),
    );
    assert_eq!(
        0x1234,
        ByteOrder::LittleEndian.read_u16_from_array([0x34, 0x12]),
    );
    assert_eq!(
        0x0001f600,
        ByteOrder::BigEndian.read_u32_from_array([0x00, 0x01, 0xf6, 0x00]),
    );
    assert_eq!(
        0x0001f600,
        ByteOrder::LittleEndian.read_u32_from_array([0x00, 0xf6, 0x01, 0x00]),
    );
    assert_eq!(
        0x0102_0304_0506_0708,
        ByteOrder::BigEndian.read_u64_from_array([1, 2, 3, 4, 5, 6, 7, 8]),
    );
    assert_eq!(
        0x0102_0304_0506_0708,
        ByteOrder::LittleEndian.read_u64_from_array([8, 7, 6, 5, 4, 3, 2, 1]),
    );
}

#[test]
fn test_read_at_returns_none_when_range_is_not_available() {
    let bytes = [0xaa, 0x12, 0x34, 0x00, 0x01, 0xf6, 0x00];
    let bytes64 = [0xaa, 1, 2, 3, 4, 5, 6, 7, 8];

    assert_eq!(Some(0x1234), ByteOrder::BigEndian.read_u16_at(&bytes, 1));
    assert_eq!(
        Some(0x0001f600),
        ByteOrder::BigEndian.read_u32_at(&bytes, 3),
    );
    assert_eq!(
        Some(0x0102_0304_0506_0708),
        ByteOrder::BigEndian.read_u64_at(&bytes64, 1),
    );
    assert_eq!(None, ByteOrder::BigEndian.read_u16_at(&bytes, 6));
    assert_eq!(None, ByteOrder::BigEndian.read_u32_at(&bytes, 4));
    assert_eq!(None, ByteOrder::BigEndian.read_u64_at(&bytes64, 2));
    assert_eq!(None, ByteOrder::BigEndian.read_u16_at(&bytes, usize::MAX));
    assert_eq!(None, ByteOrder::BigEndian.read_u32_at(&bytes, usize::MAX));
    assert_eq!(None, ByteOrder::BigEndian.read_u64_at(&bytes64, usize::MAX));
}

#[test]
fn test_read_at_unchecked_reads_after_caller_validates_bounds() {
    let bytes = [0xaa, 0x12, 0x34, 0x00, 0x01, 0xf6, 0x00, 0xbb];
    let bytes64 = [0xaa, 8, 7, 6, 5, 4, 3, 2, 1];

    assert!(2 < bytes.len());
    assert!(3 + 4 <= bytes.len());
    assert!(8 < bytes64.len());
    // SAFETY: The assertions above prove the requested byte ranges are in bounds.
    unsafe {
        assert_eq!(
            0x1234,
            ByteOrder::BigEndian.read_u16_at_unchecked(&bytes, 1)
        );
        assert_eq!(
            0x0001f600,
            ByteOrder::BigEndian.read_u32_at_unchecked(&bytes, 3),
        );
        assert_eq!(
            0x0102_0304_0506_0708,
            ByteOrder::LittleEndian.read_u64_at_unchecked(&bytes64, 1),
        );
    }
}

#[test]
fn test_to_bytes_encodes_fixed_width_values() {
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
    assert_eq!(
        [1, 2, 3, 4, 5, 6, 7, 8],
        ByteOrder::BigEndian.u64_bytes(0x0102_0304_0506_0708),
    );
    assert_eq!(
        [8, 7, 6, 5, 4, 3, 2, 1],
        ByteOrder::LittleEndian.u64_bytes(0x0102_0304_0506_0708),
    );
}

#[test]
fn test_write_at_returns_none_when_range_is_not_available() {
    let mut bytes = [0_u8; 8];
    let mut bytes64 = [0_u8; 9];

    assert_eq!(
        Some(()),
        ByteOrder::BigEndian.write_u16_at(&mut bytes, 1, 0x1234)
    );
    assert_eq!(
        Some(()),
        ByteOrder::BigEndian.write_u32_at(&mut bytes, 3, 0x0001f600),
    );
    assert_eq!(
        None,
        ByteOrder::BigEndian.write_u16_at(&mut bytes, 7, 0xabcd)
    );
    assert_eq!(
        None,
        ByteOrder::BigEndian.write_u16_at(&mut bytes, usize::MAX, 0xabcd)
    );
    assert_eq!(None, ByteOrder::BigEndian.write_u32_at(&mut bytes, 6, 0));
    assert_eq!(
        None,
        ByteOrder::BigEndian.write_u32_at(&mut bytes, usize::MAX, 0),
    );
    assert_eq!([0, 0x12, 0x34, 0x00, 0x01, 0xf6, 0x00, 0], bytes,);
    assert_eq!(
        Some(()),
        ByteOrder::BigEndian.write_u64_at(&mut bytes64, 1, 0x0102_0304_0506_0708),
    );
    assert_eq!(
        None,
        ByteOrder::BigEndian.write_u64_at(&mut bytes64, 2, 0x0102_0304_0506_0708),
    );
    assert_eq!(
        None,
        ByteOrder::BigEndian.write_u64_at(&mut bytes64, usize::MAX, 0),
    );
    assert_eq!([0, 1, 2, 3, 4, 5, 6, 7, 8], bytes64);
}

#[test]
fn test_write_at_unchecked_writes_after_caller_validates_bounds() {
    let mut bytes = [0_u8; 8];
    let mut bytes64 = [0_u8; 8];

    assert!(8 <= bytes.len());
    assert!(8 <= bytes64.len());
    // SAFETY: The assertions above prove the requested byte ranges are in bounds.
    unsafe {
        ByteOrder::BigEndian.write_u16_at_unchecked(&mut bytes, 0, 0x1234);
        ByteOrder::BigEndian.write_u32_at_unchecked(&mut bytes, 2, 0x0001f600);
        ByteOrder::LittleEndian.write_u64_at_unchecked(&mut bytes64, 0, 0x0102_0304_0506_0708);
    }

    assert_eq!([0x12, 0x34, 0x00, 0x01, 0xf6, 0x00, 0, 0], bytes);
    assert_eq!([8, 7, 6, 5, 4, 3, 2, 1], bytes64);
}

#[test]
fn test_u64_methods_are_callable_through_function_pointers() {
    let read_from_array: fn(ByteOrder, [u8; 8]) -> u64 = ByteOrder::read_u64_from_array;
    let read_at: fn(ByteOrder, &[u8], usize) -> Option<u64> = ByteOrder::read_u64_at;
    let to_bytes: fn(ByteOrder, u64) -> [u8; 8] = ByteOrder::u64_bytes;
    let write_at: fn(ByteOrder, &mut [u8], usize, u64) -> Option<()> = ByteOrder::write_u64_at;
    let read_unchecked: unsafe fn(ByteOrder, &[u8], usize) -> u64 =
        ByteOrder::read_u64_at_unchecked;
    let write_unchecked: unsafe fn(ByteOrder, &mut [u8], usize, u64) =
        ByteOrder::write_u64_at_unchecked;
    let bytes = black_box([0xaa, 1, 2, 3, 4, 5, 6, 7, 8]);
    let mut output = black_box([0_u8; 8]);

    assert_eq!(
        0x0102_0304_0506_0708,
        read_from_array(ByteOrder::BigEndian, black_box([1, 2, 3, 4, 5, 6, 7, 8])),
    );
    assert_eq!(
        Some(0x0102_0304_0506_0708),
        read_at(ByteOrder::BigEndian, &bytes, 1),
    );
    assert_eq!(
        [8, 7, 6, 5, 4, 3, 2, 1],
        to_bytes(ByteOrder::LittleEndian, 0x0102_0304_0506_0708),
    );
    assert_eq!(
        Some(()),
        write_at(
            ByteOrder::LittleEndian,
            &mut output,
            0,
            0x0102_0304_0506_0708,
        ),
    );
    // SAFETY: Both calls use ranges that are fully contained in their buffers.
    unsafe {
        assert_eq!(
            0x0102_0304_0506_0708,
            read_unchecked(ByteOrder::BigEndian, &bytes, 1),
        );
        write_unchecked(ByteOrder::BigEndian, &mut output, 0, 0x0102_0304_0506_0708);
    }
    assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], output);
}
