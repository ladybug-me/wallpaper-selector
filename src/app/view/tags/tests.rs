use super::tag_intent_message;
use crate::app::Message;
use crate::frontend::tagcloud::{TagIntent, TagMsg};

#[test]
fn tag_cloud_intents_translate_at_the_app_boundary() {
    assert!(matches!(
        tag_intent_message(TagIntent::Update(TagMsg::Autocomplete)),
        Message::Tag(TagMsg::Autocomplete)
    ));
    assert!(matches!(tag_intent_message(TagIntent::Clear), Message::ClearTags));
    assert!(matches!(tag_intent_message(TagIntent::ToggleCloud), Message::OpenTagCloud));
}
