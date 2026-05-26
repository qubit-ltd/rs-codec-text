use qubit_codec_text::{
    CharsetCodec,
    Utf8Codec,
};

fn assert_charset_codec<T>(_codec: &T)
where
    T: CharsetCodec<Unit = u8>,
{
}

#[test]
fn test_charset_codec_is_implemented_for_combined_codecs() {
    assert_charset_codec(&Utf8Codec);
}
