use crate::{screen::Screen, state::State, stdin_task::read_stdin_task, theme::Theme};
use clap::Parser;

mod drawable;
mod legend;
mod screen;
mod signal;
mod state;
mod stdin_task;
mod theme;
mod title;
mod toolbar;
mod tooltip;
mod viewport;
mod x_axis;
mod x_label;
mod y_axis;
mod y_label;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = false)]
    light: bool,
    #[arg(short, long)]
    title: Option<String>,
}

fn main() {
    let args = Args::parse();
    let theme = if args.light {
        Theme::Light
    } else {
        Theme::Dark
    };

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || read_stdin_task(tx));
    let state = State::new(rx, theme);

    let cb = ggez::ContextBuilder::new("magmar", "matheuswhite")
        .window_setup(ggez::conf::WindowSetup {
            title: args.title.unwrap_or_else(|| "Magmar".to_string()),
            ..Default::default()
        })
        // On macOS Retina/HiDPI, ggez interprets `WindowMode::dimensions()` as *physical pixels*.
        // Using `logical_size` makes the window size consistent in "points" across displays.
        .window_mode(ggez::conf::WindowMode {
            logical_size: Some(ggez::winit::dpi::LogicalSize::new(
                state.screen.width + Screen::SCREEN_WIDTH_OFFSET,
                state.screen.height,
            )),
            ..Default::default()
        });

    let (ctx, events_loop) = cb.build().unwrap();

    ggez::event::run(ctx, events_loop, state);
}
