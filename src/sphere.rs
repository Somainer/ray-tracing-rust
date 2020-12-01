use crate::vec3::{Point3d, Vec3d, Vec3};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;
use std::borrow::Borrow;
use crate::acceleration::aabb::AABB;
use std::ops::Neg;
use std::f64::consts::{PI, TAU};

#[derive(Clone)]
pub struct Sphere<M>
where M: Material + Send + Sync {
    center: Point3d,
    radius: f64,
    pub material: M
}

impl<M: Material + Send + Sync> Sphere<M> {
    pub fn new (center: Point3d, radius: f64, material: M) -> Self {
        Self {
            center, radius, material
        }
    }

    fn get_sphere_uv(p: &Point3d) -> (f64, f64) {
        let theta = p.y.neg().acos();
        let phi = p.z.neg().atan2(p.x) + PI;

        (phi / TAU, theta / PI)
    }

    property! { center: Point3d }
    property! { radius: f64 }
}

fn solve_sphere_equation(ray: &Ray, center: Point3d, radius: f64, t_min: f64, t_max: f64)
    -> Option<(f64, Point3d, Vec3d)> {
    let oc = ray.origin() - center;
    let a = ray.direction().norm_squared();
    let half_b = oc.dot(&ray.direction());
    let c = oc.norm_squared() - radius * radius;

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
    Some((root, point, (point - center) / radius))
}

impl<M: Material + Sync + Send> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        solve_sphere_equation(ray, self.center, self.radius, t_min, t_max)
            .map(|(root, point, outward_normal)| {
                // Outward normal is actually a point on the unit sphere centered at the origin.
                let (u, v) = Self::get_sphere_uv(&outward_normal);
                HitRecord::new_with_face_normal(
                    root, point, u, v, outward_normal, self.material.borrow(), ray
                )
            })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - Point3d::only(self.radius),
            self.center + Point3d::only(self.radius)
        ))
    }
}

pub struct MovingSphere<M: Material + Send + Sync> {
    center0: Point3d, center1: Point3d,
    time0: f64, time1: f64,
    radius: f64,
    pub material: M
}

impl<M: Material + Send + Sync> MovingSphere<M> {
    pub fn new(
        center0: Point3d, center1: Point3d,
        time0: f64, time1: f64,
        radius: f64,
        material: M
    ) -> Self {
        Self {
            center0, center1,
            time0, time1,
            radius, material
        }
    }

    pub fn center(&self, time: f64) -> Point3d {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<M: Material + Send + Sync> Hittable for MovingSphere<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        solve_sphere_equation(ray, self.center(ray.time()), self.radius, t_min, t_max)
            .map(|(root, point, outward_normal)| {
                let (u, v) = Sphere::<M>::get_sphere_uv(&outward_normal);
                HitRecord::new_with_face_normal(
                    root, point, u, v,outward_normal, self.material.borrow(), ray
                )
            })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let radius_vec = Vec3d::only(self.radius);
        let time0_center = self.center(time0);
        let time1_center = self.center(time1);
        let box0 = AABB::new(
            time0_center - radius_vec,
            time0_center + radius_vec
        );

        let box1 = AABB::new(
            time1_center - radius_vec,
            time1_center + radius_vec
        );

        Some(AABB::surround(box0, box1))
    }
}
