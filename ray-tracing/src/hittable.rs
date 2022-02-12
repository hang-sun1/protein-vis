pub(crate) use super::ray::Ray;
use super::vector3::Vector3;
use super::material::Material;
use super::bvh::AABB;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub u: f64,
    pub v: f64,
    pub hit_location: Vector3,
    pub normal_vector: Vector3,
    pub time: f64,
    pub front_face: bool, 
    pub material: Arc<dyn Material + Send + Sync>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self) -> Option<AABB> {
        None
    }
}

pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
    
    fn get_sphere_uv(hit_location: Vector3) -> (f64, f64) {
        let phi = hit_location.z().atan2(hit_location.x());
        let theta = hit_location.y().asin();
        let u = 1.0 - (phi + std::f64::consts::PI) / (std::f64::consts::PI * 2.0);
        let v = (theta + std::f64::consts::PI / 2.0) / std::f64::consts::PI;
        (u, v) 
    }

    // pub fn center(&self) -> Vector3 {
    //     self.center
    // }

    // pub fn radius(&self) -> f64 {
    //     self.radius
    // }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // let start = ray.origin();
        // let dir = ray.direction();

        // let a = dir * dir;
        // let b = 2.0 * (dir * (start - self.center));
        // let c = (start - self.center) * (start - self.center) - self.radius * self.radius;


        // let discriminant = b * b - 4.0 * a * c;

        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc * ray.direction();
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant >= 0.0 {
            // let t_low = (-b - discriminant.sqrt()) / (2.0 * a);
            let t_low = (-half_b - discriminant.sqrt()) / (a);
            if t_low >= t_min && t_low <= t_max {
                let hit_location = ray.at(t_low);
                let outward_normal_vector = (ray.at(t_low) - self.center) * (1.0 / self.radius);
                let time = t_low; 
               
                let (u, v) = Sphere::get_sphere_uv((hit_location - self.center) * (1.0 / self.radius));
                
                if ray.direction() * outward_normal_vector > 0.0 {
                    let normal_vector = -1.0 * outward_normal_vector;
                    let front_face = false;
                    return Some(HitRecord {
                        u,
                        v,
                        hit_location,
                        normal_vector,
                        time,
                        front_face, 
                        material: self.material.clone(),
                    })
                } else {
                    return Some(HitRecord {
                        u,
                        v,
                        hit_location,
                        normal_vector: outward_normal_vector,
                        time,
                        front_face: true,
                        material: self.material.clone(),
                    })
                }
            }
            
            // let t_high = (-b + discriminant.sqrt()) / (2.0 * a);
            let t_high = (-half_b + discriminant.sqrt()) / (a);
            if t_high >= t_min && t_high <= t_max {
                let hit_location = ray.at(t_high);
                let outward_normal_vector = (ray.at(t_high) - self.center) * (1.0 / self.radius);
                let time = t_high; 
                let (u, v) = Sphere::get_sphere_uv((hit_location - self.center) * (1.0 / self.radius));
                if ray.direction() * outward_normal_vector > 0.0 {
                    let normal_vector = -1.0 * outward_normal_vector;
                    let front_face = false;
                    return Some(HitRecord {
                        u,
                        v,
                        hit_location,
                        normal_vector,
                        time,
                        front_face,
                        material: self.material.clone(),
                    })
                } else {
                    return Some(HitRecord {
                        u,
                        v,
                        hit_location,
                        normal_vector: outward_normal_vector,
                        time,
                        front_face: true,
                        material: self.material.clone(),
                    })
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        let r = Vector3::new(self.radius, self.radius, self.radius);
        let aabb = AABB::new(self.center - r, self.center + r);
        Some(aabb)
    }
}


pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects:Vec::new(),
        }
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(hittable); 
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects.iter()
            .filter_map(|o| {o.hit(ray, t_min, t_max)})
            .min_by(|x, y| {x.time.partial_cmp(&y.time).expect("NaN")})
    }

    fn bounding_box(&self) -> Option<AABB> {
        if self.objects.is_empty() {
            return None
        }
        self.objects.iter()
            .filter_map(|o| o.bounding_box())
            .reduce(|acc, o| AABB::surrounding_box(acc, o))
    }
}

pub struct MovingSphere {
    center0: Vector3,
    center1: Vector3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl MovingSphere {
    pub fn new(center0: Vector3, center1: Vector3, time0: f64, time1: f64, radius: f64, material: Arc<dyn Material + Send + Sync>) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Vector3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().length_squared();
        let half_b = oc * ray.direction();
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant >= 0.0 {
            // let t_low = (-b - discriminant.sqrt()) / (2.0 * a);
            let t_low = (-half_b - discriminant.sqrt()) / (a);
            if t_low >= t_min && t_low <= t_max {
                let hit_location = ray.at(t_low);
                let outward_normal_vector = (ray.at(t_low) - self.center(ray.time())) * (1.0 / self.radius);
                let time = t_low; 
                let (u, v) = Sphere::get_sphere_uv((hit_location - self.center(ray.time())) * (1.0 / self.radius));
                if ray.direction() * outward_normal_vector > 0.0 {
                    let normal_vector = -1.0 * outward_normal_vector;
                    let front_face = false;
                    return Some(HitRecord {
                        u,
                        v,
                        hit_location,
                        normal_vector,
                        time,
                        front_face, 
                        material: self.material.clone(),
                    })
                } else {
                    return Some(HitRecord {
                        u,
                        v,
                        hit_location,
                        normal_vector: outward_normal_vector,
                        time,
                        front_face: true,
                        material: self.material.clone(),
                    })
                }
            }
            
            // let t_high = (-b + discriminant.sqrt()) / (2.0 * a);
            let t_high = (-half_b + discriminant.sqrt()) / (a);
            if t_high >= t_min && t_high <= t_max {
                let hit_location = ray.at(t_high);
                let outward_normal_vector = (ray.at(t_high) - self.center(ray.time())) * (1.0 / self.radius);
                let time = t_high; 
                let (u, v) = Sphere::get_sphere_uv((hit_location - self.center(ray.time())) * (1.0 / self.radius));
                if ray.direction() * outward_normal_vector > 0.0 {
                    let normal_vector = -1.0 * outward_normal_vector;
                    let front_face = false;
                    return Some(HitRecord {
                        u,
                        v,
                        hit_location,
                        normal_vector,
                        time,
                        front_face,
                        material: self.material.clone(),
                    })
                } else {
                    return Some(HitRecord {
                        u,
                        v,
                        hit_location,
                        normal_vector: outward_normal_vector,
                        time,
                        front_face: true,
                        material: self.material.clone(),
                    })
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        // Note that unwrap here is okay, sphere's always have a bounding box
        let start_sphere_box = Sphere::new(self.center0, self.radius, self.material.clone()).bounding_box().unwrap();
        let end_sphere_box = Sphere::new(self.center1, self.radius, self.material.clone()).bounding_box().unwrap();
        
        Some(AABB::surrounding_box(start_sphere_box, end_sphere_box))
    }
}

