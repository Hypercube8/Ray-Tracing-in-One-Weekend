use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::color::Color;
use crate::vec3::Vec3;
use crate::utils::random_double;

pub trait Material: Send + Sync {
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
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        assert!(fuzz <= 1.0, "Fuzz factor cannot be greater than one");
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, rin: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = Vec3::reflect(rin.direction, rec.normal);
        reflected = reflected.unit() + self.fuzz * Vec3::random_unit();
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction, rec.normal) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0.powi(2);
        r0 + (1.0-r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, rin: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::ones();
        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        
        let unit_direction = rin.direction.unit();
        let cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta.powi(2));

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, ri)
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}