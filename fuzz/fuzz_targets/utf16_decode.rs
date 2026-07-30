#![no_main]

use libfuzzer_sys::fuzz_target;
use qubit_codec::ByteOrder;
use qubit_codec_text::{CharsetDecodePolicy, CharsetDecoder, Utf16ByteCodec};

const MAX_FUZZ_INPUT_LEN: usize = 4_096;

fuzz_target!(|data: &[u8]| {
    let data = &data[..data.len().min(MAX_FUZZ_INPUT_LEN)];
    fuzz_order(data, ByteOrder::LittleEndian);
    fuzz_order(data, ByteOrder::BigEndian);
});

fn fuzz_order(data: &[u8], byte_order: ByteOrder) {
    let mut replace = CharsetDecoder::new(Utf16ByteCodec::new(byte_order));
    let mut replace_output = vec!['\0'; data.len()];
    let replace_result = replace.transcode_complete_into(data, &mut replace_output);

    let mut report = CharsetDecoder::with_policy(
        Utf16ByteCodec::new(byte_order),
        CharsetDecodePolicy::report(),
    );
    let mut report_output = vec!['\0'; data.len()];
    let report_result = report.transcode_complete_into(data, &mut report_output);

    if data.len().is_multiple_of(2) {
        let units = data
            .chunks_exact(2)
            .map(|bytes| match byte_order {
                ByteOrder::LittleEndian => u16::from_le_bytes([bytes[0], bytes[1]]),
                ByteOrder::BigEndian => u16::from_be_bytes([bytes[0], bytes[1]]),
                ByteOrder::NativeEndian => u16::from_ne_bytes([bytes[0], bytes[1]]),
            })
            .collect::<Vec<_>>();
        let expected = core::char::decode_utf16(units).collect::<Result<Vec<_>, _>>();
        if let Ok(expected) = expected {
            let replace_written = replace_result.expect("valid UTF-16 must decode");
            let report_written = report_result.expect("valid UTF-16 must report success");
            assert_eq!(expected, replace_output[..replace_written]);
            assert_eq!(expected, report_output[..report_written]);
        }
    }
}
