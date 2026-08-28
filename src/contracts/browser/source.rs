#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    Wallhaven,
    Steam,
    Unsplash,
    Pexels,
    Youtube,
    Bing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyKind {
    Static,
    Video,
    WallpaperEngine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceAvailability {
    pub source: Source,
    pub unavailable: Option<SourceUnavailableReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceUnavailableReason {
    Disabled,
    MissingCredentials,
}

impl Source {
    pub const ALL: [Self; 6] =
        [Self::Wallhaven, Self::Steam, Self::Unsplash, Self::Pexels, Self::Youtube, Self::Bing];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Wallhaven => "Wallhaven",
            Self::Steam => "Steam Workshop",
            Self::Unsplash => "Unsplash",
            Self::Pexels => "Pexels",
            Self::Youtube => "YouTube",
            Self::Bing => "Bing Daily",
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::Wallhaven => "wallhaven",
            Self::Steam => "steam",
            Self::Unsplash => "unsplash",
            Self::Pexels => "pexels",
            Self::Youtube => "youtube",
            Self::Bing => "bing",
        }
    }

    pub const fn apply_kind(self) -> ApplyKind {
        match self {
            Self::Youtube => ApplyKind::Video,
            Self::Steam => ApplyKind::WallpaperEngine,
            _ => ApplyKind::Static,
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|source| source.key() == key)
    }

    pub const fn searchable(self) -> bool {
        !matches!(self, Self::Bing)
    }
}
