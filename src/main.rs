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


fn main() {
    let mut world = Scene::new();

    let material_ground = Rc::new(
        Lambertian::new(
            Color::new(0.8, 0.8, 0.0)
        )
    );
    let material_center = Rc::new(
        Lambertian::new(
            Color::new(0.1, 0.2, 0.5)
        )
    );
    let material_left = Rc::new(
        Dielectric::new(
            1.50
        )
    );
    let material_bubble = Rc::new(
        Dielectric::new(
            1.00 / 1.50
        )
    );
    let material_right = Rc::new(
        Metal::new(
            Color::new(0.8, 0.6, 0.2),
            1.0
        )
    );

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right
    )));

    let mut cam = Camera::new(16.0 / 9.0, 400, 100, 50);
    
    let stdout = stdout().lock();
    let mut handle = BufWriter::new(stdout);
    
    cam.render(&mut handle, &world);
}