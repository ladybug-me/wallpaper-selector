#![cfg(test)]

use crate::app::tests::{browser_item, test_app};
use crate::frontend::browser::{Browser, Source};
use crate::frontend::scene::layout::GridParams;

#[test]
fn viewport_fit_snaps() {
    let mut app = test_app();
    let wall = &mut app.source_browser.wall;
    wall.set_layout(GridParams {
        cols: 4,
        rows: 4,
        thumb_w: 100.0,
        thumb_h: 60.0,
        gap_x: 8.0,
        gap_y: 8.0,
        ..GridParams::default()
    });

    wall.set_viewport(212.0, 264.0);

    assert!((wall.scene.gp.thumb_w - 50.0).abs() < 0.001);
    assert!((wall.scene.gp.thumb_h - 30.0).abs() < 0.001);
    assert!((wall.scene.gp.gap_x - 4.0).abs() < 0.001);
}

#[test]
fn begin_session_keeps_atlas() {
    let mut app = test_app();
    let mut browser = Browser::new(Source::Wallhaven);
    browser.session.items.push(browser_item("w1"));

    let wall = &mut app.source_browser.wall;
    wall.rebuild_catalogue(&browser, true);
    assert_eq!(wall.catalog.items.len(), 1);
    assert_eq!(wall.filtered, vec![0]);
    {
        let atlas = wall.atlas.as_mut().expect("remote atlas");
        atlas.near.acquire(0);
        atlas.near.mark_ready(0);
    }

    wall.begin_session();

    let atlas = wall.atlas.as_ref().expect("remote atlas");
    assert!(atlas.near.ready(0).is_some());
    wall.rebuild_catalogue(&browser, true);
    assert_eq!(wall.catalog.items.len(), 1);
    assert_eq!(wall.filtered, vec![0]);
}

#[test]
fn source_tabs_restore_each_browser_session() {
    let mut app = test_app();
    let mut wallhaven = Browser::new(Source::Wallhaven);
    wallhaven.request.query = String::from("forest");
    wallhaven.session.items.push(browser_item("wall"));
    app.source_browser.browser = Some(wallhaven);

    assert!(app.source_browser.activate(Source::Steam));
    app.source_browser.browser.as_mut().unwrap().request.query = String::from("rain");
    app.source_browser.browser.as_mut().unwrap().session.items.push(browser_item("steam"));

    assert!(app.source_browser.activate(Source::Wallhaven));
    let active = app.source_browser.browser.as_ref().unwrap();
    assert_eq!(active.request.query, "forest");
    assert_eq!(active.session.items[0].id, "wall");
    let steam = app.source_browser.tabs.get(&Source::Steam).unwrap();
    assert_eq!(steam.request.query, "rain");
    assert_eq!(steam.session.items[0].id, "steam");
}
