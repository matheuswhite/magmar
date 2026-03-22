use crate::{
    drawable::Drawable,
    screen::ScreenCoords,
    signal::{Signal, SignalCoords},
    theme::Theme,
};
use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color},
};

pub struct Tooltip {
    pub value: f32,
    pub width: f32,
    pub height: f32,
    pub name: String,
    pub color: Color,
}

pub struct TooltipDot {
    color: Color,
}

impl Tooltip {
    pub fn new(name: String, color: Color) -> Self {
        Self {
            width: 100.0,
            height: 25.0,
            name,
            color,
            value: 0.0,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl TooltipDot {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn is_inside_viewport(mouse: Vec2, position: Vec2, viewport_size: Vec2) -> bool {
        mouse.x >= position.x
            && mouse.x <= position.x + viewport_size.x
            && mouse.y >= position.y
            && mouse.y <= position.y + viewport_size.y
    }

    pub fn get_position_and_value(
        mouse: Vec2,
        viewport_pos: Vec2,
        viewport_size: Vec2,
        signal: &Signal,
        max: SignalCoords,
        min: SignalCoords,
    ) -> Option<(Vec2, f32)> {
        if signal.points.is_empty() {
            return None;
        }

        if !Self::is_inside_viewport(mouse, viewport_pos, viewport_size) {
            return None;
        }

        let mouse_viewport = mouse - viewport_pos
            + Vec2 {
                x: 0.0,
                y: viewport_size.y,
            };
        let mouse_viewport_normalized = Vec2 {
            x: mouse_viewport.x / viewport_size.x,
            y: mouse_viewport.y / viewport_size.y,
        };
        let mouse_signal_t =
            mouse_viewport_normalized.x * (signal.max.x - signal.min.x) + signal.min.x;

        let closest_point = SignalCoords {
            x: mouse_signal_t,
            y: signal.value_at(mouse_signal_t),
        };

        let point_normalized = Vec2 {
            x: (closest_point.x - min.x) / (max.x - min.x),
            y: (closest_point.y - min.y) / (max.y - min.y),
        };
        let point_viewport = Vec2 {
            x: point_normalized.x * viewport_size.x,
            y: (1.0 - point_normalized.y) * viewport_size.y,
        };
        let point_screen = point_viewport + viewport_pos;

        Some((point_screen, closest_point.y))
    }
}

impl Drawable for Tooltip {
    fn draw(&self, position: Vec2, canvas: &mut Canvas, ctx: &mut ggez::Context, theme: Theme) {
        let tooltip_pos = ScreenCoords {
            x: position.x - self.width / 2.0,
            y: position.y - self.height - 10.0,
        };

        let value = if self.value != 0.0 && (self.value < 0.01 || self.value >= 100.0) {
            format!("{:.2e}", self.value)
        } else {
            format!("{:.2}", self.value)
        };
        let text = ggez::graphics::Text::new(format!("{}: {}", self.name, value));
        let text_dims = text.measure(ctx).unwrap();
        let width = text_dims.x + 10.0;

        let rect = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(tooltip_pos.x, tooltip_pos.y, width, self.height),
            theme.control_weak(),
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
}

impl Drawable for TooltipDot {
    fn draw(&self, position: Vec2, canvas: &mut Canvas, ctx: &mut ggez::Context, _theme: Theme) {
        let circ = ggez::graphics::Mesh::new_circle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            Vec2 {
                x: position.x,
                y: position.y,
            },
            5.0,
            0.1,
            self.color,
        )
        .unwrap();
        canvas.draw(&circ, ggez::graphics::DrawParam::default());
    }
}
