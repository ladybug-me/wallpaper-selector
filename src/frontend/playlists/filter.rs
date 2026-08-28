pub fn source_colors(source: &str) -> Vec<i64> {
    source
        .split_whitespace()
        .find_map(|clause| {
            let rest = clause.strip_prefix("color:").or_else(|| clause.strip_prefix("colour:"))?;
            Some(
                rest.split(',')
                    .filter_map(|token| crate::frontend::ui::parse_color_bucket(token.trim()))
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap_or_default()
}

pub fn toggle_color_clause(source: &str, bucket: i64) -> String {
    let mut clauses: Vec<String> = source.split_whitespace().map(String::from).collect();
    let current = clauses
        .iter()
        .position(|clause| clause.starts_with("color:") || clause.starts_with("colour:"));
    let mut buckets: Vec<i64> = current
        .and_then(|index| clauses[index].split_once(':').map(|(_, value)| value))
        .map(|value| {
            value
                .split(',')
                .filter_map(|token| crate::frontend::ui::parse_color_bucket(token.trim()))
                .collect()
        })
        .unwrap_or_default();
    if let Some(position) = buckets.iter().position(|current| *current == bucket) {
        buckets.remove(position);
    } else {
        buckets.push(bucket);
    }
    if let Some(index) = current {
        clauses.remove(index);
    }
    if !buckets.is_empty() {
        let names: Vec<&str> =
            buckets.iter().map(|value| crate::frontend::ui::color_bucket_name(*value)).collect();
        clauses.push(format!("color:{}", names.join(",")));
    }
    clauses.join(" ")
}
