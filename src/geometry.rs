use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>
}

impl Sphere { 
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        if radius < 0.0 { panic!("Sphere cannot have negative radius") };
        Sphere { center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, bounds: (f64, f64)) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = Vec3::dot(r.direction, oc);
        let c = oc.length_squared() - self.radius.powi(2);
        
        let discriminant = h.powi(2) - a*c;
        if discriminant < 0.0 {
            return None;
        } 

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= bounds.0 || bounds.1 <= root {
            root = (h + sqrtd) / a;
            if root <= bounds.0 || bounds.1 <= root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius; 

        Some(HitRecord {
            p,
            normal: outward_normal,
            t: root,
            front_face: true,
            mat: self.mat.clone()
        }.set_face_normal(r, &outward_normal))
    }
}

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>

}

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    } 

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for Scene {
    fn hit(&self, r: &Ray, bounds: (f64, f64)) -> Option<HitRecord> {
        let mut closest = bounds.1;
        let mut rec: Option<HitRecord> = None;

        for object in &self.objects {
            let hit = object.hit(r, (bounds.0, closest));
            if let Some(t) = hit {
                closest = t.t;
                rec = Some(t);
            } 
        }
        rec
    } 
}