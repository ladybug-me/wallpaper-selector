use crate::contracts::settings::keys;
use skwd_config::schema::setting;

use super::super::Config;

impl Config {
    pub fn motion_fast_ms(&self) -> f32 {
        setting::motion::FAST_MS.read(self.root()) as f32
    }

    pub fn motion_standard_ms(&self) -> f32 {
        setting::motion::STANDARD_MS.read(self.root()) as f32
    }

    pub fn motion_slow_ms(&self) -> f32 {
        setting::motion::SLOW_MS.read(self.root()) as f32
    }

    pub fn motion_speed_ms(&self, path: &str, default: &str) -> f32 {
        match self.str_path(path).as_str() {
            "fast" => self.motion_fast_ms(),
            "slow" => self.motion_slow_ms(),
            "standard" => self.motion_standard_ms(),
            _ => match default {
                "fast" => self.motion_fast_ms(),
                "slow" => self.motion_slow_ms(),
                _ => self.motion_standard_ms(),
            },
        }
    }

    pub fn launch_motion_ms(&self) -> f32 {
        self.motion_speed_ms(keys::motion::LAUNCH_SPEED, "standard")
    }

    pub fn filter_swap_motion_ms(&self) -> f32 {
        self.motion_speed_ms(keys::motion::FILTER_SWAP_SPEED, "slow")
    }

    pub fn card_flip_duration_ms(&self) -> f32 {
        setting::selector::FLIP_DURATION_MS.read(self.root()) as f32
    }

    pub fn card_flip_shader(&self) -> bool {
        self.flag_default_true(keys::selector::FLIP_SHADER)
    }

    pub fn card_flip_back_reveal(&self) -> bool {
        self.flag_default_true(keys::selector::FLIP_BACK_REVEAL)
    }
}
