use iced::widget::canvas::{self, Action, Frame, Stroke};
use iced::{Alignment, Event, Point, Rectangle, Size, mouse};

use crate::frontend::browser::{BrowserIntent, BrowserMsg};
use crate::frontend::theme::Palette;

use super::model::{BrowserAct, browser_bar_items_compact, is_order_action};
use crate::frontend::ui::bar::{BarItem, BarVisualStyle, draw_swatch_with_style, item_contains};
use crate::frontend::ui::{UI_FONT, mid_text, with_alpha};

pub struct BrowserBar<'a> {
    pub browser: &'a crate::frontend::browser::Browser,
    pub pal: &'a Palette,
    pub scale: f32,
    pub compact_width: f32,
}

impl BrowserBar<'_> {
    fn items(&self) -> std::rc::Rc<Vec<(super::super::bar::BarItem, BrowserAct)>> {
        browser_bar_items_compact(self.browser, self.scale, self.compact_width)
    }

    fn hit(items: &[(BarItem, BrowserAct)], position: Point) -> Option<usize> {
        let mut best: Option<(i32, usize)> = None;
        for (index, (item, action)) in items.iter().enumerate() {
            if !matches!(action, BrowserAct::Label)
                && item_contains(item, position.x, position.y)
                && best.is_none_or(|(z, _)| item.z > z)
            {
                best = Some((item.z, index));
            }
        }
        best.map(|(_, index)| index)
    }

    fn message(action: &BrowserAct) -> BrowserIntent {
        match action {
            BrowserAct::Close => BrowserIntent::Close,
            BrowserAct::Category(bit) => BrowserIntent::Update(BrowserMsg::ToggleCategory(*bit)),
            BrowserAct::Sort(key) => BrowserIntent::Update(BrowserMsg::SetSort((*key).to_string())),
            BrowserAct::Purity(purity) => BrowserIntent::Update(BrowserMsg::TogglePurity(*purity)),
            BrowserAct::Color(value) => BrowserIntent::Update(BrowserMsg::SetColor(*value)),
            BrowserAct::TopRange(key) => {
                BrowserIntent::Update(BrowserMsg::SetTopRange((*key).to_string()))
            }
            BrowserAct::ResMode => BrowserIntent::Update(BrowserMsg::ToggleResExact),
            BrowserAct::Atleast(key) => {
                BrowserIntent::Update(BrowserMsg::SetAtleast((*key).to_string()))
            }
            BrowserAct::Atmost(key) => {
                BrowserIntent::Update(BrowserMsg::SetAtmost((*key).to_string()))
            }
            BrowserAct::Label => BrowserIntent::Capture,
            BrowserAct::Ratios(key) => {
                BrowserIntent::Update(BrowserMsg::SetRatios((*key).to_string()))
            }
            BrowserAct::Collection(id) => {
                BrowserIntent::Update(BrowserMsg::SetCollection(id.clone()))
            }
            BrowserAct::MaxDuration(key) => {
                BrowserIntent::Update(BrowserMsg::SetMaxDuration((*key).to_string()))
            }
            BrowserAct::SteamFilter(kind, value) => {
                BrowserIntent::Update(BrowserMsg::SteamFilter((*kind).to_string(), value.clone()))
            }
            BrowserAct::CatalogFilter(kind, value) => {
                BrowserIntent::Update(BrowserMsg::CatalogFilter((*kind).to_string(), value.clone()))
            }
        }
    }
}

impl canvas::Program<BrowserIntent> for BrowserBar<'_> {
    type State = ();

    fn update(
        &self,
        _state: &mut (),
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<BrowserIntent>> {
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            let position = cursor.position_in(bounds)?;
            let items = self.items();
            if let Some(index) = Self::hit(&items, position) {
                return Some(Action::publish(Self::message(&items[index].1)).and_capture());
            }
        }
        None
    }

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let items = self.items();
        let mut order: Vec<usize> = (0..items.len()).collect();
        order.sort_by_key(|&index| items[index].0.z);
        let hovered_index =
            cursor.position_in(bounds).and_then(|position| Self::hit(&items, position));
        {
            let faded = &mut crate::frontend::ui::FadeFrame::new(&mut frame, 1.0);
            for index in order {
                let item = &items[index].0;
                let hovered = hovered_index == Some(index);
                draw_folio_item(faded, item, &items[index].1, hovered, self.pal, self.scale);
            }
        }
        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &(),
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if let Some(position) = cursor.position_in(bounds)
            && Self::hit(&self.items(), position).is_some()
        {
            return mouse::Interaction::Pointer;
        }
        mouse::Interaction::default()
    }
}

mod tests;

fn draw_folio_item(
    frame: &mut crate::frontend::ui::FadeFrame<'_>,
    item: &BarItem,
    action: &BrowserAct,
    hovered: bool,
    palette: &Palette,
    scale: f32,
) {
    if matches!(action, BrowserAct::Label) {
        frame.fill_text(mid_text(
            item.label.clone(),
            Point::new(item.x, item.y + item.h / 2.0),
            with_alpha(palette.surface_text, 0.42),
            item.text_size,
            UI_FONT,
            Alignment::Start,
        ));
        let line_x = (item.x + 92.0 * scale).min(item.x + item.w);
        if line_x < item.x + item.w {
            let line = canvas::Path::line(
                Point::new(line_x, item.y + item.h / 2.0),
                Point::new(item.x + item.w, item.y + item.h / 2.0),
            );
            frame.stroke(
                &line,
                Stroke::default().with_color(with_alpha(palette.outline, 0.28)).with_width(1.0),
            );
        }
        return;
    }
    if let Some(index) = item.swatch {
        draw_swatch_with_style(frame, item, index, hovered, palette, BarVisualStyle::Wall);
        return;
    }

    if is_order_action(action) {
        let bounds = canvas::Path::rectangle(Point::new(item.x, item.y), Size::new(item.w, item.h));
        if hovered {
            frame.fill(&bounds, with_alpha(palette.surface_variant, 0.3));
        }
        if item.active {
            let marker = canvas::Path::rectangle(
                Point::new(item.x, item.y + 4.0 * scale),
                Size::new(2.0 * scale, item.h - 8.0 * scale),
            );
            frame.fill(&marker, palette.primary);
        }
        frame.fill_text(mid_text(
            item.label.clone(),
            Point::new(item.x + 8.0 * scale, item.y + item.h / 2.0),
            if item.active {
                palette.primary
            } else {
                with_alpha(palette.surface_text, if hovered { 0.88 } else { 0.54 })
            },
            item.text_size,
            if item.nerd { crate::frontend::ui::NERD_FONT } else { UI_FONT },
            Alignment::Start,
        ));
        return;
    }

    let bounds = canvas::Path::rectangle(Point::new(item.x, item.y), Size::new(item.w, item.h));
    if item.active {
        frame.fill(&bounds, with_alpha(palette.primary, 0.14));
        let marker = canvas::Path::rectangle(
            Point::new(item.x, item.y + item.h - 2.0 * scale),
            Size::new(item.w, 2.0 * scale),
        );
        frame.fill(&marker, palette.primary);
    } else if hovered {
        frame.fill(&bounds, with_alpha(palette.surface_variant, 0.34));
    }
    let divider = canvas::Path::line(
        Point::new(item.x, item.y + item.h),
        Point::new(item.x + item.w, item.y + item.h),
    );
    frame.stroke(
        &divider,
        Stroke::default()
            .with_color(with_alpha(palette.outline, if item.active { 0.08 } else { 0.22 }))
            .with_width(1.0),
    );
    frame.fill_text(mid_text(
        item.label.clone(),
        Point::new(item.x + 7.0 * scale, item.y + item.h / 2.0),
        if item.active {
            palette.primary
        } else {
            with_alpha(palette.surface_text, if hovered { 0.9 } else { 0.64 })
        },
        item.text_size,
        if item.nerd { crate::frontend::ui::NERD_FONT } else { UI_FONT },
        Alignment::Start,
    ));
}
