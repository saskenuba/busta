use std::fmt;

mod vec;

struct RGBPixel {
    r: i32,
    g: i32,
    b: i32,
}

impl fmt::Display for RGBPixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

fn output_image(width: i32, height: i32) {
    /*
    Outputs image in .PPM format.
     */
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    for i in (0..height).rev() {
        for j in 0..width {
            let r = j as f64 / width as f64;
            let g = i as f64 / height as f64;
            let b = 0.2;

            let pixel = RGBPixel {
                r: (255.99 * r) as i32,
                g: (255.99 * g) as i32,
                b: (255.99 * b) as i32,
            };

            println!("{}", pixel);
        }
    }
}

fn main() {
    let imagesize = (200, 100);
    output_image(imagesize.0, imagesize.1);
}
