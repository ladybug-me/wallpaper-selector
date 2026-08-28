#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MediaKind {
    Static,
    Video,
    WallpaperEngine,
    Other(String),
}

impl Default for MediaKind {
    fn default() -> Self {
        Self::Other(String::new())
    }
}

impl MediaKind {
    pub fn from_key(value: &str) -> Self {
        match value {
            "static" => Self::Static,
            "video" => Self::Video,
            "we" => Self::WallpaperEngine,
            other => Self::Other(other.to_string()),
        }
    }

    pub fn as_key(&self) -> &str {
        match self {
            Self::Static => "static",
            Self::Video => "video",
            Self::WallpaperEngine => "we",
            Self::Other(value) => value,
        }
    }

    pub const fn has_audio_controls(&self) -> bool {
        matches!(self, Self::Video | Self::WallpaperEngine)
    }
}

#[cfg(test)]
mod tests;
