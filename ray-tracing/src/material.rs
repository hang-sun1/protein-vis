use super::ray::Ray;
use super::hittable::HitRecord;
use super::vector3::Vector3;
use super::texture::{Texture, SolidColor};

use rand::prelude::*;
use std::sync::Arc;

pub trait Material {
    // returns (attenuation, scatter)
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vector3, Ray)>;

    fn emitted(&self, u: f64, v: f64, p: Vector3) -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    } 
}

pub struct Lambertian {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vector3, Ray)> {
        let dir = record.normal_vector + Vector3::random_unit_vector();
        let ray = Ray::new(record.hit_location, dir, ray.time());
        
        Some((self.albedo.value(record.u, record.v, record.hit_location), ray))
    }
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture + Send + Sync>) -> Lambertian {
        Lambertian {
            albedo,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Vector3,
    fuzziness: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vector3, Ray)> {
        let dir = ray.direction() - 2.0 * (ray.direction() * record.normal_vector) * record.normal_vector;
        let r = Ray::new(record.hit_location, dir + self.fuzziness * Vector3::random_in_unit_sphere(), ray.time());
        Some((self.albedo, r)) 
    }
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Metal {
        let mut fuzziness = fuzz;
        if fuzz < 0.0{
            fuzziness = 0.0;
        } else if fuzz > 0.0 {
            fuzziness = 1.0;
        }

        Metal {
            albedo, 
            fuzziness, 
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(index: f64) -> Dielectric {
        Dielectric {
            refractive_index: index, 
        }
    }
    
    fn schlick(&self, cosine: f64) -> f64 {
        let r0 = ((1.0 - self.refractive_index) / (1.0 + self.refractive_index)).powi(2);
        r0 + (1.0-r0)*(1.0-cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vector3, Ray)> {
        let dir = ray.direction().normalize();
        let mut eta_over_eta = self.refractive_index;
        let normal_vector = record.normal_vector.normalize();
        
        if record.front_face {
            eta_over_eta = 1.0 / self.refractive_index;
        }

        let cos_theta = ((-1.0 * dir) * normal_vector).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let prob = self.schlick(cos_theta);
        let mut gen = thread_rng();


        if eta_over_eta * sin_theta > 1.0 {
            let dir = ray.direction() - 2.0 * (dir * normal_vector) * normal_vector;
            let r = Ray::new(record.hit_location, dir, ray.time());
            return Some((Vector3::new(0.7, 0.7, 0.7), r));
        } else if gen.gen::<f64>() < prob {
            let dir = ray.direction() - 2.0 * (dir * normal_vector) * normal_vector;
            let r = Ray::new(record.hit_location, dir, ray.time());
            return Some((Vector3::new(0.7, 0.7, 0.7), r));
        } else {
            let cosine = (-1.0 * dir) * normal_vector;
            let parallel_part = eta_over_eta * (dir + cosine * normal_vector);
            let orthogonal_part = -1.0 * (1.0 - parallel_part.length_squared()).sqrt() *  normal_vector;

            let combined = parallel_part + orthogonal_part;
            Some((Vector3::new(1.0, 1.0, 1.0), Ray::new(record.hit_location, combined, ray.time())))
        }
    }
}

pub struct DiffuseLight {
    emitted: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn new(emitted: Arc<dyn Texture + Send + Sync>) -> DiffuseLight {
        DiffuseLight {
            emitted,
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: Vector3) -> Vector3 {
        self.emitted.value(u, v, p)
    }

    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vector3, Ray)> {
        None
    }
}