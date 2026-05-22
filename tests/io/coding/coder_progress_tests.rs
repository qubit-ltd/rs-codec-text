use qubit_text_codec::{
    CoderProgress,
    CoderStatus,
};

#[test]
fn test_coder_progress_exposes_status_and_counts() {
    let complete = CoderProgress::complete(2, 3);
    assert_eq!(CoderStatus::Complete, complete.status());
    assert_eq!(2, complete.read());
    assert_eq!(3, complete.written());

    assert_eq!(
        CoderStatus::NeedInput,
        CoderProgress::need_input(1, 1).status(),
    );
    assert_eq!(
        CoderStatus::NeedOutput,
        CoderProgress::need_output(1, 0).status(),
    );
}
