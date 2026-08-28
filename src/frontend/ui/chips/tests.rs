#![cfg(test)]

#[test]
fn chip_width_grows() {
    let short = super::chip_width("Hi", 12.0, 26.0);
    let long = super::chip_width("Much longer label", 12.0, 26.0);
    assert!(long > short);
    assert!(short >= 16.0);
}

#[test]
fn chip_skew_cap() {
    assert_eq!(super::chip_skew(34.0), 10.0);
    assert_eq!(super::chip_skew(46.0), 10.0);
    assert_eq!(super::chip_skew(20.0), 8.0);
    assert!(super::chip_skew(10.0) < 10.0);
}

#[test]
fn chip_hit_edges() {
    let hit = |px, py| super::chip_contains(0.0, 0.0, 100.0, 30.0, 10.0, px, py);
    assert!(!hit(9.0, 0.0));
    assert!(hit(11.0, 0.0));
    assert!(hit(99.0, 0.0));
    assert!(hit(1.0, 30.0));
    assert!(!hit(91.0, 30.0));
    assert!(hit(89.0, 30.0));
    assert!(!hit(50.0, 30.5));
    assert!(!hit(50.0, -0.5));
}

#[test]
fn chips_layout_scroll() {
    let pal = crate::frontend::theme::Palette::default();
    let entries: Vec<crate::domain::library::search::TagEntry> = (0..50)
        .map(|i| crate::domain::library::search::TagEntry {
            tag: format!("tag-number-{i}"),
            count: i + 1,
            selected: false,
            excluded: false,
        })
        .collect();
    let chips = super::TagChips {
        entries: std::rc::Rc::new(entries),
        pal: &pal,
        width: 300.0,
        max_h: 100.0,
        scale: 1.0,
        entrance: 1.0,
        scroll: f32::MAX,
        scroll_target: f32::MAX,
    };
    let (rects, total) = chips.layout();
    assert_eq!(rects.len(), 50);
    assert!(total > chips.max_h);
    assert_eq!(chips.eff_scroll(total), total - chips.max_h);
    let short = super::TagChips { scroll: 40.0, ..chips };
    assert_eq!(short.eff_scroll(50.0), 0.0);
}

#[test]
fn tag_cloud_body_cap() {
    let entries: Vec<crate::domain::library::search::TagEntry> = (0..20)
        .map(|i| crate::domain::library::search::TagEntry {
            tag: format!("tag-{i}"),
            count: i + 1,
            selected: false,
            excluded: false,
        })
        .collect();
    assert!(super::tag_cloud_row_count(&entries, 260.0, 1.0) > 3);
    assert_eq!(super::tag_cloud_body_height(1, 1.0), 27.0);
    assert_eq!(super::tag_cloud_body_height(2, 1.0), 60.0);
    assert_eq!(super::tag_cloud_body_height(3, 1.0), 93.0);
    assert_eq!(super::tag_cloud_body_height(20, 1.0), 93.0);
}

#[test]
fn tag_chip_click_emits_consumer_intent_and_captures() {
    let pal = crate::frontend::theme::Palette::default();
    let chips = super::TagChips {
        entries: std::rc::Rc::new(vec![crate::domain::library::search::TagEntry {
            tag: "nature".into(),
            count: 1,
            selected: false,
            excluded: false,
        }]),
        pal: &pal,
        width: 300.0,
        max_h: 100.0,
        scale: 1.0,
        entrance: 1.0,
        scroll: 0.0,
        scroll_target: 0.0,
    };
    let event = iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left));
    let cursor = iced::mouse::Cursor::Available(iced::Point::new(10.0, 10.0));
    let action = iced::widget::canvas::Program::update(
        &chips,
        &mut (),
        &event,
        iced::Rectangle::new(iced::Point::ORIGIN, iced::Size::new(300.0, 100.0)),
        cursor,
    )
    .expect("tag click");
    let (message, _, status) = action.into_inner();
    assert!(matches!(
        message,
        Some(crate::frontend::tagcloud::TagIntent::Update(
            crate::frontend::tagcloud::TagMsg::CloudClick(tag, false)
        )) if tag == "nature"
    ));
    assert_eq!(status, iced::event::Status::Captured);
}
