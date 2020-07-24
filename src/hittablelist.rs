use crate::{
    hittable::{HitRecord, HitTable},
    ray::Ray,
};
use std::{sync::Arc, vec};

pub struct HitTableList {
    pub objects: vec::Vec<Arc<dyn HitTable>>,
}

impl HitTableList {
    pub fn new() -> Self {
        Self {
            objects: vec::Vec::new(),
        }
    }

    /*pub fn clear(&mut self) {
        self.objects.clear();
    }*/

    pub fn add(&mut self, object: Arc<dyn HitTable>) {
        self.objects.push(object);
    }
}

impl Default for HitTableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HitTable for HitTableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = rec.clone();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }
        hit_anything
    }
}
