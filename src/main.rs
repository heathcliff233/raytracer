#[allow(clippy::float_cmp)]
mod camera;
mod color;
mod hittable;
mod hittablelist;
mod material;
mod ray;
mod rtweekend;
mod vec3;
use camera::Camera;
use color::{ray_color, write_color};
use hittable::Sphere;
use hittablelist::HitTableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use material::{Dielectric, Lambertian, Metal};
use rtweekend::random_double;
use std::sync::Arc;
use vec3::{Color, Point3, Vec3};

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);
    let max_depth = 50;
    // World
    let mut world = HitTableList::new();
    let material_ground = Arc::new(Lambertian {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    });
    let material_center = Arc::new(Lambertian {
        albedo: Color {
            x: 0.1,
            y: 0.2,
            z: 0.5,
        },
    });
    let material_left = Arc::new(Dielectric { ref_idx: 1.5 });
    let material_right = Arc::new(Metal::new(
        &Color {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        0.0,
    ));
    world.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        mat_ptr: material_ground,
    }));
    world.add(Arc::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat_ptr: material_center,
    }));
    world.add(Arc::new(Sphere {
        center: Point3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat_ptr: material_left.clone(),
    }));
    world.add(Arc::new(Sphere {
        center: Point3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: -0.4,
        mat_ptr: material_left,
    }));
    world.add(Arc::new(Sphere {
        center: Point3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        mat_ptr: material_right,
    }));
    // Camera
    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );
    // Main Loop
    for x in 0..image_width {
        for y in 0..image_height {
            let mut pixel_color = Color::zero();
            for _s in 0..samples_per_pixel {
                let u = (x as f64 + random_double(0.0, 1.0)) / (image_width - 1) as f64;
                let v = ((image_height - y) as f64 + random_double(0.0, 1.0))
                    / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(&mut img, x, y, &pixel_color, samples_per_pixel);
        }
        //bar.inc(1);
    }
    // Save
    img.save("output/test.png").unwrap();
    bar.finish();
}
