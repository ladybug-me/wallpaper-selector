#![cfg(test)]

#[test]
fn initial_surface_overlay() {
    let shell = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/shell/shell.rs"),
    )
    .expect("read shell.rs");
    assert!(!shell.contains("StartMode::Background"));
    assert!(
        shell.contains("exclusive_zone: -1")
            && shell.contains("KeyboardInteractivity::Exclusive")
            && shell.contains("size: Some((0, 0))")
    );
    let warm = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/app/warm.rs"),
    )
    .expect("read warm.rs");
    assert!(!warm.contains("layershell_open"));
}

#[test]
fn hide_exits_cold() {
    let warm = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/app/warm.rs"),
    )
    .expect("read warm.rs");
    assert!(warm.contains("crate::hard_exit(0)"));
    assert!(warm.contains("fn exit_picker(app: &mut App) -> !"));
    assert!(!warm.contains("fn hide(") && !warm.contains("fn show("));
    assert!(
        !warm.contains("keep_warm") && !warm.contains("warm_forever") && !warm.contains("IdleExit")
    );
    let update = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/app/update/mod.rs"),
    )
    .expect("read update.rs");
    assert!(!update.contains("crate::hard_exit"));
}

#[test]
fn wake_channel_via_app_state() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for file in [
        "src/app/warm.rs",
        "src/app/runtime/semantic.rs",
        "src/app/helpers/effects.rs",
        "src/app/update/browser.rs",
        "src/app/runtime/events.rs",
        "src/app/helpers/selection.rs",
    ] {
        let source = std::fs::read_to_string(root.join(file)).expect("read wake consumer");
        assert!(!source.contains("wake_sender"), "{file}");
    }
    let shell = std::fs::read_to_string(root.join("src/shell/shell.rs")).expect("read shell.rs");
    assert!(shell.contains("wake_sender"));
}
