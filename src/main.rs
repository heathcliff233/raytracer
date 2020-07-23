#[allow(clippy::float_cmp)]
mod camera;
mod color;
mod hittable;
mod hittablelist;
mod ray;
mod rtweekend;
mod vec3;
use camera::Camera;
use color::{ray_color, write_color};
use hittable::Sphere;
use hittablelist::HitTableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rtweekend::random_double;
use vec3::{Color, Point3};

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);
    // World
    let mut world = HitTableList::new();
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    }));
    // Camera
    let cam = Camera::new();
    // Main Loop
    for x in 0..image_width {
        for y in 0..image_height {
            let mut pixel_color = Color::zero();
            for _s in 0..samples_per_pixel {
                let u = (x as f64 + random_double(0.0, 1.0)) / (image_width - 1) as f64;
                let v = ((image_height - y) as f64 + random_double(0.0, 1.0))
                    / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            write_color(&mut img, x, y, &pixel_color, samples_per_pixel);
        }
        bar.inc(1);
    }
    // Save
    img.save("output/test.png").unwrap();
    bar.finish();
}
