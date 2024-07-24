extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

// use color::GREEN;
use glutin_window::GlutinWindow; 
use piston::window::WindowSettings;

use piston::{event_loop::*, RenderEvent, UpdateEvent};
use opengl_graphics::{GlGraphics, OpenGL}; // Add this line to import the OpenGL module
use num::complex::{Complex, ComplexFloat};

use std::io::Write;
use std::f64::consts::{E, PI};
use std::thread::current;

// pub mod dft;
mod dft;
mod interp;
mod circles;
mod colors;

#[derive(Debug)]
struct Tup(i32, i32);

//draw a rectangle in rust  using pistol 
fn main() {
    let num_circle: usize = 1000;
    let v = vec![Complex::<f64>::new(100.0,100.0),
        Complex::<f64>::new(-100.0,100.0),
        Complex::<f64>::new(-100.0,-100.0),
        Complex::<f64>::new(100.0,-100.0)];

    let screen_size: [u32 ; 2] = [1280, 720];
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("rustier", screen_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut circles = circles::Circles::new(&v, num_circle, screen_size);
    let mut drawed_lines: Vec<interp::Line> = vec![];
    
    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            graphics::clear(colors::BLACK, &mut gl);
            let t = args.viewport();
            for line in &drawed_lines{
                gl.draw(t, |c, gl|{
                    graphics::line(colors::GREEN, 2.0, line.to_array(), c.transform, gl);
                });
            }
            circles.render(t, &mut gl);
        }
        if let Some(args) = e.update_args() {
            let u = circles.get_final_point()+circles.screen_offset;
            circles.update(args.dt);
            let v = circles.get_final_point()+circles.screen_offset;
            drawed_lines.push(interp::Line::new(u,v));
        }
    }
    // dft::front_house::greet();
    // dft::hello();
}







