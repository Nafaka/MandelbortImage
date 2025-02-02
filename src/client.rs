//use std::env;
use std::fs;
use std::num::ParseIntError;
// you may need use std::env for parsing arguments
// use std::num::ParseIntError;

use crate::image::Image; // use from another module

// uncomment and implement:
pub fn parse_args() -> Result<(usize, usize, usize), ParseIntError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} <width> <height> <max_iterations>", args[0]);
        std::process::exit(1);
    }
    let width = args[1].parse()?;
    let height = args[2].parse()?;
    let max_iterations = args[3].parse()?;

    Ok((width, height, max_iterations))
}

pub fn save_to_file(image: &Image, filename: &str) {
    let mut s = String::new();
    s.push_str(&format!("P3\n{} {}\n255\n", image.width, image.height));

    for y in 0..image.height {
        for x in 0..image.width {
            let pixel = image.get(x, y).unwrap();
            s.push_str(&format!("{}\n", pixel));
        }
    }

    fs::write(filename, s).expect("Error writing to disk!");
}
