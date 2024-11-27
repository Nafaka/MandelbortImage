use crate::complex::Complex;
// uncomment the code below and implement the functions
use crate::image::{Image, Pixel};

pub fn check_pixel(c: Complex, max_iterations: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..max_iterations {
        z = z * z + c;
        if z.mag() > 4.0 {
            return Some(i);
        }
    }
    None
}

pub fn generate_image(width: usize, height: usize, max_iterations: usize) -> Image {
    let mut image: Image = Image::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let cx = (x as f64 / image.width as f64 - 0.75) * 3.5;
            let cy = (y as f64 / image.height as f64 - 0.5) * 2.0;
            let c = Complex { re: cx, im: cy };

            let pix = match check_pixel(c, max_iterations) {
                Some(_) => Pixel {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                None => Pixel { r: 0, g: 0, b: 0 },
            };
            if let Some(pixel) = image.get_mut(x, y) {
                *pixel = pix;
            }
            /* Also possible same as the one up but more spaghetti code
            match image.get_mut(x,y){
                Some(pixel) => { *pixel = pix }
                None => { }
            }
             */
        }
    }
    image
}
