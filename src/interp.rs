use std::{f64::consts::PI, ops::Add, process::Output};

use num::{complex::{Complex, ComplexFloat}, Signed};

#[derive(Debug)]
pub struct Line<T>
{
    pub start_point: Complex<T>,
    pub end_point: Complex<T>
}

impl<T> Line<T>
where 
    T: std::ops::Sub<Output = T> ,
    Complex<T>: std::marker::Copy + std::ops::Sub<,Output = Complex<T>>;
{
    pub fn abs(&self) -> f64 {
        0.0
        // self.start_point.abs()
    }
}