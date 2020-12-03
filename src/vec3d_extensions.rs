use crate::vec3::Vec3d;
use crate::util::{random_double, random_range};
use std::f64::EPSILON;
use std::ops::Neg;

pub trait RandomGen<E, T> {
    fn random() -> T;
    fn random_range(min: E, max: E) -> T;
}

impl RandomGen<f64, Vec3d> for Vec3d {
    fn random() -> Vec3d {
        Vec3d::new(
            random_double(),
            random_double(),
            random_double()
        )
    }

    fn random_range(min: f64, max: f64) -> Vec3d {
        Vec3d::new(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max)
        )
    }
}

impl Vec3d {
    pub fn random_in_unit_sphere() -> Vec3d {
        loop {
            let p = Vec3d::random_range(-1.0, 1.0);
            if p.norm_squared() >= 1.0 { continue }
            return p;
        }
    }

    pub fn random_in_hemisphere(normal: &Vec3d) -> Vec3d {
        let in_unit_sphere = Vec3d::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::new(
                random_range(-1.0, 1.0),
                random_range(-1.0, 1.0),
                0.0
            );
            if p.norm_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    pub fn reflect(&self, n: &Self) -> Self {
        *self - 2.0 * self.dot(n) * *n
    }

    pub fn refract(&self, &n: &Self, ratio: f64) -> Self {
        let cos_theta = self.neg().dot(&n).min(1.0);
        let r_out_perpendicular = ratio * (*self + cos_theta * n);
        let r_out_parallel = (1.0 - r_out_perpendicular.norm_squared()).abs().sqrt().neg() * n;

        r_out_perpendicular + r_out_parallel
    }

    pub fn random_cosine_direction() -> Self {
        let r1 = random_double();
        let r2 = random_double();
        let z = (1.0 - r2).sqrt();

        let phi = std::f64::consts::TAU * r1;
        let (sin_phi, cos_phi) = phi.sin_cos();
        let sqrt_r2 = r2.sqrt();
        let x = cos_phi * sqrt_r2;
        let y = sin_phi * sqrt_r2;

        Self::new(x, y, z)
    }

    pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Self {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = std::f64::consts::TAU * r1;
        let (sin_phi, cos_phi) = phi.sin_cos();
        let sqrt_r2 = (1.0 - z * z).sqrt();
        let x = cos_phi * sqrt_r2;
        let y = sin_phi * sqrt_r2;

        Self::new(x, y, z)
    }
}

impl std::iter::Sum for Vec3d {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Vec3d::zero(), std::ops::Add::add)
    }
}
