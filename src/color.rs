use crate::{
    hittable::{HitRecord, HitTable},
    ray::Ray,
    rtweekend::clamp,
    vec3::{random_in_unit_sphere, Color},
};
use image::{Rgb, RgbImage};
use std::f64::INFINITY;

pub fn ray_color(r: &Ray, world: &dyn HitTable, depth: i64) -> Color {
    let mut rec = HitRecord::new();
    if depth <= 0 {
        return Color::zero();
    }
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return ray_color(
            &Ray {
                orig: rec.p,
                dir: target - rec.p,
            },
            world,
            depth - 1,
        ) * 0.5;
    }
    let unit_dir = r.dir.unit();
    let t = (unit_dir.y + 1.0) * 0.5;
    Color::ones() * (1.0 - t)
        + Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        } * t
}

pub fn write_color(
    img: &mut RgbImage,
    x: u32,
    y: u32,
    pixel_color: &Color,
    samples_per_pixel: i64,
) {
    let pixel = img.get_pixel_mut(x, y);
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();
    *pixel = Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ]);
}
