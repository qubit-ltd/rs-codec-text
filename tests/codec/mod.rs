pub use qubit_codec_text::{
    Charset,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
    Unicode,
    Utf8,
    Utf16,
    Utf32,
};

mod ascii_codec_tests;
mod charset_codec_tests;
mod charset_convert_error_tests;
mod charset_converter_tests;
mod charset_decoder_tests;
mod charset_encoder_tests;
mod codec_stdlib_consistency_tests;
mod core_codec_trait_tests;
mod latin1_codec_tests;
mod malformed_action_tests;
mod unmappable_action_tests;
mod utf16_byte_codec_tests;
mod utf16_u16_codec_tests;
mod utf32_byte_codec_tests;
mod utf32_u32_codec_tests;
mod utf8_codec_tests;
