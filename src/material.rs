use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::color::Color;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, rin: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;

        true 
    }
}

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, rin: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(rin.direction, rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}