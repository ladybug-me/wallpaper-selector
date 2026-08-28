#![cfg(test)]

use super::{browser_intent_message, panel_layers};
use crate::app::tests::test_app;
use crate::app::{Message, update};

#[test]
fn designer_replaces_settings_sheet() {
    let mut app = test_app();
    app.scene.viewport = (1280.0, 720.0);
    let _ = update(&mut app, Message::ToggleSettings);
    assert!(app.panels.settings.open);
    let settings_only = panel_layers(&app).len();
    assert_eq!(settings_only, 1);

    crate::app::helpers::theme_designer_open(&mut app);
    assert!(app.panels.theme_designer.is_some());
    assert!(app.panels.settings.open);
    assert_eq!(panel_layers(&app).len(), settings_only);
}

#[test]
fn browser_intents_translate_at_the_app_boundary() {
    use crate::frontend::browser::{BrowserIntent, BrowserMsg};

    assert!(matches!(
        browser_intent_message(BrowserIntent::Update(BrowserMsg::SearchSubmit)),
        Message::Browser(BrowserMsg::SearchSubmit)
    ));
    assert!(matches!(browser_intent_message(BrowserIntent::Close), Message::CloseBrowser));
    assert!(matches!(browser_intent_message(BrowserIntent::Capture), Message::Noop));
}
