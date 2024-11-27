// implement the Complex struct and traits below
use std::ops::Add;
use std::ops::Mul;
#[derive(Copy, Clone, Debug)]

pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Add for Complex {
    type Output = Self;
    fn add(self, num: Complex) -> Self {
        Complex {
            re: self.re + num.re,
            im: self.im + num.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, num: Complex) -> Self {
        Complex {
            re: (self.re * num.re) - (self.im * num.im),
            im: (self.re * num.im) + (self.im * num.re),
        }
    }
}

impl Complex {
    pub fn mag(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}
