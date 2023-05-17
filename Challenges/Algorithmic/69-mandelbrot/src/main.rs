use image::{GrayAlphaImage, LumaA};
use num_complex::Complex as C;
use std::io::{stdout, Write};

// RANGE is the range of the complex plane to be plotted.
const RANGE: ((f64, f64), (f64, f64)) = ((-2., 0.47), (1.12, -1.12));
// RESOLUTION is the number of pixels per unit.
const RESOLUTION: f64 = 2048.;
// PRECISION corresponds to the maximum number of iterations.
const PRECISION: u32 = 255;

fn main() {
  // The width and height of the image scaled with SIZE.
  let width = (((RANGE.0).0 - (RANGE.0).1).abs() * RESOLUTION).round() as u32;
  let height = (((RANGE.1).0 - (RANGE.1).1).abs() * RESOLUTION).round() as u32;

  // Creates a new image buffer in grayscale with an alpha channel.
  let mut img = GrayAlphaImage::new(width, height);

  // Iterates over the pixels of the image and sets their color according to the
  // number of iterations it took for the complex number to diverge.
  for (x, y, pixel) in img.enumerate_pixels_mut() {
    *pixel = get_pixel(x, y, width, height);

    // Prints the progress every 5% completed.
    if y % (height / 20) == 0 && x == 0 {
      print!("\r{}%", (y as f64 / height as f64 * 100.).round());
      stdout().flush().unwrap();
    }
  }
  println!("\r100% Done!");
  stdout().flush().unwrap();

  // Saves the image.
  img.save("./images/mandelbrot.png").unwrap();
}

fn get_pixel(px: u32, py: u32, w: u32, h: u32) -> LumaA<u8> {
  // Creates a complex number c based on the x and y coordinates of the pixel.
  let c = C::new(
    px as f64 / w as f64 * ((RANGE.0).1 - (RANGE.0).0) + (RANGE.0).0,
    py as f64 / h as f64 * ((RANGE.1).1 - (RANGE.1).0) + (RANGE.1).0,
  );
  // Creates z with its first value z0 being 0+0i.
  let mut z = C::new(0., 0.);
  // Declares i to keep track of the number of iterations it took for the number
  // to diverge.
  let mut i = 0;

  // Runs z = z^2 + c PRECISION times, breaks early if the absolute value of the
  // of z exceeds 2, which happens when z diverges to infinity.
  while i < PRECISION && z.norm() <= 2. {
    z = z.powi(2) + c;
    i += 1;
  }

  // Smoothes i for a better result.
  let mut smooth_i = i as f64 + 1. - z.norm().ln().ln() / 2f64.ln();
  // When z.norm().ln() is negative (norm < 1), z.norm().ln().ln() yields NaN.
  if smooth_i.is_nan() {
    smooth_i = PRECISION as f64
  }

  // The resulting color is the number of iterations it took for the number to
  // diverge scaled to lie between 0 and 255.
  LumaA([((smooth_i / PRECISION as f64) * 255.).round() as u8; 2])
}
