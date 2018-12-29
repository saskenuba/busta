use std::fmt;

mod vec;

#[derive(Debug)]
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

fn outputImage(width: i32, height: i32) {
    for i in 0..width {
        for j in 0..height {
            let basinga = i as f64 / width as f64;
            let hehe = j as f64 / height as f64;
            let wat = 0.2;

            let okok = RGBPixel {
                r: (255.99 * basinga) as i32,
                g: (255.99 * hehe) as i32,
                b: (255.99 * wat) as i32,
            };

            println!("{:?}", okok);
        }
    }
}

fn main() {
    outputImage(5, 5);
}
