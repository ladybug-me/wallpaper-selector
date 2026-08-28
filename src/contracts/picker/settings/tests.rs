#![cfg(test)]

use super::*;

#[test]
fn whole_numbers_no_decimal() {
    assert_eq!(format_config_number(30.0), "30");
    assert_eq!(format_config_number(2.5), "2.5");
}

#[test]
fn sandy_styles_mapped() {
    for (key, _) in SANDY_SWAP_STYLES {
        assert!(sandy_style_index(key) > 0.0, "{key} unmapped");
    }
}

#[test]
fn sandy_styles_unique() {
    let mut variants = Vec::new();
    let mut indices = Vec::new();
    for (key, _) in SANDY_SWAP_STYLES {
        let style = SandyStyle::from_key(key);
        assert!(!variants.contains(&style), "{key} duplicate variant");
        let index = style.shader_index();
        assert!(
            !indices.iter().any(|taken: &f32| (taken - index).abs() < f32::EPSILON),
            "{key} reuses shader index {index}"
        );
        assert_eq!(sandy_style_index(key), index, "{key} shim");
        variants.push(style);
        indices.push(index);
    }
    assert_eq!(variants.len(), SANDY_SWAP_STYLES.len());
}

#[test]
fn sandy_shader_indices_pinned() {
    assert_eq!(SandyStyle::Vortex.shader_index(), 1.0);
    assert_eq!(SandyStyle::Hourglass.shader_index(), 2.0);
    assert_eq!(SandyStyle::Castle.shader_index(), 3.0);
    assert_eq!(SandyStyle::Saltation.shader_index(), 6.0);
    assert_eq!(SandyStyle::Pour.shader_index(), 7.0);
    assert_eq!(SandyStyle::Orbit.shader_index(), 8.0);
    assert_eq!(SandyStyle::Burst.shader_index(), 10.0);
    assert_eq!(SandyStyle::Weave.shader_index(), 11.0);
    assert_eq!(SandyStyle::Bloom.shader_index(), 13.0);
    assert_eq!(SandyStyle::Flock.shader_index(), 16.0);
    assert_eq!(SandyStyle::Ring.shader_index(), 17.0);
}

#[test]
fn unknown_sandy_key_vortex() {
    for key in ["", "serpent", "geyser", "twister", "Ring", "vortex"] {
        assert_eq!(
            SandyStyle::from_key(key).shader_index(),
            SandyStyle::Vortex.shader_index(),
            "{key}"
        );
    }
}
