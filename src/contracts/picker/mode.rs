#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Slices,
    Grid,
    Hex,
    Sandy,
}

impl Mode {
    pub fn try_from_key(value: &str) -> Option<Self> {
        match value {
            "slices" => Some(Self::Slices),
            "wall" | "grid" => Some(Self::Grid),
            "hex" => Some(Self::Hex),
            "sandy" | "nova" => Some(Self::Sandy),
            _ => None,
        }
    }

    pub fn from_key(value: &str) -> Self {
        Self::try_from_key(value).unwrap_or(Self::Slices)
    }

    pub const fn as_key(self) -> &'static str {
        match self {
            Self::Slices => "slices",
            Self::Grid => "wall",
            Self::Hex => "hex",
            Self::Sandy => "sandy",
        }
    }
}

mod tests;
