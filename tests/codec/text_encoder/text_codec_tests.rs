use qubit_text_codec::{
    TextCodec,
    Utf8Codec,
};

fn assert_text_codec<T>(_codec: &T)
where
    T: TextCodec<u8>,
{
}

#[test]
fn test_text_codec_is_implemented_for_combined_codecs() {
    assert_text_codec(&Utf8Codec);
}
