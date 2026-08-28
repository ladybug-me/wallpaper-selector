#![cfg(test)]

use super::*;

#[test]
fn parse_predicate_clauses() {
    let blocks = [
        "weekday:sat,sun",
        "time:20:00..sunrise",
        "date:07-01..07-14",
        "year:>=2030",
        "weather:cloudy,rainy",
        "time:>=sunset",
        "date:12-25",
        "year:2026",
        "power:battery",
        "battery:<=30",
        "output:DP-3",
        "outputs:>=2",
        "time:>=sunset-30",
    ]
    .into_iter()
    .map(parse_block)
    .collect::<Vec<_>>();
    assert_eq!(
        blocks,
        vec![
            Block::Weekday(vec!["sat".into(), "sun".into()]),
            Block::TimeWindow("20:00".into(), "sunrise".into()),
            Block::Date("07-01..07-14".into()),
            Block::Year(">=".into(), "2030".into()),
            Block::Weather(vec!["cloudy".into(), "rainy".into()]),
            Block::TimeCmp(">=".into(), "sunset".into()),
            Block::Date("12-25".into()),
            Block::Year(String::new(), "2026".into()),
            Block::Power("battery".into()),
            Block::Battery("<=".into(), "30".into()),
            Block::Output("DP-3".into()),
            Block::OutputCount(">=".into(), "2".into()),
            Block::TimeCmp(">=".into(), "sunset-30".into()),
        ]
    );
}

#[test]
fn predicate_round_trip_identity() {
    for value in [
        "weekday:sun",
        "weather:cloudy",
        "time:>=20:00",
        "year:2026",
        "time:sunrise..sunset",
        "date:12-25",
        "weekday:mon,tue,wed,thu,fri",
        "time:07:30..19:45",
        "time:sunrise+45..sunset-30",
        "date:2026-12-25",
        "power:external",
        "battery:<=25",
        "output:DP-3",
        "outputs:>=2",
    ] {
        assert_eq!(format_block(&parse_block(value)), value);
    }
}

#[test]
fn malformed_stays_raw() {
    let values = [
        "moonphase:full",
        "weekday:funday",
        "time:25:99",
        "date:13-40",
        "weather:hail",
        "battery:101",
        "power:solar",
        "outputs:65",
    ];
    let blocks = values.into_iter().map(parse_block).collect::<Vec<_>>();
    assert!(blocks.iter().all(|block| matches!(block, Block::Raw(_))));
    assert_eq!(blocks.iter().map(format_block).collect::<Vec<_>>(), values);
}

#[test]
fn typo_list_stays_raw() {
    assert_eq!(parse_block("weekday:sat,sunn"), Block::Raw("weekday:sat,sunn".into()));
    assert_eq!(parse_block("weather:cloudy,hail"), Block::Raw("weather:cloudy,hail".into()));
}

#[test]
fn priority_order_is_stable() {
    let row = |name: &str| RuleRow {
        name: name.into(),
        enabled: true,
        condition: ConditionNode::empty_all(),
        set: String::new(),
        mode: String::new(),
    };
    let rows = order_rows(vec![(50, 0, row("first")), (10, 1, row("early")), (50, 2, row("last"))]);
    assert_eq!(
        rows.iter().map(|row| row.name.as_str()).collect::<Vec<_>>(),
        vec!["early", "first", "last"]
    );
    assert_eq!(priority_for_position(0), 10);
    assert_eq!(priority_for_position(2), 30);
}

#[test]
fn seed_day_night_rows() {
    let solar = DayNight {
        solar: true,
        day_time: String::new(),
        night_time: String::new(),
        day_set: "random".into(),
        day_mode: "light".into(),
        night_set: "static:city.png".into(),
        night_mode: "dark".into(),
    };
    let rows = seed_day_night(&solar);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "Day");
    assert_eq!(
        rows[0].condition,
        ConditionNode::all(vec![Block::TimeWindow("sunrise".into(), "sunset".into())])
    );
    assert_eq!(rows[1].name, "Night");
    assert_eq!(rows[1].condition, ConditionNode::empty_all());

    let fixed = DayNight {
        solar: false,
        day_time: "06:30".into(),
        night_time: "21:00".into(),
        day_set: "random".into(),
        day_mode: String::new(),
        night_set: String::new(),
        night_mode: String::new(),
    };
    let rows = seed_day_night(&fixed);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].condition,
        ConditionNode::all(vec![Block::TimeWindow("06:30".into(), "21:00".into())])
    );

    let junk = DayNight {
        solar: false,
        day_time: "junk".into(),
        night_time: String::new(),
        day_set: "random".into(),
        day_mode: String::new(),
        night_set: String::new(),
        night_mode: String::new(),
    };
    assert_eq!(
        seed_day_night(&junk)[0].condition,
        ConditionNode::all(vec![Block::TimeWindow("07:00".into(), "20:00".into())])
    );
}

use proptest::prelude::*;

fn arb_predicate() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("weekday:sat,sun".to_string()),
        Just("time:20:00..sunrise".to_string()),
        Just("time:>=sunset".to_string()),
        Just("date:07-01..07-14".to_string()),
        Just("year:>=2030".to_string()),
        Just("weather:cloudy,rainy".to_string()),
        "[a-z]{2,10}".prop_map(|raw| raw),
    ]
}

proptest! {
    #[test]
    fn format_predicate_idempotent(value in arb_predicate()) {
        let once = format_block(&parse_block(&value));
        let twice = format_block(&parse_block(&once));
        prop_assert_eq!(once, twice);
    }

    #[test]
    fn parse_predicate_never_panics(value in ".{0,80}") {
        let _ = format_block(&parse_block(&value));
    }
}
