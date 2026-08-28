use serde_json::json;

use crate::domain::schedule::seed_day_night;
use crate::infrastructure::config::schedule::{
    decode_day_night, decode_schedule_rows, encode_schedule_rows,
};

#[allow(clippy::wildcard_imports)]
use super::super::*;

pub(crate) fn sched_open(app: &mut App) {
    let mut rows =
        decode_schedule_rows(&app.config.array_values(skwd_config::keys::schedule::RULES));
    let migrated = app.config.flag_default_config(skwd_config::keys::schedule::MIGRATED);
    let enabled = app.config.flag_default_config(skwd_config::keys::schedule::ENABLED);
    if !migrated && enabled {
        let day_night = decode_day_night(app.config.root());
        rows.extend(seed_day_night(&day_night));
    }
    let mut editor =
        crate::frontend::schedule_editor::ScheduleEditor::new(rows, !migrated, enabled);
    editor.set_motion_profile(app.motion_profile());
    app.panels.schedule = Some(editor);
    app.retick();
}

pub(crate) fn sched_persist(app: &mut App) {
    let Some(editor) = &app.panels.schedule else {
        return;
    };
    if editor.demo {
        return;
    }
    let rules = encode_schedule_rows(&editor.rows);
    app.config.set_key(skwd_config::keys::schedule::RULES, serde_json::Value::Array(rules));
    app.config.set_key(skwd_config::keys::schedule::ENABLED, json!(editor.enabled));
    if editor.migrated {
        app.config.set_key(skwd_config::keys::schedule::MIGRATED, json!(true));
    }
    app.config.persist();
    app.daemon.client.call("schedule.reload", json!({}));
}
