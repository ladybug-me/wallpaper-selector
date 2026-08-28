use std::collections::HashSet;

use crate::frontend::animation::smoothstep;
use crate::frontend::scene::InstanceRaw;
use crate::frontend::scene::layout::Mode;
use crate::frontend::scene::{cell_key, roll_out_cut};

use super::model::SceneCore;

const FILTER_FLIP_SWEEP: f32 = 0.6;

impl SceneCore {
    pub fn filter_storm(&mut self, from_store: Option<usize>) {
        if self.mode == Mode::Sandy {
            self.sandy.filter_from = from_store;
            self.sandy.from = None;
            self.sandy.bfrom = None;
            self.sandy.bfrom2 = None;
            self.sandy.bcut = 1.0;
            self.sandy.bto = None;
            self.sandy.storm_to = None;
            self.sandy.ring_to = Some(self.current);
            self.sandy.swirl.retarget(1.0);
            self.sandy.swirl_hold = self.xp.sandy.ring_hold.max(0.3);
            self.sandy.bmix = self.motion.profile.override_spring(
                0.0,
                (self.xp.sandy.blend_ms * 0.5 * self.xp.sandy.ring_blend).clamp(150.0, 3000.0),
            );
            self.sandy.bmix.retarget(1.0);
        }
        let cache = std::mem::take(&mut self.card.filter_cache);
        for (inst, store, roll) in cache {
            if roll < 1.0 || inst.rect[2] < 0.5 {
                continue;
            }
            let key = cell_key(inst.rect[0], inst.rect[1]);
            if self.card.filter_cell.contains_key(&key) {
                continue;
            }
            let t0 = -self.flip_phase(inst.rect[0], inst.rect[1]) * FILTER_FLIP_SWEEP;
            self.card.filter_cell.insert(key, self.card.filter_old.len());
            self.card.filter_old.push((inst, store, t0));
        }
        self.card.filter_wave = 0.0;
        self.motion.needs_frame = true;
    }

    fn flip_phase(&self, x: f32, y: f32) -> f32 {
        let (vw, vh) = self.viewport;
        let phase = match self.mode {
            Mode::Grid => (x / vw.max(1.0) + y / vh.max(1.0)) * 0.5,
            Mode::Hex => {
                let dx = (x - self.center_x()) / (vw * 0.5).max(1.0);
                let dy = (y - vh * 0.5) / (vh * 0.5).max(1.0);
                (dx * dx + dy * dy).sqrt() * 0.7
            }
            _ => x / vw.max(1.0),
        };
        phase.clamp(0.0, 1.0)
    }

    pub(super) fn flip_roll_for(&mut self, store_idx: usize, x: f32, y: f32) -> f32 {
        if self.card.filter_old.is_empty()
            && self.card.filter_in.is_empty()
            && self.card.filter_wave >= 2.0
        {
            return 1.0;
        }
        let key = cell_key(x, y);
        if let Some(&idx) = self.card.filter_cell.get(&key)
            && let Some((_, gstore, t)) = self.card.filter_old.get_mut(idx)
        {
            if *gstore == store_idx as u32 && *t <= 0.1 {
                *t = 2.0;
                return 1.0;
            }
            return smoothstep(*t);
        }
        if let Some(&t) = self.card.filter_in.get(&key) {
            return smoothstep(t);
        }
        let virt = self.card.filter_wave - self.flip_phase(x, y) * FILTER_FLIP_SWEEP;
        if virt < 1.0 {
            self.card.filter_in.insert(key, virt);
            return smoothstep(virt);
        }
        1.0
    }

    pub(super) fn push_flip_old(
        &mut self,
        instances: &mut Vec<InstanceRaw>,
        wanted: &mut HashSet<usize>,
    ) {
        for (inst, store, t) in &self.card.filter_old {
            let roll = smoothstep(*t);
            if roll >= 1.0 {
                continue;
            }
            let mut body = *inst;
            roll_out_cut(&mut body, roll);
            if body.rect[2] < 0.5 {
                continue;
            }
            wanted.insert(*store as usize);
            instances.push(body);
        }
    }

    #[cfg(test)]
    pub fn filter_flip_count(&self) -> usize {
        self.card.filter_old.len()
    }

    pub fn filter_swap_active(&self) -> bool {
        !self.card.filter_old.is_empty() || !self.card.filter_in.is_empty()
    }

    #[cfg(test)]
    pub fn filter_flip_running(&self) -> bool {
        self.filter_swap_active()
    }

    #[cfg(test)]
    pub fn finish_filter_swap(&mut self) {
        self.card.filter_old.clear();
        self.card.filter_in.clear();
        self.card.filter_cell.clear();
        self.card.filter_wave = 10.0;
    }

    #[cfg(test)]
    pub fn seed_filter_cache(&mut self, count: usize) {
        self.card.filter_cache = (0..count)
            .map(|idx| {
                let inst = InstanceRaw {
                    rect: [idx as f32 * 90.0, 120.0, 40.0, 40.0],
                    params: [0.0, 0.0, 1.0, 0.0],
                    ..Default::default()
                };
                (inst, idx as u32, 1.0)
            })
            .collect();
    }
}
