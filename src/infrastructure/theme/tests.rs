#![cfg(test)]

use serde_json::json;

use crate::contracts::picker::PaletteSpec;

use super::{
    decode_candidate, decode_palette, encode_candidate, find_saved, load_palette, remove_saved,
    saved_names, saved_palettes, upsert_saved,
};

#[test]
fn palette_alias_precedence() {
    let spec = decode_palette(&json!({
        "primaryText": "#000000",
        "on_primary": "#ffffff",
        "surface_variant": "#010101",
        "surfaceContainer": "#020202"
    }));
    assert_eq!(spec.primary_text.as_deref(), Some("#000000"));
    assert_eq!(spec.surface_variant.as_deref(), Some("#010101"));
    assert_eq!(spec.surface_container.as_deref(), Some("#020202"));
    assert!(spec.primary.is_none());
}

#[test]
fn loader_falls_back() {
    let root = std::env::temp_dir().join(format!("skwd-palette-loader-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    assert_eq!(load_palette(root.to_str().unwrap()), PaletteSpec::default());
    std::fs::write(root.join("colors.json"), "not json").unwrap();
    assert_eq!(load_palette(root.to_str().unwrap()), PaletteSpec::default());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn candidate_codec_aliases() {
    let candidate = crate::domain::theme::Candidate::from_preset("nord").unwrap();
    let value = encode_candidate(&candidate, true);
    assert_eq!(value["on_primary"], value["primaryText"]);
    assert_eq!(value["on_surface"], value["surfaceText"]);
    assert_eq!(decode_candidate(&value), candidate);
}

#[test]
fn saved_theme_storage() {
    let nord = crate::domain::theme::Candidate::from_preset("nord").unwrap();
    let dracula = crate::domain::theme::Candidate::from_preset("dracula").unwrap();

    let one = upsert_saved(&[], "Mine", &nord);
    assert_eq!(saved_names(&one), vec!["Mine"]);
    let replaced = upsert_saved(&one, "Mine", &dracula);
    assert_eq!(replaced.len(), 1);
    assert_eq!(find_saved(&replaced, "Mine"), Some(dracula.clone()));

    let two = upsert_saved(&replaced, "Other", &nord);
    assert_eq!(
        saved_palettes(&two),
        vec![("Mine".to_string(), dracula), ("Other".to_string(), nord.clone())]
    );
    let remaining = remove_saved(&two, "Mine");
    assert_eq!(saved_names(&remaining), vec!["Other"]);
    assert_eq!(find_saved(&remaining, "missing"), None);
}
