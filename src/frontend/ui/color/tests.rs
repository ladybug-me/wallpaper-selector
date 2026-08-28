#![cfg(test)]

#[test]
fn strip_bucket_names() {
    for i in 0..=12usize {
        let bucket = super::strip_bucket(i);
        let name = super::color_bucket_name(bucket);
        assert_eq!(super::parse_color_bucket(name), Some(bucket));
    }
    assert_eq!(super::strip_bucket(12), 99);
}

#[test]
fn color_cycle_ring() {
    let mut seen = Vec::new();
    let mut cur = super::cycle_color_right(-1);
    for _ in 0..13 {
        seen.push(cur);
        cur = super::cycle_color_right(cur);
    }
    assert_eq!(seen, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 99]);
    assert_eq!(cur, 0);
    for v in [0, 1, 5, 11, 99] {
        assert_eq!(super::cycle_color_left(super::cycle_color_right(v)), v);
    }
    assert_eq!(super::cycle_color_left(-1), 99);
}
