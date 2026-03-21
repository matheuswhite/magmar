use crate::{drawable::Drawable, screen::Screen};
use ggez::{
    glam::Vec2,
    graphics::{DrawParam, Mesh, Rect},
};

pub struct Viewport {
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    pub fn new(screen: &Screen) -> Self {
        Self {
            width: screen.width * 0.7,
            height: screen.height * (0.6 * 0.7),
        }
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

impl Drawable for Viewport {
    fn draw(
        &self,
        position: ggez::glam::Vec2,
        canvas: &mut ggez::graphics::Canvas,
        ctx: &mut ggez::Context,
        theme: crate::theme::Theme,
    ) {
        let background = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            Rect::new(position.x, position.y, self.width, self.height),
            theme.background(),
        )
        .unwrap();
        canvas.draw(&background, DrawParam::default());
    }
}
