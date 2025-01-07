mod camera;
mod hittable;
mod geometry;
mod vec3;
mod ray;
mod color;
mod utils;
mod material;

use camera::Camera;
use hittable::Hittable;
use geometry::{Scene, Sphere};
use vec3::{Vec3, Point3};
use color::Color;
use material::{Lambertian, Metal, Dielectric};
use std::io::{stdout, Write, BufWriter};
use std::sync::Arc;
use utils::{random_double, random_range};


fn main() {
    let mut world = Scene::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    mat if mat < 0.8 => {
                        let albedo = Color::random() * Color::random();
                        let sphere_material = Arc::new(Lambertian::new(albedo));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    },
                    mat if mat < 0.95 => {
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = random_range(0.0, 0.5);
                        let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    _ => {
                        let sphere_material = Arc::new(Dielectric::new(1.5));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));
    

    let mut cam = Camera::new(
        16.0 / 9.0, 
        720, 
        10, 
        50, 
        20.0, 
        Point3::new(13.0, 2.0, 3.0), 
        Point3::new(0.0, 0.0, 0.0), 
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0
    );
    
    let stdout = stdout().lock();
    let mut handle = BufWriter::new(stdout);
    
    cam.render(&mut handle, &world);
}