use super::vector3::Vector3;
use super::ray::Ray;
use super::hittable::{Hittable, HitRecord};

use std::sync::Arc;
use rand::prelude::*;

pub struct AABB {
    low: Vector3,
    high: Vector3,
}

impl AABB {
    pub fn new(low: Vector3, high: Vector3) -> AABB {
        AABB {
            low,
            high,
        }
    }

    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(f64, f64)> {
        let mut min_t = tmin;
        let mut max_t = tmax;
        for n in 0..3 {
            let x1;
            let x2;
            let origin;
            let dir;
            if n == 0 {
                x1 = self.low.x();
                x2 = self.high.x();
                origin = ray.origin().x();
                dir = ray.direction().x();
            } else if n == 1 {
                x1 = self.low.y();
                x2 = self.high.y();
                origin = ray.origin().y();
                dir = ray.direction().y();
            } else {
                x1 = self.low.z();
                x2 = self.high.z();
                origin = ray.origin().z();
                dir = ray.direction().z();
            }
            let t_hit_low = ((x1 - origin) / dir).min((x2 - origin) / dir);
            let t_hit_high = ((x1 - origin) / dir).max((x2 - origin) / dir);

            min_t = t_hit_low.max(min_t);
            max_t = t_hit_high.min(max_t);
            
            if max_t <= min_t {
                return None
            }
        }
        Some((min_t, max_t))
    }

    pub fn surrounding_box(a: AABB, b: AABB) -> AABB {
        let x_low = a.low.x().min(b.low.x());
        let y_low = a.low.y().min(b.low.y());
        let z_low = a.low.z().min(b.low.z());

        let x_high = a.high.x().max(b.high.x());
        let y_high = a.high.y().max(b.high.y());
        let z_high = a.high.z().max(b.high.z());

        AABB::new(Vector3::new(x_low, y_low, z_low), Vector3::new(x_high, y_high, z_high))
    }
}


pub struct BVHNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(hittables: &mut [Arc<dyn Hittable + Send + Sync>], start: usize, end: usize) -> BVHNode {
        let mut rng = thread_rng();
        let z: u8 = rng.gen_range(0, 3);
        let comparison = |a:&Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>| {
            let c = a.bounding_box().unwrap();
            let d = b.bounding_box().unwrap();
            if z == 0 {
                return c.low.x().partial_cmp(&d.low.x()).unwrap()
            } else if z == 1 {
                return c.low.y().partial_cmp(&d.low.y()).unwrap()
            } else {
                return c.low.z().partial_cmp(&d.low.z()).unwrap()
            }
        };
        let comparison2 = |a: AABB, b: AABB| {
            if z == 0 {
                return a.low.x() < b.low.x()
            } else if z == 1 {
                return a.low.y() < b.low.y()
            } else {
                return a.low.z() < b.low.z()
            }
        };
        
        let span = end - start;
        
        let left;
        let right;
        let bbox;

        if span == 1 {
            left = Arc::clone(&hittables[start]);
            right = Arc::clone(&hittables[start]);
        } else if span == 2 {
            if comparison2(hittables[0].bounding_box().unwrap(), hittables[1].bounding_box().unwrap()) {
                left = Arc::clone(&hittables[start]);
                right = Arc::clone(&hittables[start+1]);
            } else {
                left = Arc::clone(&hittables[start+1]);
                right = Arc::clone(&hittables[start]);
            }
        } else {
            let middle = start + span / 2;
            hittables.sort_by(comparison);
            left = Arc::new(BVHNode::new(hittables, start, middle));
            right = Arc::new(BVHNode::new(hittables, middle, end));
        }

        bbox = AABB::surrounding_box(left.bounding_box().unwrap(), right.bounding_box().unwrap());
        
        BVHNode {
            left,
            right,
            bbox,
        }
    }


}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(_) = self.bbox.hit(ray, t_min, t_max) {
            if let Some(r) = self.left.hit(ray, t_min, t_max) {
                return Some(r)
            }
            if let Some(r) = self.right.hit(ray, t_min, t_max) {
                return Some(r)
            }
        }
        None
    }
}

