#![cfg(test)]

use super::*;
use crate::domain::effects::EffectValue;

#[test]
fn maps_supported_effects() {
    for (id, code) in [
        ("brightness", 1),
        ("contrast", 2),
        ("saturation", 3),
        ("gamma", 4),
        ("invert", 5),
        ("grayscale", 6),
        ("sepia", 7),
        ("hue", 8),
        ("temperature", 9),
        ("tint", 10),
        ("posterize", 11),
        ("solarize", 12),
        ("threshold", 13),
        ("duotone", 14),
        ("vignette", 15),
        ("dim", 16),
        ("flip", 17),
        ("mirror", 18),
    ] {
        assert_eq!(shader_spec(id, &EffectValues::new()).map(|spec| spec.effect), Some(code));
    }
    assert_eq!(shader_spec("recolor", &EffectValues::new()), None);
}

#[test]
fn value_application() {
    let mut values = EffectValues::new();
    values.insert("factor".into(), EffectValue::Number(1.5));
    assert_eq!(shader_spec("brightness", &values).unwrap().params[0], 1.5);
    values.insert("factor".into(), EffectValue::Number(50.0));
    assert_eq!(shader_spec("contrast", &values).unwrap().params[0], 1.5);

    values.insert("mode".into(), EffectValue::Text("sigmoid".into()));
    assert_eq!(shader_spec("contrast", &values), None);

    values.clear();
    values.insert("shadow".into(), EffectValue::Text("#000000".into()));
    values.insert("highlight".into(), EffectValue::Text("#ffffff".into()));
    let spec = shader_spec("duotone", &values).unwrap();
    assert_eq!(spec.color_a, [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(spec.color_b, [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn multibyte_colour_safe() {
    for text in ["#ffcc00", "ffcc00", "80ffcc00", "héllo!", "#ÅÄÖabc", "ÅÄÖ", "", "#", "zzzzzz"]
    {
        let _ = rgba(text);
    }
    assert_eq!(rgba("#ff8000"), [1.0, 128.0 / 255.0, 0.0, 1.0]);
    assert_eq!(rgba("héllo!"), [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(rgba("ccff8000"), [1.0, 128.0 / 255.0, 0.0, 1.0]);
}
