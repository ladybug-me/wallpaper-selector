pub(crate) fn tag_input_id() -> iced::widget::Id {
    iced::widget::Id::new("back-tag-input")
}

pub(crate) fn tag_query_id() -> iced::widget::Id {
    iced::widget::Id::new("tag-cloud-search")
}

pub(crate) fn mass_tag_input_id() -> iced::widget::Id {
    iced::widget::Id::new("mass-tag-input")
}

pub(crate) fn sync_library_search(
    tags: &[String],
    numeric: &crate::domain::library::search::NumericQuery,
) -> String {
    let mut terms = tags.to_vec();
    let numeric = numeric.query_text();
    if !numeric.is_empty() {
        terms.push(numeric);
    }
    if terms.is_empty() { String::new() } else { format!("{} ", terms.join(" ")) }
}

pub(crate) fn tag_search_partial(query: &str) -> &str {
    if query.chars().last().is_some_and(char::is_whitespace) {
        return "";
    }
    let partial = query.split_whitespace().next_back().unwrap_or("").trim_start_matches('-');
    if partial.contains(':') || partial == "|" { "" } else { partial }
}
