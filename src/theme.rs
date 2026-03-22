use ggez::graphics::Color;

const fn color(r: u8, g: u8, b: u8) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

#[allow(unused)]
pub mod light {
    use super::color;
    use ggez::graphics::Color;

    pub const BACKGROUND: Color = color(242, 242, 247);
    // Gray
    pub const CONTROL_STRONG: Color = color(142, 142, 147);
    // Gray (5)
    pub const CONTROL_WEAK: Color = color(229, 229, 234);
    pub const RED: Color = color(255, 56, 60);
    pub const ORANGE: Color = color(255, 141, 40);
    pub const YELLOW: Color = color(255, 204, 0);
    pub const GREEN: Color = color(52, 199, 89);
    pub const MINT: Color = color(0, 200, 179);
    pub const TEAL: Color = color(0, 195, 208);
    pub const CYAN: Color = color(0, 192, 232);
    pub const BLUE: Color = color(0, 136, 255);
    pub const INDIGO: Color = color(97, 85, 245);
    pub const PURPLE: Color = color(203, 48, 224);
    pub const PINK: Color = color(255, 45, 85);
    pub const BROWN: Color = color(172, 127, 94);
}

#[allow(unused)]
pub mod dark {
    use super::color;
    use ggez::graphics::Color;

    pub const BACKGROUND: Color = color(28, 28, 30);
    // Gray
    pub const CONTROL_STRONG: Color = color(142, 142, 147);
    // Gray (5)
    pub const CONTROL_WEAK: Color = color(44, 44, 46);
    pub const RED: Color = color(255, 66, 69);
    pub const ORANGE: Color = color(255, 146, 48);
    pub const YELLOW: Color = color(255, 214, 0);
    pub const GREEN: Color = color(48, 209, 88);
    pub const MINT: Color = color(0, 218, 195);
    pub const TEAL: Color = color(0, 210, 224);
    pub const CYAN: Color = color(60, 211, 254);
    pub const BLUE: Color = color(0, 145, 255);
    pub const INDIGO: Color = color(109, 124, 255);
    pub const PURPLE: Color = color(219, 52, 242);
    pub const PINK: Color = color(255, 55, 95);
    pub const BROWN: Color = color(183, 138, 102);
}

#[derive(Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    const LUT: [[Color; 7]; 2] = [
        [
            light::BLUE,
            light::ORANGE,
            light::YELLOW,
            light::PURPLE,
            light::GREEN,
            light::CYAN,
            light::PINK,
        ],
        [
            dark::YELLOW,
            dark::GREEN,
            dark::BLUE,
            dark::PURPLE,
            dark::ORANGE,
            dark::CYAN,
            dark::PINK,
        ],
    ];

    pub fn gen_color(&self, index: usize) -> Color {
        let theme_index = match self {
            Theme::Light => 0,
            Theme::Dark => 1,
        };
        Self::LUT[theme_index][index % Self::LUT[theme_index].len()]
    }

    pub fn background(&self) -> Color {
        match self {
            Theme::Light => light::BACKGROUND,
            Theme::Dark => dark::BACKGROUND,
        }
    }

    pub fn control_strong(&self) -> Color {
        match self {
            Theme::Light => light::CONTROL_STRONG,
            Theme::Dark => dark::CONTROL_STRONG,
        }
    }

    pub fn control_weak(&self) -> Color {
        match self {
            Theme::Light => light::CONTROL_WEAK,
            Theme::Dark => dark::CONTROL_WEAK,
        }
    }

    pub fn toolbar_bg(&self) -> Color {
        match self {
            // White toolbar like macOS — clean elevated surface
            Theme::Light => Color::new(1.0, 1.0, 1.0, 1.0),
            // Slightly lighter than background — elevated surface in dark mode
            Theme::Dark => Color::new(44.0 / 255.0, 44.0 / 255.0, 46.0 / 255.0, 1.0),
        }
    }

    pub fn toolbar_separator(&self) -> Color {
        match self {
            Theme::Light => Color::new(0.0, 0.0, 0.0, 0.12),
            Theme::Dark => Color::new(0.0, 0.0, 0.0, 0.35),
        }
    }

    pub fn btn_selected_bg(&self) -> Color {
        match self {
            Theme::Light => light::BLUE,
            Theme::Dark => dark::BLUE,
        }
    }

    pub fn btn_hover_bg(&self) -> Color {
        match self {
            Theme::Light => Color::new(0.0, 0.0, 0.0, 0.07),
            Theme::Dark => Color::new(1.0, 1.0, 1.0, 0.1),
        }
    }
}
