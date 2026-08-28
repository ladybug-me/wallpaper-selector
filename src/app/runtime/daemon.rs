use serde_json::{Value, json};

#[allow(clippy::wildcard_imports)]
use super::super::*;

impl App {
    pub(in crate::app) fn refresh_playlists(&mut self) {
        self.call_tracked("playlist.list", json!({}), Pending::PlList);
    }

    pub(in crate::app) fn refresh_playlist_members(&mut self, id: i64) {
        self.call_tracked("playlist.members", json!({ "id": id }), Pending::PlMembers);
    }

    pub(in crate::app) fn open_card_picker(&mut self, key: String, name: String) {
        self.call_tracked("playlist.list", json!({}), Pending::CardPickerList);
        self.call_tracked(
            "playlist.memberships",
            json!({ "key": key }),
            Pending::CardPickerMembers,
        );
        self.panels.card_picker =
            Some(crate::frontend::playlists::CardPicker { key, name, ..Default::default() });
    }

    pub(in crate::app) fn refresh_card_memberships(&mut self) {
        let Some(key) = self.panels.card_picker.as_ref().map(|picker| picker.key.clone()) else {
            return;
        };
        self.call_tracked(
            "playlist.memberships",
            json!({ "key": key }),
            Pending::CardPickerMembers,
        );
    }

    pub(in crate::app) fn playlist_set(&mut self, id: i64, field: &str, value: &str) {
        let mut map = serde_json::Map::new();
        map.insert("id".to_string(), json!(id));
        if field == "dwell" {
            map.insert(
                "dwell".to_string(),
                json!(value.trim().parse::<i64>().unwrap_or(600).max(1)),
            );
        } else {
            map.insert(field.to_string(), json!(value));
        }
        self.daemon.client.call("playlist.update", Value::Object(map));
        self.refresh_playlists();
        if self.panels.playlists.as_ref().and_then(|playlist| playlist.selected) == Some(id) {
            self.refresh_playlist_members(id);
        }
    }
}
