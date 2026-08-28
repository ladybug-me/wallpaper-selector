use super::TransitionPreviewState;

#[test]
fn stop_clears_frame() {
    let mut preview = TransitionPreviewState {
        handle: Some(iced::advanced::image::Handle::from_rgba(1, 1, vec![0, 0, 0, 255])),
        ..TransitionPreviewState::default()
    };
    preview.stop();
    assert!(preview.handle.is_none());
}
