mod vec3;
mod color;
mod ray;
mod geometry;
mod hittable;

use ray::Ray;
use vec3::{Vec3, Point3};
use color::{Color, write_color};
use geometry::{HittableList, Sphere};
use hittable::{Hittable, HitRecord};

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let hit = world.hit(r, (0.0, 10000000.0));

    if let Some(t) = hit {
        let n = (r.at(t.t) - Vec3::new(0.0, 0.0, -1.0)).unit();
        return 0.5 * (t.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0-a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const IMAGE_HEIGHT: usize = 225;
    const IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64); 

    const FOCAL_LENGTH: f64 = 1.0;
    const CAMERA_CENTER: Point3 = Point3::zeroes();

    const VIEWPORT_U: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    const VIEWPORT_V: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v = VIEWPORT_V / IMAGE_HEIGHT as f64;

    let viewport_upper_left: Vec3 = CAMERA_CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0;
    let pixel00_loc: Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {}", IMAGE_HEIGHT-j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - CAMERA_CENTER;
            let r = Ray::new(CAMERA_CENTER, ray_direction);

            let pixel_color = ray_color(&r, &world);
            write_color(pixel_color);
        }
    }

    eprintln!("Done.");
}
