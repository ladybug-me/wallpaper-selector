use serde_json::Value;

use super::super::Config;

impl Config {
    pub fn slice_position(&self) -> (f32, f32) {
        use skwd_config::schema::setting::selector as sel;
        (
            sel::SLICE_STAGE_X.read(self.root()) as f32 / 100.0,
            sel::SLICE_STAGE_Y.read(self.root()) as f32 / 100.0,
        )
    }

    pub fn sandy_position(&self) -> (f32, f32) {
        use skwd_config::schema::setting::selector as sel;
        (
            sel::SANDY_STAGE_X.read(self.root()) as f32 / 100.0,
            sel::SANDY_STAGE_Y.read(self.root()) as f32 / 100.0,
        )
    }

    pub fn tag_cloud_offset(&self) -> (f32, f32) {
        (
            self.sel_num(skwd_config::keys::selector::TAG_CLOUD_OFFSET_X, 0.0, 0.0) as f32,
            self.sel_num(skwd_config::keys::selector::TAG_CLOUD_OFFSET_Y, 0.0, 0.0) as f32,
        )
    }

    pub fn filter_swap_ms(&self) -> f32 {
        self.filter_swap_motion_ms()
    }

    skwd_config::getters! {
        slice_height: sel_num(skwd_config::keys::selector::SLICE_HEIGHT, 520.0, 360.0) as f32;
        visible_count: sel_num(skwd_config::keys::selector::VISIBLE_COUNT, 12.0, 8.0) as usize;
        expanded_width: sel_num(skwd_config::keys::selector::EXPANDED_WIDTH, 924.0, 600.0) as f32;
        slice_width: sel_num(skwd_config::keys::selector::SLICE_WIDTH, 135.0, 90.0) as f32;
        slice_spacing: sel_num(skwd_config::keys::selector::SLICE_SPACING, -30.0, -30.0) as f32;
        skew_offset: sel_num(skwd_config::keys::selector::SKEW_OFFSET, 35.0, 25.0) as f32;
        grid_thumb_width: sel_num(skwd_config::keys::selector::GRID_THUMB_WIDTH, 300.0, 220.0) as f32;
        grid_thumb_height: sel_num(skwd_config::keys::selector::GRID_THUMB_HEIGHT, 169.0, 124.0) as f32;
        grid_round_corners: bool_setting(skwd_config::schema::setting::selector::GRID_ROUND_CORNERS);
        hex_radius: sel_num(skwd_config::keys::selector::HEX_RADIUS, 140.0, 100.0) as f32;
    }

    pub fn slice_corners(&self) -> [f32; 4] {
        if !self.bool_false_unless_true(skwd_config::keys::selector::ROUND_CORNERS) {
            return [0.0; 4];
        }
        let base = self.num_at(skwd_config::keys::selector::CORNER_RADIUS, 16.0);
        [
            self.num_at(skwd_config::keys::selector::CORNER_TL, base) as f32,
            self.num_at(skwd_config::keys::selector::CORNER_TR, base) as f32,
            self.num_at(skwd_config::keys::selector::CORNER_BR, base) as f32,
            self.num_at(skwd_config::keys::selector::CORNER_BL, base) as f32,
        ]
    }

    pub fn tag_cloud_width(&self) -> f32 {
        self.sel_num(skwd_config::keys::selector::TAG_CLOUD_WIDTH, 760.0, 540.0)
            .clamp(360.0, 2200.0) as f32
    }

    pub fn tag_cloud_width_override(&self) -> Option<f64> {
        self.get(skwd_config::keys::selector::TAG_CLOUD_WIDTH).and_then(Value::as_f64)
    }

    pub fn tag_cloud_height(&self) -> f32 {
        self.sel_num(skwd_config::keys::selector::TAG_CLOUD_HEIGHT, 168.0, 150.0).clamp(90.0, 600.0)
            as f32
    }

    pub fn tag_cloud_rows(&self) -> usize {
        match self.get(skwd_config::keys::selector::TAG_CLOUD_ROWS) {
            Some(Value::String(value)) => value.parse().unwrap_or(2),
            Some(Value::Number(value)) => value.as_u64().unwrap_or(2) as usize,
            _ => 2,
        }
        .clamp(1, 3)
    }

    pub fn grid_columns(&self) -> usize {
        (self.sel_num(skwd_config::keys::selector::GRID_COLUMNS, 6.0, 4.0) as usize).max(1)
    }

    pub fn grid_rows(&self) -> usize {
        (self.sel_num(skwd_config::keys::selector::GRID_ROWS, 3.0, 3.0) as usize).max(1)
    }

    pub fn grid_gap_x(&self) -> f32 {
        skwd_config::schema::setting::selector::GRID_GAP_X.read(self.root()) as f32
    }

    pub fn grid_gap_y(&self) -> f32 {
        skwd_config::schema::setting::selector::GRID_GAP_Y.read(self.root()) as f32
    }

    pub fn grid_corner_radius(&self) -> f32 {
        skwd_config::schema::setting::selector::GRID_CORNER_RADIUS.read(self.root()) as f32
    }

    pub fn grid_border_width(&self) -> f32 {
        skwd_config::schema::setting::selector::GRID_BORDER_WIDTH.read(self.root()) as f32
    }

    pub fn hex_rows(&self) -> usize {
        (self.sel_num(skwd_config::keys::selector::HEX_ROWS, 3.0, 3.0) as usize).max(1)
    }

    pub fn hex_cols(&self) -> usize {
        (self.sel_num(skwd_config::keys::selector::HEX_COLS, 7.0, 5.0) as usize).max(3)
    }

    pub fn hex_scroll_step(&self) -> usize {
        (self.sel_num(skwd_config::keys::selector::HEX_SCROLL_STEP, 1.0, 1.0) as usize).max(1)
    }

    pub fn hex_arc_intensity(&self) -> f32 {
        self.num_at(skwd_config::keys::selector::HEX_ARC_INTENSITY, 1.2) as f32
    }

    pub fn slice_wobble_strength(&self) -> f32 {
        (self.sel_num(skwd_config::keys::selector::SLICE_WOBBLE_STRENGTH, 100.0, 100.0) as f32
            / 100.0)
            .clamp(0.0, 2.0)
    }

    pub fn selector_mode(&self) -> String {
        use crate::contracts::picker::Mode;
        String::from(match Mode::from_key(&self.display_mode()) {
            Mode::Grid => "grid",
            mode => mode.as_key(),
        })
    }
}
