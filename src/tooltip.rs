use crate::{
    screen::ScreenCoords,
    signal::{Signal, SignalCoords},
    viewport::Viewport,
};
use ggez::graphics::{Canvas, Color};

pub struct Tooltip {
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub color: Color,
}

impl Tooltip {
    pub fn new(name: String, color: Color) -> Self {
        Self {
            width: 100.0,
            height: 25.0,
            name,
            color,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas,
        ctx: &mut ggez::Context,
        mouse: ScreenCoords,
        height: f32,
        signal: &Signal,
        min: SignalCoords,
        max: SignalCoords,
        viewport: &Viewport,
    ) {
        if signal.points.is_empty() {
            return;
        }

        let mouse = ScreenCoords {
            x: mouse.x,
            y: height - mouse.y,
        };
        if !viewport.is_inside(mouse) {
            return;
        }

        let mouse_viewport = mouse.to_viewport(viewport);
        let mouse_signal = mouse_viewport.to_signal(signal, viewport);

        let mut min_step = f32::MAX;
        for i in 0..signal.points.len() - 1 {
            let p1 = signal.points[i];
            let p2 = signal.points[i + 1];

            let diff = (p1.x - p2.x).abs();
            if diff < min_step {
                min_step = diff;
            }
        }

        let mut closest_point = None;
        for point in &signal.points {
            let diff = (point.x - mouse_signal.x).abs();
            if diff <= min_step {
                closest_point = Some(*point);
                break;
            }
        }

        let Some(closest_point) = closest_point else {
            return;
        };

        let point_viewport = closest_point.to_viewport(viewport, min, max);
        let point_screen = point_viewport.to_screen(viewport);

        let point = ScreenCoords {
            x: mouse.x,
            y: height - point_screen.y,
        };
        let tooltip_pos = ScreenCoords {
            x: point.x - self.width / 2.0,
            y: point.y - self.height - 10.0,
        };

        let text = ggez::graphics::Text::new(format!("{}: {:.2}", self.name, closest_point.y));
        let text_dims = text.measure(ctx).unwrap();
        let width = text_dims.x + 10.0;

        let rect = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(tooltip_pos.x, tooltip_pos.y, width, self.height),
            Color::from_rgba(
                (255.0 * 0.2) as u8,
                (255.0 * 0.2) as u8,
                (255.0 * 0.2) as u8,
                255,
            ),
        )
        .unwrap();
        canvas.draw(&rect, ggez::graphics::DrawParam::default());

        let dest_point = ggez::mint::Point2 {
            x: tooltip_pos.x + 5.0,
            y: tooltip_pos.y + 5.0,
        };
        canvas.draw(
            &text,
            ggez::graphics::DrawParam::default()
                .dest(dest_point)
                .color(self.color),
        );
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn draw_point(
        &self,
        canvas: &mut Canvas,
        ctx: &mut ggez::Context,
        mouse: ScreenCoords,
        height: f32,
        signal: &Signal,
        min: SignalCoords,
        max: SignalCoords,
        viewport: &Viewport,
    ) {
        let mouse = ScreenCoords {
            x: mouse.x,
            y: height - mouse.y,
        };
        if !viewport.is_inside(mouse) {
            return;
        }

        let mouse_viewport = mouse.to_viewport(viewport);
        let mouse_signal = mouse_viewport.to_signal(signal, viewport);

        let mut min_step = f32::MAX;
        for i in 0..signal.points.len() - 1 {
            let p1 = signal.points[i];
            let p2 = signal.points[i + 1];

            let diff = (p1.x - p2.x).abs();
            if diff < min_step {
                min_step = diff;
            }
        }

        let mut closest_point = None;
        for point in &signal.points {
            let diff = (point.x - mouse_signal.x).abs();
            if diff <= min_step {
                closest_point = Some(*point);
                break;
            }
        }

        let Some(closest_point) = closest_point else {
            return;
        };

        let point_viewport = closest_point.to_viewport(viewport, min, max);
        let point_screen = point_viewport.to_screen(viewport);

        let point = ScreenCoords {
            x: mouse.x,
            y: height - point_screen.y,
        };

        let circ = ggez::graphics::Mesh::new_circle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::mint::Point2 {
                x: point.x,
                y: point.y,
            },
            5.0,
            0.1,
            self.color,
        )
        .unwrap();
        canvas.draw(&circ, ggez::graphics::DrawParam::default());
    }
}
