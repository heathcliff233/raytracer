use crate::{
    ray::Ray,
    rtweekend::{fmax, fmin},
    vec3::Point3,
};
use std::mem::swap;

#[derive(Clone)]
pub struct AABB {
    pub _min: Point3,
    pub _max: Point3,
}

impl AABB {
    pub fn new(mi: Point3, ma: Point3) -> Self {
        Self { _min: mi, _max: ma }
    }
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        let small = Point3::new(
            fmin(box0._min.x, box1._min.x),
            fmin(box0._min.y, box1._min.y),
            fmin(box0._min.z, box1._min.z),
        );
        let big = Point3::new(
            fmax(box0._max.x, box1._max.x),
            fmax(box0._max.y, box1._max.y),
            fmax(box0._max.z, box1._max.z),
        );
        Self {
            _min: small,
            _max: big,
        }
    }
    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.dir[a];
            let mut t0 = (self._min[a] - r.orig[a]) * inv_d;
            let mut t1 = (self._max[a] - r.orig[a]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            t0 = if t0 > tmin { t0 } else { tmin };
            t1 = if t1 < tmax { t1 } else { tmax };
            if t1 <= t0 {
                return false;
            }
        }
        true
    }
}
