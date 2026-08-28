#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BrowserGrid {
    pub cols: usize,
    pub rows: usize,
    pub thumb_w: f32,
    pub thumb_h: f32,
    pub gap_x: f32,
    pub gap_y: f32,
    pub corner_radius: f32,
    pub border_width: f32,
}

impl BrowserGrid {
    #[allow(dead_code)]
    pub fn cell_h(&self) -> f32 {
        self.thumb_h + self.gap_y
    }
}

pub const SANDY_SWAP_STYLES: [(&str, &str); 11] = [
    ("vortex", "Vortex"),
    ("hourglass", "Hourglass"),
    ("castle", "Sandcastle"),
    ("saltation", "Saltation"),
    ("pour", "Glass pour"),
    ("orbit", "Maelstrom"),
    ("burst", "Burst"),
    ("weave", "Weave"),
    ("bloom", "Bloom"),
    ("flock", "Murmuration"),
    ("ring", "Ring"),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SandyStyle {
    Vortex,
    Hourglass,
    Castle,
    Saltation,
    Pour,
    Orbit,
    Burst,
    Weave,
    Bloom,
    Flock,
    Ring,
}

impl SandyStyle {
    pub fn from_key(key: &str) -> Self {
        match key {
            "hourglass" => Self::Hourglass,
            "castle" => Self::Castle,
            "saltation" => Self::Saltation,
            "pour" => Self::Pour,
            "orbit" => Self::Orbit,
            "burst" => Self::Burst,
            "weave" => Self::Weave,
            "bloom" => Self::Bloom,
            "flock" => Self::Flock,
            "ring" => Self::Ring,
            _ => Self::Vortex,
        }
    }

    pub fn shader_index(self) -> f32 {
        match self {
            Self::Vortex => 1.0,
            Self::Hourglass => 2.0,
            Self::Castle => 3.0,
            Self::Saltation => 6.0,
            Self::Pour => 7.0,
            Self::Orbit => 8.0,
            Self::Burst => 10.0,
            Self::Weave => 11.0,
            Self::Bloom => 13.0,
            Self::Flock => 16.0,
            Self::Ring => 17.0,
        }
    }
}

pub fn sandy_style_index(style: &str) -> f32 {
    SandyStyle::from_key(style).shader_index()
}

pub fn format_config_number(value: f64) -> String {
    if value.fract() == 0.0 { format!("{}", value as i64) } else { format!("{value}") }
}

mod tests;
