use std::fmt;
use assert_float_eq::assert_float_absolute_eq;
use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};
use crate::utils::{random_double, random_range}; 

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Point3 = Vec3;

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub const fn zeroes() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub const fn ones() -> Vec3 {
        Vec3 { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(random_range(min, max), random_range(min, max), random_range(min, max))
    }

    pub fn length_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn random_unit() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit();
        if Vec3::dot(on_unit_sphere, *normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(-uv, n), 1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
        r_out_perp + r_out_parallel
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x 
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x, 
            1 => &self.y,
            2 => &self.z,
            i => panic!("Index {} is out of bounds", i)
        }
    }
} 

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            i => panic!("Mutable index {} is out of bounds", i)
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        (1.0 / other) * self
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        };
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0/other;
    }
}

macro_rules! assert_vec3_eq {
    ($x:expr, $y:expr) => {
        assert_float_absolute_eq!($x.x, $y.x);
        assert_float_absolute_eq!($x.y, $y.y);
        assert_float_absolute_eq!($x.z, $y.z);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length() {
        let vec3 = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(vec3.length_squared(), 25.0);
        assert_eq!(vec3.length(), 5.0);
        assert_vec3_eq!(vec3.unit(), Vec3::new(0.6, 0.8, 0.0));
    }

    #[test]
    fn dot() {
        let vec3 = Vec3::new(3.0, 4.0, 0.0);
        let factor = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(Vec3::dot(vec3, factor), 14.0);
    }

    #[test]
    fn cross() {
        let vec3 = Vec3::new(3.0, 4.0, 0.0);
        let factor = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(Vec3::cross(vec3, factor), Vec3::new(8.0, -6.0, -2.0));
    }

    #[test]
    fn index() {
        let mut vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec3[0], 1.0);
        assert_eq!(vec3[1], 2.0);
        assert_eq!(vec3[2], 3.0);
        vec3[0] += 1.0;
        vec3[1] += 1.0;
        vec3[2] += 1.0;
        assert_eq!(vec3, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    #[should_panic(expected = "Index 3 is out of bounds")]
    fn index_out_of_bounds() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let _ = vec3[3];
    }

    #[test]
    #[should_panic(expected = "Mutable index 3 is out of bounds")]
    fn index_mut_out_of_bounds() {
        let mut vec3 = Vec3::zeroes();
        vec3[3] = 1.0;
    }

    #[test]
    fn neg() {
        let vec3 = Vec3::new(1.0, 0.0, -1.0);
        assert_eq!(-vec3, Vec3::new(-1.0, 0.0, 1.0));
    }

    #[test]
    fn add() {
        let mut vec3 = Vec3::new(1.0, 0.0, -1.0);
        let addend = Vec3::new(2.0, 2.0, 2.0);
        vec3 += Vec3::new(-1.0, 0.0, 1.0);
        assert_eq!(vec3, Vec3::zeroes());
        assert_eq!(vec3 + addend, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn sub() {
        let mut vec3 = Vec3::new(1.0, 0.0, -1.0);
        let subtrahend = Vec3::new(2.0, 2.0, 2.0);
        vec3 -= Vec3::new(1.0, 0.0, -1.0);
        assert_eq!(vec3, Vec3::zeroes());
        assert_eq!(vec3 - subtrahend, Vec3::new(-2.0, -2.0, -2.0));
    }

    #[test]
    fn mul() {
        let mut vec3 = Vec3::new(1.0, 0.0, -1.0);
        let factor = Vec3::new(2.0, 2.0, 2.0);
        vec3 *= 3.0;
        assert_eq!(vec3, Vec3::new(3.0, 0.0, -3.0));
        assert_eq!(vec3 * factor, Vec3::new(6.0, 0.0, -6.0));
        assert_eq!(vec3 * 3.0, Vec3::new(9.0, 0.0, -9.0));
        assert_eq!(3.0 * vec3, Vec3::new(9.0, 0.0, -9.0));
    }

    #[test]
    fn div() {
        let mut vec3 = Vec3::new(9.0, 0.0, -9.0);
        vec3 /= 3.0;
        assert_eq!(vec3, Vec3::new(3.0, 0.0, -3.0));
        assert_eq!(vec3 / 3.0, Vec3::new(1.0, 0.0, -1.0));
    }
}