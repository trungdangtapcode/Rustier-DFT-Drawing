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
const CIRCLE_SPEED: f64 = 20.0;
const PENDULUM_LINE_RAD: f64 = 2.0;
const DRAW_LINE_RAD: f64 = 1.0;

pub struct Circles {
    coef: Vec<Complex<f64>>,
    current_time: f64,
    screen_size: [u32; 2],
    pub screen_offset: Complex<f64>,
    drawed_lines: Vec<interp::Line>,
}

impl Circles{
    pub fn new(points: &Vec<Complex<f64>>, num_circles: usize, screen_size: [u32; 2]) -> Circles {
        let resized_points = interp::resize_interp(points, num_circles);
        let coef = dft::from_complex_vector(&resized_points);
        Circles {
            coef: coef,
            current_time: 0.0,
            screen_size: screen_size.to_owned(),
            screen_offset: Complex::<f64>::new(screen_size[0] as f64, screen_size[1] as f64)/2.0,
            drawed_lines: Vec::new(),
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
            for line in self.drawed_lines.iter(){
                let line = line.to_array();
                graphics::line(colors::WHITE, DRAW_LINE_RAD, line, c.transform, gl);
            }
            let r: f64 = self.current_time/CIRCLE_SPEED;
            let mut last = Complex::<f64>::new(0.0,0.0);
            for (k, c_k) in self.coef.iter().enumerate(){
                let cur = last + c_k*Complex::<f64>::new(0.0,2.0*PI*(k as f64)*r).exp();
                let line = interp::Line::new((last+self.screen_offset).to_owned(), (cur+self.screen_offset).to_owned()).to_array();
                graphics::line(colors::RED, PENDULUM_LINE_RAD, line, c.transform, gl);
                last = cur;
                if (k>5000){
                    break;
                }
            }
        });
    }
    pub fn update(&mut self, dt: f64){
        let u = self.get_final_point()+self.screen_offset;
        self.current_time += dt;
        let v = self.get_final_point()+self.screen_offset;
        if (self.current_time<=CIRCLE_SPEED){
            self.drawed_lines.push(interp::Line::new(u,v));
        }
    }
    pub fn get_final_point(&self) -> Complex<f64>{
        let r: f64 = self.current_time/CIRCLE_SPEED;
        let mut f = Complex::<f64>::new(0.0,0.0);
        for (k, c) in self.coef.iter().enumerate(){
            f += c*Complex::<f64>::new(0.0,2.0*PI*(k as f64)*r).exp();
            if (k>5000){
                break;
            }
        }
        f
    }
}