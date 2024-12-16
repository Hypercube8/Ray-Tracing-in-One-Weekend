use crate::ray::Ray;
use crate::color::{Color, write_color};
use crate::hittable::{Hittable, HitRecord};
use crate::vec3::{Vec3, Point3};
use std::f64::INFINITY;
use std::io::Write;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_height: usize,
    image_width: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_height: usize) -> Camera {
        if aspect_ratio <= 0.0 { panic!("Aspect ratio must be positive") };
        if image_height <= 0 { panic!("Image height must be greater than zero") };
        Camera {
            aspect_ratio,
            image_height,
            image_width: 0,
            center: Vec3::zeroes(),
            pixel00_loc: Point3::zeroes(),
            pixel_delta_u: Vec3::zeroes(),
            pixel_delta_v: Vec3::zeroes()
        }
    } 

    fn init(&mut self) {
        self.image_width = (self.image_height as f64 * self.aspect_ratio) as usize;
        self.image_width = if self.image_width > 0 { self.image_width } else { 1 };

        self.center = Point3::zeroes();
        
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

    }

    pub fn render(&mut self, stream: &mut dyn Write, world: &impl Hittable) {
        self.init();

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height-j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(&r, world);
                write_color(stream, pixel_color);
            }
        }

        eprintln!("Done.");
    }

    fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
        let rec = world.hit(r, (0.0, INFINITY));
        if let Some(hit) = rec {
            return 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = r.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0-a) * Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0);
    }
}