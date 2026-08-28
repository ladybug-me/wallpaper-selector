use serde_json::json;

#[allow(clippy::wildcard_imports)]
use super::super::*;

pub(crate) fn remove_tag_at(app: &mut App, key: &str, tag_index: usize) {
    let Some(update) = app.library_session.library.remove_tag_at(key, tag_index) else {
        return;
    };
    if app.tags.editing || app.tags.card_drawer_open {
        app.tags.card_locked.clone_from(&update.tags);
    }
    if let Some(panel) = std::sync::Arc::make_mut(&mut app.scene.render).back.as_mut() {
        panel.tags.clone_from(&update.tags);
    }
    app.daemon
        .client
        .call("wall.update_tags", json!({ "key": update.key, "tags": update.tags.join(",") }));
    app.retick();
}

fn suggest(app: &App, query: &str, exclude: &[String]) -> Vec<(String, usize)> {
    if query.is_empty() {
        return Vec::new();
    }
    let mut matches: Vec<String> = app
        .library_session
        .library
        .catalog()
        .tag_vocab
        .iter()
        .filter(|tag| tag.to_lowercase().starts_with(query))
        .filter(|tag| !exclude.iter().any(|excluded| excluded.eq_ignore_ascii_case(tag)))
        .cloned()
        .collect();
    matches.sort();
    matches.truncate(6);
    matches
        .into_iter()
        .map(|tag| {
            let count =
                app.library_session.library.catalog().tag_counts.get(&tag).copied().unwrap_or(0);
            (tag, count)
        })
        .collect()
}

pub(crate) fn mass_tag_suggestions(app: &App) -> Vec<(String, usize)> {
    let query = app.tags.mass_input.trim().to_lowercase();
    suggest(app, &query, &app.tags.mass_tags)
}

pub(crate) fn ghost_completion(input: &str, suggestions: &[(String, usize)]) -> String {
    let partial = tag_search_partial(input);
    let typed = partial.chars().count();
    suggestions
        .first()
        .filter(|(tag, _)| tag.chars().count() > typed)
        .map(|(tag, _)| {
            let prefix_bytes = input.len().saturating_sub(partial.len());
            format!(
                "{}{}{}",
                &input[..prefix_bytes],
                partial,
                tag.chars().skip(typed).collect::<String>()
            )
        })
        .unwrap_or_default()
}

pub(crate) fn commit_mass_tags(app: &mut App) {
    if app.tags.mass_tags.is_empty() || app.tags.select.is_empty() {
        return;
    }
    let tags = app.tags.mass_tags.clone();
    let selected: Vec<u32> = app.tags.select.iter().copied().collect();
    for update in app.library_session.library.add_tags_to_indices(&selected, &tags) {
        app.daemon
            .client
            .call("wall.update_tags", json!({ "key": update.key, "tags": update.tags.join(",") }));
    }
    app.tags.select.clear();
    app.retick();
}

pub(crate) fn commit_pending_tag(app: &mut App) {
    sync_card_tag_input(app, true);
}

pub(crate) fn begin_card_tag_edit(app: &mut App, tags: &[String]) {
    app.tags.input.clear();
    app.tags.card_locked = tags.iter().map(|tag| tag.trim().to_lowercase()).collect();
    app.tags.card_key = app
        .scene
        .flipped()
        .and_then(|filtered_index| app.library_session.filtered.get(filtered_index))
        .and_then(|&source_index| {
            app.library_session.library.catalog().items.get(source_index as usize)
        })
        .map(|item| item.key.clone());
}

pub(crate) fn toggle_card_tag_drawer(app: &mut App) {
    if app.tags.card_drawer_open {
        app.tags.card_drawer_open = false;
        if !app.tags.editing {
            app.tags.card_key = None;
            app.tags.card_locked.clear();
            app.tags.input.clear();
        }
        app.retick();
        return;
    }

    let tags = app
        .scene
        .flipped()
        .and_then(|filtered_index| app.library_session.filtered.get(filtered_index))
        .and_then(|&source_index| {
            app.library_session.library.catalog().items.get(source_index as usize)
        })
        .and_then(|item| app.library_session.library.catalog().tags.get(&item.key))
        .cloned();
    if let Some(tags) = tags {
        begin_card_tag_edit(app, &tags);
        app.tags.card_drawer_open = true;
        app.retick();
    }
}

pub(crate) fn sync_card_tag_input(app: &mut App, commit_partial: bool) {
    let Some(key) = app.tags.card_key.clone() else { return };
    let has_separator = app.tags.input.chars().last().is_some_and(char::is_whitespace);
    let raw: Vec<String> = app
        .tags
        .input
        .split_whitespace()
        .map(|token| token.trim().to_lowercase())
        .filter(|token| !token.is_empty())
        .collect();
    let keep_partial = !commit_partial && !has_separator;
    let pending = keep_partial.then(|| raw.last().cloned()).flatten().unwrap_or_default();
    let commit_len = raw.len().saturating_sub(usize::from(keep_partial));
    let mut tags = app.tags.card_locked.clone();
    let old_len = tags.len();
    for token in raw.into_iter().take(commit_len) {
        if !tags.iter().any(|tag| tag.eq_ignore_ascii_case(&token)) {
            tags.push(token);
        }
    }
    app.tags.input = pending;
    app.tags.card_locked.clone_from(&tags);

    if let Some(update) = app.library_session.library.replace_tags(&key, tags) {
        if let Some(panel) = std::sync::Arc::make_mut(&mut app.scene.render).back.as_mut() {
            panel.tags.clone_from(&update.tags);
            if update.tags.len() > old_len {
                panel.pop_idx = (update.tags.len() - 1) as i32;
                panel.chip_pop = 0.0;
            }
        }
        if update.tags.len() > old_len {
            app.scene.pop_tag_chip(update.tags.len() - 1);
        }
        app.daemon
            .client
            .call("wall.update_tags", json!({ "key": update.key, "tags": update.tags.join(",") }));
        app.retick();
    }
    if commit_partial {
        app.tags.input.clear();
    }
}
