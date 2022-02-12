use std::sync::Arc;

use super::vector3::Vector3;
use image::RgbImage;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vector3) -> Vector3;
}

pub struct SolidColor {
    color: Vector3,
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color: Vector3::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vector3) -> Vector3 {
        self.color
    }
}

pub struct Checkered {
    a: Arc<dyn Texture + Send + Sync>,
    b: Arc<dyn Texture + Send + Sync>, 
}

impl Checkered {
    pub fn new(a: Arc<dyn Texture + Send + Sync>, b: Arc<dyn Texture + Send + Sync>) -> Checkered {
        Checkered {
            a,
            b,
        }
    }
}

impl Texture for Checkered {
    fn value(&self, u: f64, v: f64, p: Vector3) -> Vector3 {
        if (10.0*p.x()).sin() * (10.0*p.y()).sin() * (10.0*p.z()).sin() < 0.0 {
            return self.b.value(u, v, p)
        }
        return self.a.value(u, v, p)
    }
}

pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(image: RgbImage) -> ImageTexture {
        ImageTexture {
            image,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Vector3) -> Vector3 {
        let x = (u.clamp(0.0, 1.0) * self.image.width() as f64).clamp(0.0, self.image.width() as f64 - 1.0) ;
        let y = ((1.0 - v.clamp(0.0, 1.0)) * self.image.height() as f64).clamp(0.0, self.image.height() as f64 - 1.0);
        let pixel = self.image.get_pixel(x as u32, y as u32);
        let r = pixel[0] as f64 / 255.0;
        let g = pixel[1] as f64 / 255.0;
        let b = pixel[2] as f64 / 255.0;

        Vector3::new(r, g, b)
    }
}