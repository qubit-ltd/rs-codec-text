use qubit_text_codec::{
    TextDecoder,
    TextDecodingErrorKind,
    Utf8Decoder,
};

#[test]
fn test_text_decoder_default_decode_next_covers_all_branches() {
    let decoder = Utf8Decoder;

    let mut index = 0;
    assert_eq!(
        Some('A'),
        decoder.decode_next(b"A", &mut index).expect("complete")
    );
    assert_eq!(1, index);
    assert_eq!(None, decoder.decode_next(b"A", &mut index).expect("EOF"));

    let mut incomplete_index = 0;
    let error = decoder
        .decode_next(&[0xe4], &mut incomplete_index)
        .expect_err("closed incomplete input must fail");
    assert_eq!(TextDecodingErrorKind::IncompleteSequence, error.kind());
    assert_eq!(1, error.index());

    let mut malformed_index = 1;
    let error = decoder
        .decode_next(&[b'A', 0x80], &mut malformed_index)
        .expect_err("malformed input must be offset by the cursor");
    assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());

    let mut out_of_bounds = 2;
    let error = decoder
        .decode_next(b"A", &mut out_of_bounds)
        .expect_err("out-of-bounds input index must fail");
    assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
    assert_eq!(2, error.index());
}
