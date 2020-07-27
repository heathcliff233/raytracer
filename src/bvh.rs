use crate::{
    aabb::AABB,
    hittable::{HitRecord, HitTable},
    ray::Ray,
    rtweekend::random_int,
    vec3::Point3,
};
use std::{cmp::Ordering, sync::Arc};

pub struct BVHNode {
    left: Arc<dyn HitTable>,
    right: Arc<dyn HitTable>,
    bvhbox: AABB,
}

impl BVHNode {
    pub fn new(
        objects: &mut Vec<Arc<dyn HitTable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = random_int(0, 3);
        let mut tmp: BVHNode;
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };
        let object_span = end - start;
        if object_span == 1 {
            tmp = BVHNode {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bvhbox: AABB::new(Point3::zero(), Point3::zero()),
            };
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                tmp = BVHNode {
                    left: objects[start].clone(),
                    right: objects[start + 1].clone(),
                    bvhbox: AABB::new(Point3::zero(), Point3::zero()),
                };
            } else {
                tmp = BVHNode {
                    left: objects[start + 1].clone(),
                    right: objects[start].clone(),
                    bvhbox: AABB::new(Point3::zero(), Point3::zero()),
                };
            }
        } else {
            objects.as_mut_slice()[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            tmp = BVHNode {
                left: Arc::new(BVHNode::new(objects, start, mid, time0, time1)),
                right: Arc::new(BVHNode::new(objects, mid, end, time0, time1)),
                bvhbox: AABB::new(Point3::zero(), Point3::zero()),
            };
        }
        let mut box_left = AABB::new(Point3::zero(), Point3::zero());
        let mut box_right = AABB::new(Point3::zero(), Point3::zero());
        if !tmp.left.bounding_box(time0, time1, &mut box_left)
            || !tmp.left.bounding_box(time0, time1, &mut box_right)
        {
            println!("No bounding box in bvh_node constructor.\n");
        }
        tmp.bvhbox = AABB::surrounding_box(&box_left, &box_right);
        tmp
    }
}

pub fn box_x_compare(a: &Arc<dyn HitTable>, b: &Arc<dyn HitTable>) -> Ordering {
    let mut box_a = AABB::new(Point3::zero(), Point3::zero());
    let mut box_b = AABB::new(Point3::zero(), Point3::zero());
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        println!("No bounding box in bvh_node constructor.\n");
    }
    if box_a._min.x < box_b._min.x {
        return Ordering::Less;
    } else if box_a._min.x > box_b._min.x {
        return Ordering::Greater;
    }
    Ordering::Equal
}

pub fn box_y_compare(a: &Arc<dyn HitTable>, b: &Arc<dyn HitTable>) -> Ordering {
    let mut box_a = AABB::new(Point3::zero(), Point3::zero());
    let mut box_b = AABB::new(Point3::zero(), Point3::zero());
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        println!("No bounding box in bvh_node constructor.\n");
    }
    if box_a._min.y < box_b._min.y {
        return Ordering::Less;
    } else if box_a._min.y > box_b._min.y {
        return Ordering::Greater;
    }
    Ordering::Equal
}

pub fn box_z_compare(a: &Arc<dyn HitTable>, b: &Arc<dyn HitTable>) -> Ordering {
    let mut box_a = AABB::new(Point3::zero(), Point3::zero());
    let mut box_b = AABB::new(Point3::zero(), Point3::zero());
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        println!("No bounding box in bvh_node constructor.\n");
    }
    if box_a._min.z < box_b._min.z {
        return Ordering::Less;
    } else if box_a._min.z > box_b._min.z {
        return Ordering::Greater;
    }
    Ordering::Equal
}

impl HitTable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bvhbox.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);
        hit_left || hit_right
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bvhbox.clone();
        true
    }
}
