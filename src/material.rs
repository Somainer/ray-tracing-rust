use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color3d;
use crate::vec3::{Point3d, Vec3d};
use std::ops::{Neg, Deref};
use crate::util::random_double;
use crate::texture::{Texture, SolidColor};
use std::f64::consts::PI;
use crate::onb::OrthonormalBasis;
use crate::pdf::{PDF, CosinePDF, NoPDF};

pub struct ScatterRecord<P = Box<dyn PDF>>
where P: PDF {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color3d,
    pub pdf: P
}

impl<P: PDF> ScatterRecord<P> {
    pub fn new(
        ray: Ray,
        is_specular: bool,
        attenuation: Color3d,
        pdf: P
    ) -> Self {
        Self {
            specular_ray: ray,
            is_specular, attenuation, pdf
        }
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;

    fn emitted(&self, u: f64, v: f64, p: Point3d) -> Color3d;

    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, ray_scattered: &Ray) -> f64;
}

impl Material for Box<dyn Material + Send + Sync> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        self.as_ref().scatter(ray_in, hit_record)
    }

    fn emitted(&self, u: f64, v: f64, p: Point3d) -> Color3d {
        self.as_ref().emitted(u, v, p)
    }

    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, ray_scattered: &Ray) -> f64 {
        self.as_ref().scattering_pdf(ray_in, hit_record, ray_scattered)
    }
}

impl<M: Material + Send + Sync> Material for Box<M> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        self.as_ref().scatter(ray_in, hit_record)
    }

    fn emitted(&self, u: f64, v: f64, p: Point3d) -> Color3d {
        self.as_ref().emitted(u, v, p)
    }

    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, ray_scattered: &Ray) -> f64 {
        self.as_ref().scattering_pdf(ray_in, hit_record, ray_scattered)
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

macro_rules! no_scattering_pdf {
    () => {
        #[inline]
        fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, ray_scattered: &Ray) -> f64 {
            0.0
        }
    };
}

#[derive(Clone)]
pub struct Diffuse<T: Texture> {
    pub albedo: T
}

impl<T: Texture> Diffuse<T> {
    #[inline]
    pub fn new(texture: T) -> Self {
        Diffuse { albedo: texture }
    }
}

impl Diffuse<SolidColor> {
    #[inline]
    pub fn for_color(color: Color3d) -> Self {
        Self::new(SolidColor::new(color))
    }
}

impl<T: Texture> Material for Diffuse<T> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let color = self.albedo.eval(hit_record.u, hit_record.v, hit_record.point);
        let pdf = CosinePDF::new(hit_record.normal);
        Some(ScatterRecord::new(
            *ray_in,
            false,
            color,
                Box::new(pdf)
        ))
    }

    no_emission!();

    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, ray_scattered: &Ray) -> f64 {
        let cosine = hit_record.normal.dot(&ray_scattered.direction().normalized());

        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color3d,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray_in.direction().normalized().reflect(&hit_record.normal);
        let fuzzed = reflected + self.fuzz * Vec3d::random_in_unit_sphere();

        Some(ScatterRecord::new(
            Ray::new_with_time(hit_record.point, fuzzed, ray_in.time()),
            true,
            self.albedo,
            Box::new(NoPDF)
        ))
    }

    no_emission!();
    no_scattering_pdf!();
}

#[derive(Clone)]
pub struct Dielectric {
    pub index_refraction: f64
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
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

        Some(ScatterRecord::new(
            Ray::new_with_time(hit_record.point, direction, ray_in.time()),
            true,
            Color3d::one(),
            Box::new(NoPDF)
        ))
    }

    no_emission!();
    no_scattering_pdf!();
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r * r;

        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

pub struct DiffuseLight<T: Texture> {
    emit: T
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        Self { emit }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3d) -> Color3d {
        self.emit.eval(u, v, p)
    }

    no_scattering_pdf!();
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
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let scattered =
            Ray::new_with_time(hit_record.point, Vec3d::random_in_unit_sphere(), ray_in.time());
        let attenuation = self.albedo.eval(hit_record.u, hit_record.v, hit_record.point);

        Some(ScatterRecord::new(
            scattered,
            true,
            attenuation,
            Box::new(NoPDF)
        ))
    }

    no_emission!();
    no_scattering_pdf!();
}
