use crate::domain::schedule::{
    Block, ConditionKind, ConditionNode, GroupOperator, RuleRow, WEEKDAYS,
};
use crate::frontend::animation::{MotionProfile, MotionTier, Tween};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Editing {
    Node(usize, Vec<usize>),
    Add(usize, Vec<usize>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockKind {
    Weekday,
    TimeWindow,
    TimeAfter,
    TimeBefore,
    Date,
    Year,
    Weather,
    Power,
    Battery,
    Output,
    OutputCount,
}

pub const BLOCK_KINDS: [(BlockKind, &str); 11] = [
    (BlockKind::Weekday, "schedule-kind-weekday"),
    (BlockKind::TimeWindow, "schedule-kind-time-range"),
    (BlockKind::TimeAfter, "schedule-kind-after-time"),
    (BlockKind::TimeBefore, "schedule-kind-before-time"),
    (BlockKind::Date, "schedule-kind-date"),
    (BlockKind::Year, "schedule-kind-year"),
    (BlockKind::Weather, "schedule-kind-weather"),
    (BlockKind::Power, "schedule-kind-power"),
    (BlockKind::Battery, "schedule-kind-battery"),
    (BlockKind::Output, "schedule-kind-output"),
    (BlockKind::OutputCount, "schedule-kind-output-count"),
];

pub fn default_block(kind: BlockKind) -> Block {
    match kind {
        BlockKind::Weekday => Block::Weekday(vec!["sat".into(), "sun".into()]),
        BlockKind::TimeWindow => Block::TimeWindow("sunrise".into(), "sunset".into()),
        BlockKind::TimeAfter => Block::TimeCmp(">=".into(), "20:00".into()),
        BlockKind::TimeBefore => Block::TimeCmp("<".into(), "08:00".into()),
        BlockKind::Date => Block::Date("12-25".into()),
        BlockKind::Year => Block::Year(String::new(), "2026".into()),
        BlockKind::Weather => Block::Weather(vec!["cloudy".into()]),
        BlockKind::Power => Block::Power("battery".into()),
        BlockKind::Battery => Block::Battery("<=".into(), "30".into()),
        BlockKind::Output => Block::Output("DP-1".into()),
        BlockKind::OutputCount => Block::OutputCount(">=".into(), "2".into()),
    }
}

pub struct ScheduleEditor {
    pub rows: Vec<RuleRow>,
    pub enabled: bool,
    pub selected: usize,
    pub foldout: Option<usize>,
    pub foldout_visible: Option<usize>,
    pub foldout_reveal: Tween,
    pub drag: Option<usize>,
    pub editing: Option<Editing>,
    pub migrated: bool,
    pub demo: bool,
}

impl ScheduleEditor {
    fn tree_size(node: &ConditionNode) -> usize {
        match &node.kind {
            ConditionKind::Predicate(_) => 1,
            ConditionKind::Group { children, .. } => {
                1 + children.iter().map(Self::tree_size).sum::<usize>()
            }
        }
    }

    pub fn new(rows: Vec<RuleRow>, migrated: bool, enabled: bool) -> Self {
        let motion = MotionProfile::default();
        Self {
            rows,
            enabled,
            selected: 0,
            foldout: None,
            foldout_visible: None,
            foldout_reveal: motion.tween(0.0, MotionTier::Standard),
            drag: None,
            editing: None,
            migrated,
            demo: false,
        }
    }

    pub fn demo(rows: Vec<RuleRow>) -> Self {
        let motion = MotionProfile::default();
        Self {
            rows,
            enabled: true,
            selected: 0,
            foldout: None,
            foldout_visible: None,
            foldout_reveal: motion.tween(0.0, MotionTier::Standard),
            drag: None,
            editing: None,
            migrated: false,
            demo: true,
        }
    }

    pub fn select(&mut self, idx: usize) {
        if idx < self.rows.len() {
            self.selected = idx;
            self.editing = None;
            self.drag = None;
        }
    }

    pub fn toggle_rule_options(&mut self, idx: usize) {
        if idx >= self.rows.len() {
            return;
        }
        let closing = self.foldout == Some(idx);
        self.select(idx);
        if closing {
            self.foldout = None;
            self.foldout_reveal.retarget(0.0);
        } else {
            if self.foldout_visible != Some(idx) {
                self.foldout_reveal.snap(0.0);
            }
            self.foldout = Some(idx);
            self.foldout_visible = Some(idx);
            self.foldout_reveal.retarget(1.0);
        }
    }

    pub fn add_rule(&mut self) -> usize {
        self.rows.push(RuleRow {
            name: String::new(),
            enabled: true,
            condition: ConditionNode::empty_all(),
            set: "random".to_string(),
            mode: String::new(),
        });
        let idx = self.rows.len() - 1;
        self.selected = idx;
        self.foldout = Some(idx);
        self.foldout_visible = Some(idx);
        self.foldout_reveal.run(0.0, 1.0);
        self.editing = Some(Editing::Add(idx, Vec::new()));
        idx
    }

    pub fn remove_rule(&mut self, idx: usize) {
        if idx < self.rows.len() {
            self.rows.remove(idx);
            self.foldout = self.foldout.and_then(|expanded| match expanded.cmp(&idx) {
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(expanded - 1),
                std::cmp::Ordering::Less => Some(expanded),
            });
            self.foldout_visible =
                self.foldout_visible.and_then(|visible| match visible.cmp(&idx) {
                    std::cmp::Ordering::Equal => None,
                    std::cmp::Ordering::Greater => Some(visible - 1),
                    std::cmp::Ordering::Less => Some(visible),
                });
            if self.foldout_visible.is_none() {
                self.foldout_reveal.snap(0.0);
            }
            if self.rows.is_empty() {
                self.selected = 0;
            } else if self.selected > idx {
                self.selected -= 1;
            } else {
                self.selected = self.selected.min(self.rows.len() - 1);
            }
        }
        self.editing = None;
        self.drag = None;
    }

    pub fn drag_start(&mut self, idx: usize) {
        if idx < self.rows.len() {
            self.drag = Some(idx);
            self.editing = None;
        }
    }

    pub fn drag_over(&mut self, target: usize) {
        let Some(from) = self.drag else {
            return;
        };
        if from == target || target >= self.rows.len() {
            return;
        }
        let row = self.rows.remove(from);
        self.rows.insert(target, row);
        let remap = |index: usize| {
            if index == from {
                target
            } else if from < index && target >= index {
                index - 1
            } else if from > index && target <= index {
                index + 1
            } else {
                index
            }
        };
        self.foldout = self.foldout.map(remap);
        self.foldout_visible = self.foldout_visible.map(remap);
        if self.selected == from {
            self.selected = target;
        } else if from < self.selected && target >= self.selected {
            self.selected -= 1;
        } else if from > self.selected && target <= self.selected {
            self.selected += 1;
        }
        self.drag = Some(target);
    }

    pub fn drag_end(&mut self) -> bool {
        self.drag.take().is_some()
    }

    pub fn set_motion_profile(&mut self, motion: MotionProfile) {
        motion.retime_tween(&mut self.foldout_reveal, MotionTier::Standard);
    }

    pub fn animating(&self) -> bool {
        !self.foldout_reveal.settled()
    }

    pub fn tick(&mut self, dt: f32) {
        self.foldout_reveal.tick(dt);
        if self.foldout_reveal.target == 0.0 && self.foldout_reveal.x == 0.0 {
            self.foldout_visible = None;
        }
    }

    fn node_at<'a>(node: &'a ConditionNode, path: &[usize]) -> Option<&'a ConditionNode> {
        let Some((&index, rest)) = path.split_first() else {
            return Some(node);
        };
        let ConditionKind::Group { children, .. } = &node.kind else {
            return None;
        };
        Self::node_at(children.get(index)?, rest)
    }

    fn node_at_mut<'a>(
        node: &'a mut ConditionNode,
        path: &[usize],
    ) -> Option<&'a mut ConditionNode> {
        let Some((&index, rest)) = path.split_first() else {
            return Some(node);
        };
        let ConditionKind::Group { children, .. } = &mut node.kind else {
            return None;
        };
        Self::node_at_mut(children.get_mut(index)?, rest)
    }

    pub fn node(&self, row: usize, path: &[usize]) -> Option<&ConditionNode> {
        Self::node_at(&self.rows.get(row)?.condition, path)
    }

    pub fn node_mut(&mut self, row: usize, path: &[usize]) -> Option<&mut ConditionNode> {
        Self::node_at_mut(&mut self.rows.get_mut(row)?.condition, path)
    }

    pub fn add_block(&mut self, row: usize, group_path: &[usize], kind: BlockKind) {
        if group_path.len() >= 32
            || self.rows.get(row).is_none_or(|rule| Self::tree_size(&rule.condition) >= 256)
        {
            return;
        }
        let Some(group) = self.node_mut(row, group_path) else {
            return;
        };
        let ConditionKind::Group { children, .. } = &mut group.kind else {
            return;
        };
        children.push(ConditionNode::predicate(default_block(kind)));
        let mut path = group_path.to_vec();
        path.push(children.len() - 1);
        self.editing = Some(Editing::Node(row, path));
    }

    pub fn add_group(&mut self, row: usize, group_path: &[usize], operator: GroupOperator) {
        if group_path.len() >= 32
            || self.rows.get(row).is_none_or(|rule| Self::tree_size(&rule.condition) >= 256)
        {
            return;
        }
        let Some(group) = self.node_mut(row, group_path) else {
            return;
        };
        let ConditionKind::Group { children, .. } = &mut group.kind else {
            return;
        };
        children.push(ConditionNode::group(operator, Vec::new()));
        let mut path = group_path.to_vec();
        path.push(children.len() - 1);
        self.editing = Some(Editing::Node(row, path));
    }

    pub fn remove_node(&mut self, row: usize, path: &[usize]) {
        let Some((&index, parent_path)) = path.split_last() else {
            return;
        };
        if let Some(parent) = self.node_mut(row, parent_path)
            && let ConditionKind::Group { children, .. } = &mut parent.kind
            && index < children.len()
        {
            children.remove(index);
        }
        self.editing = None;
    }

    pub fn block_mut(&mut self, row: usize, path: &[usize]) -> Option<&mut Block> {
        let ConditionKind::Predicate(block) = &mut self.node_mut(row, path)?.kind else {
            return None;
        };
        Some(block)
    }

    pub fn toggle_in_list(&mut self, row: usize, path: &[usize], value: &str) {
        let Some(blk) = self.block_mut(row, path) else {
            return;
        };
        let (Block::Weekday(list) | Block::Weather(list)) = blk else {
            return;
        };
        if let Some(pos) = list.iter().position(|item| item == value) {
            if list.len() > 1 {
                list.remove(pos);
            }
        } else {
            list.push(value.to_string());
        }
        if let Block::Weekday(days) = blk {
            days.sort_by_key(|day| WEEKDAYS.iter().position(|weekday| weekday == day).unwrap_or(7));
        }
    }

    pub fn set_group_operator(&mut self, row: usize, path: &[usize], operator: GroupOperator) {
        if let Some(node) = self.node_mut(row, path)
            && let ConditionKind::Group { operator: current, .. } = &mut node.kind
        {
            *current = operator;
        }
    }

    pub fn set_negated(&mut self, row: usize, path: &[usize], negated: bool) {
        if let Some(node) = self.node_mut(row, path) {
            node.negated = negated;
        }
    }

    pub fn reset_condition(&mut self, row: usize) {
        if let Some(rule) = self.rows.get_mut(row) {
            rule.condition = ConditionNode::empty_all();
            self.editing = Some(Editing::Add(row, Vec::new()));
        }
    }

    pub fn toggle_editing(&mut self, target: Editing) {
        self.editing = if self.editing.as_ref() == Some(&target) { None } else { Some(target) };
    }
}

mod tests;
