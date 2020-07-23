use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_len = 1.0;
        Self {
            origin: Point3::zero(),
            horizontal: Vec3 {
                x: viewport_width,
                y: 0.0,
                z: 0.0,
            },
            vertical: Vec3 {
                x: 0.0,
                y: viewport_height,
                z: 0.0,
            },
            lower_left_corner: Point3::zero()
                - Vec3 {
                    x: viewport_width / 2.0,
                    y: 0.0,
                    z: 0.0,
                }
                - Vec3 {
                    x: 0.0,
                    y: viewport_height / 2.0,
                    z: 0.0,
                }
                - Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: focal_len,
                },
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new()
    }
}
