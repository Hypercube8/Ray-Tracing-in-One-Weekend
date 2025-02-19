use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::material::Material;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(self, r: &Ray, outward_normal: &Vec3) -> HitRecord {
        let front_face = Vec3::dot(r.direction, *outward_normal) < 0.0;
        HitRecord {
            front_face,
            normal: if front_face { *outward_normal } else { -*outward_normal },
            ..self
        } 
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, bounds: (f64, f64)) -> Option<HitRecord>;
}

