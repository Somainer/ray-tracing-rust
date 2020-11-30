use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color3d;
use crate::vec3::{Point3d, Vec3d};
use std::ops::Neg;
use crate::util::random_double;
use crate::texture::{Texture, SolidColor};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)>;

    fn emitted(&self, u: f64, v: f64, p: Point3d) -> Color3d;
}

impl Material for Box<dyn Material + Send + Sync> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)> {
        self.as_ref().scatter(ray_in, hit_record)
    }

    fn emitted(&self, u: f64, v: f64, p: Point3d) -> Color3d {
        self.as_ref().emitted(u, v, p)
    }
}

macro_rules! no_emission {
    () => {
        #[inline]
        fn emitted(&self, u: f64, v: f64, p: Point3d) -> Color3d {
            Color3d::zero()
        }
    };
}

pub struct Diffuse {
    pub albedo: Box<dyn Texture>
}

impl Diffuse {
    #[inline]
    pub fn new(texture: Box<dyn Texture>) -> Self {
        Diffuse { albedo: texture }
    }

    #[inline]
    pub fn for_color(color: Color3d) -> Self {
        Self::new(Box::new(SolidColor::new(color)))
    }
}

impl Material for Diffuse {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)> {
        let scatter_direction =
            match hit_record.normal + Vec3d::random_in_unit_sphere().normalized() {
                m if m.near_zero() => hit_record.normal,
                m => m
            };

        let color = self.albedo.eval(hit_record.u, hit_record.v, hit_record.point);
        Some((color, Ray::new_with_time(hit_record.point, scatter_direction, ray_in.time())))
    }

    no_emission!();
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color3d,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)> {
        let reflected = ray_in.direction().normalized().reflect(&hit_record.normal);
        let fuzzed = reflected + self.fuzz * Vec3d::random_in_unit_sphere();

        if fuzzed.dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, Ray::new_with_time(hit_record.point, fuzzed, ray_in.time())))
        } else {
            None
        }
    }

    no_emission!();
}

#[derive(Clone)]
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
        Some((Color3d::one(), Ray::new_with_time(hit_record.point, direction, ray_in.time())))
    }

    no_emission!();
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r * r;

        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

pub struct DiffuseLight {
    emit: Box<dyn Texture>
}

impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Color3d, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3d) -> Color3d {
        self.emit.eval(u, v, p)
    }
}

pub struct Isotropic<T>
where T: Texture {
    albedo: T
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl Isotropic<SolidColor> {
    pub fn for_color(color: Color3d) -> Self {
        Self::new(SolidColor::new(color))
    }
}

impl<T> Material for Isotropic<T>
where T: Texture {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)> {
        let scattered =
            Ray::new_with_time(hit_record.point, Vec3d::random_in_unit_sphere(), ray_in.time());
        let attenuation = self.albedo.eval(hit_record.u, hit_record.v, hit_record.point);

        Some((attenuation, scattered))
    }

    no_emission!();
}
