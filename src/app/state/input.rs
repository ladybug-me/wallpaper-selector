use std::time::Instant;

use crate::domain::input::{InputMap, Mods};

pub(crate) struct InputState {
    pub(crate) mods: Mods,
    pub(crate) last_activity: Instant,
    pub(crate) help_open: bool,
    pub(crate) bindings: InputMap,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            mods: Mods::NONE,
            last_activity: Instant::now(),
            help_open: false,
            bindings: InputMap::default(),
        }
    }
}
