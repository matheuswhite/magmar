use crate::{drawable::Drawable, screen::Screen};
use ggez::{
    glam::Vec2,
    graphics::{DrawParam, Mesh},
};

pub struct Toolbar {
    size: Vec2,
}

impl Toolbar {
    pub const HEIGHT_PERCENT: f32 = 0.05;

    pub fn new(screen: &Screen) -> Self {
        Self {
            size: Vec2 {
                x: screen.width + Screen::SCREEN_WIDTH_OFFSET,
                y: screen.height * Self::HEIGHT_PERCENT,
            },
        }
    }
}

impl Drawable for Toolbar {
    fn draw(
        &self,
        position: Vec2,
        canvas: &mut ggez::graphics::Canvas,
        ctx: &mut ggez::Context,
        theme: crate::theme::Theme,
    ) {
        let rect = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(position.x, position.y, self.size.x, self.size.y),
            theme.control_strong(),
        )
        .unwrap();
        canvas.draw(&rect, DrawParam::default());
    }
}
