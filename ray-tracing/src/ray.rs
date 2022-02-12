use super::vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
    time: f64
}

impl Ray {
    pub fn at(&self, t: f64) -> Vector3 {
        &self.origin + (t * &self.direction)
    }

    pub fn new(origin: Vector3, direction: Vector3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}
