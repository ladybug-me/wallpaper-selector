use serde_json::{Map, Number, Value};

use crate::domain::effects::{
    EffectDefinition, EffectOption, EffectParam, EffectParamKind, EffectStep, EffectValue,
    EffectValues,
};

#[must_use]
pub fn decode_definitions(values: &[Value]) -> Vec<EffectDefinition> {
    values.iter().filter_map(decode_definition).collect()
}

#[must_use]
pub fn encode_values(values: &EffectValues) -> Value {
    Value::Object(
        values.iter().map(|(key, value)| (key.clone(), encode_value(value))).collect::<Map<_, _>>(),
    )
}

#[must_use]
pub fn encode_steps(steps: &[EffectStep]) -> Value {
    Value::Array(
        steps
            .iter()
            .map(|step| {
                Value::Object(Map::from_iter([
                    ("effect".to_string(), Value::String(step.effect.clone())),
                    ("params".to_string(), encode_values(&step.params)),
                ]))
            })
            .collect(),
    )
}

fn decode_definition(value: &Value) -> Option<EffectDefinition> {
    let id = string(value, "id")?.to_string();
    let label = string(value, "label").unwrap_or(&id).to_string();
    let description = string(value, "description").unwrap_or_default().to_string();
    let category = string(value, "category").unwrap_or_default().to_string();
    let params = value
        .get("params")
        .and_then(Value::as_array)
        .map(|params| params.iter().filter_map(decode_param).collect())
        .unwrap_or_default();
    Some(EffectDefinition { id, label, description, category, params })
}

fn decode_param(value: &Value) -> Option<EffectParam> {
    let id = string(value, "id")?.to_string();
    let label = string(value, "label").unwrap_or(&id).to_string();
    let kind = EffectParamKind::from(string(value, "type").unwrap_or_default());
    let min = number(value, "min").unwrap_or(0.0);
    let max = number(value, "max").unwrap_or(100.0);
    let default_step = if matches!(&kind, EffectParamKind::Integer) { 1.0 } else { 0.01 };
    let step = number(value, "step").unwrap_or(default_step);
    let default = value.get("default").and_then(decode_value);
    let options = value
        .get("options")
        .and_then(Value::as_array)
        .map(|options| options.iter().filter_map(decode_option).collect())
        .unwrap_or_default();
    Some(EffectParam { id, label, kind, min, max, step, default, options })
}

fn decode_option(value: &Value) -> Option<EffectOption> {
    if let Some(name) = value.as_str() {
        return Some(EffectOption {
            mode: name.to_string(),
            label: name.to_string(),
            swatch: Vec::new(),
        });
    }
    let mode = string(value, "mode")?.to_string();
    let label = string(value, "label").unwrap_or(&mode).to_string();
    let swatch = value
        .get("swatch")
        .and_then(Value::as_array)
        .map(|colors| colors.iter().filter_map(Value::as_str).map(String::from).collect())
        .unwrap_or_default();
    Some(EffectOption { mode, label, swatch })
}

fn decode_value(value: &Value) -> Option<EffectValue> {
    match value {
        Value::Number(value) => value.as_f64().map(EffectValue::Number),
        Value::String(value) => Some(EffectValue::Text(value.clone())),
        Value::Bool(value) => Some(EffectValue::Bool(*value)),
        Value::Null | Value::Array(_) | Value::Object(_) => None,
    }
}

fn encode_value(value: &EffectValue) -> Value {
    match value {
        EffectValue::Number(value) if value.fract() == 0.0 => {
            format!("{value:.0}").parse::<i64>().map(Number::from).map_or_else(
                |_| Number::from_f64(*value).map_or(Value::Null, Value::Number),
                Value::Number,
            )
        }
        EffectValue::Number(value) => Number::from_f64(*value).map_or(Value::Null, Value::Number),
        EffectValue::Text(value) => Value::String(value.clone()),
        EffectValue::Bool(value) => Value::Bool(*value),
    }
}

fn string<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

fn number(value: &Value, key: &str) -> Option<f64> {
    value.get(key).and_then(Value::as_f64)
}

mod tests;
