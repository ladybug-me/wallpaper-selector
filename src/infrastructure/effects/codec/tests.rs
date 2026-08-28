#![cfg(test)]

use super::*;
use serde_json::json;

#[test]
fn definitions_decode() {
    let definitions = decode_definitions(&[
        json!({
            "id": "theme",
            "category": "Colour",
            "params": [{
                "id": "theme",
                "type": "dropdown",
                "default": "Nord",
                "options": [
                    "Nord",
                    {"mode": "Catppuccin", "label": "Catppuccin", "swatch": ["#89b4fa"]}
                ]
            }]
        }),
        json!({"label": "missing id"}),
        json!({"id": "bare", "params": "invalid"}),
    ]);

    assert_eq!(definitions.len(), 2);
    assert_eq!(definitions[0].label, "theme");
    assert_eq!(definitions[0].params[0].step, 0.01);
    assert_eq!(definitions[0].params[0].options[0].mode, "Nord");
    assert_eq!(definitions[0].params[0].options[1].swatch, ["#89b4fa"]);
    assert!(definitions[1].params.is_empty());
}

#[test]
fn values_round_trip() {
    let mut values = EffectValues::new();
    values.insert("factor".into(), EffectValue::Number(1.5));
    values.insert("radius".into(), EffectValue::Number(4.0));
    values.insert("theme".into(), EffectValue::Text("Nord".into()));
    values.insert("enabled".into(), EffectValue::Bool(true));
    let encoded = encode_values(&values);
    assert_eq!(
        encoded,
        json!({
            "factor": 1.5,
            "radius": 4,
            "theme": "Nord",
            "enabled": true,
        })
    );
    assert!(encoded["radius"].as_i64().is_some());
}

#[test]
fn step_stack_encoding() {
    let steps = vec![
        EffectStep { effect: "invert".into(), params: EffectValues::new() },
        EffectStep {
            effect: "kuwahara".into(),
            params: EffectValues::from([("radius".into(), EffectValue::Number(7.0))]),
        },
    ];
    assert_eq!(
        encode_steps(&steps),
        json!([
            {"effect": "invert", "params": {}},
            {"effect": "kuwahara", "params": {"radius": 7}},
        ])
    );
}
