extern crate rand;
extern crate config;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

#[macro_use]
extern crate serde_derive;

mod map;
mod app;
mod settings;

use settings::GameSettings;
use piston::window::{ AdvancedWindow, WindowSettings };
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

fn main() {

    let settings = GameSettings::new_from_defaults().unwrap();

    // Print out our settings
    println!("Loaded settings:\n{:?}", settings);

    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window_settings = WindowSettings::new(
        "piston-rust-playground",
        [settings.graphics.window_width, settings.graphics.window_height],
    );
    window_settings.set_fullscreen(settings.graphics.fullscreen);

    let mut window: Window = window_settings
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_capture_cursor(settings.graphics.capture_cursor);

    // Create a new game and run it.
    let mut app = app::App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        app.process_inputs(&e);

        if let Some(_args) = e.idle_args() {
        }
    }
}
