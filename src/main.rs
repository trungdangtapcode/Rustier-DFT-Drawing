extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

// use color::GREEN;
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;


use piston::{event_loop::*, RenderEvent};
use opengl_graphics::{GlGraphics, OpenGL}; // Add this line to import the OpenGL module
use num::complex::{Complex, ComplexFloat};

use std::io::Write;
use std::f64::consts::{E, PI};

// pub mod dft;
mod dft;
mod interp;
use interp::*;

#[derive(Debug)]
struct Tup(i32, i32);

pub mod say_hello {
    pub fn hello() {
        println!("Hello, world!");
    }
}

//draw a rectangle in rust  using pistol 
fn main() {
    let v = vec![Complex::<f64>::new(1.0,1.0),
        Complex::<f64>::new(-1.0,1.0),
        Complex::<f64>::new(-1.0,-1.0),
        Complex::<f64>::new(1.0,-1.0)];
    // println!("{:?}",x.abs());
    // println!("{}",x.abs());
    let nv = interp::resize_interp(&v,8);
    // let nv = v;
    // println!("{:?}",nv);

    let coef = dft::from_complex_vector(&nv);
    for c in coef.iter(){
        let tmp = c.to_polar();
        // println!("{}  angle = {}",tmp.0, tmp.1/PI*180.0);
    }
    let T: i32 = 100;
    for t in 0..T{
        let r: f64 = (t as f64)/(T as f64);
        let mut f = Complex::<f64>::new(0.0,0.0);
        for (k, c) in coef.iter().enumerate(){
            f += c*Complex::<f64>::new(0.0,2.0*PI*(k as f64)*r).exp();
        }
        println!("{} = {}",r, f);
    }
    // let (mag, p) = Complex::<f64>::new(0.0,2.0).to_polar();
    // println!("{}",p);

    return;
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
            let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
            
            gl.draw(args.viewport(), |c, gl|{
                let transform = c.transform;
                graphics::clear(GREEN, gl);
                let square = rectangle::square(10 as f64, 10 as f64, 100 as f64);
                graphics::rectangle(RED, square, c.transform, gl);
            });
        }
    }
    // dft::front_house::greet();
    // dft::hello();
}







