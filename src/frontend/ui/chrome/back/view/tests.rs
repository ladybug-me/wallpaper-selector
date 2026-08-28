use super::*;

#[test]
fn flip_phases_stagger() {
    let panel = BackPanel {
        coordinated_flip: true,
        embedded: true,
        animate_flip_shader: true,
        animate_flip_back: true,
        progress: 0.78,
        ..BackPanel::default()
    };
    let phases =
        card_flip_phases(panel.progress, panel.animate_flip_shader, panel.animate_flip_back);
    assert!(phases.shader > 0.9 && phases.shader < 1.0);
    assert!(phases.surface > 0.0);
    assert_eq!(phases.content, 0.0);
}

#[test]
fn masthead_span_positive_skew() {
    let panel = BackPanel {
        embedded: true,
        cx: 500.0,
        cy: 300.0,
        hw: 400.0,
        hh: 250.0,
        skew: 80.0,
        ..BackPanel::default()
    };
    let layout = back_layout(&panel);
    let y = layout.masthead.1 + layout.masthead.3;
    let (left, right) =
        super::super::background::embedded_section_span(&panel, &layout, layout.masthead, y);

    assert!(left > layout.masthead.0);
    assert!(right < layout.masthead.0 + layout.masthead.2);
    assert!(((right - left) - (layout.masthead.2 - panel.skew.abs())).abs() < 0.01);
}

#[test]
fn masthead_span_negative_skew() {
    let mut panel = BackPanel {
        embedded: true,
        cx: 500.0,
        cy: 300.0,
        hw: 360.0,
        hh: 250.0,
        skew: -64.0,
        ..BackPanel::default()
    };
    let narrow = back_layout(&panel);
    let narrow_y = narrow.masthead.1 + narrow.masthead.3;
    let narrow_span =
        super::super::background::embedded_section_span(&panel, &narrow, narrow.masthead, narrow_y);

    panel.hw = 440.0;
    let wide = back_layout(&panel);
    let wide_y = wide.masthead.1 + wide.masthead.3;
    let wide_span =
        super::super::background::embedded_section_span(&panel, &wide, wide.masthead, wide_y);

    assert!(narrow_span.0 > narrow.masthead.0);
    assert!(narrow_span.1 < narrow.masthead.0 + narrow.masthead.2);
    assert!((wide_span.1 - wide_span.0) > (narrow_span.1 - narrow_span.0));
    assert!(((wide_span.1 - wide_span.0) - (wide.masthead.2 - panel.skew.abs())).abs() < 0.01);
}
