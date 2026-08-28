#![cfg(test)]

use super::*;

fn designer() -> ThemeDesigner {
    ThemeDesigner::new(Candidate::from_preset("nord").unwrap(), String::new())
}

#[test]
fn select_role_sync() {
    let mut designer = designer();
    designer.select_role(3);
    assert_eq!(designer.hex_buf, designer.candidate.colors[3]);
    let expected = hex_to_hsv(&designer.candidate.colors[3]).unwrap();
    assert_eq!(designer.hsv, expected);
    designer.select_role(99);
    assert_eq!(designer.selected, 3);
}

#[test]
fn wheel_edit_scope() {
    let mut designer = designer();
    let before = designer.candidate.clone();
    designer.select_role(0);
    designer.set_hue(120.0);
    designer.set_sv(1.0, 1.0);
    assert_eq!(designer.candidate.colors[0], "#00ff00");
    assert_eq!(designer.hex_buf, "#00ff00");
    for i in 1..9 {
        assert_eq!(designer.candidate.colors[i], before.colors[i]);
    }
}

#[test]
fn hue_on_grey() {
    let mut designer = designer();
    designer.pick_recent("#808080");
    designer.set_hue(0.0);
    assert_ne!(designer.candidate.colors[0], "#808080");
}

#[test]
fn hex_round_trip() {
    let mut designer = designer();
    designer.hex_buf = "#B48EAD".to_string();
    assert!(designer.apply_hex());
    assert_eq!(designer.candidate.colors[0], "#b48ead");
    designer.hex_buf = "not-a-colour".to_string();
    assert!(!designer.apply_hex());
    assert_eq!(designer.candidate.colors[0], "#b48ead");
}

#[test]
fn recent_dedupe_cap() {
    let mut designer = designer();
    for i in 0..12 {
        designer.pick_recent(&format!("#0000{i:02x}"));
        designer.push_recent();
    }
    assert_eq!(designer.recent.len(), 8);
    assert_eq!(designer.recent[0], "#000004");
    assert_eq!(designer.recent[7], "#00000b");
    designer.pick_recent("#000009");
    designer.push_recent();
    assert_eq!(designer.recent[7], "#000009");
    assert_eq!(designer.recent.iter().filter(|hex| **hex == "#000009").count(), 1);
}

#[test]
fn reset_and_dirty() {
    let mut designer = designer();
    assert!(!designer.dirty());
    designer.select_role(2);
    designer.set_hue(30.0);
    designer.name_buf = "renamed".to_string();
    assert!(designer.dirty());
    designer.reset();
    assert!(!designer.dirty());
    assert_eq!(designer.candidate, Candidate::from_preset("nord").unwrap());
    assert_eq!(designer.name_buf, "");
    assert_eq!(designer.hex_buf, designer.candidate.colors[2]);
}

#[test]
fn start_from_preset() {
    let mut designer = designer();
    designer.select_role(5);
    designer.start_from(Candidate::from_preset("dracula").unwrap());
    assert_eq!(designer.selected, 5);
    assert_eq!(designer.hex_buf, designer.candidate.colors[5]);
}

#[test]
fn preset_cleared_by_edits() {
    let nord = Candidate::from_preset("nord").unwrap();
    let mut designer = ThemeDesigner::new_from_preset(nord.clone(), String::from("nord"));

    assert_eq!(designer.selected_preset(), Some("nord"));
    designer.set_hue(120.0);
    assert_eq!(designer.selected_preset(), None);

    designer.reset();
    assert_eq!(designer.candidate, nord);
    assert_eq!(designer.selected_preset(), Some("nord"));
}

#[test]
fn preset_detaches_saved_name() {
    let mut designer = designer();
    designer.load_saved(String::from("Mine"), Candidate::from_preset("dracula").unwrap());

    designer.start_from_preset(String::from("nord"), Candidate::from_preset("nord").unwrap());

    assert_eq!(designer.name_buf, "");
    assert_eq!(designer.selected_preset(), Some("nord"));
}

#[test]
fn derive_keeps_draft_name() {
    let mut designer = designer();
    designer.load_saved(String::from("Mine"), Candidate::from_preset("dracula").unwrap());
    designer.set_name(String::from("Mine copy"));

    designer.start_from(Candidate::from_seed("#89b4fa", true).unwrap());

    assert_eq!(designer.name_buf, "Mine copy");
    assert_eq!(designer.selected_preset(), None);
}

#[test]
fn saved_name_detached_on_restart() {
    let mut designer = designer();
    designer.set_name(String::from("Draft"));
    designer.mark_saved(String::from("Draft"));

    designer.start_from_preset(String::from("dracula"), Candidate::from_preset("dracula").unwrap());

    assert_eq!(designer.name_buf, "");
    assert_eq!(designer.selected_preset(), Some("dracula"));
}

#[test]
fn save_becomes_reset_baseline() {
    let mut designer = designer();
    designer.set_hue(120.0);
    designer.set_name(String::from("Mine"));
    assert!(designer.dirty());

    designer.mark_saved(String::from("Mine"));
    let saved = designer.candidate.clone();
    assert!(!designer.dirty());

    designer.set_hue(240.0);
    designer.set_name(String::from("Changed"));
    assert!(designer.dirty());
    designer.reset();

    assert_eq!(designer.candidate, saved);
    assert_eq!(designer.name_buf, "Mine");
    assert!(!designer.dirty());
}

#[test]
fn delete_needs_two_requests() {
    let mut designer = designer();

    assert!(!designer.request_delete(String::from("Mine")));
    assert_eq!(designer.armed_delete(), Some("Mine"));

    assert!(!designer.request_delete(String::from("Other")));
    assert_eq!(designer.armed_delete(), Some("Other"));

    assert!(designer.request_delete(String::from("Other")));
    assert_eq!(designer.armed_delete(), None);
}

#[test]
fn designer_uses_folio_sheet() {
    let source = include_str!("../view.rs");
    assert!(source.contains("crate::frontend::ui::folio_sheet("));
    assert!(source.contains("crate::frontend::ui::folio_index_shell("));
    assert!(source.contains("self.ease()"));
    for retired in ["DesignerBlueprint", "scrim_style(", "iced::Shadow"] {
        assert!(!source.contains(retired), "{retired} came back");
    }
}

#[test]
fn entrance_settles() {
    let mut designer = designer();
    assert!(designer.ease() < 0.01);
    assert!(designer.animating());
    for _ in 0..240 {
        designer.tick(1.0 / 60.0);
    }
    assert!(!designer.animating());
    assert!((designer.ease() - 1.0).abs() < 0.001);
}
