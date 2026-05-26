use qubit_text_codec::{
    Charset,
    CharsetDecodeErrorKind,
    DecodeStatus,
};

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
        DecodeStatus::NeedMore {
            required: 3,
            available: 1,
        },
        DecodeStatus::NeedMore {
            required: 3,
            available: 1,
        },
    );
}

#[test]
fn test_decode_status_converts_need_more_to_incomplete_error() {
    let status = DecodeStatus::NeedMore {
        required: 7,
        available: 1,
    };

    let error = status
        .incomplete_error(Charset::UTF_8, 4)
        .expect("need-more status becomes incomplete error");

    assert_eq!(Charset::UTF_8, error.charset());
    assert_eq!(4, error.index());
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 3,
            available: 1,
        },
        error.kind(),
    );
    assert_eq!(
        None,
        DecodeStatus::Complete {
            value: 'A',
            consumed: 1,
        }
        .incomplete_error(Charset::UTF_8, 0),
    );
}
