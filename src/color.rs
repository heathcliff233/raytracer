use crate::{
    hittable::{HitRecord, HitTable},
    material::Lambertian,
    ray::Ray,
    rtweekend::clamp,
    vec3::{Color, Vec3},
};
use image::{Rgb, RgbImage};
use std::{f64::INFINITY, sync::Arc};

pub fn ray_color(r: &Ray, world: &dyn HitTable, depth: i64) -> Color {
    let mut rec = HitRecord::new(Arc::new(Lambertian {
        albedo: Color::zero(),
    }));
    if depth <= 0 {
        return Color::zero();
    }
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray {
            orig: Vec3::ones(),
            dir: Vec3::ones(),
        };
        let mut attenuation = Color::zero();
        if rec
            .mat_ptr
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return Vec3::elemul(attenuation, ray_color(&scattered, world, depth - 1));
        }
        return Color::zero();
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
