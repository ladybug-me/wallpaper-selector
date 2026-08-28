pub const WEEKDAYS: [&str; 7] = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];
pub const WEATHER: [&str; 8] =
    ["clear", "sunny", "cloudy", "rainy", "snowy", "stormy", "foggy", "windy"];
const TIME_OPS: [&str; 4] = [">=", "<=", ">", "<"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Block {
    Weekday(Vec<String>),
    TimeWindow(String, String),
    TimeCmp(String, String),
    Date(String),
    Year(String, String),
    Weather(Vec<String>),
    Power(String),
    Battery(String, String),
    Output(String),
    OutputCount(String, String),
    Raw(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupOperator {
    All,
    Any,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConditionKind {
    Predicate(Block),
    Group { operator: GroupOperator, children: Vec<ConditionNode> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConditionNode {
    pub negated: bool,
    pub kind: ConditionKind,
}

impl ConditionNode {
    pub fn predicate(block: Block) -> Self {
        Self { negated: false, kind: ConditionKind::Predicate(block) }
    }

    pub fn group(operator: GroupOperator, children: Vec<Self>) -> Self {
        Self { negated: false, kind: ConditionKind::Group { operator, children } }
    }

    pub fn all(blocks: Vec<Block>) -> Self {
        Self::group(GroupOperator::All, blocks.into_iter().map(Self::predicate).collect())
    }

    pub fn empty_all() -> Self {
        Self::group(GroupOperator::All, Vec::new())
    }
}

fn valid_time_endpoint(text: &str) -> bool {
    for solar in ["sunrise", "sunset"] {
        if let Some(rest) = text.strip_prefix(solar) {
            return rest.is_empty()
                || rest.parse::<i32>().is_ok_and(|offset| (-720..=720).contains(&offset));
        }
    }
    let Some((hour, min)) = text.split_once(':') else {
        return false;
    };
    hour.parse::<u32>().is_ok_and(|num| num < 24) && min.parse::<u32>().is_ok_and(|num| num < 60)
}

fn offset_solar_endpoint(text: &str) -> bool {
    ["sunrise", "sunset"]
        .iter()
        .any(|solar| text.strip_prefix(solar).is_some_and(|rest| !rest.is_empty()))
}

fn split_op(payload: &str) -> Option<(&'static str, &str)> {
    for op in TIME_OPS {
        if let Some(rest) = payload.strip_prefix(op) {
            return Some((op, rest));
        }
    }
    None
}

fn parse_list(
    token: &str,
    payload: &str,
    allowlist: &[&str],
    ctor: fn(Vec<String>) -> Block,
) -> Block {
    let values: Vec<String> =
        payload.split(',').filter(|value| allowlist.contains(value)).map(str::to_string).collect();
    if values.is_empty() || values.len() != payload.split(',').count() {
        return Block::Raw(token.to_string());
    }
    ctor(values)
}

fn valid_date(text: &str) -> bool {
    let parts = text.split('-').collect::<Vec<_>>();
    let (month, day) = match parts.as_slice() {
        [month, day] => (*month, *day),
        [year, month, day]
            if year.len() == 4 && year.parse::<i32>().is_ok_and(|year| year >= 1) =>
        {
            (*month, *day)
        }
        _ => return false,
    };
    month.parse::<u32>().is_ok_and(|number| (1..=12).contains(&number))
        && day.parse::<u32>().is_ok_and(|number| (1..=31).contains(&number))
}

pub fn parse_block_version(token: &str, version: u64) -> Block {
    let raw = || Block::Raw(token.to_string());
    let Some((kind, payload)) = token.split_once(':') else {
        return raw();
    };
    match kind {
        "weekday" => parse_list(token, payload, &WEEKDAYS, Block::Weekday),
        "time" => {
            if let Some((from, to)) = payload.split_once("..") {
                if valid_time_endpoint(from)
                    && valid_time_endpoint(to)
                    && (version >= 2 || !(offset_solar_endpoint(from) || offset_solar_endpoint(to)))
                {
                    return Block::TimeWindow(from.to_string(), to.to_string());
                }
                return raw();
            }
            if let Some((op, at)) = split_op(payload)
                && valid_time_endpoint(at)
                && (version >= 2 || !offset_solar_endpoint(at))
            {
                return Block::TimeCmp(op.to_string(), at.to_string());
            }
            raw()
        }
        "date" => {
            let ok = payload.split("..").all(valid_date);
            let parts = payload.split("..").count();
            if ok && (parts == 1 || parts == 2) && !payload.is_empty() {
                Block::Date(payload.to_string())
            } else {
                raw()
            }
        }
        "year" => {
            let (op, rest) = split_op(payload).unwrap_or(("", payload));
            if rest.len() == 4 && rest.parse::<i64>().is_ok() {
                Block::Year(op.to_string(), rest.to_string())
            } else {
                raw()
            }
        }
        "weather" => parse_list(token, payload, &WEATHER, Block::Weather),
        "power" if version >= 2 && matches!(payload, "battery" | "external") => {
            Block::Power(payload.to_string())
        }
        "battery" if version >= 2 => {
            let (op, percent) = split_op(payload).unwrap_or(("", payload));
            if percent.parse::<u8>().is_ok_and(|value| value <= 100) {
                Block::Battery(op.to_string(), percent.to_string())
            } else {
                raw()
            }
        }
        "output" if version >= 2 && !payload.is_empty() && payload.len() <= 128 => {
            Block::Output(payload.to_string())
        }
        "outputs" if version >= 2 => {
            let (op, count) = split_op(payload).unwrap_or(("", payload));
            if count.parse::<u32>().is_ok_and(|value| value <= 64) {
                Block::OutputCount(op.to_string(), count.to_string())
            } else {
                raw()
            }
        }
        _ => raw(),
    }
}

pub fn parse_block(token: &str) -> Block {
    parse_block_version(token, 2)
}

pub fn format_block(block: &Block) -> String {
    match block {
        Block::Weekday(days) => format!("weekday:{}", days.join(",")),
        Block::TimeWindow(from, to) => format!("time:{from}..{to}"),
        Block::TimeCmp(op, at) => format!("time:{op}{at}"),
        Block::Date(date) => format!("date:{date}"),
        Block::Year(op, year) => format!("year:{op}{year}"),
        Block::Weather(tags) => format!("weather:{}", tags.join(",")),
        Block::Power(source) => format!("power:{source}"),
        Block::Battery(op, percent) => format!("battery:{op}{percent}"),
        Block::Output(name) => format!("output:{name}"),
        Block::OutputCount(op, count) => format!("outputs:{op}{count}"),
        Block::Raw(raw) => raw.clone(),
    }
}

pub fn block_requires_v2(block: &Block) -> bool {
    match block {
        Block::TimeWindow(from, to) => offset_solar_endpoint(from) || offset_solar_endpoint(to),
        Block::TimeCmp(_, at) => offset_solar_endpoint(at),
        Block::Power(_) | Block::Battery(..) | Block::Output(_) | Block::OutputCount(..) => true,
        _ => false,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleRow {
    pub name: String,
    pub enabled: bool,
    pub condition: ConditionNode,
    pub set: String,
    pub mode: String,
}

pub(crate) fn order_rows(mut rows: Vec<(i64, usize, RuleRow)>) -> Vec<RuleRow> {
    rows.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0).then(lhs.1.cmp(&rhs.1)));
    rows.into_iter().map(|(_, _, row)| row).collect()
}

pub(crate) fn priority_for_position(index: usize) -> i64 {
    ((index + 1) * 10) as i64
}

pub struct DayNight {
    pub solar: bool,
    pub day_time: String,
    pub night_time: String,
    pub day_set: String,
    pub day_mode: String,
    pub night_set: String,
    pub night_mode: String,
}

pub fn seed_day_night(spec: &DayNight) -> Vec<RuleRow> {
    let mut rows = Vec::new();
    let (day_from, day_to) = if spec.solar {
        ("sunrise".to_string(), "sunset".to_string())
    } else {
        (
            Some(spec.day_time.trim())
                .filter(|time| valid_time_endpoint(time))
                .unwrap_or("07:00")
                .to_string(),
            Some(spec.night_time.trim())
                .filter(|time| valid_time_endpoint(time))
                .unwrap_or("20:00")
                .to_string(),
        )
    };
    if !spec.day_set.is_empty() || !spec.day_mode.is_empty() {
        rows.push(RuleRow {
            name: "Day".to_string(),
            enabled: true,
            condition: ConditionNode::all(vec![Block::TimeWindow(day_from, day_to)]),
            set: spec.day_set.clone(),
            mode: spec.day_mode.clone(),
        });
    }
    if !spec.night_set.is_empty() || !spec.night_mode.is_empty() {
        rows.push(RuleRow {
            name: "Night".to_string(),
            enabled: true,
            condition: ConditionNode::empty_all(),
            set: spec.night_set.clone(),
            mode: spec.night_mode.clone(),
        });
    }
    rows
}
