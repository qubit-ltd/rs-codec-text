use qubit_codec_text::{
    ByteOrder, Charset, CharsetCodec, CharsetDecodeErrorKind, CharsetDecodeResult,
    CharsetEncodeProbe, CharsetEncodeResult, Codec, Utf16, Utf16ByteCodec,
};

type DecodedCharResult = CharsetDecodeResult<(char, core::num::NonZeroUsize)>;
type DecodeFn = unsafe fn(&mut Utf16ByteCodec, &[u8], usize) -> DecodedCharResult;
type EncodeFn = unsafe fn(
    &mut Utf16ByteCodec,
    &char,
    &mut [u8],
    usize,
) -> CharsetEncodeResult<core::num::NonZeroUsize>;

#[test]
fn test_utf16_byte_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);

    assert_eq!(
        Charset::UTF_16LE,
        <Utf16ByteCodec as CharsetCodec>::charset(&codec)
    );
    assert_eq!(2, codec.min_units_per_value().get());
    assert_eq!(Utf16::MAX_BYTES_PER_CHAR, codec.max_units_per_value().get());
    assert_eq!(
        2,
        CharsetEncodeProbe::encode_len(&codec, 'A', 0).expect("encode UTF-16 bytes")
    );

    assert_eq!(ByteOrder::LittleEndian, codec.byte_order());
    assert_eq!(Charset::UTF_16LE, codec.charset());
}

#[test]
fn test_utf16_byte_codec_encodes_and_decodes_bytes() {
    let mut codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];

    assert_eq!(4, unsafe {
        codec
            .encode(&'😀', &mut output, 0)
            .expect("encode pair bytes")
            .get()
    });
    let (decoded, consumed) = unsafe { codec.decode(&output, 0) }.expect("decode pair bytes");
    assert_eq!('😀', decoded);
    assert_eq!(4, consumed.get());
}

#[test]
fn test_utf16_byte_codec_decodes_bmp_and_reports_closed_tail_or_malformed_bytes() {
    let mut codec = Utf16ByteCodec::new(ByteOrder::BigEndian);

    let (decoded, consumed) = unsafe { codec.decode(&[0x00, 0x41], 0) }.expect("BMP bytes");
    assert_eq!('A', decoded);
    assert_eq!(2, consumed.get());

    let error = unsafe { codec.decode(&[0x00], 0) }.expect_err("partial unit is incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 2,
            available: 1,
        },
        error.kind()
    );

    let error = unsafe { codec.decode(&[0xd8, 0x3d], 0) }
        .expect_err("partial surrogate pair is incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 4,
            available: 2,
        },
        error.kind()
    );

    let error = unsafe { codec.decode(&[], 1) }.expect_err("index outside slice should fail");
    assert_eq!(
        CharsetDecodeErrorKind::InvalidInputIndex { input_len: 0 },
        error.kind()
    );
    assert_eq!(1, error.index());

    let error = unsafe { codec.decode(&[0xd8, 0x3d, 0x00, 0x41], 0) }
        .expect_err("high surrogate followed by BMP unit should fail");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some(0x0041)
        },
        error.kind()
    );
    assert_eq!(2, error.index());

    let error =
        unsafe { codec.decode(&[0xde, 0x00], 0) }.expect_err("isolated low surrogate should fail");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some(0xde00)
        },
        error.kind()
    );
    assert_eq!(0, error.index());
}

#[test]
fn test_utf16_byte_codec_encodes_bmp_and_supplementary_scalars() {
    let mut codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];

    assert_eq!(2, unsafe {
        codec
            .encode(&'A', &mut output, 0)
            .expect("BMP byte encoding")
            .get()
    });
    assert_eq!(4, unsafe {
        codec
            .encode(&'😀', &mut output, 0)
            .expect("surrogate pair bytes")
            .get()
    });

    let error = unsafe { codec.encode(&'😀', &mut output[..2], 0) }
        .expect_err("surrogate pair needs four bytes");
    assert_eq!(Some(4), error.required());
    assert_eq!(Some(2), error.available());

    let error = unsafe { codec.encode(&'A', &mut [], 1) }.expect_err("output index outside slice");
    assert_eq!(Some(3), error.required());
    assert_eq!(Some(0), error.available());

    let error = unsafe { codec.encode(&'A', &mut [], usize::MAX) }
        .expect_err("overflowing output index should fail without panicking");
    assert_eq!(Some(usize::MAX), error.required());
    assert_eq!(Some(0), error.available());
}

#[test]
fn test_utf16_byte_codec_direct_function_items_cover_trait_methods() {
    let mut codec = Utf16ByteCodec::new(ByteOrder::BigEndian);
    let new_fn: fn(ByteOrder) -> Utf16ByteCodec = Utf16ByteCodec::new;
    let byte_order: fn(Utf16ByteCodec) -> ByteOrder = Utf16ByteCodec::byte_order;
    let inherent_charset: fn(Utf16ByteCodec) -> Charset = Utf16ByteCodec::charset;
    let trait_charset: fn(&Utf16ByteCodec) -> Charset = <Utf16ByteCodec as CharsetCodec>::charset;
    let min_units: fn(&Utf16ByteCodec) -> core::num::NonZeroUsize =
        <Utf16ByteCodec as Codec>::min_units_per_value;
    let max_units: fn(&Utf16ByteCodec) -> core::num::NonZeroUsize =
        <Utf16ByteCodec as Codec>::max_units_per_value;
    let encode_len: fn(&Utf16ByteCodec, char, usize) -> CharsetEncodeResult<usize> =
        <Utf16ByteCodec as CharsetEncodeProbe>::encode_len;
    let decode: DecodeFn = <Utf16ByteCodec as Codec>::decode;
    let encode: EncodeFn = std::hint::black_box(<Utf16ByteCodec as Codec>::encode);

    assert_eq!(
        ByteOrder::LittleEndian,
        byte_order(new_fn(ByteOrder::LittleEndian))
    );
    assert_eq!(Charset::UTF_16BE, inherent_charset(codec));
    assert_eq!(Charset::UTF_16BE, trait_charset(&codec));
    assert_eq!(2, min_units(&codec).get());
    assert_eq!(Utf16::MAX_BYTES_PER_CHAR, max_units(&codec).get());
    assert_eq!(
        4,
        encode_len(&codec, '😀', 0).expect("UTF-16 byte pair length")
    );

    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];
    assert_eq!(
        4,
        unsafe { encode(&mut codec, &'😀', &mut output, 0) }
            .expect("encode pair bytes")
            .get()
    );
    let (decoded, consumed) = unsafe { decode(&mut codec, &output, 0) }.expect("decode pair bytes");
    assert_eq!(('😀', 4), (decoded, consumed.get()));
}
