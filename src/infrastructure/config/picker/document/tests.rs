use std::os::unix::fs::PermissionsExt;

use super::Config;
use super::secure_config_file;
use serde_json::json;

#[test]
fn retained_picker_config_fixture_loads_and_saves_canonically() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/compatibility/fixtures/config-retired-picker-fields-v0.json"
    )))
    .unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    let mut config = Config::from_data(fixture["input"].clone());
    config.config_path = path.clone();

    assert_eq!(config.root(), &fixture["canonical"]);
    config.persist();

    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(saved, fixture["canonical"]);
}

#[test]
fn secure_private_noop() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    std::fs::write(&path, b"{}").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();

    assert!(!secure_config_file(&path));
    assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o7777, 0o600);
}

#[test]
fn secure_public_once() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    std::fs::write(&path, b"{}").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();

    assert!(secure_config_file(&path));
    assert!(!secure_config_file(&path));
    assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o7777, 0o600);
}

#[test]
fn legacy_we_engine_migrates() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    let mut config = Config::from_data(json!({
        "weRender": {"engine": "compatibility", "native": false, "fps": 30}
    }));
    config.config_path = path.clone();

    assert_eq!(config.str_path(skwd_config::keys::we_render::ENGINE), "native");
    config.persist();

    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(saved["weRender"]["engine"], "native");
    assert_eq!(saved["weRender"]["fps"], 30);
    assert!(saved["weRender"].get("native").is_none());
}

#[test]
fn legacy_paper_engine_migrates() {
    for retired in ["noctalia", "dms", "unknown"] {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");
        let mut config = Config::from_data(json!({"paper": {"engine": retired}}));
        config.config_path = path.clone();

        config.persist();

        let saved: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        assert_eq!(saved["paper"]["engine"], "skwd-paper");
    }
}

#[test]
fn resolution_presets_migrate() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    let mut config = Config::from_data(json!({
        "filterBar": {"resolutionPresets": [
            {"label": "FHD", "width": 1920, "height": 1080},
            {"label": "2K", "width": 2560, "height": 1440},
            {"label": "4K", "width": 3840, "height": 2160}
        ]}
    }));
    config.config_path = path.clone();

    assert_eq!(config.str_path("filterBar.resolutionPresets.0.from"), "1920x1080");
    assert_eq!(config.str_path("filterBar.resolutionPresets.0.to"), "2559x1439");
    config.persist();

    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let preset = &saved["filterBar"]["resolutionPresets"][0];
    assert_eq!(preset["from"], "1920x1080");
    assert_eq!(preset["to"], "2559x1439");
    assert_eq!(preset["orientation"], "wide");
    assert!(preset.get("width").is_none());
    assert!(preset.get("height").is_none());
    assert_eq!(saved["filterBar"]["resolutionPresets"][1]["to"], "3839x2159");
    assert_eq!(saved["filterBar"]["resolutionPresets"][2]["to"], "");
}

#[test]
fn awww_engine_kept() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    let mut config = Config::from_data(json!({"paper": {"engine": "awww"}}));
    config.config_path = path.clone();

    config.persist();

    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(saved["paper"]["engine"], "awww");
}

#[test]
fn video_optimize_retired() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    let mut config = Config::from_data(json!({
        "videoOptimize": {"enabled": true, "codec": "av1", "maxHeight": 1440},
        "paper": {"videoEngine": "tinier"}
    }));
    config.config_path = path.clone();

    config.persist();

    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert!(saved.get("videoOptimize").is_none());
    assert_eq!(saved["paper"]["videoEngine"], "tinier");
}

#[test]
fn preview_mode_retired() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    let mut config = Config::from_data(json!({
        "videoPreview": {"enabled": false, "mode": "loop", "delayMs": 500}
    }));
    config.config_path = path.clone();

    config.persist();

    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    assert!(saved["videoPreview"].get("mode").is_none());
    assert_eq!(saved["videoPreview"]["enabled"], false);
    assert_eq!(saved["videoPreview"]["delayMs"], 500);
}
