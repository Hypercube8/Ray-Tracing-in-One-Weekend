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
use std::rc::Rc;
use std::f64::consts::PI;


fn main() {
    let mut world = Scene::new();

    let R = f64::cos(PI / 4.0);

    let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Box::new(Sphere::new(Point3::new(-R, 0.0, -1.0), R, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(R, 0.0, -1.0), R, material_right)));

    let mut cam = Camera::new(16.0 / 9.0, 400, 100, 50, 90.0);
    
    let stdout = stdout().lock();
    let mut handle = BufWriter::new(stdout);
    
    cam.render(&mut handle, &world);
}