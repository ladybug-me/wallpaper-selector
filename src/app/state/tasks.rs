use std::collections::HashMap;

use crate::contracts::daemon::{TaskState, TaskStatus};

#[derive(Default)]
pub(crate) struct TaskUiState {
    by_id: HashMap<String, TaskStatus>,
    order: Vec<String>,
}

impl TaskUiState {
    pub(crate) fn replace(&mut self, tasks: impl IntoIterator<Item = TaskStatus>) {
        self.by_id.clear();
        self.order.clear();
        for task in tasks {
            self.update(task);
        }
    }

    pub(crate) fn update(&mut self, task: TaskStatus) {
        if !self.by_id.contains_key(&task.id) {
            self.order.push(task.id.clone());
        }
        self.by_id.insert(task.id.clone(), task);
        self.trim_finished();
    }

    pub(crate) fn values(&self) -> impl DoubleEndedIterator<Item = &TaskStatus> {
        self.order.iter().filter_map(|id| self.by_id.get(id))
    }

    pub(crate) fn bar_tasks(&self) -> Vec<&TaskStatus> {
        let active: Vec<_> = self.values().filter(|task| task.state.is_active()).collect();
        if !active.is_empty() {
            return active;
        }
        self.values()
            .next_back()
            .filter(|task| task.state == TaskState::Failed)
            .into_iter()
            .collect()
    }

    fn trim_finished(&mut self) {
        let finished: Vec<_> = self
            .order
            .iter()
            .filter(|id| self.by_id.get(*id).is_some_and(|task| !task.state.is_active()))
            .cloned()
            .collect();
        for id in finished.into_iter().rev().skip(4) {
            self.by_id.remove(&id);
            self.order.retain(|candidate| candidate != &id);
        }
    }
}

#[cfg(test)]
mod tests;
