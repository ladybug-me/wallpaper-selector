use super::*;
use crate::app::overlay::Overlay;

fn scene_wall() -> Value {
    let mut scene = wall("Cool Scene", "we", 5, 1);
    scene["we_id"] = json!("2057951800");
    scene
}

fn rows() -> Value {
    json!({
        "we_id": "2057951800",
        "properties": [
            {"name": "glow", "label": "Glow", "kind": "bool", "value": true, "default": true,
             "order": 1},
            {"name": "zoom", "label": "Zoom", "kind": "slider", "value": 1.0, "default": 1.0,
             "min": 0.5, "max": 3.0, "step": 0.01, "order": 2},
            {"name": "tint", "label": "Tint", "kind": "color", "value": "1 1 1",
             "default": "1 1 1", "order": 3}
        ]
    })
}

fn focus(app: &mut App, name: &str) {
    let index = app
        .library_session
        .filtered
        .iter()
        .position(|&store| app.library_session.library.catalog().items[store as usize].name == name)
        .expect("seeded wallpaper visible");
    app.scene.set_current(index, app.library_session.filtered.len());
}

fn open_on_scene(app: &mut App) -> Vec<(String, Value)> {
    seed(app, &[scene_wall(), wall("a.png", "static", 1, 0)]);
    focus(app, "Cool Scene");
    let _ = update(app, Message::OpenSceneProps);
    drain_calls(app)
}

#[test]
fn scene_opens_static_does_not() {
    let mut app = test_app();
    let calls = open_on_scene(&mut app);
    assert!(app.panels.scene_properties.is_some());
    assert_eq!(
        calls.iter().filter(|(method, _)| method == wall_proto::rpc::WALL_WE_PROPERTIES).count(),
        1
    );
    let (_, params) = calls
        .iter()
        .find(|(method, _)| method == wall_proto::rpc::WALL_WE_PROPERTIES)
        .expect("properties request");
    assert_eq!(params.get("we_id").and_then(Value::as_str), Some("2057951800"));

    app.panels.scene_properties = None;
    focus(&mut app, "a.png");
    let _ = update(&mut app, Message::OpenSceneProps);
    assert!(app.panels.scene_properties.is_none());
    assert!(drain_calls(&app).is_empty());
}

#[test]
fn reply_populates_edits_write_back() {
    let mut app = test_app();
    open_on_scene(&mut app);
    let id = app.daemon.pending.keys().copied().next().expect("tracked request");
    respond(&mut app, id, rows());

    let panel = app.panels.scene_properties.as_ref().expect("panel stays open");
    assert!(!panel.loading);
    assert_eq!(panel.editable_count(), 3);
    assert_eq!(panel.overridden_count(), 0);
    drain_calls(&app);

    let _ = update(
        &mut app,
        Message::SceneProps(crate::frontend::scene_properties::ScenePropMsg::Toggle("glow".into())),
    );
    let calls = drain_calls(&app);
    let (_, params) = calls
        .iter()
        .find(|(method, _)| method == wall_proto::rpc::WALL_SET_WE_PROPERTY)
        .expect("toggle writes");
    assert_eq!(params.get("name").and_then(Value::as_str), Some("glow"));
    assert_eq!(params.get("value"), Some(&json!(false)));
    assert_eq!(app.panels.scene_properties.as_ref().unwrap().overridden_count(), 1);
}

#[test]
fn slider_drag_writes_on_release() {
    let mut app = test_app();
    open_on_scene(&mut app);
    let id = app.daemon.pending.keys().copied().next().expect("tracked request");
    respond(&mut app, id, rows());
    drain_calls(&app);

    for step in [1.5_f64, 2.0, 2.5] {
        let _ = update(
            &mut app,
            Message::SceneProps(crate::frontend::scene_properties::ScenePropMsg::Slide(
                "zoom".into(),
                step,
            )),
        );
    }
    assert!(drain_calls(&app).is_empty());

    let _ = update(
        &mut app,
        Message::SceneProps(crate::frontend::scene_properties::ScenePropMsg::Commit("zoom".into())),
    );
    let calls = drain_calls(&app);
    let (_, params) = calls
        .iter()
        .find(|(method, _)| method == wall_proto::rpc::WALL_SET_WE_PROPERTY)
        .expect("release writes once");
    assert_eq!(params.get("value"), Some(&json!(2.5)));
}

#[test]
fn colour_commits_when_parsed() {
    let mut app = test_app();
    open_on_scene(&mut app);
    let id = app.daemon.pending.keys().copied().next().expect("tracked request");
    respond(&mut app, id, rows());
    drain_calls(&app);

    let colour_input = |text: &str| {
        Message::SceneProps(crate::frontend::scene_properties::ScenePropMsg::ColourInput(
            "tint".into(),
            text.into(),
        ))
    };
    let commit = Message::SceneProps(
        crate::frontend::scene_properties::ScenePropMsg::ColourCommit("tint".into()),
    );

    let _ = update(&mut app, colour_input("0.5 0"));
    let _ = update(&mut app, commit.clone());
    assert!(drain_calls(&app).is_empty());

    let _ = update(&mut app, colour_input("0.25 0.5 0.75"));
    let _ = update(&mut app, commit);
    let calls = drain_calls(&app);
    let (_, params) = calls
        .iter()
        .find(|(method, _)| method == wall_proto::rpc::WALL_SET_WE_PROPERTY)
        .expect("colour writes");
    assert_eq!(params.get("value"), Some(&json!("0.250 0.500 0.750")));
}

#[test]
fn reset_clears_escape_closes() {
    let mut app = test_app();
    open_on_scene(&mut app);
    let id = app.daemon.pending.keys().copied().next().expect("tracked request");
    respond(&mut app, id, rows());
    drain_calls(&app);

    let _ = update(
        &mut app,
        Message::SceneProps(crate::frontend::scene_properties::ScenePropMsg::Reset),
    );
    let calls = drain_calls(&app);
    let (_, params) = calls
        .iter()
        .find(|(method, _)| method == wall_proto::rpc::WALL_SET_WE_PROPERTY)
        .expect("reset writes");
    assert_eq!(params.get("reset").and_then(Value::as_bool), Some(true));
    assert!(params.get("name").is_none());

    assert_eq!(app.topmost_overlay(), Some(Overlay::SceneProperties));
    assert!(app.close_topmost_overlay());
    assert!(app.panels.scene_properties.is_none());
}

#[test]
fn failed_request_surfaces_error() {
    let mut app = test_app();
    open_on_scene(&mut app);
    let id = app.daemon.pending.keys().copied().next().expect("tracked request");
    reject(&mut app, id, "no such item");
    let panel = app.panels.scene_properties.as_ref().expect("panel stays open");
    assert!(!panel.loading);
    assert!(panel.error.as_deref().is_some_and(|error| error.contains("no such item")));
}
