#![no_main]

use libfuzzer_sys::fuzz_target;
use qubit_codec_text::{CharsetDecodePolicy, CharsetDecoder, Utf8Codec};

fuzz_target!(|data: &[u8]| {
    let mut replace = CharsetDecoder::new(Utf8Codec);
    let _ = replace.decode_to_string(data);

    let mut report = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());
    let _ = report.decode_to_string(data);
});
