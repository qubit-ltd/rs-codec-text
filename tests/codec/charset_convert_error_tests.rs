use qubit_codec_text::{
    Charset,
    CharsetConvertError,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
};

#[test]
fn test_charset_convert_error_wraps_decode_and_encode_errors() {
    let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
    let decode = CharsetConvertError::from(CharsetDecodeError::new(
        Charset::UTF_8,
        kind,
        2,
    ));
    assert!(
        decode
            .to_string()
            .contains("Failed to decode source charset")
    );

    let kind = CharsetEncodeErrorKind::BufferTooSmall {
        required: 4,
        available: 0,
    };
    let encode = CharsetConvertError::from(CharsetEncodeError::new(
        Charset::UTF_8,
        kind,
        4,
    ));
    assert!(
        encode
            .to_string()
            .contains("Failed to encode target charset")
    );
}
