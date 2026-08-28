use crate::frontend::animation::{MotionProfile, MotionTier, Timeline};
use crate::frontend::effects::dark_chrome;
use crate::frontend::theme::Palette;

use super::model::{Effects, EffectsMode};

pub(in crate::frontend::effects) struct EffectsPanelState {
    pub(in crate::frontend::effects) animation: Timeline,
    pub(in crate::frontend::effects) busy: bool,
    pub(in crate::frontend::effects) preview_dirty: bool,
    pub(in crate::frontend::effects) status: String,
    pub(in crate::frontend::effects) busy_spin: f32,
    pub(in crate::frontend::effects) committing: bool,
    pub(in crate::frontend::effects) apply_hold: f32,
    pub(in crate::frontend::effects) mode: EffectsMode,
    pub(in crate::frontend::effects) chrome: Palette,
    pub(in crate::frontend::effects) motion: MotionProfile,
}

impl EffectsPanelState {
    pub(super) fn new() -> Self {
        let motion = MotionProfile::default();
        Self {
            animation: Timeline::new()
                .with("open", 0.0, 1.0, motion.approach_tau(MotionTier::Fast))
                .with("params", 0.0, 1.0, motion.approach_tau(MotionTier::Slow))
                .with("list", 0.0, 1.0, motion.approach_tau(MotionTier::Slow))
                .with("fade", 1.0, 1.0, motion.approach_tau(MotionTier::Slow))
                .with("bar", 0.0, 1.0, motion.approach_tau(MotionTier::Standard))
                .with("apply", 0.0, 0.0, motion.approach_tau(MotionTier::Slow)),
            busy: false,
            preview_dirty: false,
            status: String::new(),
            busy_spin: 0.0,
            committing: false,
            apply_hold: 0.0,
            mode: EffectsMode::Studio,
            chrome: dark_chrome(&Palette::default()),
            motion,
        }
    }

    pub(super) fn set_motion_profile(&mut self, motion: MotionProfile) {
        self.motion = motion;
        for (name, tier) in [
            ("open", MotionTier::Fast),
            ("params", MotionTier::Slow),
            ("list", MotionTier::Slow),
            ("fade", MotionTier::Slow),
            ("bar", MotionTier::Standard),
            ("apply", MotionTier::Slow),
        ] {
            self.animation.set_tau(name, motion.approach_tau(tier));
        }
    }
}

impl Effects {
    pub fn tick(&mut self, delta_seconds: f32) {
        if self.panel.committing {
            self.panel.animation.toward("apply", 1.0);
            if self.panel.animation.get("apply") > 0.99 && !self.panel.busy {
                self.panel.apply_hold += delta_seconds;
            }
        }
        self.panel.animation.tick(delta_seconds);
        if self.panel.busy {
            self.panel.busy_spin =
                (self.panel.busy_spin + delta_seconds * 4.0) % std::f32::consts::TAU;
        } else {
            self.panel.busy_spin = 0.0;
        }
        for monitor in &self.displays.monitors {
            let target = f32::from(self.displays.selected_outputs.contains(&monitor.target));
            if let Some(animation) = self.displays.tile_animation.get_mut(&monitor.target) {
                *animation =
                    self.panel.motion.approach(*animation, target, delta_seconds, MotionTier::Fast);
                if (*animation - target).abs() < 0.003 {
                    *animation = target;
                }
            } else {
                self.displays.tile_animation.insert(monitor.target.clone(), target);
            }
        }
    }

    pub fn animating(&self) -> bool {
        !self.panel.animation.settled()
            || self.panel.committing
            || self.panel.busy
            || self.displays.monitors.iter().any(|monitor| {
                let target = f32::from(self.displays.selected_outputs.contains(&monitor.target));
                self.displays
                    .tile_animation
                    .get(&monitor.target)
                    .is_some_and(|animation| (*animation - target).abs() >= 0.003)
            })
    }

    pub fn begin_apply(&mut self) {
        self.panel.committing = true;
        self.panel.animation.restart("apply", 0.0);
    }

    pub fn apply_ease(&self) -> f32 {
        self.panel.animation.ease("apply")
    }

    pub fn apply_finished(&self) -> bool {
        self.panel.committing && self.panel.apply_hold > 0.35
    }

    pub fn fade_ease(&self) -> f32 {
        self.panel.animation.ease("fade")
    }

    pub fn open_ease(&self) -> f32 {
        self.panel.animation.ease("open")
    }

    pub fn mark_preview_queued(&mut self) {
        self.panel.preview_dirty = true;
    }

    pub fn begin_request(&mut self) {
        self.panel.busy = true;
        self.panel.preview_dirty = false;
    }

    pub fn fail_request(&mut self, error: &str) {
        self.panel.busy = false;
        self.panel.preview_dirty = false;
        self.panel.status = crate::i18n::tr_args!("effects-failed", error => error);
    }
}
