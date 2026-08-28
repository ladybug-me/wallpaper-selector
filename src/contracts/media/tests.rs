use super::MediaKind;

#[test]
fn default_preserves_the_wire_empty_state() {
    assert_eq!(MediaKind::default(), MediaKind::Other(String::new()));
    assert_eq!(MediaKind::default().as_key(), "");
    assert!(!MediaKind::default().has_audio_controls());
}

#[test]
fn future_kinds_round_trip_without_gaining_known_behaviour() {
    let kind = MediaKind::from_key("future-scene");
    assert_eq!(kind.as_key(), "future-scene");
    assert!(!kind.has_audio_controls());
}
