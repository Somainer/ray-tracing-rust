use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color3d;
use crate::vec3::{Point3d, Vec3d};
use std::ops::Neg;
use crate::util::random_double;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)>;
}

pub struct Diffuse {
    pub albedo: Color3d
}

impl Material for Diffuse {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)> {
        let scatter_direction =
            match hit_record.normal + Vec3d::random_in_unit_sphere().normalized() {
                m if m.near_zero() => hit_record.normal,
                m => m
            };

        Some((self.albedo, Ray::new(hit_record.point, scatter_direction)))
    }
}

pub struct Metal {
    pub albedo: Color3d,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)> {
        let reflected = ray_in.direction().normalized().reflect(&hit_record.normal);
        let fuzzed = reflected + self.fuzz * Vec3d::random_in_unit_sphere();

        if fuzzed.dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, Ray::new(hit_record.point, fuzzed)))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub index_refraction: f64
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)> {
        let refraction_ratio =
            if hit_record.front_face() {
                1.0 / self.index_refraction
            } else {
                self.index_refraction
            };
        let unit_redirection = ray_in.direction().normalized();
        let cos_theta = unit_redirection.neg().dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let full_reflect = refraction_ratio * sin_theta > 1.0;
        let direction =
            if full_reflect || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
                unit_redirection.reflect(&hit_record.normal)
            } else {
                unit_redirection.refract(&hit_record.normal, refraction_ratio)
            };
        Some((Color3d::one(), Ray::new(hit_record.point, direction)))
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r * r;

        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}
