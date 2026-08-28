#![cfg(test)]

use proptest::prelude::*;
use serde_json::{Value, json};

use super::{decode_day_night, decode_schedule_rows, encode_schedule_rows};
use crate::domain::schedule::{Block, ConditionKind, ConditionNode, GroupOperator, RuleRow};

fn condition(values: &[&str]) -> Value {
    json!({
        "version": 1,
        "root": {
            "kind": "group",
            "operator": "all",
            "children": values
                .iter()
                .map(|value| json!({"kind": "predicate", "value": value}))
                .collect::<Vec<_>>()
        }
    })
}

#[test]
fn rows_priority_order() {
    let rules = vec![
        json!({"priority": 100, "name": "Night", "condition": condition(&[]), "set": "static:n.png", "mode": "dark"}),
        json!({"priority": 10, "name": "Xmas", "condition": condition(&["date:12-25"]), "set": "static:x.png", "mode": ""}),
        json!({"priority": 50, "name": "Day", "condition": condition(&["time:sunrise..sunset"]), "set": "random", "mode": "light"}),
    ];
    let rows = decode_schedule_rows(&rules);
    assert_eq!(
        rows.iter().map(|row| row.name.as_str()).collect::<Vec<_>>(),
        vec!["Xmas", "Day", "Night"]
    );
    let encoded = encode_schedule_rows(&rows);
    assert_eq!(
        encoded.iter().map(|row| row["priority"].as_i64().unwrap()).collect::<Vec<_>>(),
        vec![10, 20, 30]
    );
    assert!(encoded[0].get("when").is_none());
    assert_eq!(encoded[0]["condition"]["version"], 1);
    assert_eq!(decode_schedule_rows(&encoded), rows);

    let empty_set = vec![RuleRow {
        name: "n".into(),
        enabled: true,
        condition: ConditionNode::empty_all(),
        set: String::new(),
        mode: "dark".into(),
    }];
    assert_eq!(encode_schedule_rows(&empty_set)[0]["set"], "random");
}

#[test]
fn day_night_legacy_decode() {
    let config = json!({
        "schedule": {
            "trigger": "fixed",
            "dayTime": "06:30",
            "nightTime": "21:00",
            "daySet": "random",
            "dayMode": "light",
            "nightSet": "static:night.jpg",
            "nightMode": "dark"
        }
    });
    let decoded = decode_day_night(&config);
    assert!(!decoded.solar);
    assert_eq!(decoded.day_time, "06:30");
    assert_eq!(decoded.night_time, "21:00");
    assert_eq!(decoded.day_set, "random");
    assert_eq!(decoded.day_mode, "light");
    assert_eq!(decoded.night_set, "static:night.jpg");
    assert_eq!(decoded.night_mode, "dark");
}

#[test]
fn nested_tree_roundtrip() {
    let rows = vec![RuleRow {
        name: "Weekend weather".into(),
        enabled: true,
        condition: ConditionNode::group(
            GroupOperator::All,
            vec![
                ConditionNode::predicate(Block::Weekday(vec!["sat".into(), "sun".into()])),
                ConditionNode {
                    negated: true,
                    kind: ConditionKind::Group {
                        operator: GroupOperator::Any,
                        children: vec![
                            ConditionNode::predicate(Block::Weather(vec!["rainy".into()])),
                            ConditionNode::predicate(Block::TimeCmp(">=".into(), "sunset".into())),
                        ],
                    },
                },
            ],
        ),
        set: "random".into(),
        mode: String::new(),
    }];
    let encoded = encode_schedule_rows(&rows);
    assert!(encoded[0].get("when").is_none());
    assert_eq!(encoded[0]["condition"]["root"]["children"][1]["operator"], "any");
    assert_eq!(encoded[0]["condition"]["root"]["children"][1]["negated"], true);
    assert_eq!(decode_schedule_rows(&encoded), rows);
}

#[test]
fn version_two_roundtrip() {
    let rules = vec![json!({
        "priority": 10,
        "name": "Docked on AC",
        "enabled": false,
        "condition": {"version": 2, "root": {
            "kind": "group", "operator": "all", "children": [
                {"kind": "predicate", "value": "time:>=sunset-30"},
                {"kind": "predicate", "value": "power:external"},
                {"kind": "predicate", "value": "battery:>=50"},
                {"kind": "predicate", "value": "output:DP-3"},
                {"kind": "predicate", "value": "outputs:>=2"}
            ]
        }},
        "set": "random"
    })];
    let rows = decode_schedule_rows(&rules);
    assert!(!rows[0].enabled);
    let encoded = encode_schedule_rows(&rows);
    assert_eq!(encoded[0]["enabled"], false);
    assert_eq!(encoded[0]["condition"]["version"], 2);
    assert_eq!(decode_schedule_rows(&encoded), rows);
}

#[test]
fn version_one_keeps_raw() {
    let rows = decode_schedule_rows(&[json!({
        "set": "random",
        "condition": {"version": 1, "root": {
            "kind": "group", "operator": "all", "children": [
                {"kind": "predicate", "value": "power:battery"}
            ]
        }}
    })]);
    assert!(matches!(
        rows[0].condition.kind,
        ConditionKind::Group { ref children, .. }
            if matches!(children[0].kind, ConditionKind::Predicate(Block::Raw(_)))
    ));
}

#[test]
fn malformed_condition_fails_closed() {
    for rule in [
        json!({"set": "random", "when": "weekday:sun"}),
        json!({"set": "random", "condition": {"version": 1, "root": {
            "kind": "group", "operator": "any", "children": [
                {"kind": "predicate", "value": "weekday:sun"},
                {"kind": "group", "operator": "xor", "children": []}
            ]
        }}}),
    ] {
        let rows = decode_schedule_rows(&[rule]);
        let ConditionKind::Group { children, .. } = &rows[0].condition.kind else {
            panic!("invalid condition must normalize to a fail-closed root");
        };
        assert!(matches!(
            children.as_slice(),
            [ConditionNode { kind: ConditionKind::Predicate(Block::Raw(_)), .. }]
        ));
    }
}

fn arb_predicates() -> impl Strategy<Value = Vec<String>> {
    let token = prop_oneof![
        Just("weekday:sat,sun".to_string()),
        Just("time:20:00..sunrise".to_string()),
        Just("time:>=sunset".to_string()),
        Just("date:07-01..07-14".to_string()),
        Just("year:>=2030".to_string()),
        Just("weather:cloudy,rainy".to_string()),
        "[a-z]{2,10}".prop_map(|raw| raw),
    ];
    prop::collection::vec(token, 0..4)
}

fn arb_rule_value() -> impl Strategy<Value = Value> {
    (0i64..200, "[a-z ]{0,12}", arb_predicates(), "[a-z0-9:./]{0,16}").prop_map(
        |(priority, name, predicates, set)| {
            let children = predicates
                .into_iter()
                .map(|value| json!({"kind": "predicate", "value": value}))
                .collect::<Vec<_>>();
            json!({
                "priority": priority,
                "name": name,
                "condition": {"version": 1, "root": {
                    "kind": "group", "operator": "all", "children": children
                }},
                "set": set
            })
        },
    )
}

proptest! {
    #[test]
    fn rows_roundtrip_fixed_point(
        rules in prop::collection::vec(arb_rule_value(), 0..6)
    ) {
        let normalized =
            decode_schedule_rows(&encode_schedule_rows(&decode_schedule_rows(&rules)));
        let again = decode_schedule_rows(&encode_schedule_rows(&normalized));
        prop_assert_eq!(normalized, again);
    }

    #[test]
    fn encoded_condition_normalized(predicates in arb_predicates()) {
        let rows = vec![RuleRow {
            name: String::new(),
            enabled: true,
            condition: ConditionNode::all(
                predicates.iter().map(|value| crate::domain::schedule::parse_block(value)).collect(),
            ),
            set: "random".to_string(),
            mode: String::new(),
        }];
        let encoded = encode_schedule_rows(&rows);
        prop_assert_eq!(decode_schedule_rows(&encoded), rows);
    }
}
