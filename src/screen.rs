use ggez::mint::Point2;

use crate::viewport::{Viewport, ViewportCoords};

pub struct Screen {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy)]
pub struct ScreenCoords {
    pub x: f32,
    pub y: f32,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
        }
    }
}

impl Screen {
    pub fn fix_coords(&self, x: f32, y: f32) -> ScreenCoords {
        ScreenCoords {
            x,
            y: self.height - y,
        }
    }
}

impl ScreenCoords {
    pub fn to_viewport(self, viewport: &Viewport) -> ViewportCoords {
        ViewportCoords {
            x: self.x - viewport.x,
            y: self.y - viewport.y,
        }
    }
}

impl From<ScreenCoords> for Point2<f32> {
    fn from(coords: ScreenCoords) -> Self {
        Point2 {
            x: coords.x,
            y: coords.y,
        }
    }
}
