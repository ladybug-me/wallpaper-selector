#![cfg(test)]

use super::preview_candidates;

#[test]
fn candidates_hover_or_engagement() {
    assert_eq!(preview_candidates(None, 5, false).count(), 0);
    assert_eq!(preview_candidates(None, 5, true).collect::<Vec<_>>(), vec![5]);
    assert_eq!(preview_candidates(Some(2), 5, false).collect::<Vec<_>>(), vec![2]);
    assert_eq!(preview_candidates(Some(2), 5, true).collect::<Vec<_>>(), vec![2, 5]);
}
