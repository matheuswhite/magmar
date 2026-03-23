use crate::{
    drawable::Drawable,
    screen::{Screen, ScreenCoords},
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
    pub color: Color,
    screen_width: f32,
}

pub struct TooltipDot {
    color: Color,
}

impl Tooltip {
    pub fn new(color: Color, screen_width: f32, value: f32) -> Self {
        Self {
            width: 100.0,
            height: 25.0,
            color,
            value,
            screen_width,
        }
    }
}

impl TooltipDot {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn get_position_and_value(
        mouse: Vec2,
        viewport_pos: Vec2,
        viewport_size: Vec2,
        signal: &Signal,
        max: SignalCoords,
        min: SignalCoords,
    ) -> Option<(Vec2, SignalCoords)> {
        if signal.points.is_empty() {
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

        Some((point_screen, closest_point))
    }
}

impl Drawable for Tooltip {
    fn draw(&self, position: Vec2, canvas: &mut Canvas, ctx: &mut ggez::Context, theme: Theme) {
        let mut tooltip_pos = ScreenCoords {
            x: position.x - self.width / 2.0,
            y: position.y - self.height - 10.0,
        };

        let value =
            if self.value.abs() != 0.0 && (self.value.abs() < 0.01 || self.value.abs() >= 100.0) {
                format!("{:.2e}", self.value)
            } else {
                format!("{:.2}", self.value)
            };
        let text = ggez::graphics::Text::new(value);
        let text_dims = text.measure(ctx).unwrap();
        let width = text_dims.x + 10.0;

        if tooltip_pos.x + width >= self.screen_width + Screen::SCREEN_WIDTH_OFFSET {
            let overflow =
                tooltip_pos.x + width - (self.screen_width + Screen::SCREEN_WIDTH_OFFSET);
            tooltip_pos.x -= overflow;
        }

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
        let color_offset = 0.15;
        let circ = ggez::graphics::Mesh::new_circle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            Vec2 {
                x: position.x,
                y: position.y,
            },
            5.0,
            0.1,
            Color::new(
                self.color.r - color_offset,
                self.color.g - color_offset,
                self.color.b - color_offset,
                1.0,
            ),
        )
        .unwrap();
        canvas.draw(&circ, ggez::graphics::DrawParam::default());
    }
}
