use super::*;

#[test]
fn bar_reveal_survives_close() {
    let motion = MotionProfile::default();
    let mut state = SettingsState::default();
    state.toggle_bar(String::from("field"), motion);
    state.tick_bars(0.05);
    state.tick_bars(0.05);
    let open_x = state.bar_reveals["field"].x;
    assert!(open_x > 0.0 && open_x < 1.0);

    state.toggle_bar(String::from("field"), motion);
    state.tick_bars(0.05);
    assert!(state.bar_reveals["field"].x < open_x);
    assert!(state.bar_reveals.contains_key("field"));
    state.tick_bars(0.05);
    assert!(!state.bar_reveals.contains_key("field"));
}
