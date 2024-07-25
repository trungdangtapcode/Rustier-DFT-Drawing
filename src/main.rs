extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

// use color::GREEN;
use glutin_window::GlutinWindow; 
use piston::window::WindowSettings;

use piston::{event_loop::*, Button, MouseCursorEvent, PressEvent, ReleaseEvent, RenderEvent, UpdateEvent};
use opengl_graphics::{GlGraphics, OpenGL}; // Add this line to import the OpenGL module
use num::complex::{Complex, ComplexFloat};

use std::io::Write;
use std::f64::consts::{E, PI};
use std::thread::current;


trait ComplexTrait{
    fn myexp(&self) -> Complex<f64>;
}
impl ComplexTrait for Complex<f64>{
    fn myexp(&self) -> Complex<f64>{
        return Complex::<f64>::new(f64::cos(self.im), f64::sin(self.im));
    }
}

// pub mod dft;
mod dft;
mod interp;
mod circles;
mod colors;
mod painting;

const IS_SHOW_POINTS: bool = false;

#[derive(Debug)]
struct Tup(i32, i32);

//draw a rectangle in rust  using pistol 
fn main() {
    // println!("{}", f64::sin(PI));
    // println!("{}", Complex::<f64>::new(0.0, PI).exp());
    // println!("{}", Complex::<f64>::new(0.0, PI).myexp());
    let num_circle: usize = 5000;
    let screen_size: [u32 ; 2] = [1280, 720];
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("rustier", screen_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut circles = circles::Circles::new(&vec![], num_circle, screen_size);
    let mut board: painting::Board = painting::Board::new(screen_size);
    let mut isClicked: bool = false;
    
    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            graphics::clear(colors::BLACK, &mut gl);
            let t = args.viewport();
            board.render(t, &mut gl);
            if (IS_SHOW_POINTS){
                let tmp = interp::resize_interp(&board.points, num_circle);
                gl.draw(t, |c, gl|{
                    for p in &tmp{
                        let rad = 2.0;
                        let rect =  [p.re-rad, p.im-rad, 2.0*rad, 2.0*rad];
                        graphics::ellipse(colors::YELLOW, rect, c.transform, gl);
                    }
                });
            }
            if !isClicked{
                circles.render(t, &mut gl);
            }
        }
        if let Some(args) = e.mouse_cursor_args() {
            if isClicked {
                board.update_mouse_pos(args);
            }
        }
        if let Some(Button::Mouse(button)) = e.press_args() {
            isClicked = true;
            board = painting::Board::new(screen_size);
        }
        if let Some(Button::Mouse(button)) = e.release_args() {
            isClicked = false;
            let mut point_screen_to_coor: Vec<Complex<f64>> =  vec![];
            for point in &board.points{
                point_screen_to_coor.push(point-board.screen_offset);
            }
            circles = circles::Circles::new(&point_screen_to_coor, num_circle, screen_size);
        }
            
        if let Some(args) = e.update_args() {
            circles.update(args.dt);
        }
    }
    // dft::front_house::greet();
    // dft::hello();
}







