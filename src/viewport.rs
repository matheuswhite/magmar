use crate::{
    drawable::Drawable, screen::Screen, title::Title, x_axis::XAxis, x_label::XLabel,
    y_axis::YAxis, y_label::YLabel,
};
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
            width: screen.width * (1.0 - YLabel::WIDTH_PERCENT - YAxis::WIDTH_PERCENT),
            height: screen.height
                * (1.0 - XLabel::HEIGHT_PERCENT - XAxis::HEIGHT_PERCENT - Title::HEIGHT_PERCENT),
        }
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    pub fn is_inside(&self, viewport_position: Vec2, position: Vec2) -> bool {
        let relative_pos = position - viewport_position;
        relative_pos.x >= 0.0
            && relative_pos.y >= 0.0
            && relative_pos.x <= self.width
            && relative_pos.y <= self.height
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
