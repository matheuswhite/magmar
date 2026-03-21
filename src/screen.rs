use ggez::mint::Point2;

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
            width: 750.0,
            height: 600.0,
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
