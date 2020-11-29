use crate::vec3::Vec3d;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::acceleration::aabb::AABB;

pub struct Translate {
    offset: Vec3d,
    hittable: Box<dyn Hittable + Send + Sync>
}

impl Translate {
    pub fn new(old: Box<dyn Hittable + Sync + Send>, displacement: Vec3d) -> Self {
        Self {
            offset: displacement,
            hittable: old
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new_with_time(ray.origin() - self.offset, ray.direction(), ray.time());
        self.hittable.hit(&moved_ray, t_min, t_max).map(|record| {
            HitRecord::new_with_face_normal(
                record.t, record.point + self.offset,
                record.u, record.v, record.normal, record.material,
                &moved_ray
            )
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.hittable.bounding_box(time0, time1).map(|mut record| {
            record.minimum += self.offset;
            record.maximum += self.offset;

            record
        })
    }
}
