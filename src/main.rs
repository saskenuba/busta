use busta::ray::Ray;
use busta::vec::Vec3;
use std::fmt;

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

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for i in (0..height).rev() {
        for j in 0..width {
            let r = f64::from(j) / f64::from(width);
            let g = f64::from(i) / f64::from(height);

            let new_ray_calc = &(&lower_left_corner + &(&horizontal * r) + &vertical * g);
            let new_ray = Ray::new(&origin, new_ray_calc);
            let col = Vec3::color(&new_ray);

            let pixel = RGBPixel {
                r: (255.99 * col.vector[0]) as i32,
                g: (255.99 * col.vector[1]) as i32,
                b: (255.99 * col.vector[2]) as i32,
            };

            println!("{}", pixel);
        }
    }
}

fn main() {
    let imagesize = (200, 100);
    output_image(imagesize.0, imagesize.1);
}
