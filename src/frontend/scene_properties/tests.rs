#![cfg(test)]

use super::state::SceneProperties;
use crate::domain::scene_properties::{SceneProperty, ScenePropertyKind, ScenePropertyValue};

fn property(name: &str, kind: ScenePropertyKind, value: ScenePropertyValue) -> SceneProperty {
    SceneProperty {
        name: name.to_string(),
        label: name.to_string(),
        kind,
        default: value.clone(),
        value,
        ..SceneProperty::default()
    }
}

fn loaded() -> SceneProperties {
    let mut panel = SceneProperties::opening("scene-a", "Scene A");
    panel.accept(
        "scene-a",
        vec![
            property("header", ScenePropertyKind::Group, ScenePropertyValue::Absent),
            property("glow", ScenePropertyKind::Flag, ScenePropertyValue::Flag(true)),
            property(
                "tint",
                ScenePropertyKind::Colour,
                ScenePropertyValue::Vector(vec![1.0, 1.0, 1.0]),
            ),
            property("zoom", ScenePropertyKind::Range, ScenePropertyValue::Number(1.0)),
            property("shortcut", ScenePropertyKind::Unsupported, ScenePropertyValue::Absent),
        ],
    );
    panel
}

#[test]
fn foreign_reply_ignored() {
    let mut panel = SceneProperties::opening("scene-a", "Scene A");
    panel.accept(
        "scene-b",
        vec![property("glow", ScenePropertyKind::Flag, ScenePropertyValue::Flag(true))],
    );
    assert!(panel.rows.is_empty());
    assert!(panel.loading);

    panel.accept(
        "scene-a",
        vec![property("glow", ScenePropertyKind::Flag, ScenePropertyValue::Flag(true))],
    );
    assert_eq!(panel.rows.len(), 1);
    assert!(!panel.loading);
}

#[test]
fn editable_and_overridden_counts() {
    let mut panel = loaded();
    assert_eq!(panel.editable_count(), 3);
    assert_eq!(panel.overridden_count(), 0);

    panel.set_local("glow", ScenePropertyValue::Flag(false));
    assert_eq!(panel.overridden_count(), 1);
    assert!(panel.row("glow").unwrap().overridden);

    panel.set_local("glow", ScenePropertyValue::Flag(true));
    assert_eq!(panel.overridden_count(), 0);
}

#[test]
fn colour_draft_seeding() {
    let mut panel = loaded();
    assert_eq!(panel.colour_draft("tint"), Some("1.000 1.000 1.000"));
    assert_eq!(panel.colour_draft("glow"), None);

    panel.set_colour_draft("tint", "0.5 0");
    assert_eq!(panel.colour_draft("tint"), Some("0.5 0"));

    panel.accept(
        "scene-a",
        vec![property(
            "tint",
            ScenePropertyKind::Colour,
            ScenePropertyValue::Vector(vec![0.25, 0.5, 0.75]),
        )],
    );
    assert_eq!(panel.colour_draft("tint"), Some("0.250 0.500 0.750"));
}

#[test]
fn failure_keeps_message() {
    let mut panel = SceneProperties::opening("scene-a", "Scene A");
    panel.fail("daemon said no");
    assert!(!panel.loading);
    assert_eq!(panel.error.as_deref(), Some("daemon said no"));
}
