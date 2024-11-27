
mod client; // includes a module
mod complex;
mod image;
mod mandelbrot;



fn main() {
    // uncomment and implement argument parsing, priting an error message in case of a parsing error
    let (width, height, max_iterations) = client::parse_args().expect("Invalid");

    // if the above was implemented correctly you can uncomment this line
    println!(
        "Generating Mandelbrot for {}x{} image (max_iterations: {})",
        width, height, max_iterations
    );

    let image = mandelbrot::generate_image(width, height, max_iterations);


    let mandelbrot_pixel_count = image.get_mandelbrot_pixels();


    // if the above line worked you should be able to uncomment this line
    println!("Pixels in the set: {}", mandelbrot_pixel_count);

    // uncomment and call after you implement the mandelbrot functions, and handle the possible error
    client::save_to_file(&image, "mandelbrot.ppm");
}
