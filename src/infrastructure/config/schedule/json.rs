use serde_json::{Value, json};

use crate::domain::schedule::{
    ConditionKind, ConditionNode, DayNight, GroupOperator, RuleRow, block_requires_v2,
    format_block, order_rows, parse_block, parse_block_version, priority_for_position,
};

fn decode_node(
    value: &Value,
    version: u64,
    depth: usize,
    remaining: &mut usize,
) -> Option<ConditionNode> {
    if depth > 32 || *remaining == 0 {
        return None;
    }
    *remaining -= 1;
    let negated = match value.get("negated") {
        None | Some(Value::Bool(false)) => false,
        Some(Value::Bool(true)) => true,
        Some(_) => return None,
    };
    let kind = match value.get("kind").and_then(Value::as_str)? {
        "predicate" => {
            ConditionKind::Predicate(parse_block_version(value.get("value")?.as_str()?, version))
        }
        "group" => {
            let operator = match value.get("operator").and_then(Value::as_str)? {
                "all" => GroupOperator::All,
                "any" => GroupOperator::Any,
                _ => return None,
            };
            let children = value
                .get("children")?
                .as_array()?
                .iter()
                .map(|child| decode_node(child, version, depth + 1, remaining))
                .collect::<Option<Vec<_>>>()?;
            ConditionKind::Group { operator, children }
        }
        _ => return None,
    };
    Some(ConditionNode { negated, kind })
}

fn decode_condition(rule: &Value) -> ConditionNode {
    let Some(value) = rule.get("condition") else {
        return ConditionNode::all(vec![parse_block("condition:missing")]);
    };
    let Some(version @ (1 | 2)) = value.get("version").and_then(Value::as_u64) else {
        return ConditionNode::all(vec![parse_block("condition:version")]);
    };
    let mut remaining = 256;
    if value.get("root").and_then(|root| root.get("kind")).and_then(Value::as_str) == Some("group")
        && let Some(root) =
            value.get("root").and_then(|root| decode_node(root, version, 0, &mut remaining))
    {
        return root;
    }
    ConditionNode::all(vec![parse_block("condition:v1")])
}

fn node_requires_v2(node: &ConditionNode) -> bool {
    match &node.kind {
        ConditionKind::Predicate(block) => block_requires_v2(block),
        ConditionKind::Group { children, .. } => children.iter().any(node_requires_v2),
    }
}

fn encode_node(node: &ConditionNode) -> Value {
    match &node.kind {
        ConditionKind::Predicate(block) => json!({
            "kind": "predicate",
            "value": format_block(block),
            "negated": node.negated,
        }),
        ConditionKind::Group { operator, children } => json!({
            "kind": "group",
            "operator": match operator { GroupOperator::All => "all", GroupOperator::Any => "any" },
            "negated": node.negated,
            "children": children.iter().map(encode_node).collect::<Vec<_>>(),
        }),
    }
}

pub fn decode_schedule_rows(rules: &[Value]) -> Vec<RuleRow> {
    let rows = rules
        .iter()
        .enumerate()
        .map(|(source_index, rule)| {
            let priority = skwd_config::i64_ref(rule, "priority", 100);
            let row = RuleRow {
                name: skwd_config::str_ref(rule, "name", "").to_string(),
                enabled: rule.get("enabled").and_then(Value::as_bool) != Some(false),
                condition: decode_condition(rule),
                set: skwd_config::str_ref(rule, "set", "").to_string(),
                mode: skwd_config::str_ref(rule, "mode", "").to_string(),
            };
            (priority, source_index, row)
        })
        .collect();
    order_rows(rows)
}

pub fn encode_schedule_rows(rows: &[RuleRow]) -> Vec<Value> {
    rows.iter()
        .enumerate()
        .map(|(index, row)| {
            let version = if node_requires_v2(&row.condition) { 2 } else { 1 };
            json!({
                "priority": priority_for_position(index),
                "name": row.name,
                "enabled": row.enabled,
                "condition": { "version": version, "root": encode_node(&row.condition) },
                "set": if row.set.is_empty() { "random" } else { row.set.as_str() },
                "mode": row.mode,
            })
        })
        .collect()
}

pub fn decode_day_night(config: &Value) -> DayNight {
    let text = |path| skwd_config::str_at(config, path, "");
    DayNight {
        solar: text(skwd_config::keys::schedule::TRIGGER) != "fixed",
        day_time: text(skwd_config::keys::schedule::DAY_TIME),
        night_time: text(skwd_config::keys::schedule::NIGHT_TIME),
        day_set: text(skwd_config::keys::schedule::DAY_SET),
        day_mode: text(skwd_config::keys::schedule::DAY_MODE),
        night_set: text(skwd_config::keys::schedule::NIGHT_SET),
        night_mode: text(skwd_config::keys::schedule::NIGHT_MODE),
    }
}
