use serde_json::Value;
use wall_proto::{WeProperty, we_property_kind};

use crate::domain::scene_properties::{
    SceneChoice, SceneProperty, ScenePropertyKind, ScenePropertyValue, parse_vector,
};

pub fn decode_rows(rows: Vec<WeProperty>) -> Vec<SceneProperty> {
    rows.into_iter().map(row).collect()
}

fn row(property: WeProperty) -> SceneProperty {
    let kind = kind_of(&property.kind);
    SceneProperty {
        value: value_of(&property.value, kind),
        default: value_of(&property.default, kind),
        name: property.name,
        label: property.label,
        kind,
        overridden: property.overridden,
        min: property.min,
        max: property.max,
        step: property.step,
        choices: property
            .options
            .into_iter()
            .map(|option| SceneChoice { label: option.label, value: option.value })
            .collect(),
        order: property.order,
    }
}

fn kind_of(kind: &str) -> ScenePropertyKind {
    match kind {
        we_property_kind::BOOL => ScenePropertyKind::Flag,
        we_property_kind::COLOR => ScenePropertyKind::Colour,
        we_property_kind::SLIDER => ScenePropertyKind::Range,
        we_property_kind::COMBO => ScenePropertyKind::Choice,
        we_property_kind::GROUP => ScenePropertyKind::Group,
        _ => ScenePropertyKind::Unsupported,
    }
}

fn value_of(value: &Value, kind: ScenePropertyKind) -> ScenePropertyValue {
    match value {
        Value::Null | Value::Object(_) => ScenePropertyValue::Absent,
        Value::Bool(flag) => ScenePropertyValue::Flag(*flag),
        Value::Number(number) => ScenePropertyValue::Number(number.as_f64().unwrap_or_default()),
        Value::Array(parts) => ScenePropertyValue::Vector(
            parts.iter().filter_map(Value::as_f64).map(|part| part as f32).collect(),
        ),
        Value::String(text) => match kind {
            ScenePropertyKind::Colour => parse_vector(text)
                .map_or_else(|| ScenePropertyValue::Text(text.clone()), ScenePropertyValue::Vector),
            _ => ScenePropertyValue::Text(text.trim().to_string()),
        },
    }
}

#[must_use]
pub fn encode(value: &ScenePropertyValue) -> Value {
    match value {
        ScenePropertyValue::Absent => Value::Null,
        ScenePropertyValue::Flag(flag) => Value::Bool(*flag),
        ScenePropertyValue::Number(number) => {
            serde_json::Number::from_f64(*number).map_or(Value::Null, Value::Number)
        }
        ScenePropertyValue::Vector(parts) => Value::String(
            parts.iter().map(|part| format!("{part:.3}")).collect::<Vec<_>>().join(" "),
        ),
        ScenePropertyValue::Text(text) => Value::String(text.clone()),
    }
}

#[cfg(test)]
#[path = "scene_properties_tests.rs"]
mod tests;
