#![allow(dead_code, unused_variables, unused_imports)]
#[macro_use] extern crate impl_ops;

use hsl::HSL;
use image::{RgbImage, Rgb};
use std::path::Path;
use indicatif::ProgressBar;

use vector3::Vector3;
use ray::Ray;
use hittable::*;

use hittable::HittableList;
use rand::prelude::*;
use camera::Camera;
use std::sync::Arc;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::convert::TryInto;

mod vector3;
mod ray;
mod hittable; 
mod camera;
mod material;
mod bvh;
mod texture;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let _width: u32 = 384;
    let width: u32 = 1920;
    let height = (width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 40;
    
    let mut world = HittableList::new();
    let solid_color: Arc<dyn texture::Texture + Send + Sync> = Arc::new(texture::SolidColor::new(0.8, 0.8, 0.0));
    let solid_color2: Arc<dyn texture::Texture + Send + Sync> = Arc::new(texture::SolidColor::new(100.0, 0.0, 0.0));
    let checkered: Arc<dyn texture::Texture + Send + Sync> = Arc::new(texture::Checkered::new(Arc::clone(&solid_color), Arc::clone(&solid_color2)));
    let lam = material::Lambertian::new(solid_color2.clone());
    let lam2 = material::DiffuseLight::new(Arc::clone(&solid_color2));
    let metal = material::Metal::new(Vector3::new(1.0, 0.0, 0.0), 0.4);
    let dielectric = material::Dielectric::new(1.5);
    let material_metal = Arc::new(metal);
    let material_lam = Arc::new(lam);
    let material_lam2 = Arc::new(lam2);
    let material_dielectric = Arc::new(dielectric);

    let num_objects = std::env::args().skip(1).count();

    for (i, coords) in std::env::args().skip(1).enumerate() {
        let coords_iter = coords.split(",")
            .map(|s| {
                s.parse::<f64>().expect("could not parse coordinate as f64")
            })
            .collect::<Vec<_>>();
            
        let x = coords_iter[0] / 5.0;
        let y = coords_iter[1] / 5.0;
        let z = coords_iter[2] / 5.0;
        let rainbow_color = get_color_from_rainbow(i, num_objects); 
        let color: Arc<dyn texture::Texture + Send + Sync> = Arc::new(texture::SolidColor::new(rainbow_color.x(), rainbow_color.y(), rainbow_color.z()));
        let material = Arc::new(material::Lambertian::new(color.clone()));

        let sphere: Arc<dyn Hittable + Send + Sync> = 
            Arc::new(Sphere::new(Vector3::new(x, y, z), 0.35, material.clone()));
        
        world.add(Arc::clone(&sphere));
    }

    let light: Arc<dyn Hittable+Send+Sync> =
        Arc::new(Sphere::new(Vector3::new(40.0, 40.0, 100.0), 40.0, material_lam2.clone()));
    world.add(light);

    eprintln!("Beginning Render!");


    let world_lock = Arc::new(world);
    let look_from = Vector3::new(0.0, 0.0, 30.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(look_from, look_at, vup, 20.0, (look_from - look_at).length(), 0.0, 0.0, 1.0);
    let camera_lock = Arc::new(camera);
    
    let mut img = RgbImage::new(width, height);
    let pb = ProgressBar::new(height as u64);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let clone_of_world = world_lock.clone();
        let clone_of_camera = camera_lock.clone();
        let clone_of_sender = tx.clone();
        pool.execute(move || {
            let mut rng = rand::thread_rng();
            let mut pixels = Vec::new();
            for x in 0..width {
                let color: Vector3 = (0..samples_per_pixel).map(|_| {
                    let r1: f64 = rng.gen();
                    let r2: f64 = rng.gen();
                    let u = (x as f64 + r1) / ((width -1) as f64);
                    let v = (y as f64 + r2) / ((height - 1) as f64);
                    let r = clone_of_camera.get_ray(u, v);
                    ray_color(&r, &*clone_of_world, 140)
                })
                .fold(Vector3::new(0.0, 0.0, 0.0), |acc, x| acc + x) * (1.0/(samples_per_pixel as f64));
                let ir = (256.0 * clamp(color.x().sqrt(), 0.0, 0.999)) as u8;
                let ig = (256.0 * clamp(color.y().sqrt(), 0.0, 0.999)) as u8;
                let ib = (256.0 * clamp(color.z().sqrt(), 0.0, 0.999)) as u8;
                pixels.push((x, height-y-1, Rgb([ir, ig, ib])))
            }
            clone_of_sender.send(pixels).unwrap();
        });
    }
    rx.iter().take(height.try_into().unwrap()).for_each(|p| {
        for (x, y, rgb) in p {
            img.put_pixel(x, y, rgb);
        }
        pb.inc(1)
    });
    pb.finish();
    
    eprintln!("Renderer finished, saving to file now");
    
    img.save(Path::new("./test.png")).unwrap();
    eprintln!("Done!");
    
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min
    } 
    if x > max {
        return max
    }
    x
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: usize) -> Vector3 {
    if depth == 0 {
        return Vector3::new(0.0, 0.0, 0.0)
    }
    
    if let Some(record) = world.hit(ray, 0.001, std::f64::INFINITY) {
        let u = record.u;
        let v = record.v;
        let p = record.hit_location;
        let material = &record.material;
        match material.scatter(ray, &record) {
            Some((att, r)) => {
                return record.material.emitted(u, v, p) + (att ^ ray_color(&r, world, depth -1))
            },
            None => {
                return record.material.emitted(u, v, p)
            }
        }
    } else {
        let t = 0.5 * (ray.direction().normalize() + Vector3::new(1.0, 1.0, 1.0));
        return (Vector3::new(1.0, 1.0, 1.0) - t) ^ Vector3::new(1.0, 1.0, 1.0) + t ^ Vector3::new(0.5, 0.7, 1.0);
        // return Vector3::new(0.0, 0.0, 0.0);
    }
}

fn get_color_from_rainbow(n: usize, total: usize) -> Vector3 {
    let hue = 360.0 / (total as f64) * (n as f64);
    let hsl = HSL { h: hue, s: 0.5, l: 0.5 };
    let (r, g, b) = hsl.to_rgb();
    let red = r as f64 / 255.0;
    let green = g as f64 / 255.0;
    let blue = b as f64 / 255.0;

    return Vector3::new(red, green, blue);
}

