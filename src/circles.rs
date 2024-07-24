extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

use glutin_window::GlutinWindow; // gl module is used to create a window
use graphics::*; // all graphics
use piston::window::WindowSettings; 

use piston::{event_loop::*, RenderEvent}; // loop for events
use opengl_graphics::{GlGraphics, OpenGL}; // Add this line to import the OpenGL module
use num::complex::{Complex, ComplexFloat};

use std::io::Write;
use std::f64::consts::{E, PI};

use crate::interp;
use crate::dft;
use crate::colors;


pub struct Circles {
    coef: Vec<Complex<f64>>,
}

impl Circles{
    pub fn new(points: &Vec<Complex<f64>>, num_circles: usize) -> Circles {
        let resized_points = interp::resize_interp(points, num_circles);
        let coef = dft::from_complex_vector(&resized_points);
        Circles {
            coef: coef,
        }
    }

    pub fn print_coef(&self){
        for (k,c) in self.coef.iter().enumerate() {
            let tmp = c.to_polar();
            println!("k = {}: amp = {}  phase = {}", k,tmp.0, tmp.1/PI*180.0);
        }
    }

    pub fn print_test(&self){
        let T: i32 = 100;
        for t in 0..T{
            let r: f64 = (t as f64)/(T as f64);
            let mut f = Complex::<f64>::new(0.0,0.0);
            for (k, c) in self.coef.iter().enumerate(){
                f += c*Complex::<f64>::new(0.0,2.0*PI*(k as f64)*r).exp();
            }
            println!("{} = {}",r, f);
        }
    }

    pub fn render(&self, t: graphics::Viewport, gl: &mut GlGraphics){
        gl.draw(t, |c, gl|{
            let mut x = 0.0;
            let mut y = 0.0;
            let mut prev_x = 0.0;
            let mut prev_y = 0.0;
            let mut prev_c = Complex::<f64>::new(0.0,0.0);
            for (k, c) in self.coef.iter().enumerate(){
                let tmp = c.to_polar();
                let amp = tmp.0;
                let phase = tmp.1;
                x += amp*phase.cos();
                y += amp*phase.sin();
                let color = colors::get_color(k);
                let line = Line::new(color, 1.0);
                line.draw([prev_x, prev_y, x, y], &c.draw_state, c.transform, gl);
                prev_x = x;
                prev_y = y;
            }
        });
    }
}