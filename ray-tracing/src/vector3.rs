use std::ops;
use rand::prelude::*;

// THE BITWISE OR IS THE CROSS PRODUCT

impl_op_ex!(+ |a: &Vector3, b: &Vector3| -> Vector3 {Vector3::new(a.x + b.x, a.y+b.y, a.z + b.z)});
impl_op_ex!(- |a: &Vector3, b: &Vector3| -> Vector3 {Vector3::new(a.x - b.x, a.y-b.y, a.z - b.z)});
impl_op_ex_commutative!(* |a: &Vector3, b: f64| -> Vector3 {Vector3::new(a.x * b, a.y * b, a.z * b)});
impl_op_ex!(* |a: &Vector3, b: &Vector3| -> f64 {a.x * b.x + a.y * b.y + a.z * b.z});
impl_op_ex!(^ |a: &Vector3, b: &Vector3| -> Vector3 {Vector3::new(a.x * b.x, a.y * b.y, a.z * b.z)});
impl_op_ex!(| |a: &Vector3, b: &Vector3| -> Vector3 {
    Vector3 {
        x: a.y * b.z - b.y * a.z,
        y: a.z * b.x - b.z * a.x,
        z: a.x * b.y - b.x * a.y,
    }
});

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn x(&self) -> f64 {
        self.x
    } 
    
    pub fn y(&self) -> f64 {
        self.y
    } 
    
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            x,
            y,
            z,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        self * (1.0/self.length())
    }
    
    pub fn random_in_unit_sphere() -> Vector3 {
        let mut rng = rand::thread_rng();
        loop {
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();
            let z: f64 = rng.gen();
            let v = Vector3::new(x, y, z);
            if v.length_squared() < 1.0 {
                return v
            }
        }
    }

    pub fn random_unit_vector() -> Vector3 {
        let mut rng = rand::thread_rng();
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let a: f64 = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let r = (1.0 - z * z).sqrt();
        Vector3::new(r * a.cos(), r * a.sin(), z)
    }
}
