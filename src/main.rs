mod camera;
mod hittable;
mod geometry;
mod vec3;
mod ray;
mod color;
mod utils;

use camera::Camera;
use hittable::Hittable;
use geometry::{Scene, Sphere};
use vec3::{Vec3, Point3};
use std::io::{stdout, Write, BufWriter};


fn main() {
    let mut world = Scene::new();

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0
    )));

    let mut cam = Camera::new(16.0 / 9.0, 400, 100);
    
    let stdout = stdout().lock();
    let mut handle = BufWriter::new(stdout);
    
    cam.render(&mut handle, &world);
}