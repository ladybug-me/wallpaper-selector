use serde_json::Value;

use super::super::Config;

fn percent(value: f32) -> f64 {
    (f64::from(value) * 100_000.0).round() / 1_000.0
}

impl Config {
    pub fn selector_preset_snapshot(&self) -> Value {
        let mut map = serde_json::Map::new();
        match self.selector_mode().as_str() {
            "grid" => super::preset_grid::snapshot(self, &mut map),
            "hex" => {
                map.insert("hexRadius".into(), Value::from(self.hex_radius() as f64));
                map.insert("hexRows".into(), Value::from(self.hex_rows() as u64));
                map.insert("hexCols".into(), Value::from(self.hex_cols() as u64));
                map.insert("hexScrollStep".into(), Value::from(self.hex_scroll_step() as u64));
                map.insert("hexArc".into(), Value::from(self.hex_arc()));
                map.insert("hexArcIntensity".into(), Value::from(self.hex_arc_intensity() as f64));
                map.insert("hexCurve".into(), Value::from(self.hex_curve()));
                map.insert("hexShape".into(), Value::from(self.hex_shape()));
                map.insert(
                    "hexCurveFrequency".into(),
                    Value::from(self.hex_curve_frequency() as f64),
                );
                map.insert("hexGapX".into(), Value::from(self.hex_gap_x() as f64));
                map.insert("hexGapY".into(), Value::from(self.hex_gap_y() as f64));
                map.insert("hexAspect".into(), Value::from(percent(self.hex_aspect())));
                map.insert("hexStagger".into(), Value::from(percent(self.hex_stagger())));
                map.insert("hexLens".into(), Value::from(percent(self.hex_lens())));
                map.insert("hexLensRadius".into(), Value::from(self.hex_lens_radius() as f64));
                map.insert("hexOrbit".into(), Value::from(percent(self.hex_orbit())));
                map.insert("hexOrbitRadius".into(), Value::from(self.hex_orbit_radius() as f64));
                map.insert("hexTwist".into(), Value::from(self.hex_twist() as f64));
                map.insert("hexScatter".into(), Value::from(self.hex_scatter() as f64));
                let [x, y, scale, rotation, perspective, shear_x, shear_y, depth_angle] =
                    self.hex_stage();
                map.insert("hexStageX".into(), Value::from(percent(x)));
                map.insert("hexStageY".into(), Value::from(percent(y)));
                map.insert("hexStageScale".into(), Value::from(percent(scale)));
                map.insert("hexStageRotation".into(), Value::from(rotation as f64));
                map.insert("hexStagePerspective".into(), Value::from(percent(perspective)));
                map.insert("hexStageShearX".into(), Value::from(percent(shear_x)));
                map.insert("hexStageShearY".into(), Value::from(percent(shear_y)));
                map.insert("hexStageDepthAngle".into(), Value::from(depth_angle as f64));
            }
            "sandy" => self.snapshot_sandy(&mut map),
            _ => self.snapshot_slices(&mut map),
        }
        Value::Object(map)
    }

    fn snapshot_sandy(&self, map: &mut serde_json::Map<String, Value>) {
        let (x, y) = self.sandy_position();
        map.insert("sandyStageX".into(), Value::from(percent(x)));
        map.insert("sandyStageY".into(), Value::from(percent(y)));
        map.insert("sandyCenter".into(), Value::from(self.sandy_center() as f64));
        map.insert("sandySliceWidth".into(), Value::from(self.sandy_slice_width() as f64));
        map.insert("sandySliceHeight".into(), Value::from(self.sandy_slice_height() as f64));
        map.insert("sandySkew".into(), Value::from(self.sandy_skew() as f64));
        map.insert("sandySpacing".into(), Value::from(self.sandy_spacing() as f64));
        map.insert("sandyDuration".into(), Value::from(self.sandy_duration() as f64));
        map.insert("sandyBlend".into(), Value::from(self.sandy_blend() as f64));
        map.insert("sandyStrands".into(), Value::from(self.sandy_strands() as f64));
        map.insert("sandyTwist".into(), Value::from((self.sandy_twist() * 100.0) as f64));
        map.insert("sandyOrbit".into(), Value::from((self.sandy_orbit() * 100.0) as f64));
        map.insert("sandyTurbulence".into(), Value::from((self.sandy_turbulence() * 100.0) as f64));
        map.insert("sandyWaist".into(), Value::from((self.sandy_waist() * 100.0) as f64));
        map.insert("sandyFront".into(), Value::from((self.sandy_front() / 0.65 * 100.0) as f64));
        map.insert("sandyFan".into(), Value::from((self.sandy_fan() / 0.6 * 100.0) as f64));
        map.insert("sandyArc".into(), Value::from((self.sandy_arc() * 100.0) as f64));
        map.insert("sandySwapLoop".into(), Value::from(self.sandy_swap_loop()));
        map.insert("sandySwapStyle".into(), Value::from(self.sandy_swap_style()));
        map.insert(
            "sandyEdgeSpeed".into(),
            Value::from((self.sandy_edge_speed() / 14.0 * 100.0) as f64),
        );
        map.insert("sandyRingSpin".into(), Value::from((self.sandy_ring_spin() * 100.0) as f64));
        map.insert("sandyRingSize".into(), Value::from((self.sandy_ring_size() * 100.0) as f64));
        map.insert("sandyRingWave".into(), Value::from((self.sandy_ring_wave() * 100.0) as f64));
        map.insert("sandyRingSoft".into(), Value::from((self.sandy_ring_soft() * 100.0) as f64));
        map.insert("sandyRingBlend".into(), Value::from((self.sandy_ring_blend() * 100.0) as f64));
        map.insert("sandyRingHold".into(), Value::from((self.sandy_ring_hold() * 1000.0) as f64));
        map.insert("sandyGrain".into(), Value::from(self.sandy_grain() as f64));
        map.insert("sandyResScale".into(), Value::from((self.sandy_res_scale() * 100.0) as f64));
        map.insert("sandyLod".into(), Value::from(self.sandy_lod() as f64));
        map.insert("sandyLodAuto".into(), Value::from(self.sandy_lod_auto()));
        map.insert("sandyOutgoingLive".into(), Value::from(self.sandy_video_out_live()));
    }

    fn snapshot_slices(&self, map: &mut serde_json::Map<String, Value>) {
        let (x, y) = self.slice_position();
        map.insert("sliceStageX".into(), Value::from(percent(x)));
        map.insert("sliceStageY".into(), Value::from(percent(y)));
        map.insert("sliceHeight".into(), Value::from(self.slice_height() as f64));
        map.insert("visibleCount".into(), Value::from(self.visible_count() as u64));
        map.insert("expandedWidth".into(), Value::from(self.expanded_width() as f64));
        map.insert("sliceWidth".into(), Value::from(self.slice_width() as f64));
        map.insert("sliceSpacing".into(), Value::from(self.slice_spacing() as f64));
        map.insert("skewOffset".into(), Value::from(self.skew_offset() as f64));
        map.insert("sliceWobble".into(), Value::from(self.slice_wobble()));
        map.insert(
            "sliceWobbleStrength".into(),
            Value::from((self.slice_wobble_strength() * 100.0) as f64),
        );
        let round = self.bool_false_unless_true(skwd_config::keys::selector::ROUND_CORNERS);
        map.insert("roundCorners".into(), Value::from(round));
        let base = self.num_at(skwd_config::keys::selector::CORNER_RADIUS, 16.0);
        for key in ["cornerTL", "cornerTR", "cornerBR", "cornerBL"] {
            let value = self.num_at(&format!("components.wallpaperSelector.{key}"), base);
            map.insert(key.into(), Value::from(value));
        }
    }

    pub fn selector_presets(&self, mode: &str) -> Vec<(String, Value)> {
        self.get(&format!("components.wallpaperSelector.presets.{mode}"))
            .and_then(Value::as_array)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|entry| {
                        Some((
                            entry.get("name")?.as_str()?.to_string(),
                            entry.get("params")?.clone(),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn selected_preset(&self, mode: &str) -> Option<String> {
        self.get(&format!("components.wallpaperSelector.activePreset.{mode}"))
            .and_then(Value::as_str)
            .filter(|name| !name.is_empty())
            .map(String::from)
    }

    pub fn next_preset_name(&self, mode: &str, candidate: impl FnMut(usize) -> String) -> String {
        let existing = self.selector_presets(mode);
        (1..=existing.len() + 1)
            .map(candidate)
            .find(|name| !existing.iter().any(|(other, _)| other == name))
            .expect("one of n + 1 generated preset names must be available")
    }
}
