use crate::{screen::Screen, signal::SignalCoords, theme::Theme, viewport::Viewport};
use ggez::{
    glam::Vec2,
    graphics::{Canvas, DrawParam, Drawable, Mesh, Rect, Text},
};

pub struct Grid {
    pub steps: (usize, usize),
    pub x_label: String,
    pub y_label: String,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            steps: (5, 5),
            x_label: "Time (s)".to_string(),
            y_label: "Signals".to_string(),
        }
    }
}

impl Grid {
    pub fn set_x_label(&mut self, label: String) {
        self.x_label = label;
    }

    fn draw_y_axis(
        &self,
        canvas: &mut Canvas,
        ctx: &mut ggez::Context,
        min: SignalCoords,
        max: SignalCoords,
        viewport: &Viewport,
        screen: &Screen,
        theme: Theme,
    ) -> Result<(), ggez::GameError> {
        let step = (max.y - min.y) / self.steps.1 as f32;

        for i in 0..=self.steps.1 {
            let grid_len = if i == 0 {
                viewport.width
            } else {
                viewport.width * 0.01
            };

            let i = i as f32 * step + min.y;
            let normalized_y = (i - min.y) / (max.y - min.y) * viewport.height + viewport.y;

            let text = Text::new(format!("{:.2}", i));
            let dest_point = screen.fix_coords(viewport.x - 40.0, normalized_y + 10.0);
            canvas.draw(
                &text,
                DrawParam::new()
                    .dest(dest_point)
                    .color(theme.control_strong()),
            );

            let line = Mesh::new_line(
                ctx,
                &[
                    screen.fix_coords(viewport.x, normalized_y),
                    screen.fix_coords(viewport.x + grid_len, normalized_y),
                ],
                1.0,
                theme.control_strong(),
            )?;
            line.draw(canvas, DrawParam::default());
        }

        let signals = Text::new(&self.y_label);
        let dest_point = Vec2 {
            x: 15.0,
            y: screen.height / 2.0,
        };
        canvas.draw(
            &signals,
            DrawParam::new()
                .dest(dest_point)
                .color(theme.control_strong())
                .rotation(-std::f32::consts::FRAC_PI_2),
        );

        Ok(())
    }

    fn draw_x_axis(
        &self,
        canvas: &mut Canvas,
        ctx: &mut ggez::Context,
        min: SignalCoords,
        max: SignalCoords,
        viewport: &Viewport,
        screen: &Screen,
        theme: Theme,
    ) -> Result<(), ggez::GameError> {
        let step = (max.x - min.x) / self.steps.0 as f32;

        for i in 0..=self.steps.0 {
            let grid_len = if i == 0 {
                viewport.height
            } else {
                viewport.height * 0.01
            };

            let i = i as f32 * step + min.x;
            let normalized_x = (i - min.x) / (max.x - min.x) * viewport.width + viewport.x;
            let text = Text::new(format!("{:.2}", i));
            let dest_point = screen.fix_coords(normalized_x - 10.0, viewport.y - 10.0);

            canvas.draw(
                &text,
                DrawParam::new()
                    .dest(dest_point)
                    .color(theme.control_strong()),
            );

            let line = Mesh::new_line(
                ctx,
                &[
                    screen.fix_coords(normalized_x, viewport.y),
                    screen.fix_coords(normalized_x, viewport.y + grid_len),
                ],
                1.0,
                theme.control_strong(),
            )?;
            line.draw(canvas, DrawParam::default());
        }

        let time = Text::new(&self.x_label);
        let dest_point = screen.fix_coords(screen.width / 2.0, 35.0);
        canvas.draw(
            &time,
            DrawParam::new()
                .dest(dest_point)
                .color(theme.control_strong()),
        );

        Ok(())
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas,
        ctx: &mut ggez::Context,
        min: SignalCoords,
        max: SignalCoords,
        viewport: &Viewport,
        screen: &Screen,
        theme: Theme,
    ) -> Result<(), ggez::GameError> {
        let grid_pos = screen.fix_coords(viewport.x, viewport.y + viewport.height);
        let grid_background = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            Rect::new(grid_pos.x, grid_pos.y, viewport.width, viewport.height),
            theme.background(),
        )?;
        grid_background.draw(canvas, DrawParam::default());

        self.draw_y_axis(canvas, ctx, min, max, viewport, screen, theme)?;
        self.draw_x_axis(canvas, ctx, min, max, viewport, screen, theme)?;

        Ok(())
    }
}
