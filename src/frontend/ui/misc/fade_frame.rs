use iced::widget::canvas::{self, Frame, Path, Stroke, Text};
use iced::{Color, Rectangle, Vector};

fn mul_alpha(color: Color, factor: f32) -> Color {
    Color { a: color.a * factor, ..color }
}

pub struct FadeFrame<'a> {
    raw: &'a mut Frame,
    fade: f32,
}

impl<'a> FadeFrame<'a> {
    pub fn new(raw: &'a mut Frame, fade: f32) -> Self {
        Self { raw, fade }
    }

    pub fn fill(&mut self, path: &Path, color: Color) {
        self.raw.fill(path, mul_alpha(color, self.fade));
    }

    pub fn stroke(&mut self, path: &Path, stroke: Stroke<'_>) {
        let style = match stroke.style {
            canvas::Style::Solid(color) => canvas::Style::Solid(mul_alpha(color, self.fade)),
            other @ canvas::Style::Gradient(_) => other,
        };
        self.raw.stroke(path, Stroke { style, ..stroke });
    }

    pub fn fill_text(&mut self, text: Text) {
        let color = mul_alpha(text.color, self.fade);
        self.raw.fill_text(Text { color, ..text });
    }

    pub fn translate(&mut self, offset: Vector) {
        self.raw.translate(offset);
    }

    pub fn scale(&mut self, scale: f32) {
        self.raw.scale(scale);
    }

    pub fn with_save(&mut self, body: impl FnOnce(&mut FadeFrame<'_>)) {
        let fade = self.fade;
        self.raw.with_save(|frame| body(&mut FadeFrame { raw: frame, fade }));
    }

    pub fn with_clip(&mut self, rect: Rectangle, body: impl FnOnce(&mut FadeFrame<'_>)) {
        let fade = self.fade;
        self.raw.with_clip(rect, |frame| body(&mut FadeFrame { raw: frame, fade }));
    }
}
