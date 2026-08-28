use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum EffectValue {
    Number(f64),
    Text(String),
    Bool(bool),
}

impl EffectValue {
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(value) => Some(*value),
            Self::Text(_) | Self::Bool(_) => None,
        }
    }

    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value),
            Self::Number(_) | Self::Bool(_) => None,
        }
    }
}

impl From<f64> for EffectValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for EffectValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for EffectValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<bool> for EffectValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

pub type EffectValues = BTreeMap<String, EffectValue>;

#[derive(Debug, Clone, PartialEq)]
pub struct EffectStep {
    pub effect: String,
    pub params: EffectValues,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectParamKind {
    Integer,
    Number,
    Dropdown,
    Color,
    Other(String),
}

impl EffectParamKind {
    #[must_use]
    pub fn is_numeric(&self) -> bool {
        matches!(self, Self::Integer | Self::Number)
    }
}

impl From<&str> for EffectParamKind {
    fn from(kind: &str) -> Self {
        match kind {
            "integer" => Self::Integer,
            "number" => Self::Number,
            "dropdown" => Self::Dropdown,
            "color" => Self::Color,
            other => Self::Other(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectOption {
    pub mode: String,
    pub label: String,
    pub swatch: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EffectParam {
    pub id: String,
    pub label: String,
    pub kind: EffectParamKind,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub default: Option<EffectValue>,
    pub options: Vec<EffectOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EffectDefinition {
    pub id: String,
    pub label: String,
    pub description: String,
    pub category: String,
    pub params: Vec<EffectParam>,
}
