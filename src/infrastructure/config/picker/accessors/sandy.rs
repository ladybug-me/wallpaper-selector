use crate::contracts::picker::sandy_style_index;

use super::super::Config;

impl Config {
    pub fn sandy_center(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_CENTER, 440.0, 330.0) as f32).max(160.0)
    }

    pub fn sandy_slice_width(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_SLICE_WIDTH, 96.0, 68.0) as f32).max(24.0)
    }

    pub fn sandy_slice_height(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_SLICE_HEIGHT, 180.0, 130.0) as f32)
            .max(60.0)
    }

    skwd_config::getters! {
        sandy_skew: sel_num(skwd_config::keys::selector::SANDY_SKEW, 12.0, 8.0) as f32;
        sandy_spacing: sel_num(skwd_config::keys::selector::SANDY_SPACING, 26.0, 20.0) as f32;
    }

    pub fn sandy_duration(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_DURATION, 1250.0, 1250.0) as f32)
            .clamp(200.0, 6000.0)
    }

    pub fn sandy_blend(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_BLEND, 700.0, 700.0) as f32)
            .clamp(100.0, 4000.0)
    }

    pub fn sandy_strands(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_STRANDS, 22.0, 18.0) as f32)
            .clamp(2.0, 64.0)
    }

    pub fn sandy_twist(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_TWIST, 100.0, 100.0) as f32 / 100.0)
            .clamp(0.0, 3.0)
    }

    pub fn sandy_orbit(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_ORBIT, 100.0, 100.0) as f32 / 100.0)
            .clamp(0.0, 3.0)
    }

    pub fn sandy_turbulence(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_TURBULENCE, 100.0, 100.0) as f32 / 100.0)
            .clamp(0.0, 3.0)
    }

    pub fn sandy_waist(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_WAIST, 100.0, 100.0) as f32 / 100.0)
            .clamp(0.1, 3.0)
    }

    pub fn sandy_front(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_FRONT, 100.0, 100.0) as f32 / 100.0 * 0.65)
            .clamp(0.1, 1.6)
    }

    pub fn sandy_arc(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_ARC, 100.0, 100.0) as f32 / 100.0)
            .clamp(0.0, 3.0)
    }

    pub fn sandy_edge_speed(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_EDGE_SPEED, 100.0, 100.0) as f32 / 100.0
            * 14.0)
            .clamp(0.0, 80.0)
    }

    pub fn sandy_ring_spin(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_RING_SPIN, 100.0, 100.0) as f32 / 100.0)
            .clamp(0.0, 3.0)
    }

    pub fn sandy_ring_size(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_RING_SIZE, 100.0, 100.0) as f32 / 100.0)
            .clamp(0.25, 3.0)
    }

    pub fn sandy_ring_wave(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_RING_WAVE, 100.0, 100.0) as f32 / 100.0)
            .clamp(0.0, 3.0)
    }

    pub fn sandy_ring_soft(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_RING_SOFT, 100.0, 100.0) as f32 / 100.0)
            .clamp(0.25, 3.0)
    }

    pub fn sandy_ring_blend(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_RING_BLEND, 100.0, 100.0) as f32 / 100.0)
            .clamp(0.25, 4.0)
    }

    pub fn sandy_ring_hold(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_RING_HOLD, 200.0, 200.0) as f32 / 1000.0)
            .clamp(0.03, 2.0)
    }

    pub fn sandy_grain(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_GRAIN, 3.0, 3.0) as f32).clamp(1.0, 32.0)
    }

    pub fn sandy_res_scale(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_RES_SCALE, 100.0, 100.0) as f32 / 100.0)
            .clamp(0.25, 1.0)
    }

    pub fn sandy_lod(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_LOD, 2.0, 2.0) as f32).clamp(1.0, 4.0)
    }

    pub fn sandy_lod_auto(&self) -> bool {
        self.flag_default_config(skwd_config::keys::selector::SANDY_LOD_AUTO)
    }

    pub fn sandy_swap_style(&self) -> String {
        let style = self.str_path(skwd_config::keys::selector::SANDY_SWAP_STYLE);
        if style.is_empty() { String::from("vortex") } else { style }
    }

    pub fn sandy_swap_style_index(&self) -> f32 {
        sandy_style_index(&self.sandy_swap_style())
    }

    pub fn sandy_fan(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SANDY_FAN, 100.0, 100.0) as f32 / 100.0 * 0.6)
            .clamp(0.1, 1.2)
    }
}
