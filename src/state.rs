use crate::{
    grid::Grid,
    screen::{Screen, ScreenCoords},
    signal::{Signal, SignalCoords},
    stdin_task::Command,
    theme::Theme,
    viewport::Viewport,
};
use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{Canvas, DrawParam, Drawable, Image, ImageFormat, Mesh, Rect, ScreenImage, Text},
};
use std::{path::PathBuf, sync::mpsc::Receiver};

pub struct State {
    signals: Vec<Signal>,
    viewport: Viewport,
    pub screen: Screen,
    rx: Receiver<Command>,
    pos: Vec2,
    save_paths: Vec<PathBuf>,
    pending_screenshot: Option<(Image, Vec<PathBuf>)>,
    screen_image: Option<ScreenImage>,
    theme: Theme,
    grid: Grid,
}

impl State {
    pub fn new(rx: Receiver<Command>) -> Self {
        let screen = Screen::default();

        Self {
            signals: Vec::new(),
            viewport: Viewport::new(60.0, 25.0, &screen),
            screen,
            rx,
            pos: Vec2::ZERO,
            grid: Grid::default(),
            save_paths: Vec::new(),
            pending_screenshot: None,
            screen_image: None,
            theme: Theme::Light,
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

    fn save_screenshot(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        // Read pixels from the aligned image blit'd in the PREVIOUS frame
        // (already submitted by end_frame, so the texture is fully populated).
        if let Some((image, paths)) = self.pending_screenshot.take() {
            let pixels = image.to_pixels(ctx)?;
            for path in paths {
                image::save_buffer(
                    &path,
                    &pixels,
                    image.width(),
                    image.height(),
                    image::ColorType::Rgba8,
                )
                .map_err(|e| ggez::GameError::RenderError(e.to_string()))?;
                println!("Saved screenshot to {}", path.display());
            }
        }
        Ok(())
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        while let Ok(command) = self.rx.try_recv() {
            match command {
                Command::Save(path) => {
                    self.save_paths.push(path);
                    // Stop processing commands so this frame renders the current
                    // state before consuming any further data points or saves.
                    break;
                }
                Command::NewPoints(points) => {
                    if self.signals.len() + 1 < points.len() {
                        for i in self.signals.len()..(points.len() - 1) {
                            self.signals.push(Signal::new(i, self.theme));
                        }
                    }

                    let mut points = points.into_iter();

                    let Some(time) = points.next() else {
                        continue;
                    };

                    for (point, signal) in points.zip(self.signals.iter_mut()) {
                        signal.add_point(time, point);
                    }
                }
                Command::NewNames(names) => {
                    if self.signals.len() + 1 < names.len() {
                        for i in self.signals.len()..(names.len() - 1) {
                            self.signals.push(Signal::new(i, self.theme));
                        }
                    }

                    let mut names = names.into_iter();
                    if let Some(x_name) = names.next() {
                        self.grid.set_x_label(x_name);
                    }

                    for (name, signal) in names.zip(self.signals.iter_mut()) {
                        signal.set_name(name);
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        // Save the PREVIOUS frame's content (already submitted by end_frame).
        self.save_screenshot(ctx)?;

        let screen_image = self
            .screen_image
            .get_or_insert_with(|| ScreenImage::new(ctx, ImageFormat::Rgba8UnormSrgb, 1., 1., 1));
        let image = screen_image.image(ctx);
        let mut canvas = Canvas::from_image(ctx, image.clone(), self.theme.control_weak());

        // Keep our coordinate system in logical units (points), so UI/layout math
        // stays stable across HiDPI displays (Retina) while still rendering sharp.
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, self.screen.width, self.screen.height));

        let max = self.max();
        let min = self.min();

        self.grid.draw(
            &mut canvas,
            ctx,
            min,
            max,
            &self.viewport,
            &self.screen,
            self.theme,
        )?;

        let mouse = ScreenCoords {
            x: self.pos.x,
            y: self.pos.y,
        };

        for signal in &self.signals {
            signal.draw(
                &mut canvas,
                ctx,
                &self.viewport,
                self.screen.height,
                min,
                max,
                mouse,
            );
        }

        for signal in &self.signals {
            signal.tooltip.draw(
                &mut canvas,
                ctx,
                mouse,
                self.screen.height,
                signal,
                min,
                max,
                &self.viewport,
                self.theme,
            );
        }

        canvas.finish(ctx)?;

        // Blit the offscreen image to the actual window frame
        let mut frame_canvas = Canvas::from_frame(ctx, None);
        frame_canvas.draw(&image, DrawParam::default());
        frame_canvas.finish(ctx)?;

        // If saves are pending: blit to an aligned image inside THIS frame's encoder.
        // end_frame will submit it; next draw() call reads the completed texture.
        if !self.save_paths.is_empty() {
            let w = image.width();
            let h = image.height();
            // wgpu COPY_BYTES_PER_ROW_ALIGNMENT = 256; with RGBA8 (4 bytes/px) → width must be multiple of 64.
            let aligned_w = ((w + 63) / 64) * 64;
            let save_image =
                Image::new_canvas_image(ctx, ImageFormat::Rgba8UnormSrgb, aligned_w, h, 1);
            let mut save_canvas =
                Canvas::from_image(ctx, save_image.clone(), self.theme.control_weak());
            save_canvas.draw(&image, DrawParam::default());
            save_canvas.finish(ctx)?;
            let paths = std::mem::take(&mut self.save_paths);
            self.pending_screenshot = Some((save_image, paths));
        }

        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        ctx: &mut ggez::Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), ggez::GameError> {
        // ggez gives mouse coordinates in physical pixels; convert to logical units.
        let scale_factor = ctx.gfx.window().scale_factor() as f32;
        self.pos = Vec2::new(x / scale_factor, y / scale_factor);

        Ok(())
    }
}
