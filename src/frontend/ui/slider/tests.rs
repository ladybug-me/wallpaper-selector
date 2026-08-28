#![cfg(test)]

#[test]
fn slider_value_map() {
    let map = |x| super::slider_value(0.0, 100.0, 1.0, x, 120.0);
    assert_eq!(map(0.0), 0.0);
    assert_eq!(map(120.0), 100.0);
    assert_eq!(map(60.0), 50.0);
    assert_eq!(map(-50.0), 0.0);
    assert_eq!(map(500.0), 100.0);
    assert_eq!(super::slider_value(0.0, 10.0, 2.0, 60.0, 120.0), 6.0);
}

#[test]
fn marker_snap_raster() {
    for step in 0..400 {
        let raw = 7.0 + step as f32 * 0.173;
        let snapped = super::marker_snap(raw);
        assert_eq!(snapped * 2.0, (snapped * 2.0).round());
        assert!((snapped - raw).abs() <= 0.25 + 1e-4, "snap drift");
    }
}

#[test]
fn slider_fraction_clamps() {
    assert_eq!(super::slider_fraction(0.0, 100.0, 50.0), 0.5);
    assert_eq!(super::slider_fraction(0.0, 100.0, -1.0), 0.0);
    assert_eq!(super::slider_fraction(0.0, 100.0, 101.0), 1.0);
    assert_eq!(super::slider_fraction(4.0, 4.0, 4.0), 0.0);
    assert_eq!(super::slider_fraction(5.0, 4.0, 4.5), 0.0);
}
