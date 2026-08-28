use serde_json::{Map, Value};

use super::super::Config;

fn percent(value: f32) -> f64 {
    (f64::from(value) * 100_000.0).round() / 1_000.0
}

pub(super) fn snapshot(config: &Config, map: &mut Map<String, Value>) {
    map.insert("gridColumns".into(), Value::from(config.grid_columns() as u64));
    map.insert("gridRows".into(), Value::from(config.grid_rows() as u64));
    map.insert("gridThumbWidth".into(), Value::from(config.grid_thumb_width() as f64));
    map.insert("gridThumbHeight".into(), Value::from(config.grid_thumb_height() as f64));
    map.insert("gridGapX".into(), Value::from(config.grid_gap_x() as f64));
    map.insert("gridGapY".into(), Value::from(config.grid_gap_y() as f64));
    map.insert("gridRoundCorners".into(), Value::from(config.grid_round_corners()));
    map.insert("gridCornerRadius".into(), Value::from(config.grid_corner_radius() as f64));
    map.insert("gridBorderWidth".into(), Value::from(config.grid_border_width() as f64));
    map.insert("gridLayout".into(), Value::from(config.grid_layout()));
    map.insert("gridStagger".into(), Value::from(percent(config.grid_stagger())));
    map.insert("gridSelectedScale".into(), Value::from(percent(config.grid_selected_scale())));
    map.insert("gridFlowWave".into(), Value::from(config.grid_flow_wave() as f64));
    map.insert("gridFlowFrequency".into(), Value::from(config.grid_flow_frequency() as f64));
    map.insert("gridScatter".into(), Value::from(config.grid_scatter() as f64));
    map.insert("gridScaleVariance".into(), Value::from(percent(config.grid_scale_variance())));
    map.insert("gridCylinderBend".into(), Value::from(percent(config.grid_cylinder_bend())));
    map.insert("gridCylinderRadius".into(), Value::from(config.grid_cylinder_radius() as f64));
    let [x, y, scale, rotation, perspective, shear_x, shear_y, depth_angle] = config.grid_stage();
    map.insert("gridStageX".into(), Value::from(percent(x)));
    map.insert("gridStageY".into(), Value::from(percent(y)));
    map.insert("gridStageScale".into(), Value::from(percent(scale)));
    map.insert("gridStageRotation".into(), Value::from(rotation as f64));
    map.insert("gridStagePerspective".into(), Value::from(percent(perspective)));
    map.insert("gridStageShearX".into(), Value::from(percent(shear_x)));
    map.insert("gridStageShearY".into(), Value::from(percent(shear_y)));
    map.insert("gridStageDepthAngle".into(), Value::from(depth_angle as f64));
}
