use iced::Task;

use crate::domain::schedule::Block;

#[allow(clippy::wildcard_imports)]
use super::super::*;

use crate::frontend::schedule_editor::SchedMsg;

pub(super) fn update(app: &mut App, msg: SchedMsg) -> Task<Message> {
    match msg {
        SchedMsg::Close => {
            crate::app::helpers::sched_persist(app);
            app.panels.schedule = None;
            app.retick();
            Task::none()
        }
        SchedMsg::Submit => {
            crate::app::helpers::sched_persist(app);
            Task::none()
        }
        SchedMsg::SetEnabled(enabled) => sched_mutate(app, true, |ed| ed.enabled = enabled),
        SchedMsg::SetRuleEnabled(idx, enabled) => sched_mutate(app, true, |ed| {
            if let Some(row) = ed.rows.get_mut(idx as usize) {
                row.enabled = enabled;
            }
        }),
        SchedMsg::ToggleRuleOptions(idx) => {
            sched_mutate(app, false, |ed| ed.toggle_rule_options(idx as usize))
        }
        SchedMsg::AddRule => sched_mutate(app, true, |ed| {
            ed.add_rule();
        }),
        SchedMsg::RemoveRule(idx) => sched_mutate(app, true, |ed| ed.remove_rule(idx as usize)),
        SchedMsg::DragStart(idx) => sched_mutate(app, false, |ed| ed.drag_start(idx as usize)),
        SchedMsg::DragOver(idx) => sched_mutate(app, false, |ed| ed.drag_over(idx as usize)),
        SchedMsg::DragEnd => sched_drag_end(app),
        SchedMsg::EditNode(idx, path) => sched_mutate(app, false, |ed| {
            ed.toggle_editing(crate::frontend::schedule_editor::Editing::Node(
                idx as usize,
                path_usize(&path),
            ));
        }),
        SchedMsg::EditAdd(idx, path) => sched_mutate(app, false, |ed| {
            ed.toggle_editing(crate::frontend::schedule_editor::Editing::Add(
                idx as usize,
                path_usize(&path),
            ));
        }),
        SchedMsg::NameInput(idx, name) => sched_mutate(app, false, |ed| {
            if let Some(row) = ed.rows.get_mut(idx as usize) {
                row.name = name;
            }
        }),
        SchedMsg::SetInput(idx, val) => sched_mutate(app, false, |ed| {
            if let Some(row) = ed.rows.get_mut(idx as usize) {
                row.set = val;
            }
        }),
        SchedMsg::AddBlock(idx, path, kind_idx) => sched_mutate(app, true, |ed| {
            if let Some((kind, _)) =
                crate::frontend::schedule_editor::BLOCK_KINDS.get(kind_idx as usize)
            {
                ed.add_block(idx as usize, &path_usize(&path), *kind);
            }
        }),
        SchedMsg::AddGroup(idx, path, any) => sched_mutate(app, true, |ed| {
            ed.add_group(
                idx as usize,
                &path_usize(&path),
                if any {
                    crate::domain::schedule::GroupOperator::Any
                } else {
                    crate::domain::schedule::GroupOperator::All
                },
            );
        }),
        SchedMsg::RemoveNode(idx, path) => {
            sched_mutate(app, true, |ed| ed.remove_node(idx as usize, &path_usize(&path)))
        }
        SchedMsg::ToggleVal(idx, path, val) => {
            sched_mutate(app, true, |ed| ed.toggle_in_list(idx as usize, &path_usize(&path), &val))
        }
        SchedMsg::TimePart(idx, path, which, val) => sched_time_part(app, idx, &path, which, val),
        SchedMsg::TextBlock(idx, path, val) => {
            sched_mutate(app, false, |ed| match ed.block_mut(idx as usize, &path_usize(&path)) {
                Some(Block::Date(date)) => *date = val,
                Some(Block::Year(op, year)) => {
                    let (nop, ny) = ["<=", ">=", "<", ">"]
                        .iter()
                        .find_map(|pre| {
                            val.strip_prefix(pre).map(|rest| ((*pre).to_string(), rest.to_string()))
                        })
                        .unwrap_or((String::new(), val));
                    *op = nop;
                    *year = ny;
                }
                Some(Block::Battery(_, percent) | Block::OutputCount(_, percent)) => {
                    *percent = val;
                }
                Some(Block::Output(output)) => *output = val,
                Some(Block::Raw(raw)) => *raw = val,
                _ => {}
            })
        }
        SchedMsg::BlockChoice(idx, path, val) => sched_mutate(app, true, |ed| {
            if let Some(Block::Power(source)) = ed.block_mut(idx as usize, &path_usize(&path)) {
                *source = val;
            }
        }),
        SchedMsg::Comparison(idx, path, val) => sched_mutate(app, true, |ed| {
            if let Some(Block::Battery(op, _) | Block::OutputCount(op, _)) =
                ed.block_mut(idx as usize, &path_usize(&path))
            {
                *op = val;
            }
        }),
        SchedMsg::SetGroupOperator(idx, path, any) => sched_mutate(app, true, |ed| {
            ed.set_group_operator(
                idx as usize,
                &path_usize(&path),
                if any {
                    crate::domain::schedule::GroupOperator::Any
                } else {
                    crate::domain::schedule::GroupOperator::All
                },
            );
        }),
        SchedMsg::SetNegated(idx, path, negated) => sched_mutate(app, true, |ed| {
            ed.set_negated(idx as usize, &path_usize(&path), negated);
        }),
        SchedMsg::ResetCondition(idx) => {
            sched_mutate(app, true, |ed| ed.reset_condition(idx as usize))
        }
        SchedMsg::Mode(idx, mode) => sched_mutate(app, true, |ed| {
            if let Some(row) = ed.rows.get_mut(idx as usize) {
                row.mode = mode;
            }
        }),
    }
}

fn path_usize(path: &[u16]) -> Vec<usize> {
    path.iter().map(|index| usize::from(*index)).collect()
}

fn sched_mutate(
    app: &mut App,
    persist: bool,
    func: impl FnOnce(&mut crate::frontend::schedule_editor::ScheduleEditor),
) -> Task<Message> {
    if let Some(ed) = app.panels.schedule.as_mut() {
        func(ed);
        if persist {
            crate::app::helpers::sched_persist(app);
        }
        app.retick();
    }
    Task::none()
}

fn sched_drag_end(app: &mut App) -> Task<Message> {
    let committed = app
        .panels
        .schedule
        .as_mut()
        .is_some_and(crate::frontend::schedule_editor::ScheduleEditor::drag_end);
    if committed {
        crate::app::helpers::sched_persist(app);
        app.retick();
    }
    Task::none()
}

fn sched_time_part(app: &mut App, idx: u16, path: &[u16], which: u8, val: String) -> Task<Message> {
    let chip = val == "sunrise" || val == "sunset" || which == 3;
    sched_mutate(app, chip, |ed| match (ed.block_mut(idx as usize, &path_usize(path)), which) {
        (Some(Block::TimeWindow(from, _)), 0) => *from = val,
        (Some(Block::TimeWindow(_, to)), 1) => *to = val,
        (Some(Block::TimeCmp(_, at)), 2) => *at = val,
        (Some(Block::TimeCmp(op, _)), 3) => *op = val,
        _ => {}
    })
}
