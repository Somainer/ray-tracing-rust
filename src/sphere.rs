use crate::vec3::Point3d;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use std::rc::Rc;
use crate::material::Material;

pub struct Sphere {
    center: Point3d,
    radius: f64,
    pub material: Rc<dyn Material>
}

impl Sphere {
    pub fn new (center: Point3d, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center, radius, material
        }
    }
    property! { center: Point3d }
    property! { radius: f64 }

    #[inline]
    pub fn material_clone(&self) -> Rc<dyn Material> {
        self.material.clone()
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None
            }
        }

        let point = ray.at(root);
        Some(HitRecord::new_with_face_normal(
            root, point, (point - self.center) / self.radius,
            self.material_clone(), &ray,
        ))
    }
}
