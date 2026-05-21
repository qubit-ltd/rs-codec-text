use qubit_text_codec::DecodeStatus;

#[test]
fn test_decode_status_variants_expose_payloads() {
    assert_eq!(
        DecodeStatus::Complete {
            value: 'A',
            consumed: 1,
        },
        DecodeStatus::Complete {
            value: 'A',
            consumed: 1,
        },
    );
    assert_eq!(
        DecodeStatus::<char>::NeedMore {
            required: 3,
            available: 1,
        },
        DecodeStatus::NeedMore {
            required: 3,
            available: 1,
        },
    );
}
