use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord::new()
    }
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir * (*outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}

pub trait HitTable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl HitTable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dir * r.dir;
        let b = oc * r.dir;
        let c = oc * oc - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let tmp = (-b - root) / a;
            if tmp < t_max && tmp > t_min {
                rec.t = tmp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                return true;
            }
            let tmp = (-b + root) / a;
            if tmp < t_max && tmp > t_min {
                rec.t = tmp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                return true;
            }
        }
        false
    }
}
