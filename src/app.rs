use std::collections::HashMap;

use piston::input::*;
use opengl_graphics::{ GlGraphics };

type AxisValues = HashMap<(i32, u8), f64>;

pub struct App {

    // OpenGL drawing backend.
    pub gl: GlGraphics,

    // Rotation for the square.
    pub rotation: f64,

    pub axis_values: AxisValues,

    pub cursor: (f64, f64),

    // Prevent literal instantiation of this struct
    _secret: ()
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App {
            gl,
            rotation: 0.0,
            axis_values: HashMap::new(),
            cursor: (0.0, 0.0),
            _secret: ()
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    pub fn process_inputs(&mut self, e: &Event) {

        if let Some(c) = e.cursor_args() {
            if c {
                //println!("Mouse entered");
            } else {
                //println!("Mouse left");
            }
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(key) => {
                    if key == Key::C {
                        //println!("Pressed C");
                    }

                    //println!("Pressed keyboard key '{:?}'", key);
                },
                Button::Mouse(button) => {
                    //println!("Pressed mouse button '{:?}'", button)
                },
                Button::Controller(button) => {
                    //println!("Released controller button '{:?}'", button)
                },
            }
        }

        if let Some(button) = e.button_args() {
            //println!("Scancode {:?}", button.scancode);
        }

        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => {
                    //println!("Released keyboard key '{:?}'", key)
                },
                Button::Mouse(button) => {
                    //println!("Released mouse button '{:?}'", button)
                },
                Button::Controller(button) => {
                    //println!("Released controller button '{:?}'", button)
                },
            }
        }
        if let Some(axis_args) = e.controller_axis_args() {
            self.axis_values.insert((axis_args.id, axis_args.axis), axis_args.position);
        }

        e.mouse_cursor(|x, y| {
            self.cursor = (x, y);
            //println!("Mouse moved '{} {}'", x, y);
        });
        e.mouse_scroll(|dx, dy| {
            //println!("Scrolled mouse '{}, {}'", dx, dy)
        });
        e.mouse_relative(|dx, dy| {
            //println!("Relative mouse moved '{} {}'", dx, dy)
        });
        e.text(|text| {
            //println!("Typed '{}'", text)
        });
        e.resize(|w, h| {
            //println!("Resized '{}, {}'", w, h)
        });
    }
}