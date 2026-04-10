use crate::{
    aim::Aim, drawable::Drawable, legend::Legend, screen::Screen, signal::Signal,
    stdin_task::Command, theme::Theme, title::Title, viewport::Viewport, x_axis::XAxis,
    x_label::XLabel, y_axis::YAxis, y_label::YLabel,
};
use ggez::{
    event::EventHandler,
    glam::Vec2,
    graphics::{Canvas, DrawParam, Image, ImageFormat, Rect, ScreenImage},
    input::keyboard::KeyCode,
};
use std::{path::PathBuf, sync::mpsc::Receiver};

pub struct State {
    viewport: Viewport,
    title: Title,
    x_axis: XAxis,
    y_axis: YAxis,
    x_label: XLabel,
    y_label: YLabel,
    pub screen: Screen,
    rx: Receiver<Command>,
    save_paths: Vec<PathBuf>,
    pending_screenshot: Option<(Image, Vec<PathBuf>)>,
    screen_image: Option<ScreenImage>,
    theme: Theme,
    legend: Legend,
    is_legend_enabled: bool,
    aim: Aim,
}

impl State {
    pub fn new(rx: Receiver<Command>, theme: Theme) -> Self {
        let screen = Screen::default();

        let viewport = Viewport::new(&screen);
        let x_axis = XAxis::new(&viewport, &screen, 5);
        let y_axis = YAxis::new(&viewport, &screen, 5);
        let x_label = XLabel::new(&viewport, &screen, "Time (s)");
        let y_label = YLabel::new(&viewport, &screen, "Signals");
        let legend = Legend::new(&viewport);

        Self {
            aim: Aim::new(&viewport),
            viewport,
            title: Title::new(&screen, "Magmar"),
            x_axis,
            y_axis,
            x_label,
            y_label,
            screen,
            rx,
            save_paths: Vec::new(),
            pending_screenshot: None,
            screen_image: None,
            theme,
            legend,
            is_legend_enabled: true,
        }
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
                    let signals_len = self.aim.signals().len();
                    if signals_len + 1 < points.len() {
                        for i in signals_len..(points.len() - 1) {
                            self.aim.signals_mut().push(Signal::new(
                                i,
                                &self.viewport,
                                self.theme,
                                &self.screen,
                            ));
                        }
                    }

                    let mut points = points.into_iter();

                    let Some(time) = points.next() else {
                        continue;
                    };

                    for (point, signal) in points.zip(self.aim.signals_mut().iter_mut()) {
                        signal.add_point(time, point);
                    }
                }
                Command::NewNames(names) => {
                    let signals_len = self.aim.signals().len();
                    if signals_len + 1 < names.len() {
                        for i in signals_len..(names.len() - 1) {
                            self.aim.signals_mut().push(Signal::new(
                                i,
                                &self.viewport,
                                self.theme,
                                &self.screen,
                            ));
                        }
                    }

                    let mut names = names.into_iter();
                    if let Some(x_name) = names.next() {
                        self.x_label.set_text(x_name);
                    }

                    for (name, signal) in names.zip(self.aim.signals_mut().iter_mut()) {
                        signal.set_name(name.clone());
                        self.legend.add_signal(name, signal.color);
                    }
                }
                Command::LegendPos(pos) => {
                    self.is_legend_enabled = true;
                    self.legend.set_position(pos);
                    println!("Legend position set to {:?}", pos);
                }
                Command::DisableLegend => {
                    self.is_legend_enabled = false;
                    println!("Legend disabled");
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
        canvas.set_screen_coordinates(Rect::new(
            0.0,
            0.0,
            self.screen.width + Screen::SCREEN_WIDTH_OFFSET,
            self.screen.height,
        ));

        let max = self.aim.max();
        let min = self.aim.min();

        self.x_axis.set_min_max(min.x, max.x);
        self.y_axis.set_min_max(min.y, max.y);

        self.title
            .draw(Vec2 { x: 0.0, y: 0.0 }, &mut canvas, ctx, self.theme);
        let viewport_pos = Vec2 {
            x: self.screen.width * (YLabel::WIDTH_PERCENT + YAxis::WIDTH_PERCENT),
            y: self.screen.height * Title::HEIGHT_PERCENT,
        };
        self.viewport
            .draw(viewport_pos, &mut canvas, ctx, self.theme);
        self.y_label.draw(
            Vec2 {
                x: 0.0,
                y: self.screen.height * Title::HEIGHT_PERCENT,
            },
            &mut canvas,
            ctx,
            self.theme,
        );
        self.y_axis.draw(
            Vec2 {
                x: self.screen.width * YLabel::WIDTH_PERCENT,
                y: self.screen.height * Title::HEIGHT_PERCENT,
            },
            &mut canvas,
            ctx,
            self.theme,
        );
        self.x_axis.draw(
            Vec2 {
                x: self.screen.width * (YLabel::WIDTH_PERCENT + YAxis::WIDTH_PERCENT),
                y: self.viewport.height + (self.screen.height * Title::HEIGHT_PERCENT),
            },
            &mut canvas,
            ctx,
            self.theme,
        );
        self.x_label.draw(
            Vec2 {
                x: self.screen.width * (YLabel::WIDTH_PERCENT + YAxis::WIDTH_PERCENT),
                y: self.viewport.height
                    + (self.screen.height * (Title::HEIGHT_PERCENT + XAxis::HEIGHT_PERCENT)),
            },
            &mut canvas,
            ctx,
            self.theme,
        );

        for signal in self.aim.signals_mut() {
            signal.set_global_max_min(max, min);
            signal.draw(viewport_pos, &mut canvas, ctx, self.theme);

            for tooltip_info in signal.get_tooltips_info() {
                if !self.viewport.is_inside(viewport_pos, tooltip_info.position) {
                    continue;
                }

                tooltip_info
                    .dot
                    .draw(tooltip_info.position, &mut canvas, ctx, self.theme);
            }
        }

        self.aim.draw(viewport_pos, &mut canvas, ctx, self.theme);

        for signal in self.aim.signals() {
            for tooltip_info in signal.get_tooltips_info() {
                if !self.viewport.is_inside(viewport_pos, tooltip_info.position) {
                    continue;
                }

                tooltip_info
                    .tooltip
                    .draw(tooltip_info.position, &mut canvas, ctx, self.theme);
            }
        }

        if self.is_legend_enabled {
            self.legend.draw(viewport_pos, &mut canvas, ctx, self.theme);
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
        let pos = Vec2::new(x / scale_factor, y / scale_factor);
        self.aim.set_mouse(pos);

        let viewport_pos = Vec2 {
            x: self.screen.width * (YLabel::WIDTH_PERCENT + YAxis::WIDTH_PERCENT),
            y: self.screen.height * Title::HEIGHT_PERCENT,
        };
        let cursor = if self.aim.is_mouse_inside_viewport(viewport_pos) {
            ggez::winit::window::CursorIcon::Cell
        } else {
            ggez::winit::window::CursorIcon::Default
        };

        ctx.gfx.window().set_cursor_icon(cursor);

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        let viewport_pos = Vec2 {
            x: self.screen.width * (YLabel::WIDTH_PERCENT + YAxis::WIDTH_PERCENT),
            y: self.screen.height * Title::HEIGHT_PERCENT,
        };

        if button == ggez::event::MouseButton::Left {
            self.aim.mark_tooltip(viewport_pos);
        } else if button == ggez::event::MouseButton::Right {
            self.aim.remove_tooltip(viewport_pos);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        use ggez::input::keyboard::KeyCode;

        let viewport_pos = Vec2 {
            x: self.screen.width * (YLabel::WIDTH_PERCENT + YAxis::WIDTH_PERCENT),
            y: self.screen.height * Title::HEIGHT_PERCENT,
        };

        match input.keycode {
            Some(KeyCode::Tab) => self.aim.next_signal(),
            Some(KeyCode::Z) => self.aim.reset_zoom(viewport_pos),
            _ => {}
        }

        self.title.set_zoom(self.aim.zoom());

        Ok(())
    }

    fn mouse_wheel_event(
        &mut self,
        ctx: &mut ggez::Context,
        _x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        let viewport_pos = Vec2 {
            x: self.screen.width * (YLabel::WIDTH_PERCENT + YAxis::WIDTH_PERCENT),
            y: self.screen.height * Title::HEIGHT_PERCENT,
        };

        if y == 0.0
            || !(ctx.keyboard.is_key_pressed(KeyCode::RControl)
                || ctx.keyboard.is_key_pressed(KeyCode::LControl))
        {
            return Ok(());
        }

        if y > 0.0 {
            self.aim.zoom_in(viewport_pos);
        } else {
            self.aim.zoom_out(viewport_pos);
        }

        self.title.set_zoom(self.aim.zoom());

        Ok(())
    }
}
