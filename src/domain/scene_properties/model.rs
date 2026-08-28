#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScenePropertyKind {
    Flag,
    Colour,
    Range,
    Choice,
    Group,
    #[default]
    Unsupported,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ScenePropertyValue {
    #[default]
    Absent,
    Flag(bool),
    Number(f64),
    Vector(Vec<f32>),
    Text(String),
}

impl ScenePropertyValue {
    #[must_use]
    pub fn flag(&self) -> bool {
        match self {
            Self::Flag(flag) => *flag,
            Self::Number(number) => *number != 0.0,
            Self::Vector(parts) => parts.first().is_some_and(|part| *part != 0.0),
            Self::Text(text) => {
                let trimmed = text.trim();
                !(trimmed.is_empty() || trimmed == "0" || trimmed.eq_ignore_ascii_case("false"))
            }
            Self::Absent => false,
        }
    }

    #[must_use]
    pub fn number(&self) -> f64 {
        match self {
            Self::Number(number) => *number,
            Self::Flag(flag) => f64::from(u8::from(*flag)),
            Self::Vector(parts) => parts.first().map_or(0.0, |part| f64::from(*part)),
            Self::Text(text) => text.trim().parse().unwrap_or_default(),
            Self::Absent => 0.0,
        }
    }

    #[must_use]
    pub fn colour(&self) -> Option<[f32; 3]> {
        match self {
            Self::Vector(parts) => match parts[..] {
                [red, green, blue] => Some([red, green, blue]),
                _ => None,
            },
            Self::Text(text) => parse_vector(text).and_then(|parts| match parts[..] {
                [red, green, blue] => Some([red, green, blue]),
                _ => None,
            }),
            _ => None,
        }
    }

    #[must_use]
    pub fn display(&self) -> String {
        match self {
            Self::Absent => String::new(),
            Self::Flag(flag) => flag.to_string(),
            Self::Number(number) => format_number(*number),
            Self::Vector(parts) => {
                parts.iter().map(|part| format!("{part:.3}")).collect::<Vec<_>>().join(" ")
            }
            Self::Text(text) => text.trim().to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SceneChoice {
    pub label: String,
    pub value: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SceneProperty {
    pub name: String,
    pub label: String,
    pub kind: ScenePropertyKind,
    pub value: ScenePropertyValue,
    pub default: ScenePropertyValue,
    pub overridden: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub choices: Vec<SceneChoice>,
    pub order: i64,
}

impl SceneProperty {
    #[must_use]
    pub fn editable(&self) -> bool {
        matches!(
            self.kind,
            ScenePropertyKind::Flag
                | ScenePropertyKind::Colour
                | ScenePropertyKind::Range
                | ScenePropertyKind::Choice
        )
    }

    #[must_use]
    pub fn is_group(&self) -> bool {
        self.kind == ScenePropertyKind::Group
    }

    #[must_use]
    pub fn range(&self) -> (f64, f64, f64) {
        let min = self.min.unwrap_or(0.0);
        let max = self.max.unwrap_or(if min < 1.0 { 1.0 } else { min * 2.0 });
        let max = if max > min { max } else { min + 1.0 };
        let step = self.step.filter(|step| *step > 0.0).unwrap_or((max - min) / 100.0);
        (min, max, step.max(f64::EPSILON))
    }
}

#[must_use]
pub fn parse_vector(text: &str) -> Option<Vec<f32>> {
    let parts: Vec<f32> = text.split_whitespace().filter_map(|part| part.parse().ok()).collect();
    (!parts.is_empty()).then_some(parts)
}

#[must_use]
pub fn format_number(value: f64) -> String {
    if (value - value.round()).abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.3}")
    }
}
