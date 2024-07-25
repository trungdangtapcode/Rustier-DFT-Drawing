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
use crate::colors;

pub struct Board{
    screen_size: [u32; 2],
    pub screen_offset: Complex<f64>,
    current_time: f64,
    pub points: Vec<Complex<f64>>,
}

impl Board{
    pub fn new(screen_size: [u32; 2]) -> Board{
        Board{
            screen_size: screen_size,
            screen_offset: Complex::<f64>::new(screen_size[0] as f64, screen_size[1] as f64)/2.0,
            current_time: 0.0,
            points: Vec::new(),
        }
    }
    pub fn update(&mut self, dt: f64){
        self.current_time += dt;
    }
    pub fn update_mouse_pos(&mut self, mouse_pos: [f64; 2]){
        self.points.push(Complex::<f64>::new(mouse_pos[0], mouse_pos[1]));
    }
    pub fn render(&mut self, t: graphics::Viewport, gl: &mut GlGraphics){
        gl.draw(t, |c, gl|{
            let lines = interp::list_complex_to_lines(&self.points);
            for line in lines{
                graphics::line(colors::GREEN,1.0, line.to_array(), c.transform, gl);
            }
        });
    }
}