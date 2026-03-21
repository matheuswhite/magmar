use crate::{drawable::Drawable, screen::Screen, viewport::Viewport};
use ggez::{
    glam::Vec2,
    graphics::{DrawParam, Mesh},
};

pub struct YAxis {
    size: Vec2,
    steps: usize,
    tick_len: f32,
    viewport_width: f32,
    min: f32,
    max: f32,
}

impl YAxis {
    pub const WIDTH_PERCENT: f32 = 0.15;

    pub fn new(viewport: &Viewport, screen: &Screen, steps: usize) -> Self {
        Self {
            size: Vec2 {
                x: screen.width * Self::WIDTH_PERCENT,
                y: viewport.height,
            },
            steps,
            tick_len: viewport.width * 0.02,
            viewport_width: viewport.width,
            max: 0.0,
            min: 0.0,
        }
    }

    pub fn set_min_max(&mut self, min: f32, max: f32) {
        self.min = min;
        self.max = max;
    }
}

impl Drawable for YAxis {
    fn draw(
        &self,
        position: Vec2,
        canvas: &mut ggez::graphics::Canvas,
        ctx: &mut ggez::Context,
        theme: crate::theme::Theme,
    ) {
        let step = (self.max - self.min) / self.steps as f32;

        for i in 0..=self.steps {
            let value = self.min + i as f32 * step;
            let label = if value != 0.0 && (value < 0.01 || value >= 100.0) {
                format!("{:.2e}", value)
            } else {
                format!("{:.2}", value)
            };
            let text = ggez::graphics::Text::new(label);
            let text_size = text.measure(ctx).unwrap();
            let y_pos = self.size.y - (i as f32 * self.size.y / self.steps as f32);
            let dest_pos = position
                + Vec2 {
                    x: self.size.x - text_size.x - 5.0,
                    y: y_pos - text_size.y,
                };
            canvas.draw(
                &text,
                ggez::graphics::DrawParam::new()
                    .dest(dest_pos)
                    .color(theme.control_strong()),
            );

            let tick_left = Mesh::new_line(
                ctx,
                &[
                    position
                        + Vec2 {
                            x: self.size.x,
                            y: y_pos,
                        },
                    position
                        + Vec2 {
                            x: self.size.x + self.tick_len,
                            y: y_pos,
                        },
                ],
                1.0,
                theme.control_strong(),
            )
            .unwrap();
            canvas.draw(&tick_left, DrawParam::new());

            let tick_right = Mesh::new_line(
                ctx,
                &[
                    position
                        + Vec2 {
                            x: self.size.x + self.viewport_width,
                            y: y_pos,
                        },
                    position
                        + Vec2 {
                            x: self.size.x + self.viewport_width - self.tick_len,
                            y: y_pos,
                        },
                ],
                1.0,
                theme.control_strong(),
            )
            .unwrap();
            canvas.draw(&tick_right, DrawParam::new());
        }

        let left_bar = Mesh::new_line(
            ctx,
            &[
                position
                    + Vec2 {
                        x: self.size.x,
                        y: 0.0,
                    },
                position
                    + Vec2 {
                        x: self.size.x,
                        y: self.size.y,
                    },
            ],
            1.0,
            theme.control_strong(),
        )
        .unwrap();
        canvas.draw(&left_bar, DrawParam::new());

        let right_bar = Mesh::new_line(
            ctx,
            &[
                position
                    + Vec2 {
                        x: self.size.x + self.viewport_width,
                        y: 0.0,
                    },
                position
                    + Vec2 {
                        x: self.size.x + self.viewport_width,
                        y: self.size.y,
                    },
            ],
            1.0,
            theme.control_strong(),
        )
        .unwrap();
        canvas.draw(&right_bar, DrawParam::new());
    }
}
