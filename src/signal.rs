use crate::{
    screen::{Screen, ScreenCoords},
    theme::Theme,
    tooltip::Tooltip,
    viewport::{Viewport, ViewportCoords},
};
use core::f32;
use ggez::graphics::{Canvas, Color};

pub struct Signal {
    pub color: Color,
    pub points: Vec<SignalCoords>,
    pub min: SignalCoords,
    pub max: SignalCoords,
    pub name: String,
    pub tooltip: Tooltip,
}

#[derive(Clone, Copy)]
pub struct SignalCoords {
    pub x: f32,
    pub y: f32,
}

impl Signal {
    pub fn new(index: usize, theme: Theme) -> Self {
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
            name: name.clone(),
            tooltip: Tooltip::new(name, color),
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

    pub fn draw(
        &self,
        canvas: &mut Canvas,
        ctx: &mut ggez::Context,
        viewport: &Viewport,
        min: SignalCoords,
        max: SignalCoords,
        mouse: ScreenCoords,
        screen: &Screen,
    ) {
        if self.points.len() < 2 {
            return;
        }

        let points = self
            .points
            .iter()
            .map(|&point| {
                let normalized = point.to_viewport(viewport, min, max);
                screen.fix_coords(normalized.x + viewport.x, normalized.y + viewport.y)
            })
            .collect::<Vec<_>>();

        let line = ggez::graphics::Mesh::new_line(ctx, points.as_slice(), 2.0, self.color).unwrap();
        canvas.draw(&line, ggez::graphics::DrawParam::default());

        self.tooltip
            .draw_point(canvas, ctx, mouse, self, min, max, viewport, screen);
    }
}

impl SignalCoords {
    pub fn to_viewport(
        self,
        viewport: &Viewport,
        min: SignalCoords,
        max: SignalCoords,
    ) -> ViewportCoords {
        let x_range = max.x - min.x;
        let y_range = max.y - min.y;

        let x = if x_range.abs() <= f32::EPSILON {
            viewport.width * 0.5
        } else {
            (self.x - min.x) / x_range * viewport.width
        };

        let y = if y_range.abs() <= f32::EPSILON {
            viewport.height * 0.5
        } else {
            (self.y - min.y) / y_range * viewport.height
        };

        ViewportCoords { x, y }
    }
}
