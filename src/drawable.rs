use crate::theme::Theme;
use ggez::{glam::Vec2, graphics::Canvas};

pub trait Drawable {
    fn draw(&self, position: Vec2, canvas: &mut Canvas, ctx: &mut ggez::Context, theme: Theme);
}
