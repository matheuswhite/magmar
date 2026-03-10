use crate::{
    screen::{Screen, ScreenCoords},
    signal::{Signal, SignalCoords},
    viewport::Viewport,
};
use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{
        Canvas, Color, DrawParam, Drawable, Image, ImageFormat, Mesh, Rect, ScreenImage, Text,
    },
    mint::Point2,
};
use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
};

mod screen;
mod signal;
mod tooltip;
mod viewport;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || read_stdin_task(tx));
    let state = State::new(rx);

    let cb = ggez::ContextBuilder::new("magmar", "matheuswhite")
        .window_setup(ggez::conf::WindowSetup {
            title: "Magmar".to_string(),
            ..Default::default()
        })
        // On macOS Retina/HiDPI, ggez interprets `WindowMode::dimensions()` as *physical pixels*.
        // Using `logical_size` makes the window size consistent in "points" across displays.
        .window_mode(ggez::conf::WindowMode {
            logical_size: Some(ggez::winit::dpi::LogicalSize::new(
                state.screen.width,
                state.screen.height,
            )),
            ..Default::default()
        });

    let (ctx, events_loop) = cb.build().unwrap();

    ggez::event::run(ctx, events_loop, state);
}

pub enum Command {
    Save(PathBuf),
    NewPoints(Vec<f32>),
    NewNames(Vec<String>),
}

pub fn read_stdin_task(sender: Sender<Command>) -> ! {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut splitted = input.split(",");

        let first = splitted.next();

        match first {
            Some("!save") => {
                let filename = splitted.next().unwrap_or("output.png");
                sender.send(Command::Save(PathBuf::from(filename))).unwrap();
            }
            Some(_) => {
                match first.and_then(|time| time.parse::<f32>().ok()) {
                    Some(time) => {
                        let values = splitted
                            .map(|s| s.trim().parse::<f32>())
                            .filter_map(Result::ok)
                            .collect::<Vec<_>>();

                        let mut data = vec![time as f32];
                        data.extend(values);
                        sender.send(Command::NewPoints(data)).unwrap();
                    }
                    None => {
                        // If the first value isn't a valid float, treat the entire line as signal names.
                        let names = input.split(",").map(|s| s.trim().to_string()).collect();
                        sender.send(Command::NewNames(names)).unwrap();
                        continue;
                    }
                }
            }
            None => {}
        }
    }
}

pub struct State {
    signals: Vec<Signal>,
    viewport: Viewport,
    screen: Screen,
    rx: Receiver<Command>,
    pos: Vec2,
    x_name: String,
    save_paths: Vec<PathBuf>,
    pending_screenshot: Option<(Image, Vec<PathBuf>)>,
    screen_image: Option<ScreenImage>,
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
            x_name: "Time".to_string(),
            save_paths: Vec::new(),
            pending_screenshot: None,
            screen_image: None,
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
                            self.signals.push(Signal::new(i));
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
                            self.signals.push(Signal::new(i));
                        }
                    }

                    let mut names = names.into_iter();
                    if let Some(x_name) = names.next() {
                        self.x_name = x_name;
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
        let mut canvas = Canvas::from_image(ctx, image.clone(), Color::BLACK);

        // Keep our coordinate system in logical units (points), so UI/layout math
        // stays stable across HiDPI displays (Retina) while still rendering sharp.
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, self.screen.width, self.screen.height));

        let max = self.max();
        let min = self.min();

        let step = (max.y - min.y) / 5.0;
        for i in 0..6 {
            let i = i as f32 * step + min.y;
            let normalized_y =
                (i - min.y) / (max.y - min.y) * self.viewport.height + self.viewport.y;
            let text = Text::new(format!("{:.2}", i));
            let dest_point = Point2 {
                x: self.viewport.x - 40.0,
                y: self.screen.height - normalized_y - 10.0,
            };
            canvas.draw(&text, DrawParam::new().dest(dest_point).color(Color::WHITE));

            let line = Mesh::new_line(
                ctx,
                &[
                    Point2 {
                        x: self.viewport.x,
                        y: self.screen.height - normalized_y,
                    },
                    Point2 {
                        x: self.viewport.x + self.viewport.width,
                        y: self.screen.height - normalized_y,
                    },
                ],
                1.0,
                ggez::graphics::Color::from_rgba(
                    (255.0 * 0.6) as u8,
                    (255.0 * 0.6) as u8,
                    (255.0 * 0.6) as u8,
                    255,
                ),
            )?;
            line.draw(&mut canvas, DrawParam::default());
        }

        let step = (max.x - min.x) / 5.0;
        for i in 0..6 {
            let i = i as f32 * step + min.x;
            let normalized_x =
                (i - min.x) / (max.x - min.x) * self.viewport.width + self.viewport.x;
            let text = Text::new(format!("{:.2}", i));
            let dest_point = Point2 {
                x: normalized_x - 10.0,
                y: self.screen.height - self.viewport.y + 10.0,
            };
            canvas.draw(&text, DrawParam::new().dest(dest_point).color(Color::WHITE));

            let line = Mesh::new_line(
                ctx,
                &[
                    Point2 {
                        x: normalized_x,
                        y: self.screen.height - self.viewport.y,
                    },
                    Point2 {
                        x: normalized_x,
                        y: self.screen.height - self.viewport.y - self.viewport.height,
                    },
                ],
                1.0,
                ggez::graphics::Color::from_rgba(
                    (255.0 * 0.6) as u8,
                    (255.0 * 0.6) as u8,
                    (255.0 * 0.6) as u8,
                    255,
                ),
            )?;
            line.draw(&mut canvas, DrawParam::default());
        }

        let scale = 1.3;
        let time = Text::new(&self.x_name);
        let dest_point = Point2 {
            x: self.screen.width / 2.0,
            y: self.screen.height - 35.0,
        };
        canvas.draw(
            &time,
            DrawParam::new()
                .dest(dest_point)
                .scale([scale, scale])
                .color(Color::WHITE),
        );
        let signal = Text::new("Signals");
        let dest_point = Point2 {
            x: 15.0,
            y: self.screen.height / 2.0,
        };
        canvas.draw(
            &signal,
            DrawParam::new()
                .dest(dest_point)
                .rotation(-std::f32::consts::FRAC_PI_2)
                .scale([scale, scale])
                .color(Color::WHITE),
        );

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
            let mut save_canvas = Canvas::from_image(ctx, save_image.clone(), Color::BLACK);
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
