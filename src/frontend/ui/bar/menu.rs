use iced::{Point, Rectangle};

use super::action::BarAction;
use super::canvas::FilterBar;
use super::catalog::{MENU_MAX_ROWS, MENU_ROW_H};
use super::model::{BarItem, BarModel, BarVisualStyle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuKind {
    Folders,
    Backends,
}

pub(super) fn folder_depth(folder: &str) -> usize {
    folder.matches('/').count()
}

pub(super) fn folder_leaf(folder: &str) -> &str {
    folder.rsplit('/').next().unwrap_or(folder)
}

pub(in crate::frontend::ui) fn item_contains(item: &BarItem, x: f32, y: f32) -> bool {
    if y < item.y || y > item.y + item.h {
        return false;
    }
    let fraction = (y - item.y) / item.h;
    let left = item.x + item.skew * (1.0 - fraction);
    let right = item.x + item.w - item.skew * fraction;
    x >= left && x <= right
}

impl FilterBar<'_> {
    fn folder_anchor(&self) -> Option<&BarItem> {
        self.model.items.iter().find(|item| matches!(item.action, Some(BarAction::FolderToggle)))
    }

    pub(super) fn active_menu(&self) -> Option<MenuKind> {
        if self.menu_open {
            Some(MenuKind::Folders)
        } else if self.backend_menu_open {
            Some(MenuKind::Backends)
        } else {
            None
        }
    }

    pub(super) fn menu_len(&self) -> usize {
        match self.active_menu() {
            Some(MenuKind::Folders) => self.folder_options.len(),
            Some(MenuKind::Backends) => self.backend_options.len(),
            None => 0,
        }
    }

    fn menu_anchor(&self) -> Option<&BarItem> {
        match self.active_menu()? {
            MenuKind::Folders => self.folder_anchor(),
            MenuKind::Backends => self
                .model
                .items
                .iter()
                .find(|item| matches!(item.action, Some(BarAction::ThemeBackendToggle))),
        }
    }

    pub(super) fn menu_row_h(&self) -> f32 {
        MENU_ROW_H * self.scale
    }

    pub(super) fn menu_max_scroll(&self) -> f32 {
        let count = self.menu_len();
        let visible = count.min(MENU_MAX_ROWS);
        (count.saturating_sub(visible)) as f32 * self.menu_row_h()
    }

    pub(super) fn menu_rect(&self) -> Option<Rectangle> {
        let kind = self.active_menu()?;
        let anchor = self.menu_anchor()?;
        let row_height = self.menu_row_h();
        let visible = self.menu_len().min(MENU_MAX_ROWS) as f32;
        let height = 6.0 + visible * row_height + 8.0;
        let width = match kind {
            MenuKind::Folders => {
                let max_depth = self
                    .folder_options
                    .iter()
                    .map(|folder| folder_depth(folder))
                    .max()
                    .unwrap_or(0);
                170.0 * self.scale + (max_depth.min(6) as f32) * 16.0 * self.scale
            }
            MenuKind::Backends => 150.0 * self.scale,
        }
        .min(self.model.width.max(120.0));
        Some(Rectangle {
            x: anchor.x.min(self.model.width - width).max(0.0),
            y: if self.model.menu_up { 0.0 } else { self.model.height - height },
            width,
            height,
        })
    }

    pub(super) fn menu_index_at(&self, x: f32, y: f32) -> Option<usize> {
        let rectangle = self.menu_rect()?;
        if x < rectangle.x
            || x > rectangle.x + rectangle.width
            || y < rectangle.y + 4.0
            || y > rectangle.y + rectangle.height
        {
            return None;
        }
        let index = ((y - rectangle.y - 4.0 + self.menu_scroll) / self.menu_row_h()) as usize;
        (index < self.menu_len()).then_some(index)
    }

    pub(super) fn hit(&self, x: f32, y: f32) -> Option<usize> {
        hit_item(&self.model, self.visual_style, x, y)
    }

    pub(super) fn over_menu(&self, position: Point) -> bool {
        let Some(rectangle) = self.menu_rect() else {
            return false;
        };
        position.x >= rectangle.x
            && position.x <= rectangle.x + rectangle.width
            && position.y >= rectangle.y
            && position.y <= rectangle.y + rectangle.height
    }
}

fn hit_item(model: &BarModel, style: BarVisualStyle, x: f32, y: f32) -> Option<usize> {
    let mut best: Option<(i32, usize)> = None;
    for (index, item) in model.items.iter().enumerate() {
        if item.action.is_some()
            && visual_item_contains(item, style, x, y)
            && best.is_none_or(|(max_depth, _)| item.z >= max_depth)
        {
            best = Some((item.z, index));
        }
    }
    best.map(|(_, index)| index)
}

fn visual_item_contains(item: &BarItem, style: BarVisualStyle, x: f32, y: f32) -> bool {
    if style == BarVisualStyle::Slices {
        item_contains(item, x, y)
    } else {
        y >= item.y
            && y <= item.y + item.h
            && x >= item.x
            && x <= item.x + (item.w - item.skew).max(1.0)
    }
}
