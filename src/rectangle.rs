use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::acceleration::aabb::AABB;
use crate::vec3::{Point3d, Vec3d};
use std::borrow::Borrow;
use crate::hittable_list::HittableList;
use crate::color::Color3d;

type Point2d = (f64, f64);

// pub struct XYRect {
//     p0: Point2d,
//     p1: Point2d,
//     k: f64,
//     material: Box<dyn Material + Send + Sync>
// }
//
// impl XYRect {
//     pub fn new(p0: Point2d, p1: Point2d, k: f64, material: Box<dyn Material + Send + Sync>) -> Self {
//         Self {
//             p0, p1, k, material
//         }
//     }
// }
//
// impl Hittable for XYRect {
//     fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         let t = (self.k - ray.origin().z) / ray.direction().z;
//         if t < t_min || t > t_max {
//             return None
//         }
//
//         let (x0, y0) = self.p0;
//         let (x1, y1) = self.p1;
//         let x = ray.origin().x + t * ray.direction().x;
//         let y = ray.origin().y + t * ray.direction().y;
//
//         if x < x0 || x > x1 || y < y0 || y > y1 {
//             return None
//         }
//
//         let u = (x - x0) / (x1 - x0);
//         let v = (y - y0) / (y1 - y0);
//         let outward_normal = Vec3d::new(0.0, 0.0, 1.0);
//         Some(HitRecord::new_with_face_normal(
//             t, ray.at(t), u, v, outward_normal,
//             self.material.borrow(), ray
//         ))
//     }
//
//     fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
//         let (x0, y0) = self.p0;
//         let (x1, y1) = self.p1;
//
//         Some(AABB::new(
//             Point3d::new(x0, y0, self.k - 0.0001),
//             Point3d::new(x1, y1, self.k + 0.0001)
//         ))
//     }
// }

macro_rules! impl_rectangle {
    (define $rect_name:ident that
        parallel with $x: ident and $y: ident, perpendicular to $z: ident;
        corner is ($x0: ident, $y0: ident) to ($x1: ident, $y1: ident);
        bounding box is [($min_box_x:expr, $min_box_y: expr, $min_box_z: expr), ($max_box_x: expr, $max_box_y: expr, $max_box_z: expr)];
        normal is ($norm_x: expr, $norm_y: expr, $norm_z: expr);) => {
        pub struct $rect_name<M: Material + Send + Sync> {
            p0: Point2d,
            p1: Point2d,
            k: f64,
            material: M
        }
        impl<M: Material + Send + Sync> $rect_name<M> {
            pub fn new(p0: Point2d, p1: Point2d, k: f64, material: M) -> Self {
                Self {
                    p0, p1, k, material
                }
            }
        }

        impl<M: Material + Send + Sync> Hittable for $rect_name<M> {
            fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
                let t = (self.k - ray.origin().$z) / ray.direction().$z;
                if t < t_min || t > t_max {
                    return None
                }

                let ($x0, $y0) = self.p0;
                let ($x1, $y1) = self.p1;
                let $x = ray.origin().$x + t * ray.direction().$x;
                let $y = ray.origin().$y + t * ray.direction().$y;

                if $x < $x0 || $x > $x1 || $y < $y0 || $y > $y1 {
                    return None
                }

                let u = ($x - $x0) / ($x1 - $x0);
                let v = ($y - $y0) / ($y1 - $y0);
                let outward_normal = Vec3d::new($norm_x, $norm_y, $norm_z);
                Some(HitRecord::new_with_face_normal(
                    t, ray.at(t), u, v, outward_normal,
                    self.material.borrow(), ray
                ))
            }

            fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
                let ($x0, $y0) = self.p0;
                let ($x1, $y1) = self.p1;
                let $z = self.k;

                Some(AABB::new(
                    Point3d::new($min_box_x, $min_box_y, $min_box_z),
                    Point3d::new($max_box_x, $max_box_y, $max_box_z)
                ))
            }
        }
    };
}

impl_rectangle! {
    define XYRect that
        parallel with x and y, perpendicular to z;
        corner is (x0, y0) to (x1, y1);
        bounding box is [(x0, y0, z - 0.0001), (x1, y1, z + 0.0001)];
        normal is (0.0, 0.0, 1.0);
}

impl_rectangle! {
    define XZRect that
        parallel with x and z, perpendicular to y;
        corner is (x0, z0) to (x1, z1);
        bounding box is [(x0, y - 0.0001, z0), (x1, y + 0.0001, z1)];
        normal is (0.0, 1.0, 0.0);
}

impl_rectangle! {
    define YZRect that
        parallel with y and z, perpendicular to x;
        corner is (y0, z0) to (y1, z1);
        bounding box is [(x - 0.0001, y0, z0), (x + 0.0001, y1, z1)];
        normal is (1.0, 0.0, 0.0);
}

#[derive(Copy, Clone)]
pub struct DummyMaterial;
impl Material for DummyMaterial {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3d) -> Color3d {
        Color3d::zero()
    }
}

pub struct RectBox<M: Material + Send + Sync> {
    min: Point3d,
    max: Point3d,
    sides: HittableList,
    material: M
}

impl<M: Material + Send + Sync> RectBox<M> {
    pub fn new(min: Point3d, max: Point3d, material: M) -> Self {
        let mut sides = HittableList::new();
        let dummy = DummyMaterial;
        sides.add(Box::new(
            XYRect::new((min.x, min.y), (max.x, max.y), min.z, dummy)
        ));
        sides.add(Box::new(
            XYRect::new((min.x, min.y), (max.x, max.y), max.z, dummy)
        ));

        sides.add(Box::new(
            XZRect::new((min.x, min.z), (max.x, max.z), min.y, dummy)
        ));
        sides.add(Box::new(
            XZRect::new((min.x, min.z), (max.x, max.z), max.y, dummy)
        ));

        sides.add(Box::new(
            YZRect::new((min.y, min.z), (max.y, max.z), min.x, dummy)
        ));
        sides.add(Box::new(
            YZRect::new((min.y, min.z), (max.y, max.z), max.x, dummy)
        ));

        Self {
            min, max,
            sides,
            material
        }
    }
}

impl<M: Material + Send + Sync> Hittable for RectBox<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max).map(|mut record| {
            record.material = self.material.borrow();
            record
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}
