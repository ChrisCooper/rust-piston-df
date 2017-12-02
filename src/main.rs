extern crate rand;
extern crate config;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

#[macro_use]
extern crate serde_derive;


//use std::io;

mod map;
mod app;
mod settings;

use settings::GameSettings;

fn main() {

    let settings = GameSettings::new_from_defaults();

    // Print out our settings
    println!("Loaded settings:\n{:?}", settings);

    /*let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "piston-rust-playground",
        [200, 200]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = app::App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }*/
}

