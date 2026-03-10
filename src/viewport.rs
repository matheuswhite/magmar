use crate::{
    screen::{Screen, ScreenCoords},
    signal::{Signal, SignalCoords},
};

pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy)]
pub struct ViewportCoords {
    pub x: f32,
    pub y: f32,
}

impl Viewport {
    pub fn new(padding: f32, offset: f32, screen: &Screen) -> Self {
        Self {
            x: padding + offset,
            y: padding + offset,
            width: screen.width - 2.0 * padding,
            height: screen.height - 2.0 * padding,
        }
    }

    pub fn is_inside(&self, coords: ScreenCoords) -> bool {
        coords.x >= self.x
            && coords.x <= self.x + self.width
            && coords.y >= self.y
            && coords.y <= self.y + self.height
    }
}

impl ViewportCoords {
    pub fn to_screen(self, viewport: &Viewport) -> ScreenCoords {
        ScreenCoords {
            x: self.x + viewport.x,
            y: self.y + viewport.y,
        }
    }

    pub fn to_signal(self, signal: &Signal, viewport: &Viewport) -> SignalCoords {
        let normalized = SignalCoords {
            x: self.x / viewport.width,
            y: self.y / viewport.height,
        };

        SignalCoords {
            x: normalized.x * (signal.max.x - signal.min.x) + signal.min.x,
            y: normalized.y * (signal.max.y - signal.min.y) + signal.min.y,
        }
    }
}
