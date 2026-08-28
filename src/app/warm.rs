use super::{App, Message};
use crate::infrastructure::runtime::Wake;

static SHOWN_AT: std::sync::Mutex<Option<std::time::Instant>> = std::sync::Mutex::new(None);

fn note_show_requested() {
    *SHOWN_AT.lock().unwrap_or_else(std::sync::PoisonError::into_inner) =
        Some(std::time::Instant::now());
}

pub(crate) fn note_overlay_drawn() {
    let taken = SHOWN_AT.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    if let Some(shown) = taken {
        log::info!(
            "warm: overlay drawn {:.1} ms after show",
            shown.elapsed().as_secs_f64() * 1000.0
        );
    }
}

pub(crate) fn adopt_first_window(
    app: &mut App,
    id: iced::window::Id,
    width: f32,
    height: f32,
) -> iced::Task<Message> {
    if app.runtime_state.overlay.is_some() {
        return iced::Task::none();
    }
    app.runtime_state.overlay = Some(id);
    app.scene.viewport = (width, height);
    app.config.set_screen_width(width);
    app.theme.suspended = false;
    note_show_requested();
    app.scene.begin_open_fade();
    app.retick();
    log::info!("adopted initial surface {id:?} as the overlay");
    iced::Task::none()
}

#[cfg(target_os = "linux")]
pub(crate) fn exit_picker(app: &mut App) -> ! {
    app.on_hidden();
    crate::hard_exit(0)
}

#[cfg(not(target_os = "linux"))]
pub(crate) fn exit_picker(_app: &mut App) -> ! {
    crate::hard_exit(0)
}

pub(crate) fn toggle(app: &mut App) -> iced::Task<Message> {
    if app.runtime_state.overlay.is_some() {
        exit_picker(app);
    }
    iced::Task::none()
}

pub(crate) fn request_hide(app: &App) {
    let _ = app.runtime_state.wake_tx.unbounded_send(Wake::Hide);
}

#[cfg(test)]
mod tests;
