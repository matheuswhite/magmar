use crate::{drawable::Drawable, viewport::Viewport};
use ggez::{
    glam::Vec2,
    graphics::{Color, DrawMode, DrawParam, Mesh, Rect, Text},
};

pub struct Legend {
    names: Vec<String>,
    colors: Vec<Color>,
    position: LegendPosition,
    size: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub enum LegendPosition {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Legend {
    pub fn new(viewport: &Viewport) -> Self {
        Self {
            names: vec![],
            colors: vec![],
            position: LegendPosition::TopRight,
            size: viewport.size(),
        }
    }

    pub fn add_signal(&mut self, name: impl AsRef<str>, color: Color) {
        self.names.push(name.as_ref().to_string());
        self.colors.push(color);
    }

    pub fn set_position(&mut self, pos: LegendPosition) {
        self.position = pos;
    }
}

impl Drawable for Legend {
    fn draw(
        &self,
        position: ggez::glam::Vec2,
        canvas: &mut ggez::graphics::Canvas,
        ctx: &mut ggez::Context,
        theme: crate::theme::Theme,
    ) {
        if self.names.is_empty() {
            return;
        }

        let texts = self
            .names
            .iter()
            .map(|name| Text::new(name))
            .collect::<Vec<_>>();
        let text_offset_x = 25.0;
        let padding_x = 5.0;
        let padding_y = 2.0;
        let max_width = texts
            .iter()
            .map(|text| text.measure(ctx).unwrap().x)
            .fold(0.0, f32::max)
            + padding_x * 2.0
            + text_offset_x;
        let height_per_text = texts[0].measure(ctx).unwrap().y + padding_y * 2.0;

        let offset = 10.0;
        let position = match self.position {
            LegendPosition::TopLeft => {
                position
                    + Vec2 {
                        x: offset,
                        y: offset,
                    }
            }
            LegendPosition::Top => {
                position
                    + Vec2 {
                        x: self.size.x / 2.0 - max_width / 2.0,
                        y: offset,
                    }
            }
            LegendPosition::TopRight => {
                position
                    + Vec2 {
                        x: self.size.x - max_width - offset,
                        y: offset,
                    }
            }
            LegendPosition::Left => {
                position
                    + Vec2 {
                        x: offset,
                        y: self.size.y / 2.0 - height_per_text * texts.len() as f32,
                    }
            }
            LegendPosition::Right => {
                position
                    + Vec2 {
                        x: self.size.x - max_width - offset,
                        y: self.size.y / 2.0 - height_per_text * texts.len() as f32,
                    }
            }
            LegendPosition::BottomLeft => {
                position
                    + Vec2 {
                        x: offset,
                        y: self.size.y - height_per_text * texts.len() as f32 - offset,
                    }
            }
            LegendPosition::Bottom => {
                position
                    + Vec2 {
                        x: self.size.x / 2.0 - max_width / 2.0,
                        y: self.size.y - height_per_text * texts.len() as f32 - offset,
                    }
            }
            LegendPosition::BottomRight => {
                position
                    + Vec2 {
                        x: self.size.x - max_width - offset,
                        y: self.size.y - height_per_text * texts.len() as f32 - offset,
                    }
            }
        };

        let background = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(
                position.x,
                position.y,
                max_width,
                height_per_text * texts.len() as f32,
            ),
            theme.control_weak(),
        )
        .unwrap();
        canvas.draw(&background, DrawParam::new());

        for (i, (text, color)) in texts.iter().zip(&self.colors).enumerate() {
            let dest_pos = position
                + Vec2 {
                    x: padding_x + text_offset_x,
                    y: padding_y + i as f32 * height_per_text,
                };
            canvas.draw(text, DrawParam::new().dest(dest_pos).color(*color));

            let text_height = text.measure(ctx).unwrap().y;
            let line = Mesh::new_line(
                ctx,
                &[
                    position
                        + Vec2 {
                            x: padding_x,
                            y: padding_y + i as f32 * height_per_text + text_height / 2.0,
                        },
                    position
                        + Vec2 {
                            x: padding_x + text_offset_x - 5.0,
                            y: padding_y + i as f32 * height_per_text + text_height / 2.0,
                        },
                ],
                2.0,
                *color,
            )
            .unwrap();
            canvas.draw(&line, DrawParam::new());
        }
    }
}
