#![cfg(test)]

use super::*;
use crate::contracts::picker::format_config_number as fmt_num;
use serde_json::{Value, json};

#[test]
fn fmt_num_whole() {
    assert_eq!(fmt_num(30.0), "30");
    assert_eq!(fmt_num(1.0), "1");
    assert_eq!(fmt_num(0.0), "0");
    assert_eq!(fmt_num(2.5), "2.5");
    assert_eq!(fmt_num(165.0), "165");
}

fn cfg(data: Value) -> Config {
    Config {
        data,
        config_path: std::env::temp_dir().join("skwd_preset_test.json"),
        small: false,
        transient: false,
        on_battery: false,
    }
}

#[test]
fn preset_round_trip() {
    let mut conf = cfg(json!({"components":{"wallpaperSelector":{
            "displayMode":"slices","sliceWidth":200.0,"sliceSpacing":10.0}}}));
    conf.save_selector_preset("slices", "A");
    conf.set_key("components.wallpaperSelector.sliceWidth", json!(50.0));
    assert_eq!(conf.slice_width(), 50.0);
    conf.apply_selector_preset("slices", "A");
    assert_eq!(conf.slice_width(), 200.0);
    assert_eq!(conf.slice_spacing(), 10.0);
    let presets = conf.selector_presets("slices");
    assert_eq!(presets.len(), 1);
    assert_eq!(conf.selector_preset_snapshot(), presets[0].1);
}

#[test]
fn presets_per_mode() {
    let mut conf = cfg(json!({"components":{"wallpaperSelector":{"displayMode":"slices"}}}));
    conf.save_selector_preset("slices", "S1");
    conf.save_selector_preset("grid", "G1");
    assert_eq!(conf.selector_presets("slices").len(), 1);
    assert_eq!(conf.selector_presets("grid").len(), 1);
    conf.delete_selector_preset("slices", "S1");
    assert_eq!(conf.selector_presets("slices").len(), 0);
    assert_eq!(conf.selector_presets("grid").len(), 1);
}

#[test]
fn preset_name_skips_persisted() {
    let conf = cfg(json!({"components":{"wallpaperSelector":{
        "displayMode":"slices",
        "presets":{"slices":[
            {"name":"Stil 1","params":{}},
            {"name":"Stil 2","params":{}}
        ]}
    }}}));

    let next = conf.next_preset_name("slices", |number| format!("Stil {number}"));

    assert_eq!(next, "Stil 3");
}

#[test]
fn sandy_preset_keeps_ring_size() {
    let conf = cfg(json!({"components":{"wallpaperSelector":{
        "displayMode":"sandy","sandyRingSize":165.0,"sandyStageX":25.0,"sandyStageY":-15.0
    }}}));
    let snapshot = conf.selector_preset_snapshot();
    assert_eq!(snapshot["sandyRingSize"], json!(165.0));
    assert_eq!(snapshot["sandyStageX"], json!(25.0));
    assert_eq!(snapshot["sandyStageY"], json!(-15.0));
}

#[test]
fn slices_preset_keeps_position() {
    let conf = cfg(json!({"components":{"wallpaperSelector":{
        "displayMode":"slices","sliceStageX":35.0,"sliceStageY":-20.0
    }}}));
    let snapshot = conf.selector_preset_snapshot();
    assert_eq!(snapshot["sliceStageX"], json!(35.0));
    assert_eq!(snapshot["sliceStageY"], json!(-20.0));
}

#[test]
fn wall_preset_keeps_appearance() {
    let conf = cfg(json!({"components":{"wallpaperSelector":{
        "displayMode":"grid",
        "gridGapX":24.0,
        "gridGapY":12.0,
        "gridRoundCorners":false,
        "gridCornerRadius":20.0,
        "gridBorderWidth":3.0,
        "gridLayout":"editorial",
        "gridStageRotation":12.0,
        "gridStageShearX":18.0,
        "gridStageDepthAngle":72.0,
        "gridSelectedScale":145.0,
        "gridFlowWave":44.0,
        "gridScatter":21.0,
        "gridScaleVariance":26.0,
        "gridCylinderBend":68.0,
        "gridCylinderRadius":760.0
    }}}));
    let snapshot = conf.selector_preset_snapshot();
    assert_eq!(snapshot["gridGapX"], json!(24.0));
    assert_eq!(snapshot["gridGapY"], json!(12.0));
    assert_eq!(snapshot["gridRoundCorners"], json!(false));
    assert_eq!(snapshot["gridCornerRadius"], json!(20.0));
    assert_eq!(snapshot["gridBorderWidth"], json!(3.0));
    assert_eq!(snapshot["gridLayout"], json!("editorial"));
    assert_eq!(snapshot["gridStageRotation"], json!(12.0));
    assert_eq!(snapshot["gridStageShearX"], json!(18.0));
    assert_eq!(snapshot["gridStageDepthAngle"], json!(72.0));
    assert_eq!(snapshot["gridSelectedScale"], json!(145.0));
    assert_eq!(snapshot["gridFlowWave"], json!(44.0));
    assert_eq!(snapshot["gridScatter"], json!(21.0));
    assert_eq!(snapshot["gridScaleVariance"], json!(26.0));
    assert_eq!(snapshot["gridCylinderBend"], json!(68.0));
    assert_eq!(snapshot["gridCylinderRadius"], json!(760.0));
}

#[test]
fn hex_preset_keeps_field_geometry() {
    let conf = cfg(json!({"components":{"wallpaperSelector":{
        "displayMode":"hex", "hexCurve":"wave", "hexShape":"rhombus",
        "hexCurveFrequency":2.0,
        "hexAspect":130.0, "hexGapX":18.0, "hexLens":35.0,
        "hexStageScale":120.0, "hexStageRotation":-9.0,
        "hexStageShearY":-16.0, "hexStageDepthAngle":-40.0,
        "hexOrbit":55.0, "hexOrbitRadius":780.0, "hexTwist":28.0,
        "hexScatter":14.0
    }}}));
    let snapshot = conf.selector_preset_snapshot();
    assert_eq!(snapshot["hexCurve"], json!("wave"));
    assert_eq!(snapshot["hexShape"], json!("rhombus"));
    assert_eq!(snapshot["hexCurveFrequency"], json!(2.0));
    assert_eq!(snapshot["hexAspect"], json!(130.0));
    assert_eq!(snapshot["hexGapX"], json!(18.0));
    assert_eq!(snapshot["hexLens"], json!(35.0));
    assert_eq!(snapshot["hexStageScale"], json!(120.0));
    assert_eq!(snapshot["hexStageRotation"], json!(-9.0));
    assert_eq!(snapshot["hexStageShearY"], json!(-16.0));
    assert_eq!(snapshot["hexStageDepthAngle"], json!(-40.0));
    assert_eq!(snapshot["hexOrbit"], json!(55.0));
    assert_eq!(snapshot["hexOrbitRadius"], json!(780.0));
    assert_eq!(snapshot["hexTwist"], json!(28.0));
    assert_eq!(snapshot["hexScatter"], json!(14.0));
}

#[test]
fn grid_dimensions_never_zero() {
    let conf = cfg(json!({"components":{"wallpaperSelector":{
            "displayMode":"grid","gridColumns":-999,"gridRows":0}}}));
    assert!(conf.grid_columns() >= 1);
    assert!(conf.grid_rows() >= 1);
}
