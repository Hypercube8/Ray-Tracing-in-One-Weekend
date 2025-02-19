use crate::vec3::{Vec3, Point3};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3 
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let ray = Ray::new(Point3::zeroes(), Vec3::ones());
        assert_eq!(ray.at(2.0), Vec3::new(2.0, 2.0, 2.0));
    }
}