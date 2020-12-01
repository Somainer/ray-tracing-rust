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
}

pub trait Hittable {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64
    ) -> Option<HitRecord>;

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

impl Hittable for Box<dyn Hittable + Send + Sync> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_ref().hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.as_ref().bounding_box(time0, time1)
    }
}
