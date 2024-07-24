use std::{f64::consts::PI, ops::Add, process::Output};

use num::{
    complex::{Complex, ComplexFloat},
    Signed,
};
use std::cmp::min;

#[derive(Debug, Clone)]
pub struct Line {
    pub start_point: Complex<f64>,
    pub end_point: Complex<f64>,
}

impl Line {
    pub fn new(start_point: Complex<f64>, end_point: Complex<f64>) -> Line {
        Line {
            start_point: start_point,
            end_point: end_point,
        }
    }
    pub fn abs(&self) -> f64 {
        let diff = self.end_point - self.start_point;
        diff.norm()
    }
    pub fn normalize(&self) -> Complex<f64> {
        let diff = self.end_point - self.start_point;
        if (diff.norm() == 0.0) {
            return Complex::new(0.0, 0.0);
        }
        diff / diff.norm()
    }
    pub fn to_array(&self) -> [f64; 4] {
        [self.start_point.re, self.start_point.im, 
        self.end_point.re, self.end_point.im]
    }
}

pub fn list_complex_to_lines(points: &Vec<Complex<f64>>) -> Vec<Line> {
    let mut lines = Vec::new();
    for i in 0..points.len() - 1 {
        lines.push(Line {
            start_point: points[i],
            end_point: points[i + 1],
        });
    }
    lines.push(Line {
        start_point: points[points.len() - 1],
        end_point: points[0],
    });
    lines
}

pub fn resize_interp(points: &Vec<Complex<f64>>, num_points: usize) -> Vec<Complex<f64>> {
    let mut new_points = Vec::new();
    let lines = list_complex_to_lines(points);
    let mut sum_len: f64 = 0.0;
    for line in &lines {
        sum_len += line.abs();
    }
    let each_len: f64 = sum_len / num_points as f64;
    let mut remain_len: f64 = 0.0;
    for line in &lines {
        let mut cur_len: f64 = line.abs();
        if (remain_len >= cur_len) {
            remain_len -= cur_len;
            continue;
        }
        let mut cur_point = line.start_point + line.normalize() * remain_len;
        cur_len -= remain_len;
        while (cur_len > 0.0) {
            // println!("cur_point: {:?}", cur_point);
            new_points.push(cur_point);
            cur_point = cur_point.to_owned() + line.normalize() * each_len;
            cur_len -= each_len;
        }
        remain_len = -cur_len;
    }
    new_points
}
