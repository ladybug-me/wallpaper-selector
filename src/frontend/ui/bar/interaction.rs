use iced::Point;
use iced::mouse;
use iced::widget::canvas::Action;

use crate::contracts::picker::theme_setting;
use crate::frontend::theme_designer::ThemeMsg;

use super::action::{BarAction, BarIntent};
use super::canvas::{BarState, FilterBar};
use super::menu::MenuKind;

impl FilterBar<'_> {
    pub(super) fn on_move(&self, position: Point) -> Option<Action<BarIntent>> {
        let (hit, menu_hit) = if self.active_menu().is_some() {
            (None, self.menu_index_at(position.x, position.y))
        } else {
            (self.hit(position.x, position.y), None)
        };
        if hit == self.hover && menu_hit == self.menu_hover {
            return None;
        }
        Some(Action::publish(BarIntent::Hover(hit, menu_hit)))
    }

    pub(super) fn on_click(&self, position: Point) -> Option<Action<BarIntent>> {
        if let Some(kind) = self.active_menu() {
            return Some(self.menu_click(kind, position));
        }
        let index = self.hit(position.x, position.y)?;
        let action = self.model.items[index].action.clone()?;
        Some(Action::publish(BarIntent::Activate(action)).and_capture())
    }

    fn menu_click(&self, kind: MenuKind, position: Point) -> Action<BarIntent> {
        let index = self.menu_index_at(position.x, position.y);
        match (kind, index) {
            (MenuKind::Folders, Some(index)) => {
                Action::publish(BarIntent::SelectFolder(self.folder_options[index].clone()))
                    .and_capture()
            }
            (MenuKind::Folders, None) => Action::publish(BarIntent::ToggleFolderMenu).and_capture(),
            (MenuKind::Backends, Some(index)) => {
                let value = self.backend_options[index].0;
                Action::publish(BarIntent::Theme(ThemeMsg::Option(theme_setting::BACKEND, value)))
                    .and_capture()
            }
            (MenuKind::Backends, None) => {
                Action::publish(BarIntent::Theme(ThemeMsg::BackendMenu)).and_capture()
            }
        }
    }

    pub(super) fn on_scroll(
        &self,
        state: &mut BarState,
        position: Point,
        delta: &mouse::ScrollDelta,
    ) -> Option<Action<BarIntent>> {
        let delta_y = match crate::frontend::components::norm_wheel(delta)? {
            mouse::ScrollDelta::Lines { y, .. } => y,
            mouse::ScrollDelta::Pixels { y, .. } => y / 40.0,
        };
        if self.over_menu(position) && self.menu_max_scroll() > 0.0 {
            if delta_y == 0.0 {
                return None;
            }
            let step = BarIntent::ScrollFolderMenu(-delta_y * self.menu_row_h());
            return Some(Action::publish(step).and_capture());
        }
        let Some(index) = self.hit(position.x, position.y) else {
            state.audio_accum = 0.0;
            return None;
        };
        if !matches!(self.model.items[index].action, Some(BarAction::Audio)) {
            state.audio_accum = 0.0;
            return None;
        }
        if delta_y == 0.0 {
            return None;
        }
        state.audio_accum += delta_y;
        let steps = state.audio_accum.trunc();
        if steps == 0.0 {
            return Some(Action::capture());
        }
        state.audio_accum -= steps;
        Some(Action::publish(BarIntent::StepVolume(steps as i32 * 5)).and_capture())
    }
}
