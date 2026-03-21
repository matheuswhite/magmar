use ggez::{
    glam::Vec2,
    graphics::{DrawParam, Text},
};

use crate::{drawable::Drawable, screen::Screen, viewport::Viewport};

pub struct XLabel {
    size: Vec2,
    text: Text,
}

impl XLabel {
    pub fn new(viewport: &Viewport, screen: &Screen, x_label: impl AsRef<str>) -> Self {
        Self {
            size: Vec2 {
                x: viewport.width,
                y: screen.height * 0.15,
            },
            text: Text::new(x_label.as_ref()),
        }
    }

    pub fn set_text(&mut self, text: impl AsRef<str>) {
        self.text = Text::new(text.as_ref());
    }
}

impl Drawable for XLabel {
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
                x: self.size.x / 2.0 - text_size.x / 2.0,
                y: self.size.y / 2.0 - text_size.y / 2.0,
            };

        canvas.draw(
            &self.text,
            DrawParam::new()
                .dest(dest_pos)
                .color(theme.control_strong()),
        );
    }
}
