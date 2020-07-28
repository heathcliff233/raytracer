mod aabb;
mod aarect;
mod bvh;
#[allow(clippy::float_cmp)]
mod camera;
mod color;
mod hittable;
mod hittablelist;
mod material;
mod ray;
mod rtweekend;
mod texture;
mod vec3;
use aarect::XYRect;
use bvh::BVHNode;
use camera::Camera;
use color::{ray_color, write_color};
use hittable::Sphere;
use hittablelist::HitTableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use material::{Dielectric, DiffuseLight, Lambertian, Metal};
use rtweekend::random_double;
use std::sync::Arc;
use texture::CheckerTexture;
use vec3::{randomvec, Color, Point3, Vec3};

pub fn simple_light() -> HitTableList {
    let mut world = HitTableList::new();
    let checker = Arc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian {
            albedo: checker.clone(),
        }),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian { albedo: checker }),
    )));
    let difflight = Arc::new(DiffuseLight::new(Color::new(4.0, 0.0, 4.0)));
    world.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    world
}

pub fn check(world: &HitTableList, center: &Point3) -> bool {
    for object in &world.objects {
        let dis = object.distance(center);
        if dis < center.y {
            return false;
        }
    }
    true
}

pub fn random_scene() -> HitTableList {
    let mut world = HitTableList::new();
    let checker = Arc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian { albedo: checker }),
    )));
    let material_1 = Arc::new(CheckerTexture::new(
        Color::new(254.0, 67.0, 101.0) / 255.0 * 1.7,
        Color::new(249.0, 205.0, 173.0) / 255.0 * 1.7,
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(DiffuseLight { emit: material_1 }),
    )));
    for a in -15..15 {
        for b in -15..15 {
            let choose_mat = random_double(0.0, 1.0);
            let center = Point3::new(
                a as f64 + 0.9 * random_double(0.0, 1.0),
                random_double(0.05, 0.5),
                b as f64 + 0.9 * random_double(0.0, 1.0),
            );
            if !check(&world, &center) {
                continue;
            }
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.4 {
                    let difflight = randomvec().elemul(randomvec()) * 1.7;
                    let sphere_material = Arc::new(DiffuseLight::new(difflight));
                    world.add(Arc::new(Sphere::new(center, center.y, sphere_material)));
                } else if choose_mat < 0.6 {
                    let albedo = randomvec().elemul(randomvec());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, center.y, sphere_material)));
                } else if choose_mat < 0.8 {
                    let albedo = randomvec().elemul(randomvec());
                    let fuzz = random_double(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, center.y, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, center.y, sphere_material)));
                }
            }
        }
    }
    world
}

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 3000;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 400;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);
    let max_depth = 50;
    // World
    // let mut world = random_scene();
    let mut world = random_scene();
    let length = world.objects.len();
    let world = BVHNode::new(&mut world.objects, 0, length, 0.0, 0.1);
    let background = Color::new(0.0, 0.0, 0.0);
    // Camera
    let lookfrom = Point3::new(13.0, 5.0, 10.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 15.0;
    let aperture = 0.2;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        45.0,
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
                pixel_color += ray_color(&r, &background, &world, max_depth);
            }
            write_color(&mut img, x, y, &pixel_color, samples_per_pixel);
        }
        bar.inc(1);
    }
    // Save
    img.save("output/test.png").unwrap();
    bar.finish();
}
