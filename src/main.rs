use crate::{state::State, stdin_task::read_stdin_task};

mod grid;
mod screen;
mod signal;
mod state;
mod stdin_task;
mod theme;
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
