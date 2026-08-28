use super::{ARRAY_ACTIONS, remove_index};
use crate::frontend::settings::ActionId;

fn variant_name(id: ActionId) -> String {
    format!("{id:?}").split('(').next().unwrap_or_default().to_string()
}

#[test]
fn array_actions_cover_ids() {
    let mut covered: Vec<String> = ARRAY_ACTIONS
        .iter()
        .flat_map(|row| [variant_name(row.add), variant_name((row.remove)(0))])
        .collect();
    covered.sort_unstable();
    let mut declared: Vec<String> = include_str!("../../../frontend/settings/action.rs")
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("Add") || line.starts_with("Remove"))
        .map(|line| line.split(['(', ',']).next().unwrap_or_default().to_string())
        .collect();
    declared.sort_unstable();
    assert_eq!(declared, covered);
}

#[test]
fn array_rows_distinct_keys() {
    let mut keys: Vec<&str> = ARRAY_ACTIONS.iter().map(|row| row.key).collect();
    keys.sort_unstable();
    let unique = keys.len();
    keys.dedup();
    assert_eq!(keys.len(), unique);
    for row in ARRAY_ACTIONS {
        assert_eq!(remove_index((row.remove)(9)), Some(9));
        assert_eq!(remove_index(row.add), None);
        assert!(!row.add.is_destructive(), "{:?}", row.add);
    }
}

#[test]
fn destructive_actions_arm() {
    for id in [ActionId::ClearCache, ActionId::OptimizeImages] {
        assert!(id.is_destructive(), "{id:?}");
    }
    for id in [ActionId::RecomputeColors, ActionId::RefreshBackdrop, ActionId::ResetMotionSlow] {
        assert!(!id.is_destructive(), "{id:?}");
    }
}
