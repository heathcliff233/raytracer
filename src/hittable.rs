use crate::ray::Ray;
use crate::{
    aabb::AABB,
    material::Material,
    vec3::{Point3, Vec3},
};
use std::{f64::consts::PI, sync::Arc};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(m: Arc<dyn Material>) -> Self {
        Self {
            p: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3::zero(),
            mat_ptr: m,
            t: 0.0,
            u: 0.0,
            v: 0.0,
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

pub trait HitTable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool;
    fn distance(&self, other_center: &Point3) -> f64;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center: c,
            radius: r,
            mat_ptr: m,
        }
    }
    pub fn get_sphere_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        *u = 1.0 - (phi + PI) / (2.0 * PI);
        *v = (theta + PI / 2.0) / PI;
    }
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
                Sphere::get_sphere_uv(
                    &((rec.p - self.center) / self.radius),
                    &mut rec.u,
                    &mut rec.v,
                );
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            let tmp = (-b + root) / a;
            if tmp < t_max && tmp > t_min {
                rec.t = tmp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                Sphere::get_sphere_uv(
                    &((rec.p - self.center) / self.radius),
                    &mut rec.u,
                    &mut rec.v,
                );
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        false
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
    fn distance(&self, other_center: &Point3) -> f64 {
        other_center.distance(self.center) - self.radius
    }
}
