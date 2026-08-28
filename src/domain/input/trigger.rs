#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Mods {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl Mods {
    pub const NONE: Self = Self { ctrl: false, alt: false, shift: false };

    pub const fn new(ctrl: bool, alt: bool, shift: bool) -> Self {
        Self { ctrl, alt, shift }
    }

    fn label(self, output: &mut String) {
        if self.ctrl {
            output.push_str("Ctrl+");
        }
        if self.alt {
            output.push_str("Alt+");
        }
        if self.shift {
            output.push_str("Shift+");
        }
    }

    fn config(self, output: &mut String) {
        if self.ctrl {
            output.push_str("ctrl+");
        }
        if self.alt {
            output.push_str("alt+");
        }
        if self.shift {
            output.push_str("shift+");
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyId {
    Char(String),
    Left,
    Right,
    Up,
    Down,
    Enter,
    Tab,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeySpec {
    pub mods: Mods,
    pub id: KeyId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseSpec {
    pub mods: Mods,
    pub button: MouseButton,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trigger {
    Key(KeySpec),
    Mouse(MouseSpec),
}

pub const UNBOUND: &str = "none";

impl Trigger {
    pub fn parse(text: &str) -> Option<Self> {
        let normalized = text.trim().to_lowercase();
        if normalized.is_empty() {
            return None;
        }
        let tokens: Vec<&str> = if normalized == "+" {
            vec!["+"]
        } else if let Some(head) = normalized.strip_suffix("++") {
            let mut parts: Vec<&str> = head.split('+').map(str::trim).collect();
            parts.push("+");
            parts
        } else {
            normalized.split('+').map(str::trim).collect()
        };
        let (last, modifiers) = tokens.split_last()?;
        let mut mods = Mods::NONE;
        for token in modifiers {
            match *token {
                "shift" => mods.shift = true,
                "ctrl" | "control" => mods.ctrl = true,
                "alt" => mods.alt = true,
                _ => return None,
            }
        }
        let button = match *last {
            "click" | "left-click" => Some(MouseButton::Left),
            "right-click" => Some(MouseButton::Right),
            "middle-click" => Some(MouseButton::Middle),
            _ => None,
        };
        if let Some(button) = button {
            return Some(Self::Mouse(MouseSpec { mods, button }));
        }
        let id = match *last {
            "left" => KeyId::Left,
            "right" => KeyId::Right,
            "up" => KeyId::Up,
            "down" => KeyId::Down,
            "enter" | "return" => KeyId::Enter,
            "tab" => KeyId::Tab,
            "space" => KeyId::Char(" ".to_string()),
            key if key.chars().count() == 1 => KeyId::Char(key.to_string()),
            _ => return None,
        };
        Some(Self::Key(KeySpec { mods, id }))
    }

    pub fn label(&self) -> String {
        let mut output = String::new();
        match self {
            Self::Key(spec) => {
                spec.mods.label(&mut output);
                match &spec.id {
                    KeyId::Left => output.push_str("Left"),
                    KeyId::Right => output.push_str("Right"),
                    KeyId::Up => output.push_str("Up"),
                    KeyId::Down => output.push_str("Down"),
                    KeyId::Enter => output.push_str("Enter"),
                    KeyId::Tab => output.push_str("Tab"),
                    KeyId::Char(character) if character == " " => output.push_str("Space"),
                    KeyId::Char(character) => output.push_str(&character.to_uppercase()),
                }
            }
            Self::Mouse(spec) => {
                spec.mods.label(&mut output);
                output.push_str(match spec.button {
                    MouseButton::Left => "Click",
                    MouseButton::Right => "Right-click",
                    MouseButton::Middle => "Middle-click",
                });
            }
        }
        output
    }

    pub fn config(&self) -> String {
        let mut output = String::new();
        match self {
            Self::Key(spec) => {
                spec.mods.config(&mut output);
                match &spec.id {
                    KeyId::Left => output.push_str("left"),
                    KeyId::Right => output.push_str("right"),
                    KeyId::Up => output.push_str("up"),
                    KeyId::Down => output.push_str("down"),
                    KeyId::Enter => output.push_str("enter"),
                    KeyId::Tab => output.push_str("tab"),
                    KeyId::Char(character) if character == " " => output.push_str("space"),
                    KeyId::Char(character) => output.push_str(character),
                }
            }
            Self::Mouse(spec) => {
                spec.mods.config(&mut output);
                output.push_str(match spec.button {
                    MouseButton::Left => "click",
                    MouseButton::Right => "right-click",
                    MouseButton::Middle => "middle-click",
                });
            }
        }
        output
    }

    pub const fn is_mouse(&self) -> bool {
        matches!(self, Self::Mouse(_))
    }
}

pub fn parse_binding(text: &str) -> Option<Vec<Trigger>> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.eq_ignore_ascii_case(UNBOUND) {
        return Some(Vec::new());
    }
    trimmed.split(',').map(Trigger::parse).collect()
}

pub fn binding_config(triggers: &[Trigger]) -> String {
    if triggers.is_empty() {
        return String::from(UNBOUND);
    }
    triggers.iter().map(Trigger::config).collect::<Vec<_>>().join(", ")
}

pub fn binding_label(triggers: &[Trigger]) -> String {
    triggers.iter().map(Trigger::label).collect::<Vec<_>>().join(" \u{b7} ")
}
