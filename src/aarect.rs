use crate::{
    aabb::AABB,
    hittable::HitTable,
    material::Material,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

pub struct XYRect {
    pub mp: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl XYRect {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
        }
    }
}

impl HitTable for XYRect {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t0: f64,
        t1: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t0 || t > t1 {
            return false;
        }
        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 0.1);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut crate::aabb::AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}
