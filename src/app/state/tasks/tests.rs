use super::*;

#[test]
fn replace_drops_stale_tasks() {
    let mut state = TaskUiState::default();
    state.update(TaskStatus::running("old-index", "index", "Building index"));

    let mut completed = TaskStatus::running("scan", "scan", "Scanning wallpapers");
    completed.state = TaskState::Completed;
    state.replace([completed]);

    let tasks = state.values().collect::<Vec<_>>();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, "scan");
    assert_eq!(tasks[0].state, TaskState::Completed);
}

#[test]
fn finished_tasks_hidden() {
    for finished in [TaskState::Completed, TaskState::Cancelled] {
        let mut state = TaskUiState::default();
        let mut task = TaskStatus::running("scan", "scan", "Scanning wallpapers");
        task.state = finished;
        state.update(task);
        assert!(state.bar_tasks().is_empty());
    }
}

#[test]
fn failed_task_stays_visible() {
    let mut state = TaskUiState::default();
    let mut task = TaskStatus::running("semantic-index", "index", "Building index");
    task.state = TaskState::Failed;
    state.update(task);

    let tasks = state.bar_tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].state, TaskState::Failed);
}
