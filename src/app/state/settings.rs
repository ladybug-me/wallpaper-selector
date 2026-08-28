#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};

use crate::domain::input::Trigger;
use crate::frontend::animation::{MotionProfile, MotionTier, Tween};
use crate::frontend::settings::{ActionId, SettingsFocus, SettingsSearchResult};

pub(crate) struct KeybindCapture {
    pub(crate) path: String,
    pub(crate) action: crate::domain::input::InputAction,
    pub(crate) title_key: &'static str,
    pub(crate) triggers: Vec<Trigger>,
    pub(crate) edited: bool,
}

pub(crate) struct SettingsInputEdit {
    pub(crate) key: String,
    pub(crate) original: String,
    pub(crate) we_dirty: bool,
    pub(crate) playlist_dirty: bool,
    pub(crate) schedule_dirty: bool,
}

pub(crate) struct SettingsState {
    pub(crate) open: bool,
    pub(crate) entrance: Tween,
    pub(crate) tab_anim: Tween,
    pub(crate) inputs: HashMap<String, String>,
    pub(crate) tab: String,
    pub(crate) section: usize,
    pub(crate) control_page: usize,
    pub(crate) focus: SettingsFocus,
    pub(crate) focused_control: usize,
    pub(crate) focused_choice: Option<usize>,
    pub(crate) expanded_details: HashSet<String>,
    pub(crate) bar_reveals: HashMap<String, Tween>,
    pub(crate) search_open: bool,
    pub(crate) search_query: String,
    pub(crate) search_results: Vec<SettingsSearchResult>,
    pub(crate) input_edit: Option<SettingsInputEdit>,
    pub(crate) keybind_capture: Option<KeybindCapture>,
    pub(crate) section_anim: Tween,
    pub(crate) control_anim: Tween,
    pub(crate) inset_target: f32,
    pub(crate) armed: Option<ActionId>,
    pub(crate) we_dirty: bool,
    pub(crate) playlist_dirty: bool,
    pub(crate) schedule_dirty: bool,
    pub(crate) preview_desired_path: String,
    pub(crate) preview_in_flight_path: String,
    pub(crate) preview_allocated_path: String,
    pub(crate) preview_allocation: Option<iced_runtime::image::Allocation>,
    pub(crate) semantic_importing: bool,
    pub(crate) semantic_import_status: String,
}

impl Default for SettingsState {
    fn default() -> Self {
        let motion = MotionProfile::default();
        Self {
            open: false,
            entrance: motion.tween(0.0, MotionTier::Standard),
            tab_anim: motion.tween(1.0, MotionTier::Fast),
            inputs: HashMap::new(),
            tab: String::from("picker"),
            section: 0,
            control_page: 0,
            focus: SettingsFocus::Sections,
            focused_control: 0,
            focused_choice: None,
            expanded_details: HashSet::new(),
            bar_reveals: HashMap::new(),
            search_open: false,
            search_query: String::new(),
            search_results: Vec::new(),
            input_edit: None,
            keybind_capture: None,
            section_anim: motion.tween(1.0, MotionTier::Standard),
            control_anim: motion.tween(1.0, MotionTier::Fast),
            inset_target: 0.0,
            armed: None,
            we_dirty: false,
            playlist_dirty: false,
            schedule_dirty: false,
            preview_desired_path: String::new(),
            preview_in_flight_path: String::new(),
            preview_allocated_path: String::new(),
            preview_allocation: None,
            semantic_importing: false,
            semantic_import_status: String::new(),
        }
    }
}

impl SettingsState {
    pub(crate) fn set_motion_profile(&mut self, motion: MotionProfile) {
        motion.retime_tween(&mut self.entrance, MotionTier::Standard);
        motion.retime_tween(&mut self.tab_anim, MotionTier::Fast);
        motion.retime_tween(&mut self.section_anim, MotionTier::Standard);
        motion.retime_tween(&mut self.control_anim, MotionTier::Fast);
        for reveal in self.bar_reveals.values_mut() {
            motion.retime_tween(reveal, MotionTier::Fast);
        }
    }

    pub(crate) fn open_bar(&mut self, id: String, motion: MotionProfile) {
        self.bar_reveals
            .entry(id)
            .or_insert_with(|| motion.tween(0.0, MotionTier::Fast))
            .retarget(1.0);
    }

    pub(crate) fn toggle_bar(&mut self, id: String, motion: MotionProfile) {
        let reveal =
            self.bar_reveals.entry(id).or_insert_with(|| motion.tween(0.0, MotionTier::Fast));
        reveal.retarget(if reveal.target > 0.5 { 0.0 } else { 1.0 });
    }

    pub(crate) fn bars_animating(&self) -> bool {
        self.bar_reveals.values().any(|reveal| !reveal.settled())
    }

    pub(crate) fn tick_bars(&mut self, dt: f32) {
        for reveal in self.bar_reveals.values_mut() {
            reveal.tick(dt);
        }
        self.bar_reveals.retain(|_, reveal| reveal.target > 0.0 || reveal.x > 0.0);
    }
}
