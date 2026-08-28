use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &str) -> String {
    std::fs::read_to_string(root().join(path))
        .unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn folio_solid_planes() {
    let browser = read("src/frontend/browser/view.rs");
    let filters = read("src/frontend/ui/browser_bar/canvas.rs");
    for (surface, source) in [("browser", browser), ("filter rail", filters)] {
        assert!(!source.contains("Gradient"), "{surface} gradient");
        assert!(!source.contains("gradient::"), "{surface} gradient builder");
    }
}

#[test]
fn theme_backend_read_only() {
    let theme_update = read("src/app/update/theme.rs");
    let theme_helper = read("src/app/helpers/theme.rs");
    let ui_command = read("src/app/update/ui_command.rs");
    assert!(!theme_update.contains("set_key(theme::BACKEND"));
    assert!(!theme_helper.contains("set_key(skwd_config::keys::theme::BACKEND"));
    assert!(!ui_command.contains("set_key(skwd_config::keys::theme::BACKEND"));
}
