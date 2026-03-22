use crate::{screen::Screen, theme::Theme};
use ggez::{
    glam::Vec2,
    graphics::{Color, DrawParam, Image, Mesh, MeshBuilder, Rect},
};

const CURSOR_LIGHT: &[u8] = include_bytes!("../icons/cursor_light.png");
const CURSOR_DARK: &[u8] = include_bytes!("../icons/cursor_dark.png");
const ADD_MARKER_LIGHT: &[u8] = include_bytes!("../icons/add_marker_light.png");
const ADD_MARKER_DARK: &[u8] = include_bytes!("../icons/add_marker_dark.png");
const REMOVE_MARKER_LIGHT: &[u8] = include_bytes!("../icons/remove_marker_light.png");
const REMOVE_MARKER_DARK: &[u8] = include_bytes!("../icons/remove_marker_dark.png");
const ZOOM_IN_LIGHT: &[u8] = include_bytes!("../icons/zoom_in_light.png");
const ZOOM_IN_DARK: &[u8] = include_bytes!("../icons/zoom_in_dark.png");
const ZOOM_OUT_LIGHT: &[u8] = include_bytes!("../icons/zoom_out_light.png");
const ZOOM_OUT_DARK: &[u8] = include_bytes!("../icons/zoom_out_dark.png");

#[derive(Clone, Copy, PartialEq)]
pub enum Tool {
    Cursor,
    AddMarker,
    RemoveMarker,
    ZoomIn,
    ZoomOut,
}

pub struct Toolbar {
    size: Vec2,
    pub selected: Tool,
    pub hovered: Option<Tool>,
    icons_light: Option<[Image; 5]>,
    icons_dark: Option<[Image; 5]>,
}

/// Build a filled rounded-rectangle mesh.
fn rounded_rect(
    ctx: &mut ggez::Context,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    radius: f32,
    color: Color,
) -> Mesh {
    use std::f32::consts::FRAC_PI_2;
    let r = radius.min(w / 2.0).min(h / 2.0);
    const SEGS: usize = 10;
    let mut pts: Vec<Vec2> = Vec::with_capacity(SEGS * 4 + 4);

    // Arc center positions and starting angles for each corner (TL, TR, BR, BL)
    for (cx, cy, start) in [
        (x + r, y + r, std::f32::consts::PI),
        (x + w - r, y + r, 3.0 * FRAC_PI_2),
        (x + w - r, y + h - r, 0.0_f32),
        (x + r, y + h - r, FRAC_PI_2),
    ] {
        for j in 0..=SEGS {
            let angle = start + (j as f32 / SEGS as f32) * FRAC_PI_2;
            pts.push(Vec2::new(cx + r * angle.cos(), cy + r * angle.sin()));
        }
    }

    let mut builder = MeshBuilder::new();
    builder
        .polygon(ggez::graphics::DrawMode::fill(), &pts, color)
        .unwrap();
    Mesh::from_data(ctx, builder.build())
}

const TOOLS: [Tool; 5] = [
    Tool::Cursor,
    Tool::AddMarker,
    Tool::RemoveMarker,
    Tool::ZoomIn,
    Tool::ZoomOut,
];

impl Toolbar {
    pub const HEIGHT_PERCENT: f32 = 0.07;

    // Geometry constants — all derived from the toolbar height at draw time.
    const V_PAD: f32 = 7.0;
    const H_PAD: f32 = 10.0;
    const BTN_GAP: f32 = 3.0;
    const BTN_CORNER: f32 = 7.0;
    // Icons occupy 62 % of the button size; the rest is inset padding.
    const ICON_FILL: f32 = 0.62;

    pub fn new(screen: &Screen) -> Self {
        Self {
            size: Vec2 {
                x: screen.width + Screen::SCREEN_WIDTH_OFFSET,
                y: screen.height * Self::HEIGHT_PERCENT,
            },
            selected: Tool::Cursor,
            hovered: None,
            icons_light: None,
            icons_dark: None,
        }
    }

    fn ensure_icons(&mut self, ctx: &mut ggez::Context) {
        if self.icons_light.is_none() {
            self.icons_light = Some([
                Image::from_bytes(ctx, CURSOR_LIGHT).unwrap(),
                Image::from_bytes(ctx, ADD_MARKER_LIGHT).unwrap(),
                Image::from_bytes(ctx, REMOVE_MARKER_LIGHT).unwrap(),
                Image::from_bytes(ctx, ZOOM_IN_LIGHT).unwrap(),
                Image::from_bytes(ctx, ZOOM_OUT_LIGHT).unwrap(),
            ]);
        }
        if self.icons_dark.is_none() {
            self.icons_dark = Some([
                Image::from_bytes(ctx, CURSOR_DARK).unwrap(),
                Image::from_bytes(ctx, ADD_MARKER_DARK).unwrap(),
                Image::from_bytes(ctx, REMOVE_MARKER_DARK).unwrap(),
                Image::from_bytes(ctx, ZOOM_IN_DARK).unwrap(),
                Image::from_bytes(ctx, ZOOM_OUT_DARK).unwrap(),
            ]);
        }
    }

    /// Returns the button size derived from the current toolbar height.
    fn btn_size(&self) -> f32 {
        self.size.y - Self::V_PAD * 2.0
    }

    pub fn draw(
        &mut self,
        position: Vec2,
        canvas: &mut ggez::graphics::Canvas,
        ctx: &mut ggez::Context,
        theme: Theme,
    ) {
        // ── 1. Toolbar background ────────────────────────────────────────────
        let bg = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            Rect::new(position.x, position.y, self.size.x, self.size.y),
            theme.toolbar_bg(),
        )
        .unwrap();
        canvas.draw(&bg, DrawParam::default());

        // ── 2. Bottom separator (1 px) ───────────────────────────────────────
        let sep = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            Rect::new(position.x, position.y + self.size.y - 1.0, self.size.x, 1.0),
            theme.toolbar_separator(),
        )
        .unwrap();
        canvas.draw(&sep, DrawParam::default());

        // ── 3. Buttons ───────────────────────────────────────────────────────
        self.ensure_icons(ctx);

        let icons_white = self.icons_dark.as_ref().unwrap(); // white icons (for dark bg)
        let icons_dark = self.icons_light.as_ref().unwrap(); // dark icons (for light bg)

        let btn = self.btn_size();
        let icon_px = btn * Self::ICON_FILL;
        let icon_scale = icon_px / 64.0;
        let icon_inset = (btn - icon_px) / 2.0;

        for (i, tool) in TOOLS.iter().enumerate() {
            let btn_x = position.x + Self::H_PAD + i as f32 * (btn + Self::BTN_GAP);
            let btn_y = position.y + Self::V_PAD;

            // Button background
            let fill = if *tool == self.selected {
                Some(theme.btn_selected_bg())
            } else if self.hovered == Some(*tool) {
                Some(theme.btn_hover_bg())
            } else {
                None
            };

            if let Some(color) = fill {
                let rect = rounded_rect(ctx, btn_x, btn_y, btn, btn, Self::BTN_CORNER, color);
                canvas.draw(&rect, DrawParam::default());
            }

            // Icon: white on selected (always legible on blue), theme-aware otherwise
            let use_white = *tool == self.selected || matches!(theme, Theme::Dark);
            let icon = if use_white {
                &icons_white[i]
            } else {
                &icons_dark[i]
            };

            canvas.draw(
                icon,
                DrawParam::default()
                    .dest(Vec2::new(btn_x + icon_inset, btn_y + icon_inset))
                    .scale(Vec2::new(icon_scale, icon_scale)),
            );
        }
    }

    /// Returns which tool (if any) is under the given logical-coordinate point.
    pub fn hovered_tool(&self, x: f32, y: f32, position: Vec2) -> Option<Tool> {
        let btn = self.btn_size();
        let btn_y = position.y + Self::V_PAD;
        if y < btn_y || y > btn_y + btn {
            return None;
        }

        for (i, tool) in TOOLS.iter().enumerate() {
            let btn_x = position.x + Self::H_PAD + i as f32 * (btn + Self::BTN_GAP);
            if x >= btn_x && x <= btn_x + btn {
                return Some(*tool);
            }
        }
        None
    }

    pub fn handle_click(&mut self, x: f32, y: f32, position: Vec2) {
        if let Some(tool) = self.hovered_tool(x, y, position) {
            self.selected = tool;
        }
    }
}
