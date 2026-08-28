use serde_json::{Value, json};

use crate::domain::theme::{Candidate, ThemeRole};

const ROLE_KEYS: [(&str, ThemeRole); 9] = [
    ("primary", ThemeRole::Primary),
    ("primaryText", ThemeRole::PrimaryText),
    ("tertiary", ThemeRole::Tertiary),
    ("surface", ThemeRole::Surface),
    ("surfaceText", ThemeRole::SurfaceText),
    ("surfaceVariant", ThemeRole::SurfaceVariant),
    ("surfaceContainer", ThemeRole::SurfaceContainer),
    ("background", ThemeRole::Background),
    ("outline", ThemeRole::Outline),
];

pub fn decode_candidate(value: &Value) -> Candidate {
    let mut candidate = Candidate::default();
    for (key, role) in ROLE_KEYS {
        if let Some(color) = value.get(key).and_then(Value::as_str) {
            candidate.colors[role.index()] = color.to_string();
        }
    }
    candidate
}

pub fn encode_candidate(candidate: &Candidate, include_aliases: bool) -> Value {
    let mut map = serde_json::Map::new();
    for (key, role) in ROLE_KEYS {
        map.insert(key.to_string(), json!(candidate.colors[role.index()]));
    }
    if include_aliases {
        map.insert(
            "on_primary".to_string(),
            json!(candidate.colors[ThemeRole::PrimaryText.index()]),
        );
        map.insert(
            "on_surface".to_string(),
            json!(candidate.colors[ThemeRole::SurfaceText.index()]),
        );
    }
    Value::Object(map)
}

pub fn upsert_saved(themes: &[Value], name: &str, candidate: &Candidate) -> Vec<Value> {
    let mut output = themes.to_vec();
    let mut entry = encode_candidate(candidate, false);
    if let Some(map) = entry.as_object_mut() {
        map.insert("name".to_string(), json!(name));
    }
    match output.iter().position(|theme| saved_name(theme) == Some(name)) {
        Some(position) => output[position] = entry,
        None => output.push(entry),
    }
    output
}

pub fn remove_saved(themes: &[Value], name: &str) -> Vec<Value> {
    themes.iter().filter(|theme| saved_name(theme) != Some(name)).cloned().collect()
}

pub fn saved_names(themes: &[Value]) -> Vec<String> {
    themes
        .iter()
        .filter_map(saved_name)
        .filter(|name| !name.is_empty())
        .map(str::to_string)
        .collect()
}

pub fn saved_palettes(themes: &[Value]) -> Vec<(String, Candidate)> {
    themes
        .iter()
        .filter_map(|theme| {
            let name = saved_name(theme)?.trim();
            (!name.is_empty()).then(|| (name.to_string(), decode_candidate(theme)))
        })
        .collect()
}

pub fn find_saved(themes: &[Value], name: &str) -> Option<Candidate> {
    themes.iter().find(|theme| saved_name(theme) == Some(name)).map(decode_candidate)
}

fn saved_name(theme: &Value) -> Option<&str> {
    theme.get("name").and_then(Value::as_str)
}
