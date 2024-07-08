extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

pub mod dft;

// use color::GREEN;
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use piston::{event_loop::*, RenderEvent};
use opengl_graphics::{GlGraphics, OpenGL}; // Add this line to import the OpenGL module
use num::complex::Complex;

use std::io::Write;
use std::f64::consts::PI;


#[derive(Debug)]
struct Tup(i32, i32);

pub mod say_hello {
    pub fn hello() {
        println!("Hello, world!");
    }
}

//draw a rectangle in rust  using pistol 
fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("rustier", [1280, 720])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    
    let mut events = Events::new(EventSettings::new()).ups(10);
    // let mut events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            use graphics::*;
            let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
            
            gl.draw(args.viewport(), |c, gl|{
                let transform = c.transform;
                graphics::clear(GREEN, gl);
                let square = rectangle::square(10 as f64, 10 as f64, 100 as f64);
                // graphics::rectangle(RED, )
            });
        }
    }
    // dft::front_house::greet();
    // dft::hello();
}







