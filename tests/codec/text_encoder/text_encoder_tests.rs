use qubit_text_codec::{
    TextEncoder,
    TextEncodingErrorKind,
    Utf8Encoder,
};

#[test]
fn test_text_encoder_default_encode_code_point_rejects_invalid_code_point() {
    let error = Utf8Encoder
        .encode_code_point(0x110000, &mut [0_u8; 4])
        .expect_err("invalid code point must fail");

    assert_eq!(TextEncodingErrorKind::InvalidCodePoint, error.kind());
}
