#![no_main]

use libfuzzer_sys::fuzz_target;
use qubit_codec::ByteOrder;
use qubit_codec_text::{CharsetDecodePolicy, CharsetDecoder, Utf16ByteCodec};

fuzz_target!(|data: &[u8]| {
    fuzz_order(data, ByteOrder::LittleEndian);
    fuzz_order(data, ByteOrder::BigEndian);
});

fn fuzz_order(data: &[u8], byte_order: ByteOrder) {
    let mut replace = CharsetDecoder::new(Utf16ByteCodec::new(byte_order));
    let _ = replace.decode_to_string(data);

    let mut report = CharsetDecoder::with_policy(
        Utf16ByteCodec::new(byte_order),
        CharsetDecodePolicy::report(),
    );
    let _ = report.decode_to_string(data);
}
