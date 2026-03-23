use crate::{
    drawable::Drawable,
    signal::{Signal, SignalCoords},
    tooltip::TooltipDot,
    viewport::Viewport,
};
use ggez::{
    glam::Vec2,
    graphics::{DrawMode, DrawParam, Mesh, Rect, Text},
};

pub struct Aim {
    signal_index: usize,
    viewport_size: Vec2,
    mouse: Vec2,
    signals: Vec<Signal>,
}

impl Aim {
    pub fn new(viewport: &Viewport) -> Self {
        Self {
            signal_index: 0,
            viewport_size: viewport.size(),
            mouse: Vec2::ZERO,
            signals: vec![],
        }
    }

    pub fn max(&self) -> SignalCoords {
        self.signals.iter().fold(
            SignalCoords {
                x: f32::MIN,
                y: f32::MIN,
            },
            |max, signal| SignalCoords {
                x: max.x.max(signal.max.x),
                y: max.y.max(signal.max.y),
            },
        )
    }

    pub fn min(&self) -> SignalCoords {
        self.signals.iter().fold(
            SignalCoords {
                x: f32::MAX,
                y: f32::MAX,
            },
            |min, signal| SignalCoords {
                x: min.x.min(signal.min.x),
                y: min.y.min(signal.min.y),
            },
        )
    }

    pub fn set_mouse(&mut self, pos: Vec2) {
        self.mouse = pos;
    }

    pub fn is_mouse_inside_viewport(&self, position: Vec2) -> bool {
        let relative_mouse = self.mouse - position;
        relative_mouse.x >= 0.0
            && relative_mouse.y >= 0.0
            && relative_mouse.x <= self.viewport_size.x
            && relative_mouse.y <= self.viewport_size.y
    }

    pub fn next_signal(&mut self) {
        self.signal_index = (self.signal_index + 1) % self.signals.len();
    }

    pub fn mark_tooltip(&mut self, viewport_position: Vec2) {
        if self.signals.is_empty() {
            return;
        }

        if !self.is_mouse_inside_viewport(viewport_position) {
            return;
        }

        let max = self.max();
        let min = self.min();

        let signal = &mut self.signals[self.signal_index];

        let (pos, value) = TooltipDot::get_position_and_value(
            self.mouse,
            viewport_position,
            self.viewport_size,
            signal,
            max,
            min,
        )
        .unwrap();

        signal.mark_tooltip(pos, value.y);
    }

    pub fn remove_tooltip(&mut self, viewport_position: Vec2) {
        if self.signals.is_empty() {
            return;
        }

        if !self.is_mouse_inside_viewport(viewport_position) {
            return;
        }

        let max = self.max();
        let min = self.min();

        let signal = &mut self.signals[self.signal_index];

        let (pos, _) = TooltipDot::get_position_and_value(
            self.mouse,
            viewport_position,
            self.viewport_size,
            signal,
            max,
            min,
        )
        .unwrap();

        signal.remove_tooltip(pos);
    }

    pub fn signals_mut(&mut self) -> &mut Vec<Signal> {
        &mut self.signals
    }

    pub fn signals(&self) -> &[Signal] {
        &self.signals
    }
}

impl Drawable for Aim {
    fn draw(
        &self,
        position: Vec2,
        canvas: &mut ggez::graphics::Canvas,
        ctx: &mut ggez::Context,
        theme: crate::theme::Theme,
    ) {
        if !self.is_mouse_inside_viewport(position) {
            return;
        }

        if self.signals.is_empty() {
            return;
        }

        let color = self.signals[self.signal_index].color;
        let Some((pos, value)) = TooltipDot::get_position_and_value(
            self.mouse,
            position,
            self.viewport_size,
            &self.signals[self.signal_index],
            self.max(),
            self.min(),
        ) else {
            return;
        };

        let vertical_line = Mesh::new_line(
            ctx,
            &[
                position
                    + Vec2 {
                        x: self.mouse.x - position.x,
                        y: 0.0,
                    },
                position
                    + Vec2 {
                        x: self.mouse.x - position.x,
                        y: self.viewport_size.y,
                    },
            ],
            1.0,
            color,
        )
        .unwrap();
        canvas.draw(&vertical_line, DrawParam::new());

        let horizontal_line = Mesh::new_line(
            ctx,
            &[
                Vec2 {
                    x: position.x,
                    y: pos.y,
                },
                Vec2 {
                    x: position.x + self.viewport_size.x,
                    y: pos.y,
                },
            ],
            1.0,
            color,
        )
        .unwrap();
        canvas.draw(&horizontal_line, DrawParam::new());

        {
            let y_offset = Vec2 { x: 10.0, y: 5.0 };
            let y_text = if value.y != 0.0 && (value.y < 0.01 || value.y >= 100.0) {
                format!("{:.2e}", value.y)
            } else {
                format!("{:.2}", value.y)
            };
            let y_text = Text::new(y_text);
            let y_background_size = Vec2::from(y_text.measure(ctx).unwrap()) + y_offset;
            let y_background = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(
                    position.x - y_background_size.x - 5.0,
                    pos.y - y_background_size.y / 2.0,
                    y_background_size.x,
                    y_background_size.y,
                ),
                color,
            )
            .unwrap();
            canvas.draw(&y_background, DrawParam::new());
            canvas.draw(
                &y_text,
                DrawParam::new()
                    .dest(Vec2 {
                        x: position.x - y_background_size.x - 5.0 + y_offset.x / 2.0,
                        y: pos.y - y_background_size.y / 2.0 + y_offset.y / 2.0,
                    })
                    .color(theme.background()),
            );
        }

        {
            let x_offset = Vec2 { x: 10.0, y: 5.0 };
            let x_text = if value.x != 0.0 && (value.x < 0.01 || value.x >= 100.0) {
                format!("{:.2e}", value.x)
            } else {
                format!("{:.2}", value.x)
            };
            let x_text = Text::new(x_text);
            let x_background_size = Vec2::from(x_text.measure(ctx).unwrap()) + x_offset;
            let x_background = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(
                    pos.x - x_background_size.x / 2.0,
                    position.y + self.viewport_size.y + 5.0,
                    x_background_size.x,
                    x_background_size.y,
                ),
                color,
            )
            .unwrap();
            canvas.draw(&x_background, DrawParam::new());
            canvas.draw(
                &x_text,
                DrawParam::new()
                    .dest(Vec2 {
                        x: pos.x - x_background_size.x / 2.0 + x_offset.x / 2.0,
                        y: position.y + self.viewport_size.y + 5.0 + x_offset.y / 2.0,
                    })
                    .color(theme.background()),
            );
        }
    }
}
