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
use vec3::{randomvec, Color, Point3, Vec3};

pub fn random_scene() -> HitTableList {
    let mut world = HitTableList::new();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0.0, 1.0);
            let center = Point3::new(
                a as f64 + 0.9 * random_double(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * random_double(0.0, 1.0),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = randomvec().elemul(randomvec());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = randomvec().elemul(randomvec());
                    let fuzz = random_double(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material_1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));
    let material_2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));
    let material_3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));
    world
}

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);
    let max_depth = 50;
    // World
    let world = random_scene();
    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
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
        bar.inc(1);
    }
    // Save
    img.save("output/test.png").unwrap();
    bar.finish();
}
