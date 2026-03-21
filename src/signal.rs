use crate::{
    drawable::Drawable,
    theme::Theme,
    tooltip::{Tooltip, TooltipDot},
    viewport::Viewport,
};
use core::f32;
use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color},
};

pub struct Signal {
    pub color: Color,
    pub points: Vec<SignalCoords>,
    pub min: SignalCoords,
    pub max: SignalCoords,
    global_max: SignalCoords,
    global_min: SignalCoords,
    pub name: String,
    size: Vec2,
    pub tooltip: Tooltip,
    pub tooltip_dot: TooltipDot,
}

#[derive(Clone, Copy)]
pub struct SignalCoords {
    pub x: f32,
    pub y: f32,
}

impl Signal {
    pub fn new(index: usize, viweport: &Viewport, theme: Theme) -> Self {
        let name = format!("Y{}", index + 1);
        let color = theme.gen_color(index);

        Self {
            color,
            points: Vec::new(),
            min: SignalCoords {
                x: f32::MAX,
                y: f32::MAX,
            },
            max: SignalCoords {
                x: f32::MIN,
                y: f32::MIN,
            },
            global_max: SignalCoords { x: 0.0, y: 0.0 },
            global_min: SignalCoords { x: 0.0, y: 0.0 },
            name: name.clone(),
            size: Vec2 {
                x: viweport.width,
                y: viweport.height,
            },
            tooltip: Tooltip::new(name, color),
            tooltip_dot: TooltipDot::new(color),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name.clone();
        self.tooltip.set_name(name);
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        let point = SignalCoords { x, y };
        self.points.push(point);

        if point.x < self.min.x {
            self.min.x = point.x;
        }
        if point.y < self.min.y {
            self.min.y = point.y;
        }
        if point.x > self.max.x {
            self.max.x = point.x;
        }
        if point.y > self.max.y {
            self.max.y = point.y;
        }
    }

    pub fn set_global_max_min(&mut self, global_max: SignalCoords, global_min: SignalCoords) {
        self.global_max = global_max;
        self.global_min = global_min;
    }

    pub fn value_at(&self, t: f32) -> f32 {
        for i in 0..self.points.len() - 1 {
            let p1 = self.points[i];
            let p2 = self.points[i + 1];

            if t <= p2.x {
                let alpha = (t - p1.x) / (p2.x - p1.x);
                let value = p1.y + alpha * (p2.y - p1.y);
                return value;
            }
        }

        0.0
    }
}

impl SignalCoords {
    fn normalize(self, max: SignalCoords, min: SignalCoords) -> Vec2 {
        let x_range = max.x - min.x;
        let y_range = max.y - min.y;

        let x = if x_range.abs() <= f32::EPSILON {
            0.0
        } else {
            (self.x - min.x) / x_range
        };

        let y = if y_range.abs() <= f32::EPSILON {
            0.0
        } else {
            (self.y - min.y) / y_range
        };

        Vec2 { x, y }
    }
}

impl Drawable for Signal {
    fn draw(
        &self,
        position: ggez::glam::Vec2,
        canvas: &mut Canvas,
        ctx: &mut ggez::Context,
        _theme: Theme,
    ) {
        let position = Vec2 {
            x: position.x,
            y: position.y,
        };

        if self.points.len() < 2 {
            return;
        }

        let points = self
            .points
            .iter()
            .map(|&point| {
                let normalized = point.normalize(self.global_max, self.global_min);
                let viewport_scaled = Vec2 {
                    x: normalized.x * self.size.x,
                    y: (1.0 - normalized.y) * self.size.y,
                };

                position + viewport_scaled
            })
            .collect::<Vec<_>>();

        let line = ggez::graphics::Mesh::new_line(ctx, points.as_slice(), 2.0, self.color).unwrap();
        canvas.draw(&line, ggez::graphics::DrawParam::default());
    }
}
