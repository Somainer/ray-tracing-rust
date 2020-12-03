use crate::vec3::Point3d;
use crate::ray::Ray;

#[derive(Clone)]
pub struct AABB {
    pub minimum: Point3d,
    pub maximum: Point3d
}

impl AABB {
    pub fn new(minimum: Point3d, maximum: Point3d) -> Self {
        Self {
            minimum,
            maximum
        }
    }

    pub fn infinity() -> Self {
        Self::new(
            Point3d::only(f64::NEG_INFINITY),
            Point3d::only(f64::INFINITY)
        )
    }

    #[allow(unused_assignments)]
    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        macro_rules! test_on_axis {
            ($axis: ident) => {
                let inv_d = 1.0 / ray.direction().$axis;
                let t0 = (self.minimum.$axis - ray.origin().$axis) * inv_d;
                let t1 = (self.maximum.$axis - ray.origin().$axis) * inv_d;
                t_max = t0.max(t1);
                t_min = t0.min(t1);
                if t_max <= t_min {
                    return false
                }
            };
        }

        test_on_axis!(x);
        test_on_axis!(y);
        test_on_axis!(z);
        true
    }

    #[inline]
    pub fn surround(box0: Self, box1: Self) -> Self {
        box0.surround_with(&box1)
    }

    #[inline]
    pub fn surround_with(&self, box1: &Self) -> Self {
        let small = Point3d::element_wise_min(
            self.minimum, box1.minimum
        );

        let big = Point3d::element_wise_max(
            self.maximum, box1.maximum
        );

        Self::new(small, big)
    }
}