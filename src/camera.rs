use crate::ray::Ray;
use crate::{
    rtweekend::degrees_to_radians,
    vec3::{random_in_unit_disk, Point3, Vec3},
};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let ww = (lookfrom - lookat).unit();
        let uu = vup.cross(ww).unit();
        let vv = ww.cross(uu);
        Self {
            origin: lookfrom,
            horizontal: uu * viewport_width * focus_dist,
            vertical: vv * viewport_height * focus_dist,
            lower_left_corner: lookfrom
                - uu / 2.0 * viewport_width * focus_dist
                - vv / 2.0 * viewport_height * focus_dist
                - ww * focus_dist,
            u: uu,
            v: vv,
            w: ww,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
        }
    }
}
