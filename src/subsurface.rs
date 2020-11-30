use crate::hittable::{Hittable, HitRecord};
use crate::texture::{Texture, SolidColor};
use crate::material::{Material, Isotropic};
use crate::color::Color3d;
use crate::ray::Ray;
use crate::acceleration::aabb::AABB;
use crate::util::random_double;

pub struct ConstantMedium<H, M>
where
    H: Hittable + Send + Sync,
    M: Material + Send + Sync {
    boundary: H,
    phase_function: M,
    neg_inv_density: f64
}

impl<H: Hittable + Sync + Send, T: Texture> ConstantMedium<H, Isotropic<T>> {
    pub fn new(boundary: H, density: f64, texture: T) -> Self {
        let phase_function = Isotropic::new(texture);
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function
        }
    }
}

impl <H: Hittable + Send + Sync> ConstantMedium<H, Isotropic<SolidColor>> {
    pub fn for_color(boundary: H, density: f64, color: Color3d) -> Self {
        Self::new(boundary, density, SolidColor::new(color))
    }
}

impl<H, M> Hittable for ConstantMedium<H, M>
where
    H: Hittable + Send + Sync,
    M: Material + Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record1 =
            self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY)?;
        let mut record2 =
            self.boundary.hit(ray, record1.t + 0.0001, f64::INFINITY)?;

        if record1.t < t_min {
            record1.t = t_min;
        }
        if record2.t > t_max {
            record2.t = t_max;
        }

        if record1.t >= record2.t {
            return None
        }

        if record1.t < 0.0 {
            record1.t = 0.0
        }

        let ray_length = ray.direction().norm();
        let distance_inside_boundary = (record2.t - record1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return None
        }

        let t = record1.t + hit_distance / ray_length;
        let point = ray.at(t);
        Some(HitRecord::new_with_face_normal(t,
                                             point,
                                             record1.u, record1.v,
                                             record1.normal,
                                             &self.phase_function, ray))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}