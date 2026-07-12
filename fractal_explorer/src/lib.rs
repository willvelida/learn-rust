use std::ops::{Add, Mul};
use std::fs::File;
use std::io::{BufWriter, Write};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    re: f64,
    im: f64,
}

pub struct Mandelbrot;

pub struct Julia {
    pub c: Complex,
}

pub trait Fractal {
    fn name(&self) -> &str;
    fn escape_time(&self, point: Complex, max_iters: u32) -> Option<u32>;
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    fn norm_squared(self) -> f64 {
        self.re * self.re + self.im * self.im 
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,)
    }
}

impl Fractal for Mandelbrot {
    fn name(&self) -> &str {
        "mandelbrot"
    }

    fn escape_time(&self, c: Complex, max_iters: u32) -> Option<u32> {
        let mut z = Complex::new(0.0, 0.0);
        for i in 0..max_iters  {
            if z.norm_squared() > 4.0 {
                return Some(i);
            }
            z = z * z + c;
        }
        None 
    }
}

impl Fractal for Julia {
    fn name(&self) -> &str {
        "julia"
    }

    fn escape_time(&self, point: Complex, max_iters: u32) -> Option<u32> {
        let mut z = point;
        for i in 0..max_iters  {
            if z.norm_squared() > 4.0 {
                return Some(i);
            }
            z = z * z + self.c;
        }
        None
    }
}

fn pixel_to_point(px: usize, py: usize, width: usize, height: usize) -> Complex {
    let re = -2.5 + (px as f64 / width as f64) * 3.5;
    let im = -1.15 + (py as f64 / height as f64) * 2.3;
    Complex::new(re, im)
}

fn color(escape: Option<u32>, max_iters: u32) -> (u8, u8, u8) {
    match escape {
        None => (0,0,0),
        Some(n) => {
            let t = n as f64 / max_iters as f64;
            let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
            let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
            let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;
            (r, g, b)
        }
    }
}

pub fn render(fractal: &(dyn Fractal + Sync), 
    width: usize,
    height: usize, 
    max_iters: u32) -> Vec<(u8, u8, u8)> {
    (0..height)
        .into_par_iter()
        .flat_map(|py| {
            (0..width)
                .map(|px| {
                    let point = pixel_to_point(px, py, width, height);
                    color(fractal.escape_time(point, max_iters), max_iters)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn write_ppm(path: &str, pixels: &[(u8, u8, u8)], width: usize, height: usize) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut out = BufWriter::new(file);
    writeln!(out, "P3\n{width} {height}\n255")?;
    for (r, g, b) in pixels  {
        writeln!(out, "{r} {g} {b}")?;
    }
    Ok(())
}