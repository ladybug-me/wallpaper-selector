use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

pub type DecodeResult<T> = Result<T, DecodeError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    ExpectedObject { family: &'static str },
    InvalidField { family: &'static str, field: &'static str, expected: &'static str },
    UnknownVersion { family: &'static str, version: u64 },
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedObject { family } => write!(formatter, "{family}: expected object"),
            Self::InvalidField { family, field, expected } => {
                write!(formatter, "{family}.{field}: expected {expected}")
            }
            Self::UnknownVersion { family, version } => {
                write!(formatter, "{family}: unsupported schema version {version}")
            }
        }
    }
}

pub(super) fn envelope<'a>(
    family: &'static str,
    value: &'a Value,
) -> DecodeResult<&'a Map<String, Value>> {
    let object = value.as_object().ok_or(DecodeError::ExpectedObject { family })?;
    match object.get("schema_version") {
        None => Ok(object),
        Some(Value::Number(number)) => match number.as_u64() {
            Some(1) => Ok(object),
            Some(version) => Err(DecodeError::UnknownVersion { family, version }),
            None => Err(invalid(family, "schema_version", "positive integer")),
        },
        Some(_) => Err(invalid(family, "schema_version", "positive integer")),
    }
}

pub(super) fn array<'a>(
    family: &'static str,
    object: &'a Map<String, Value>,
    field: &'static str,
) -> DecodeResult<Option<&'a [Value]>> {
    object
        .get(field)
        .map(|value| value.as_array().map(Vec::as_slice).ok_or(invalid(family, field, "array")))
        .transpose()
}

pub(super) fn required_array<'a>(
    family: &'static str,
    object: &'a Map<String, Value>,
    field: &'static str,
) -> DecodeResult<&'a [Value]> {
    array(family, object, field)?.ok_or_else(|| invalid(family, field, "array"))
}

pub(super) fn string(
    family: &'static str,
    object: &Map<String, Value>,
    field: &'static str,
) -> DecodeResult<Option<String>> {
    object
        .get(field)
        .map(|value| value.as_str().map(str::to_string).ok_or(invalid(family, field, "string")))
        .transpose()
}

pub(super) fn required_string(
    family: &'static str,
    object: &Map<String, Value>,
    field: &'static str,
) -> DecodeResult<String> {
    string(family, object, field)?.ok_or_else(|| invalid(family, field, "string"))
}

pub(super) fn integer(
    family: &'static str,
    object: &Map<String, Value>,
    field: &'static str,
) -> DecodeResult<Option<i64>> {
    object
        .get(field)
        .map(|value| value.as_i64().ok_or(invalid(family, field, "integer")))
        .transpose()
}

pub(super) fn typed<T: DeserializeOwned>(
    family: &'static str,
    object: &Map<String, Value>,
    field: &'static str,
    expected: &'static str,
) -> DecodeResult<Option<T>> {
    object
        .get(field)
        .map(|value| {
            serde_json::from_value(value.clone()).map_err(|_| invalid(family, field, expected))
        })
        .transpose()
}

pub(super) fn required_typed<T: DeserializeOwned>(
    family: &'static str,
    object: &Map<String, Value>,
    field: &'static str,
    expected: &'static str,
) -> DecodeResult<T> {
    typed(family, object, field, expected)?.ok_or_else(|| invalid(family, field, expected))
}

pub(super) fn root<T: DeserializeOwned>(
    family: &'static str,
    value: &Value,
    expected: &'static str,
) -> DecodeResult<T> {
    serde_json::from_value(value.clone()).map_err(|_| invalid(family, "result", expected))
}

pub(super) fn strings(values: &[Value]) -> Vec<String> {
    values.iter().filter_map(Value::as_str).map(str::to_string).collect()
}

pub(super) fn invalid(
    family: &'static str,
    field: &'static str,
    expected: &'static str,
) -> DecodeError {
    DecodeError::InvalidField { family, field, expected }
}
