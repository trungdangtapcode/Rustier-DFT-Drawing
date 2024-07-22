extern crate num;

use std::f64::consts::PI;

use num::complex::Complex;
// pub fn get_primitive_complex<T, V, U>(n: T, i: V, k: U) -> Complex<f64> {
//     Complex::<f64>::new(0.0,-(i as f64)*2.0*PI/(n as f64)*(k as f64)).exp()
// }
pub fn from_complex_vector(points: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let N: usize = points.len();
    let num_cyc: usize = points.len();
    let mut c = vec![Complex::<f64>::new(0.0,0.0);num_cyc];
    for k in 0..num_cyc {
        for (i, p) in points.iter().enumerate(){
            c[k] += p*Complex::<f64>::new(0.0,-(i as f64)*2.0*PI/(N as f64)*(k as f64)).exp();
        }
        c[k] /= N as f64;
    }
    c
}