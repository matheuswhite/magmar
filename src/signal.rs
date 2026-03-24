use std::collections::VecDeque;

use crate::{
    drawable::Drawable,
    screen::Screen,
    theme::Theme,
    tooltip::{Tooltip, TooltipDot},
    viewport::Viewport,
};
use ggez::{
    glam::Vec2,
    graphics::{Canvas, Color},
};

pub struct TooltipInfo {
    pub tooltip: Tooltip,
    pub dot: TooltipDot,
    pub position: Vec2,
}

pub struct Signal {
    pub color: Color,
    points: Vec<SignalCoords>,
    global_max: SignalCoords,
    global_min: SignalCoords,
    pub name: String,
    size: Vec2,
    screen_width: f32,
    tooltips: Vec<TooltipInfo>,
    zoom: f32,
    points_with_zoom: Option<VecDeque<SignalCoords>>,
    zoom_history: Vec<(Vec<SignalCoords>, Vec<SignalCoords>)>,
}

#[derive(Clone, Copy, Debug)]
pub struct SignalCoords {
    pub x: f32,
    pub y: f32,
}

impl Signal {
    pub fn new(index: usize, viweport: &Viewport, theme: Theme, screen: &Screen) -> Self {
        let name = format!("Y{}", index + 1);
        let color = theme.gen_color(index);

        Self {
            color,
            points: Vec::new(),
            global_max: SignalCoords { x: 0.0, y: 0.0 },
            global_min: SignalCoords { x: 0.0, y: 0.0 },
            name: name.clone(),
            size: Vec2 {
                x: viweport.width,
                y: viweport.height,
            },
            screen_width: screen.width,
            tooltips: vec![],
            points_with_zoom: None,
            zoom: 100.0,
            zoom_history: vec![],
        }
    }

    pub fn mark_tooltip(&mut self, position: Vec2, value: f32) {
        let tooltip_info = TooltipInfo {
            tooltip: Tooltip::new(self.color, self.screen_width, value),
            dot: TooltipDot::new(self.color),
            position,
        };
        self.tooltips.push(tooltip_info);
    }

    pub fn remove_tooltip(&mut self, position: Vec2) {
        let Some(tooltip_index) = self
            .tooltips
            .iter()
            .rev()
            .position(|t| (t.position.x - position.x).abs() <= 5.0)
        else {
            return;
        };
        let tooltip_index = self.tooltips.len() - 1 - tooltip_index;

        self.tooltips.remove(tooltip_index);
    }

    pub fn get_tooltips_info(&self) -> &[TooltipInfo] {
        &self.tooltips
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name.clone();
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        let point = SignalCoords { x, y };

        self.points.push(point);

        if let Some(points_with_zoom) = self.points_with_zoom.as_mut() {
            points_with_zoom.push_back(point);
        } else {
            self.zoom_history.clear();
        }
    }

    pub fn set_global_max_min(&mut self, global_max: SignalCoords, global_min: SignalCoords) {
        self.global_max = global_max;
        self.global_min = global_min;
    }

    pub fn value_at(&self, t: f32) -> f32 {
        for i in 0..self.points().len() - 1 {
            let p1 = self.points()[i];
            let p2 = self.points()[i + 1];

            if t <= p2.x {
                let alpha = (t - p1.x) / (p2.x - p1.x);
                let value = p1.y + alpha * (p2.y - p1.y);
                return value;
            }
        }

        0.0
    }

    pub fn min(&self) -> SignalCoords {
        self.points().iter().fold(
            SignalCoords {
                x: f32::MAX,
                y: f32::MAX,
            },
            |current, coords| SignalCoords {
                x: current.x.min(coords.x),
                y: current.y.min(coords.y),
            },
        )
    }

    pub fn max(&self) -> SignalCoords {
        self.points().iter().fold(
            SignalCoords {
                x: f32::MIN,
                y: f32::MIN,
            },
            |current, coords| SignalCoords {
                x: current.x.max(coords.x),
                y: current.y.max(coords.y),
            },
        )
    }

    pub fn zoom_in(&mut self, drop_left_percent: f32, zoom_factor: f32) {
        self.zoom += zoom_factor;

        if self.points_with_zoom.is_none() {
            self.points_with_zoom = Some(VecDeque::from_iter(self.points.clone()));
        }

        let points_with_zoom = self.points_with_zoom.as_mut().unwrap();
        let points_with_zoom_len = points_with_zoom.len();

        let dropped_samples = points_with_zoom_len as f32 * (zoom_factor / 100.0);

        let drop_left = ((dropped_samples * drop_left_percent) as usize).max(1);
        let drop_right = ((dropped_samples * (1.0 - drop_left_percent)) as usize).max(1);

        if points_with_zoom_len.saturating_sub(drop_left + drop_right) < 2 {
            return;
        }

        let mut left = vec![];
        for _ in 0..drop_left {
            let Some(item) = points_with_zoom.pop_front() else {
                continue;
            };

            left.push(item);
        }

        let mut right = vec![];
        for _ in 0..drop_right {
            let Some(item) = points_with_zoom.pop_back() else {
                continue;
            };

            right.push(item);
        }

        self.zoom_history.push((left, right));
    }

    pub fn zoom_out(&mut self, zoom_factor: f32) {
        self.zoom -= zoom_factor;

        if self.zoom == 100.0 {
            self.points_with_zoom = None;
            self.zoom_history.clear();

            return;
        }

        let points_with_zoom = self.points_with_zoom.as_mut().unwrap();

        let Some((left, right)) = self.zoom_history.pop() else {
            return;
        };

        for item in left.into_iter().rev() {
            points_with_zoom.push_front(item);
        }

        for item in right.into_iter().rev() {
            points_with_zoom.push_back(item);
        }
    }

    pub fn reset_zoom(&mut self) {
        self.points_with_zoom = None;
        self.zoom_history.clear();
    }

    pub fn points(&self) -> &[SignalCoords] {
        match self.points_with_zoom.as_ref() {
            Some(points_with_zoom) => points_with_zoom.as_slices().0,
            None => &self.points,
        }
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
        if self.points().len() < 2 {
            return;
        }

        let points = self
            .points()
            .iter()
            .map(|point| {
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
