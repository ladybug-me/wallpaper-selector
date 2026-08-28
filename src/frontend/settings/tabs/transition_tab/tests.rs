#![cfg(test)]

use super::{pretty, shader_scope, shader_scope_path};
use crate::contracts::settings::{keys, transitions};
use crate::frontend::settings::tables::{
    SHADER_FAMILIES, SHADERS, family_default, shader_family, shaders_in_family,
};
use crate::frontend::settings::test_source::FakeSettingsSource;

#[test]
fn shaders_match_transition_catalog() {
    use std::collections::HashSet;
    let gui: HashSet<&str> = SHADERS.iter().copied().collect();
    let catalog: HashSet<&str> = transitions::TRANSITIONS.iter().map(|spec| spec.key).collect();
    assert_eq!(gui, catalog);
}

#[test]
fn shader_family_coverage() {
    let keys: Vec<&str> = SHADER_FAMILIES.iter().map(|(key, _, _)| *key).collect();
    for shader in SHADERS {
        let family = shader_family(shader);
        assert!(keys.contains(&family), "{shader} -> {family}");
    }
    let total: usize = keys.iter().map(|key| shaders_in_family(key).len()).sum();
    assert_eq!(total, SHADERS.len());
}

#[test]
fn family_sizes_balanced() {
    for (key, label, _) in SHADER_FAMILIES {
        let count = shaders_in_family(key).len();
        assert!(count > 0, "{label}");
        assert!(count < SHADERS.len() / 2, "{label} holds {count}");
    }
}

#[test]
fn family_default_keeps_fit() {
    assert_eq!(family_default("sand-helix", "sand"), "sand-helix");
    assert_eq!(family_default("random", "random"), "random");
    let moved = family_default("sand-helix", "warp");
    assert_eq!(shader_family(&moved), "warp");
    assert!(SHADERS.contains(&moved.as_str()), "{moved}");
}

#[test]
fn shader_names_read_as_words() {
    assert_eq!(pretty("sand-helix"), "Sand Helix");
    assert_eq!(pretty("polka-dots-curtain"), "Polka Dots Curtain");
    assert_eq!(pretty("iris"), "Iris");
}

#[test]
fn shader_order_pins_random_first() {
    assert_eq!(SHADERS[0], "random");
    assert!(SHADERS.contains(&"sand-tornado"));
}

#[test]
fn shader_scope_paths_distinct() {
    assert_eq!(shader_scope_path("fade"), "transition.shaderScopes.fade");
    assert_ne!(shader_scope_path("fade"), shader_scope_path("glitch"));
}

#[test]
fn shader_scope_migration() {
    let config = FakeSettingsSource::default()
        .with_text(keys::transition::SAND_SCOPE, "primary")
        .with_text(&shader_scope_path("sand-donut"), "all")
        .with_text(&shader_scope_path("fade"), "primary");

    assert_eq!(shader_scope(&config, "sand-helix"), "primary");
    assert_eq!(shader_scope(&config, "sand-donut"), "all");
    assert_eq!(shader_scope(&config, "fade"), "primary");
    assert_eq!(shader_scope(&config, "glitch"), "all");
}
