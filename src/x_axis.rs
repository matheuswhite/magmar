use crate::{drawable::Drawable, screen::Screen, viewport::Viewport};
use ggez::{
    glam::Vec2,
    graphics::{DrawParam, Mesh},
};

pub struct XAxis {
    size: Vec2,
    steps: usize,
    tick_len: f32,
    viewport_height: f32,
    min: f32,
    max: f32,
}

impl XAxis {
    pub const HEIGHT_PERCENT: f32 = 0.05;

    pub fn new(viewport: &Viewport, screen: &Screen, steps: usize) -> Self {
        Self {
            size: Vec2 {
                x: viewport.width,
                y: screen.height * Self::HEIGHT_PERCENT,
            },
            steps,
            tick_len: viewport.height * 0.02,
            viewport_height: viewport.height,
            max: 0.0,
            min: 0.0,
        }
    }

    pub fn set_min_max(&mut self, min: f32, max: f32) {
        self.min = min;
        self.max = max;
    }
}

impl Drawable for XAxis {
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
            let label = if value.abs() != 0.0 && (value.abs() < 0.01 || value.abs() >= 100.0) {
                format!("{:.2e}", value)
            } else {
                format!("{:.2}", value)
            };
            let text = ggez::graphics::Text::new(label);
            let text_width = text.measure(ctx).unwrap().x;
            let x_pos = i as f32 * self.size.x / self.steps as f32;
            let dest_pos = position
                + Vec2 {
                    x: x_pos - text_width / 2.0,
                    y: 0.0,
                };
            canvas.draw(
                &text,
                ggez::graphics::DrawParam::new()
                    .dest(dest_pos)
                    .color(theme.control_strong()),
            );

            let tick_bottom = Mesh::new_line(
                ctx,
                &[
                    position + Vec2 { x: x_pos, y: 0.0 },
                    position
                        + Vec2 {
                            x: x_pos,
                            y: -self.tick_len,
                        },
                ],
                1.0,
                theme.control_strong(),
            )
            .unwrap();
            canvas.draw(&tick_bottom, DrawParam::new());

            let tick_top = Mesh::new_line(
                ctx,
                &[
                    position
                        + Vec2 {
                            x: x_pos,
                            y: -self.viewport_height,
                        },
                    position
                        + Vec2 {
                            x: x_pos,
                            y: -self.viewport_height + self.tick_len,
                        },
                ],
                1.0,
                theme.control_strong(),
            )
            .unwrap();
            canvas.draw(&tick_top, DrawParam::new());
        }

        let bottom_bar = Mesh::new_line(
            ctx,
            &[
                position,
                position
                    + Vec2 {
                        x: self.size.x,
                        y: 0.0,
                    },
            ],
            1.0,
            theme.control_strong(),
        )
        .unwrap();
        canvas.draw(&bottom_bar, DrawParam::new());

        let top_bar = Mesh::new_line(
            ctx,
            &[
                position
                    + Vec2 {
                        x: 0.0,
                        y: -self.viewport_height,
                    },
                position
                    + Vec2 {
                        x: self.size.x,
                        y: -self.viewport_height,
                    },
            ],
            1.0,
            theme.control_strong(),
        )
        .unwrap();
        canvas.draw(&top_bar, DrawParam::new());
    }
}
