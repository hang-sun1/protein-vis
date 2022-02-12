use super::vector3::Vector3;
use super::ray::Ray;
use rand::prelude::*;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f64,
    t0: f64,
    t1: f64,
}

impl Camera {
    pub fn new(look_from: Vector3, look_at: Vector3, vup: Vector3, fov: f64, focus_dist: f64, aperature: f64, t0: f64, t1: f64) -> Camera {
        let fov_radians = fov * 0.01745329251; 
        let h = (fov_radians / 2.0).tan();
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;


        let w = (look_from - look_at).normalize();
        let u = (vup | w).normalize();
        let v = w | u;
        
        let origin = look_from;
        let horizontal = viewport_width * u * focus_dist;
        let vertical = viewport_height * v * focus_dist;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w*focus_dist;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperature / 2.0,
            t0,
            t1,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let mut x;
        let mut y;
        let mut rng = thread_rng();
        loop {
            x = rng.gen_range(-1.0, 1.0);
            y = rng.gen_range(-1.0, 1.0);
            if x * x + y * y < 1.0 {
                break;
            }
        }
        x *= self.lens_radius;
        y *= self.lens_radius;
        let offset = self.u * x + self.v * y;
        Ray::new(self.origin + offset, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin - offset
            , rng.gen_range(self.t0, self.t1))
    }
}
