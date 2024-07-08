extern crate num;

use num::complex::Complex;

pub fn from_complex_array(arr: i32) -> Vec<Complex<f64>> {
    println!("Hello, world!");
    let res: Vec<Complex<f64>> = vec![Complex::new(1.0,1.0); 20];
    res
}