#![no_main]

use libfuzzer_sys::fuzz_target;
use qubit_codec_text::{CharsetDecodePolicy, CharsetDecoder, Utf8Codec};

const MAX_FUZZ_INPUT_LEN: usize = 4_096;

fuzz_target!(|data: &[u8]| {
    let data = &data[..data.len().min(MAX_FUZZ_INPUT_LEN)];
    let mut replace = CharsetDecoder::new(Utf8Codec);
    let mut replace_output = vec!['\0'; data.len()];
    let replace_result = replace.transcode_complete_into(data, &mut replace_output);

    let mut report = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());
    let mut report_output = vec!['\0'; data.len()];
    let report_result = report.transcode_complete_into(data, &mut report_output);

    if let Ok(expected) = core::str::from_utf8(data) {
        let expected = expected.chars().collect::<Vec<_>>();
        let replace_written = replace_result.expect("valid UTF-8 must decode");
        let report_written = report_result.expect("valid UTF-8 must report success");
        assert_eq!(expected, replace_output[..replace_written]);
        assert_eq!(expected, report_output[..report_written]);
    }
});
