#![cfg(test)]

use super::*;
use crate::domain::scene_properties::ScenePropertyKind;

fn decode(value: &Value) -> Vec<SceneProperty> {
    value
        .get("properties")
        .cloned()
        .and_then(|rows| serde_json::from_value(rows).ok())
        .map_or_else(Vec::new, decode_rows)
}

fn payload() -> Value {
    serde_json::json!({
        "we_id": "123",
        "properties": [
            {"name": "header", "label": "Look", "kind": "group", "order": 0},
            {"name": "tint", "label": "Tint", "kind": "color", "value": "1 0 0",
             "default": "1 1 1", "overridden": true, "order": 1},
            {"name": "glow", "label": "Glow", "kind": "bool", "value": true,
             "default": true, "order": 2},
            {"name": "zoom", "label": "Zoom", "kind": "slider", "value": 2.5,
             "default": 1.0, "overridden": true, "min": 0.5, "max": 3.0, "step": 0.01,
             "order": 3},
            {"name": "mode", "label": "Mode", "kind": "combo", "value": 1, "default": 0,
             "options": [{"label": "Day", "value": 0.0}, {"label": "Night", "value": 1.0}],
             "order": 4},
            {"name": "shortcut", "label": "Shortcut", "kind": "unsupported",
             "value": "ctrl+g", "order": 5}
        ]
    })
}

#[test]
fn wire_rows_typed() {
    let rows = decode(&payload());
    assert_eq!(rows.len(), 6);
    assert_eq!(
        rows.iter().map(|row| row.kind).collect::<Vec<_>>(),
        [
            ScenePropertyKind::Group,
            ScenePropertyKind::Colour,
            ScenePropertyKind::Flag,
            ScenePropertyKind::Range,
            ScenePropertyKind::Choice,
            ScenePropertyKind::Unsupported,
        ]
    );

    let tint = &rows[1];
    assert_eq!(tint.value, ScenePropertyValue::Vector(vec![1.0, 0.0, 0.0]));
    assert_eq!(tint.default, ScenePropertyValue::Vector(vec![1.0, 1.0, 1.0]));
    assert!(tint.overridden);

    let zoom = &rows[3];
    assert_eq!(zoom.value, ScenePropertyValue::Number(2.5));
    assert_eq!(zoom.range(), (0.5, 3.0, 0.01));

    let mode = &rows[4];
    assert_eq!(mode.choices.len(), 2);
    assert_eq!(mode.choices[1].label, "Night");

    assert_eq!(rows[5].value, ScenePropertyValue::Text("ctrl+g".into()));
}

#[test]
fn malformed_payload_empty() {
    assert!(decode(&serde_json::json!({})).is_empty());
    assert!(decode(&serde_json::json!({"properties": "nope"})).is_empty());
}

#[test]
fn typed_values_encode() {
    assert_eq!(encode(&ScenePropertyValue::Flag(true)), Value::Bool(true));
    assert_eq!(encode(&ScenePropertyValue::Number(2.5)), serde_json::json!(2.5));
    assert_eq!(
        encode(&ScenePropertyValue::Vector(vec![1.0, 0.0, 0.5])),
        Value::String("1.000 0.000 0.500".into())
    );
    assert_eq!(encode(&ScenePropertyValue::Text("hi".into())), Value::String("hi".into()));
    assert_eq!(encode(&ScenePropertyValue::Absent), Value::Null);
}

#[test]
fn bare_colour_string() {
    let rows = decode(&serde_json::json!({
        "we_id": "9",
        "properties": [{"name": "c", "kind": "color", "value": "0.25 0.5 0.75"}]
    }));
    assert_eq!(rows[0].value.colour(), Some([0.25, 0.5, 0.75]));

    let unparsable = decode(&serde_json::json!({
        "we_id": "9",
        "properties": [{"name": "c", "kind": "color", "value": "not a colour"}]
    }));
    assert_eq!(unparsable[0].value, ScenePropertyValue::Text("not a colour".into()));
}
