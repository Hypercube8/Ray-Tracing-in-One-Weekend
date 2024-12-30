use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::material::Material;
use std::rc::Rc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(self, r: &Ray, outward_normal: &Vec3) -> HitRecord {
        HitRecord {
            front_face: Vec3::dot(r.direction, *outward_normal) < 0.0,
            normal: if self.front_face { *outward_normal } else { -*outward_normal },
            ..self
        } 
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, bounds: (f64, f64)) -> Option<HitRecord>;
}

