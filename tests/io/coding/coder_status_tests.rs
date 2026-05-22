use qubit_text_codec::CoderStatus;

#[test]
fn test_coder_status_variants_are_distinct() {
    assert_ne!(CoderStatus::Complete, CoderStatus::NeedInput);
    assert_ne!(CoderStatus::NeedInput, CoderStatus::NeedOutput);
}
