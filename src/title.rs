use crate::{drawable::Drawable, screen::Screen, theme::Theme};
use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam, Text},
};

pub struct Title {
    size: Vec2,
    text: Text,
    title: String,
}

impl Title {
    pub const HEIGHT_PERCENT: f32 = 0.1;

    pub fn new(screen: &Screen, title: impl AsRef<str>) -> Self {
        Self {
            size: Vec2 {
                x: screen.width,
                y: screen.height * Self::HEIGHT_PERCENT,
            },
            text: Text::new(title.as_ref()),
            title: title.as_ref().to_string(),
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        let text = if zoom == 100.0 {
            self.title.clone()
        } else {
            format!("{} ({}%)", self.title, zoom as usize)
        };

        self.text = Text::new(text);
    }
}

impl Drawable for Title {
    fn draw(&self, position: Vec2, canvas: &mut Canvas, ctx: &mut ggez::Context, theme: Theme) {
        let text_width = self.text.measure(ctx).unwrap().x;
        let dest_pos = position
            + Vec2 {
                x: self.size.x / 2.0 - text_width / 2.0,
                y: self.size.y / 2.0,
            };

        canvas.draw(
            &self.text,
            DrawParam::new()
                .dest(dest_pos)
                .color(theme.control_strong()),
        );
    }
}
