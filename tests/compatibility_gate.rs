use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use serde_json::Value;

const RETAINED_DECODERS: [&str; 2] =
    ["config.retired-picker-fields-v0", "identifier.settings-category-v0"];

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn manifest() -> Value {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/compatibility/manifest.json"
    )))
    .expect("valid compatibility manifest")
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_else(|| panic!("missing string field {key}"))
}

fn generation(value: &Value, key: &str) -> u64 {
    value[key].as_u64().unwrap_or_else(|| panic!("missing generation field {key}"))
}

#[test]
fn retained_decoder_inventory_and_fixtures_are_complete() {
    let manifest = manifest();
    assert_eq!(manifest["schema"], 1);
    assert_eq!(manifest["corpus"]["version"], 1);
    assert_eq!(manifest["corpus"]["owner"], "Skwd project");
    assert_eq!(manifest["corpus"]["license"], "GPL-3.0-or-later");
    assert_eq!(manifest["corpus"]["provenance"], "synthetic");

    let decoders = manifest["decoders"].as_array().expect("decoder inventory");
    let actual: BTreeSet<&str> = decoders.iter().map(|entry| text(entry, "id")).collect();
    let expected: BTreeSet<&str> = RETAINED_DECODERS.into_iter().collect();
    assert_eq!(actual, expected, "decoder inventory changed without an explicit gate edit");

    let families: BTreeSet<&str> = decoders.iter().map(|entry| text(entry, "family")).collect();
    assert_eq!(families, BTreeSet::from(["config", "identifier"]));

    let root = repository_root();
    for entry in decoders {
        verify_entry(&root, &manifest, entry);
    }
}

fn verify_entry(root: &Path, manifest: &Value, entry: &Value) {
    let id = text(entry, "id");
    assert_eq!(text(entry, "owner"), "skwd-wall");
    assert!(!text(entry, "oldest_input_release").is_empty());
    assert!(!text(entry, "newest_input_release").is_empty());
    assert!(
        generation(entry, "oldest_input_generation")
            <= generation(entry, "newest_input_generation")
    );

    let retirement = &entry["retirement"];
    assert!(!text(retirement, "condition").is_empty());

    match text(retirement, "status") {
        "retained" => verify_retained_entry(root, entry),
        "retired" => verify_retired_entry(root, manifest, entry),
        status => panic!("decoder {id} has unknown retirement status {status}"),
    }
}

fn verify_retained_entry(root: &Path, entry: &Value) {
    let id = text(entry, "id");
    let retirement = &entry["retirement"];
    assert_eq!(retirement["fixture_retired"], false);
    assert!(retirement["retired_in"].is_null());

    let fixture = Path::new(text(entry, "fixture"));
    assert!(!fixture.is_absolute(), "fixture for {id} must be relative");
    assert!(
        !fixture.components().any(|part| part == Component::ParentDir),
        "fixture for {id} escapes the repository"
    );
    let fixture = root.join(fixture);
    let metadata = std::fs::metadata(&fixture)
        .unwrap_or_else(|error| panic!("fixture for {id} is missing: {error}"));
    assert!(metadata.is_file(), "fixture for {id} is not a file");
    assert!(metadata.len() <= 64 * 1024, "fixture for {id} exceeds 64 KiB");
    let fixture_value: Value = serde_json::from_slice(&std::fs::read(fixture).unwrap())
        .unwrap_or_else(|error| panic!("fixture for {id} is invalid JSON: {error}"));
    assert_eq!(fixture_value["schema"], 1);
    assert_eq!(fixture_value["input_release"], entry["oldest_input_release"]);
    assert_eq!(fixture_value["input_release"], entry["newest_input_release"]);

    for (path_key, needle_key) in
        [("decoder_source", "decoder_needle"), ("test_source", "test_needle")]
    {
        let path = root.join(text(entry, path_key));
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{path_key} for {id} is missing: {error}"));
        assert!(source.contains(text(entry, needle_key)), "{needle_key} for {id} disappeared");
    }
}

fn verify_retired_entry(root: &Path, manifest: &Value, entry: &Value) {
    let id = text(entry, "id");
    let floor_release = manifest["release_support"]["oldest_supported_release"]
        .as_str()
        .filter(|release| !release.is_empty())
        .unwrap_or_else(|| panic!("decoder {id} retired without a released support floor"));
    let floor_generation = manifest["release_support"]["oldest_supported_generation"]
        .as_u64()
        .unwrap_or_else(|| panic!("decoder {id} retired without a support generation"));
    assert!(!floor_release.is_empty());
    assert!(
        floor_generation > generation(entry, "newest_input_generation"),
        "decoder {id} retired before its accepted inputs left support"
    );
    assert_eq!(entry["retirement"]["fixture_retired"], true);
    assert!(
        entry["retirement"]["retired_in"].as_str().is_some_and(|release| !release.is_empty()),
        "decoder {id} has no retirement release"
    );
    assert!(!root.join(text(entry, "fixture")).exists(), "retired fixture for {id} still exists");
}

#[test]
fn current_manifest_blocks_retirement_without_a_release_support_floor() {
    let manifest = manifest();
    assert!(manifest["release_support"]["oldest_supported_release"].is_null());
    assert!(manifest["release_support"]["oldest_supported_generation"].is_null());
    assert!(manifest["decoders"].as_array().unwrap().iter().all(|entry| {
        entry["retirement"]["status"] == "retained"
            && entry["retirement"]["fixture_retired"] == false
    }));
}
