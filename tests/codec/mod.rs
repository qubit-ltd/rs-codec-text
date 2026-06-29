mod ascii_codec_tests;
mod assertions_tests;
mod charset_codec_tests;
mod codec_stdlib_consistency_tests;
mod core_codec_trait_tests;
mod latin1_codec_tests;
mod property_tests;
mod utf16_byte_codec_tests;
mod utf16_u16_codec_tests;
mod utf32_byte_codec_tests;
mod utf32_u32_codec_tests;
mod utf8_codec_tests;

pub(crate) use assertions_tests::{
    incomplete_required,
    invalid_source,
};
