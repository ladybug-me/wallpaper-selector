use super::*;

#[test]
fn playlist_name_persists() {
    let mut app = test_app();
    app.open_card_picker(String::from("wallpaper-key"), String::from("Wallpaper"));
    let _ = drain_calls(&app);

    let _ =
        update(&mut app, Message::CardPicker(crate::frontend::playlists::CardPickerMsg::NewSubmit));

    let calls = drain_calls(&app);
    assert!(
        calls.iter().any(|(method, params)| {
            method == "playlist.create" && params["name"] == "Playlist 1"
        })
    );
    assert_eq!(app.panels.card_picker.as_ref().unwrap().new_buf, "");
}
