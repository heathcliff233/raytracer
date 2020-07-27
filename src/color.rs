use crate::{
    hittable::{HitRecord, HitTable},
    material::Lambertian,
    ray::Ray,
    rtweekend::clamp,
    texture::ConstTexture,
    vec3::{Color, Point3, Vec3},
};
use image::{Rgb, RgbImage};
use std::{f64::INFINITY, sync::Arc};

pub fn ray_color(r: &Ray, background: &Color, world: &dyn HitTable, depth: i64) -> Color {
    let mut rec = HitRecord::new(Arc::new(Lambertian {
        albedo: Arc::new(ConstTexture {
            color_value: Color::zero(),
        }),
    }));
    if depth <= 0 {
        return Color::zero();
    }
    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return *background;
    }
    let mut scattered = Ray {
        orig: Point3::zero(),
        dir: Vec3::zero(),
    };
    let mut attenuation = Color::zero();
    let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
    if !rec
        .mat_ptr
        .scatter(r, &rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }
    emitted
        + Vec3::elemul(
            attenuation,
            ray_color(&scattered, background, world, depth - 1),
        )
}

pub fn write_color(
    img: &mut RgbImage,
    pixel_x: u32,
    pixel_y: u32,
    pixel_color: &Color,
    samples_per_pixel: i64,
) {
    let pixel = img.get_pixel_mut(pixel_x, pixel_y);
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
