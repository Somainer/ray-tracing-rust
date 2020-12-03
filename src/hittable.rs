use crate::vec3::{Point3d, Vec3, Vec3d};
use crate::ray::Ray;
use crate::material::Material;
use crate::acceleration::aabb::AABB;
use std::rc::Rc;
use crate::hittable_list::HittableList;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub point: Point3d,
    pub normal: Vec3d,
    pub t: f64,
    pub u: f64, pub v: f64,
    pub material: &'a (dyn Material + Send + Sync),
    front_face: bool
}

impl<'a> HitRecord<'a> {
    pub fn new_with_face_normal(
        t: f64,
        point: Point3d,
        u: f64, v: f64,
        outward_normal: Vec3d,
        material: &'a (dyn Material + Send + Sync),
        ray: &Ray,
    ) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Self {
            t, point, normal, front_face, material, u, v
        }
    }

    property! { front_face: bool }

    pub fn flipped(mut self) -> Self {
        self.front_face = !self.front_face;
        self
    }
}

pub trait Hittable {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64
    ) -> Option<HitRecord>;

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;

    fn pdf_value(&self, origin: Point3d, v: Vec3d) -> f64 {
        Default::default()
    }

    fn random(&self, origin: Point3d) -> Vec3d {
        Vec3d::new(1.0, 0.0, 0.0)
    }
}

impl Hittable for Box<dyn Hittable + Send + Sync> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_ref().hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.as_ref().bounding_box(time0, time1)
    }

    fn pdf_value(&self, origin: Point3d, v: Vec3d) -> f64 {
        self.as_ref().pdf_value(origin, v)
    }

    fn random(&self, origin: Point3d) -> Vec3d {
        self.as_ref().random(origin)
    }
}

impl<H: Hittable> Hittable for &H {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (*self).hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        (*self).bounding_box(time0, time1)
    }

    fn pdf_value(&self, origin: Point3d, v: Vec3d) -> f64 {
        (*self).pdf_value(origin, v)
    }

    fn random(&self, origin: Point3d) -> Vec3d {
        (*self).random(origin)
    }
}
