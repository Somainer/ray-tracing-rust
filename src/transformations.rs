use crate::vec3::{Vec3d, Point3d};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::acceleration::aabb::AABB;
use crate::util::Angle;

pub struct Translate<T>
where
    T: Hittable + Send + Sync {
    offset: Vec3d,
    hittable: T
}

impl<T: Hittable + Send + Sync> Translate<T> {
    pub fn new(old: T, displacement: Vec3d) -> Self {
        Self {
            offset: displacement,
            hittable: old
        }
    }
}

impl<T: Hittable + Send + Sync> Hittable for Translate<T> {
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

pub trait Transformable
where
    Self: Hittable + Send + Sync + Sized
{
    fn translate(self, offset: Vec3d) -> Translate<Self> {
        Translate::new(self, offset)
    }

    fn rotate_x(self, angle: Angle) -> RotateX<Self> {
        RotateX::new(self, angle)
    }

    fn rotate_y(self, angle: Angle) -> RotateY<Self> {
        RotateY::new(self, angle)
    }

    fn rotate_z(self, angle: Angle) -> RotateZ<Self> {
        RotateZ::new(self, angle)
    }

    fn flip(self) -> FlipFace<Self> {
        FlipFace { object: self }
    }
}

impl<T: Hittable + Send + Sync + Sized> Transformable for T {}

macro_rules! impl_rotation {
    (define $rotation_name: ident where
        ($x: ident, $y: ident, $z: ident) - ($sin_theta: ident, $cos_theta: ident) ->
        ($after_x: expr, $after_y: expr, $after_z: expr);
    ) => {
        pub struct $rotation_name<T>
        where
            T: Hittable + Send + Sync {
            hittable: T,
            sin_theta: f64,
            cos_theta: f64,
            bounds: Option<AABB>
        }

        impl<T> $rotation_name<T>
        where
            T: Hittable + Send + Sync {
            pub fn new(hittable: T, angle: Angle) -> Self {
                let rad_angle = angle.rad();
                let (sin_theta, cos_theta) = rad_angle.sin_cos();
                let bounding_box = hittable.bounding_box(0.0, 1.0);
                let bounds = bounding_box.map(|bbox| {
                    let mut min = Point3d::only(f64::INFINITY);
                    let mut max = Point3d::only(f64::NEG_INFINITY);
                    for i in 0..2 {
                        for j in 0..2 {
                            for k in 0..2 {
                                let x = i as f64 * bbox.maximum.x + (1.0 - i as f64) * bbox.minimum.x;
                                let y = j as f64 * bbox.maximum.y + (1.0 - j as f64) * bbox.minimum.y;
                                let z = k as f64 * bbox.maximum.z + (1.0 - k as f64) * bbox.minimum.z;

                                let tester =
                                    Self::rotated_by(Vec3d::new(x, y, z), -sin_theta, cos_theta);

                                min = Vec3d::element_wise_min(min, tester);
                                max = Vec3d::element_wise_max(max, tester);
                            }
                        }
                    }

                    AABB::new(min, max)
                });

                Self {
                    hittable, sin_theta, cos_theta, bounds
                }
            }

            #[inline]
            fn rotated_by(vector: Vec3d, $sin_theta: f64, $cos_theta: f64) -> Vec3d {
                let Vec3d { $x, $y, $z } = vector;
                Vec3d::new(
                    $after_x,
                    $after_y,
                    $after_z
                )
            }

            #[inline]
            fn rotate(&self, vector: Vec3d) -> Vec3d {
                let $rotation_name { sin_theta, cos_theta, ..} = self;

                Self::rotated_by(vector, *sin_theta, *cos_theta)
            }

            #[inline]
            fn reverse_rotate(&self, vector: Vec3d) -> Vec3d {
                let $rotation_name { sin_theta, cos_theta, ..} = self;

                Self::rotated_by(vector, -*sin_theta, *cos_theta)
            }
        }

        impl<T> Hittable for $rotation_name<T>
        where
            T: Hittable + Send + Sync {
            fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
                let origin = self.rotate(ray.origin());
                let direction = self.rotate(ray.direction());

                let rotated_ray = Ray::new_with_time(origin, direction, ray.time());

                self.hittable.hit(&rotated_ray, t_min, t_max).map(|record| {
                    HitRecord::new_with_face_normal(
                        record.t,
                        self.reverse_rotate(record.point),
                        record.u, record.v,
                        self.reverse_rotate(record.normal),
                        record.material,
                        &rotated_ray
                    )
                })
            }

            fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
                self.bounds.clone()
            }
        }
    };
}

impl_rotation! {
    define RotateX where
        (x, y, z) - (sin_theta, cos_theta) ->
            (x, cos_theta * y - sin_theta * z, sin_theta * y + cos_theta * z);
}

impl_rotation! {
    define RotateY where
        (x, y, z) - (sin_theta, cos_theta) ->
            (cos_theta * x - sin_theta * z, y, sin_theta * x + cos_theta * z);
}

impl_rotation! {
    define RotateZ where
        (x, y, z) - (sin_theta, cos_theta) ->
            (cos_theta * x - sin_theta * y, sin_theta * y + cos_theta * z, z);
}

pub struct FlipFace<H: Hittable + Send + Sync> {
    object: H
}

impl<H: Hittable + Send + Sync> Hittable for FlipFace<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let hit = self.object.hit(ray, t_min, t_max)?;
        hit.flipped().into()
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.object.bounding_box(time0, time1)
    }
}
