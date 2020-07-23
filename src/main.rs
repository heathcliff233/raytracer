mod ray;
#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use ray::Ray;
use vec3::{Color, Point3, Vec3};

fn main() {
    // given
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);
    // image
    let image_width = 400;
    let image_height = 225;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(400);

    // Camera
    let viewport_h = 2.0;
    let viewport_w = 16.0 / 9.0 * viewport_h;
    let focal_len = 1.0;
    let origin = Point3::zero();
    let horizontal = Vec3 {
        x: viewport_w,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_h,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_len,
        };

    for x in 0..image_width {
        for y in 0..image_height {
            let u = x as f64 / (image_width - 1) as f64;
            let v = (image_height - y) as f64 / (image_height - 1) as f64;
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + horizontal * u + vertical * v - origin,
            };
            let pixel_color = r.ray_color();
            let pixel = img.get_pixel_mut(x, y);
            *pixel = image::Rgb([
                pixel_color.x as u8,
                pixel_color.y as u8,
                pixel_color.z as u8,
            ]);
        }
        bar.inc(1);
    }

    // save
    img.save("output/test.png").unwrap();
    bar.finish();
}
