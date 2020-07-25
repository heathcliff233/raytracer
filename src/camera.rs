use crate::ray::Ray;
use crate::{
    rtweekend::degrees_to_radians,
    vec3::{Point3, Vec3},
};

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        Self {
            origin: lookfrom,
            horizontal: u * viewport_width,
            vertical: v * viewport_height,
            lower_left_corner: lookfrom - u / 2.0 * viewport_width - v / 2.0 * viewport_height - w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin,
        }
    }
}
