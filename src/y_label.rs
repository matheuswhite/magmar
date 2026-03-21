use ggez::{
    glam::Vec2,
    graphics::{DrawParam, Text},
};

use crate::{drawable::Drawable, screen::Screen, viewport::Viewport};

pub struct YLabel {
    size: Vec2,
    text: Text,
}

impl YLabel {
    pub const WIDTH_PERCENT: f32 = 0.05;

    pub fn new(viewport: &Viewport, screen: &Screen, y_label: impl AsRef<str>) -> Self {
        Self {
            size: Vec2 {
                x: screen.width * Self::WIDTH_PERCENT,
                y: viewport.height,
            },
            text: Text::new(y_label.as_ref()),
        }
    }
}

impl Drawable for YLabel {
    fn draw(
        &self,
        position: Vec2,
        canvas: &mut ggez::graphics::Canvas,
        ctx: &mut ggez::Context,
        theme: crate::theme::Theme,
    ) {
        let text_size = self.text.measure(ctx).unwrap();
        let dest_pos = position
            + Vec2 {
                x: self.size.x / 2.0 - text_size.y / 2.0,
                y: self.size.y / 2.0 + text_size.x / 2.0,
            };

        canvas.draw(
            &self.text,
            DrawParam::new()
                .dest(dest_pos)
                .rotation(-std::f32::consts::FRAC_PI_2)
                .color(theme.control_strong()),
        );
    }
}
