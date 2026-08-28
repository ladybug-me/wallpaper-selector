use std::collections::HashMap;

use crate::contracts::preview::{PREVIEW_HEIGHT, PREVIEW_WIDTH};

pub const BC_ATLAS_SIZE: u32 = 2048;
pub const RGBA_ATLAS_SIZE: u32 = 1024;
pub const NEAR_W: u32 = PREVIEW_WIDTH;
pub const NEAR_H: u32 = PREVIEW_HEIGHT;
pub const FAR_W: u32 = 160;
pub const FAR_H: u32 = 88;
pub const BC_FAR_CAP: u32 = 768;
pub const BC_NEAR_CAP: u32 = 64;
pub const BC_NEAR_INIT_LAYERS: u32 = 16;
pub const BC_NEAR_GROW_CHUNK: u32 = 8;
pub const BC_FAR_INIT_LAYERS: u32 = 1;
pub const RGBA_FAR_LAYERS: u32 = 2;
pub const RGBA_NEAR_CAP: u32 = 4;
pub const RGBA_FAR_CAP: u32 = far_slots_per_layer(RGBA_ATLAS_SIZE) * RGBA_FAR_LAYERS;
pub const BROWSER_NEAR_CAP: u32 = 24;
pub const BROWSER_NEAR_INIT_LAYERS: u32 = 8;
pub const BROWSER_NEAR_GROW_CHUNK: u32 = 8;
pub const NEAR_MIP_MIN: u32 = 16;

const fn far_slots_per_layer(size: u32) -> u32 {
    (size / FAR_W) * (size / FAR_H)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtlasBudget {
    pub size: u32,
    pub near_cap: u32,
    pub near_init_layers: u32,
    pub near_grow_chunk: u32,
    pub far_initial_layers: u32,
    pub far_cap: u32,
}

pub const fn budget(compressed: bool) -> AtlasBudget {
    if compressed {
        AtlasBudget {
            size: BC_ATLAS_SIZE,
            near_cap: BC_NEAR_CAP,
            near_init_layers: BC_NEAR_INIT_LAYERS,
            near_grow_chunk: BC_NEAR_GROW_CHUNK,
            far_initial_layers: BC_FAR_INIT_LAYERS,
            far_cap: BC_FAR_CAP,
        }
    } else {
        AtlasBudget {
            size: RGBA_ATLAS_SIZE,
            near_cap: RGBA_NEAR_CAP,
            near_init_layers: RGBA_NEAR_CAP,
            near_grow_chunk: RGBA_NEAR_CAP,
            far_initial_layers: RGBA_FAR_LAYERS,
            far_cap: RGBA_FAR_CAP,
        }
    }
}

pub const fn browser_budget(near_all_cards: bool) -> AtlasBudget {
    let base = budget(false);
    if near_all_cards {
        AtlasBudget {
            near_cap: BROWSER_NEAR_CAP,
            near_init_layers: BROWSER_NEAR_INIT_LAYERS,
            near_grow_chunk: BROWSER_NEAR_GROW_CHUNK,
            ..base
        }
    } else {
        base
    }
}

pub const fn near_mip_count() -> u32 {
    let (mut w, mut h, mut count) = (NEAR_W, NEAR_H, 0u32);
    loop {
        count += 1;
        let (nw, nh) = (w / 2, h / 2);
        if (if nw < nh { nw } else { nh }) < NEAR_MIP_MIN {
            break;
        }
        w = nw;
        h = nh;
    }
    count
}

pub const NEAR_MIPS: u32 = near_mip_count();

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlotState {
    Pending,
    Ready,
}

#[derive(Debug, Clone, Copy)]
pub struct Slot {
    pub id: u32,
    pub state: SlotState,
    last_used: u64,
}

pub struct TierMap {
    pub slot_of: HashMap<usize, Slot>,
    free: Vec<u32>,
    clock: u64,
    previous_frame: std::collections::HashSet<usize>,
    current_frame: std::collections::HashSet<usize>,
}

impl TierMap {
    fn new(capacity: u32) -> Self {
        Self {
            slot_of: HashMap::new(),
            free: (0..capacity).rev().collect(),
            clock: 0,
            previous_frame: std::collections::HashSet::new(),
            current_frame: std::collections::HashSet::new(),
        }
    }

    fn tick(&mut self) -> u64 {
        self.clock += 1;
        self.clock
    }

    pub fn touch(&mut self, idx: usize) {
        let now = self.tick();
        if let Some(slot) = self.slot_of.get_mut(&idx) {
            slot.last_used = now;
        }
    }

    pub fn begin_frame(&mut self) {
        self.previous_frame = std::mem::take(&mut self.current_frame);
    }

    pub fn pin(&mut self, idx: usize) {
        self.current_frame.insert(idx);
        self.touch(idx);
    }

    pub fn layer_extent(&self) -> u32 {
        self.slot_of.values().map(|slot| slot.id + 1).max().unwrap_or(0)
    }

    pub fn acquire(&mut self, idx: usize) -> Option<u32> {
        let now = self.tick();
        if let Some(slot) = self.slot_of.get_mut(&idx) {
            slot.last_used = now;
            return None;
        }
        let id = if let Some(id) = self.free.pop() {
            id
        } else {
            let cand = self
                .slot_of
                .iter()
                .filter(|(candidate, slot)| {
                    slot.state == SlotState::Ready
                        && !self.previous_frame.contains(candidate)
                        && !self.current_frame.contains(candidate)
                })
                .min_by_key(|(_, slot)| slot.last_used)
                .map(|(&idx, _)| idx)?;
            self.slot_of.remove(&cand).map(|slot| slot.id)?
        };
        self.slot_of.insert(idx, Slot { id, state: SlotState::Pending, last_used: now });
        Some(id)
    }

    pub fn mark_ready(&mut self, idx: usize) {
        if let Some(slot) = self.slot_of.get_mut(&idx) {
            slot.state = SlotState::Ready;
        }
    }

    pub fn ready(&self, idx: usize) -> Option<u32> {
        match self.slot_of.get(&idx) {
            Some(slot) if slot.state == SlotState::Ready => Some(slot.id),
            _ => None,
        }
    }

    pub fn is_known(&self, idx: usize) -> bool {
        self.slot_of.contains_key(&idx)
    }

    pub fn release(&mut self, idx: usize) {
        if let Some(slot) = self.slot_of.remove(&idx) {
            self.free.push(slot.id);
        }
    }
}

mod tier_tests;

pub struct AtlasMap {
    pub near: TierMap,
    pub far: TierMap,
    pub far_layers: u32,
    pub failed: std::collections::HashSet<usize>,
    pub near_failed: std::collections::HashSet<usize>,
    size: u32,
    far_cols: u32,
    far_per_layer: u32,
    near_capacity: u32,
}

impl AtlasMap {
    pub fn new(item_count: usize) -> Self {
        Self::with_budget(item_count, budget(crate::contracts::preview::compressed_thumbnails()))
    }

    pub(crate) fn with_budget(item_count: usize, budget: AtlasBudget) -> Self {
        let size = budget.size;
        let far_cols = size / FAR_W;
        let far_per_layer = far_slots_per_layer(size);
        let far_capacity = (item_count as u32).clamp(1, budget.far_cap);
        let far_layers = far_capacity.div_ceil(far_per_layer).max(1);
        Self {
            near: TierMap::new(budget.near_cap),
            far: TierMap::new(far_capacity),
            far_layers,
            failed: std::collections::HashSet::new(),
            near_failed: std::collections::HashSet::new(),
            size,
            far_cols,
            far_per_layer,
            near_capacity: budget.near_cap,
        }
    }

    pub fn near_capacity(&self) -> u32 {
        self.near_capacity
    }

    pub fn near_px(id: u32) -> (u32, u32, u32) {
        (id, 0, 0)
    }

    pub fn far_px(&self, id: u32) -> (u32, u32, u32) {
        let layer = id / self.far_per_layer;
        let within = id % self.far_per_layer;
        let x = (within % self.far_cols) * FAR_W;
        let y = (within / self.far_cols) * FAR_H;
        (layer, x, y)
    }

    pub fn near_uv(id: u32) -> ([f32; 2], [f32; 2], u32) {
        ([0.0, 0.0], [1.0, 1.0], id)
    }

    pub fn far_uv(&self, id: u32) -> ([f32; 2], [f32; 2], u32) {
        let (layer, x, y) = self.far_px(id);
        uv_rect(x, y, FAR_W, FAR_H, layer, self.size)
    }
}

fn uv_rect(x: u32, y: u32, w: u32, h: u32, layer: u32, size: u32) -> ([f32; 2], [f32; 2], u32) {
    let size = size as f32;
    let inset = 0.5;
    (
        [(x as f32 + inset) / size, (y as f32 + inset) / size],
        [(w as f32 - 2.0 * inset) / size, (h as f32 - 2.0 * inset) / size],
        layer,
    )
}
