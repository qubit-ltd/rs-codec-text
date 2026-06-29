use qubit_codec_text::UnmappableAction;

#[test]
fn test_unmappable_action_default_replaces() {
    assert_eq!(UnmappableAction::Replace, UnmappableAction::default());
}
