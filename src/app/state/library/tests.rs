use super::*;

#[test]
fn preheat_coalesces_focus_changes() {
    let mut state = PreheatState::default();
    let start = Instant::now();
    let delay = Duration::from_millis(120);

    assert_eq!(state.poll(Some("a"), start, delay), None);
    assert_eq!(state.poll(Some("b"), start + Duration::from_millis(50), delay), None);
    assert_eq!(state.poll(Some("c"), start + Duration::from_millis(100), delay), None);
    assert_eq!(state.poll(Some("c"), start + Duration::from_millis(219), delay), None);
    assert_eq!(
        state.poll(Some("c"), start + Duration::from_millis(220), delay),
        Some("c".to_string())
    );
    assert_eq!(state.poll(Some("c"), start + Duration::from_secs(1), delay), None);
}

#[test]
fn preheat_cancels_on_unfocus() {
    let mut state = PreheatState::default();
    let start = Instant::now();
    let delay = Duration::from_millis(120);

    assert_eq!(state.poll(Some("a"), start, delay), None);
    assert_eq!(state.poll(None, start + delay, delay), None);
    assert_eq!(state.poll(Some("a"), start + delay * 2, delay), None);
    assert_eq!(state.poll(Some("a"), start + delay * 3, delay), Some("a".to_string()));
}
