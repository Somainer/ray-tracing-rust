use crate::vec3::{Point3d, Vec3, Vec3d};
use crate::ray::Ray;
use crate::material::Material;
use std::rc::Rc;
use std::borrow::Borrow;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub point: Point3d,
    pub normal: Vec3d,
    pub t: f64,
    pub material: &'a (dyn Material + Send + Sync),
    front_face: bool
}

impl<'a> HitRecord<'a> {
    pub fn new_with_face_normal(
        t: f64,
        point: Point3d,
        outward_normal: Vec3d,
        material: &'a (dyn Material + Send + Sync),
        ray: &Ray,
    ) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Self {
            t, point, normal, front_face, material
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
}
