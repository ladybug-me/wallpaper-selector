use super::help_intent_message;
use crate::app::Message;
use crate::frontend::ui::HelpIntent;

#[test]
fn help_intents_translate_at_the_app_boundary() {
    assert!(matches!(help_intent_message(HelpIntent::Close), Message::ToggleHelp));
    assert!(matches!(help_intent_message(HelpIntent::Capture), Message::Noop));
}
