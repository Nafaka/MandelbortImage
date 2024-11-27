// implement the Pixel struct and traits below

use std::fmt;

#[derive(Clone, Copy, Debug,PartialEq)]


pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

// implement the Image struct and traits below

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let data = vec![Pixel { r: 0, g: 0, b: 0 }; width * height];
        Self { width, height, data }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x < self.width || y < self.height {
            self.data.get(y * self.width + x)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x < self.width || y < self.height {
            self.data.get_mut(y * self.width + x)
        } else {
            None
        }
    }

    pub fn get_mandelbrot_pixels(&self) -> usize {
        // check the again
        let mut sum: usize = 0;
        for i in &self.data {
            if i.r == 0 && i.b == 0 && i.g == 0 {
                sum += 1;
            }
        }
        sum
        // &self.data.iter().filter(|i| i.r == 0 && i.b == 0 && i.g == 0 ).count() with iter and closure
    }
}
