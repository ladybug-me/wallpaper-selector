#![cfg(test)]

use super::*;
use crate::domain::schedule::{ConditionKind, ConditionNode, GroupOperator};

fn row(name: &str, predicates: &str) -> RuleRow {
    RuleRow {
        name: name.to_string(),
        enabled: true,
        condition: ConditionNode::all(
            predicates.split_whitespace().map(crate::domain::schedule::parse_block).collect(),
        ),
        set: "random".to_string(),
        mode: String::new(),
    }
}

fn editor(names: &[&str]) -> ScheduleEditor {
    ScheduleEditor::new(names.iter().map(|name| row(name, "")).collect(), false, true)
}

fn order(ed: &ScheduleEditor) -> Vec<&str> {
    ed.rows.iter().map(|rule| rule.name.as_str()).collect()
}

#[test]
fn drag_reorder() {
    let mut ed = editor(&["a", "b", "c", "d"]);
    ed.drag_start(3);
    ed.drag_over(1);
    assert_eq!(order(&ed), vec!["a", "d", "b", "c"]);
    assert_eq!(ed.drag, Some(1));
    ed.drag_over(0);
    assert_eq!(order(&ed), vec!["d", "a", "b", "c"]);
    assert_eq!(ed.selected, 1);
    assert!(ed.drag_end());
    assert!(!ed.drag_end());
}

#[test]
fn drag_without_start() {
    let mut ed = editor(&["a", "b"]);
    ed.drag_over(0);
    assert_eq!(order(&ed), vec!["a", "b"]);
}

#[test]
fn add_rule() {
    let mut ed = editor(&["a"]);
    let idx = ed.add_rule();
    assert_eq!(idx, 1);
    assert_eq!(ed.rows[1].set, "random");
    assert_eq!(ed.selected, 1);
    assert_eq!(ed.foldout, Some(1));
    assert_eq!(ed.editing, Some(Editing::Add(1, Vec::new())));
    ed.remove_rule(1);
    assert_eq!(order(&ed), vec!["a"]);
    assert_eq!(ed.editing, None);
    assert_eq!(ed.selected, 0);
}

#[test]
fn rule_options_follow_rule() {
    let mut ed = editor(&["a", "b", "c"]);
    ed.toggle_rule_options(1);
    assert_eq!(ed.selected, 1);
    assert_eq!(ed.foldout, Some(1));
    assert_eq!(ed.foldout_visible, Some(1));
    assert_eq!(ed.foldout_reveal.target, 1.0);
    ed.toggle_rule_options(1);
    assert_eq!(ed.foldout, None);
    assert_eq!(ed.foldout_visible, Some(1));
    assert_eq!(ed.foldout_reveal.target, 0.0);
    ed.toggle_rule_options(2);
    ed.drag_start(2);
    ed.drag_over(0);
    assert_eq!(ed.selected, 0);
    assert_eq!(ed.foldout, Some(0));
    ed.remove_rule(0);
    assert_eq!(ed.foldout, None);
}

#[test]
fn rule_options_animate_unmount() {
    let mut ed = editor(&["a"]);
    ed.toggle_rule_options(0);
    ed.tick(0.04);
    assert!(ed.foldout_reveal.x > 0.0 && ed.foldout_reveal.x < 1.0);
    for _ in 0..10 {
        ed.tick(0.05);
    }
    assert_eq!(ed.foldout_reveal.x, 1.0);
    ed.toggle_rule_options(0);
    ed.tick(0.04);
    assert!(ed.foldout_reveal.x > 0.0 && ed.foldout_reveal.x < 1.0);
    for _ in 0..10 {
        ed.tick(0.05);
    }
    assert_eq!(ed.foldout_visible, None);
}

#[test]
fn rule_options_motion_profile() {
    let mut ed = editor(&["a"]);
    ed.set_motion_profile(MotionProfile::new(80.0, 360.0, 720.0));
    assert_eq!(ed.foldout_reveal.duration_ms(), 360.0);
}

#[test]
fn demo_editor_transient() {
    let ed = ScheduleEditor::demo(vec![row("temporary", "")]);
    assert!(ed.demo);
    assert!(ed.enabled);
    assert!(!ed.migrated);
}

#[test]
fn selection_removal_bounds() {
    let mut ed = editor(&["a", "b", "c"]);
    ed.select(2);
    assert_eq!(ed.selected, 2);
    ed.remove_rule(0);
    assert_eq!(ed.selected, 1);
    ed.remove_rule(1);
    assert_eq!(ed.selected, 0);
    ed.select(99);
    assert_eq!(ed.selected, 0);
}

#[test]
fn block_defaults_dsl() {
    let mut ed = editor(&["a"]);
    for (kind, _) in BLOCK_KINDS {
        ed.add_block(0, &[], kind);
    }
    let ConditionKind::Group { children, .. } = &ed.rows[0].condition.kind else {
        panic!("rule root must be a group");
    };
    assert_eq!(children.len(), BLOCK_KINDS.len());
    assert!(children.iter().all(|node| matches!(node.kind, ConditionKind::Predicate(_))));
}

#[test]
fn new_rule_context_blocks() {
    let mut ed = editor(&[]);
    let row = ed.add_rule();
    assert!(ed.rows[row].enabled);
    for kind in [BlockKind::Power, BlockKind::Battery, BlockKind::Output, BlockKind::OutputCount] {
        ed.add_block(row, &[], kind);
    }
    let ConditionKind::Group { children, .. } = &ed.rows[row].condition.kind else {
        panic!("rule root must be a group");
    };
    assert_eq!(children.len(), 4);
}

#[test]
fn toggle_days() {
    let mut ed = editor(&["a"]);
    ed.add_block(0, &[], BlockKind::Weekday);
    ed.toggle_in_list(0, &[0], "mon");
    assert_eq!(
        ed.block_mut(0, &[0]).cloned(),
        Some(Block::Weekday(vec!["mon".into(), "sat".into(), "sun".into()]))
    );
    ed.toggle_in_list(0, &[0], "sat");
    ed.toggle_in_list(0, &[0], "sun");
    ed.toggle_in_list(0, &[0], "mon");
    assert_eq!(ed.block_mut(0, &[0]).cloned(), Some(Block::Weekday(vec!["mon".into()])));
}

#[test]
fn toggle_editing() {
    let mut ed = editor(&["a"]);
    ed.toggle_editing(Editing::Add(0, Vec::new()));
    assert_eq!(ed.editing, Some(Editing::Add(0, Vec::new())));
    ed.toggle_editing(Editing::Add(0, Vec::new()));
    assert_eq!(ed.editing, None);
}

#[test]
fn nested_group_paths() {
    let mut ed = editor(&["a"]);
    ed.add_group(0, &[], GroupOperator::Any);
    ed.add_block(0, &[0], BlockKind::Weather);
    ed.add_group(0, &[0], GroupOperator::All);
    ed.add_block(0, &[0, 1], BlockKind::TimeAfter);
    ed.set_negated(0, &[0, 1], true);
    assert!(ed.node(0, &[0, 1]).is_some_and(|node| node.negated));
    ed.remove_node(0, &[0, 0]);
    let Some(ConditionNode { kind: ConditionKind::Group { children, .. }, .. }) = ed.node(0, &[0])
    else {
        panic!("nested group must remain");
    };
    assert_eq!(children.len(), 1);
}

#[test]
fn tree_limits() {
    let mut ed = editor(&["a"]);
    for _ in 0..300 {
        ed.add_block(0, &[], BlockKind::Date);
    }
    let ConditionKind::Group { children, .. } = &ed.rows[0].condition.kind else {
        panic!("rule root must be a group");
    };
    assert_eq!(children.len(), 255);

    let mut ed = editor(&["a"]);
    let mut path = Vec::new();
    for _ in 0..32 {
        ed.add_group(0, &path, GroupOperator::All);
        path.push(0);
    }
    ed.add_block(0, &path, BlockKind::Date);
    let Some(ConditionNode { kind: ConditionKind::Group { children, .. }, .. }) = ed.node(0, &path)
    else {
        panic!("deepest permitted group must exist");
    };
    assert!(children.is_empty());
}

#[test]
fn reset_condition_recovers() {
    let mut ed = editor(&["a"]);
    ed.rows[0].condition = ConditionNode::all(vec![Block::Raw("condition:missing".into())]);
    ed.reset_condition(0);
    assert_eq!(ed.rows[0].condition, ConditionNode::empty_all());
    assert_eq!(ed.editing, Some(Editing::Add(0, Vec::new())));
}
