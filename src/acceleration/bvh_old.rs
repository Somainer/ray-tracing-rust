use crate::acceleration::aabb::AABB;
use crate::hittable_list::HittableList;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use std::borrow::Borrow;
use crate::util::random_in_range;
use std::cmp::Ordering;

pub enum BVH<'a> {
    Leaf(AABB, &'a (dyn Hittable + Send + Sync)),
    Branch {
        left: Box<BVH<'a>>,
        right: Box<BVH<'a>>,
        bounds: AABB
    },
}

impl<'a> BVH<'a> {
    fn unit(object: &'a (dyn Hittable + Send + Sync), time0: f64, time1: f64) -> Self {
        BVH::Leaf(
            object.bounding_box(time0, time1).unwrap(),
            object
        )
    }

    pub fn from_hittable_list(list: &'a HittableList, time0: f64, time1: f64) -> Self {
        Self::from_list(&list.objects.iter().map(|x| x.borrow()).collect(), time0, time1)
    }

    pub fn from_list(list: &Vec<&'a (dyn Hittable + Send + Sync)>, time0: f64, time1: f64) -> Self {
        let axis = random_in_range(0, 2);

        let mut objects: Vec<&(dyn Hittable + Sync + Send)> = list.clone();
        objects.sort_unstable_by(|a, b| {
            let box_a = a.bounding_box(0.0, 0.0).unwrap();
            let box_b = b.bounding_box(0.0, 0.0).unwrap();

            box_a.minimum[axis].partial_cmp(&box_b.minimum[axis])
                .unwrap_or(Ordering::Equal)
        });

        Self::recursive_build(
            &objects,
            0,
            objects.len(),
            time0, time1
        )
    }

    fn recursive_build(
        objects: &Vec<&'a (dyn Hittable + Send + Sync)>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64
    ) -> Self {
        let object_span = end - start;

        if object_span == 1 {
            Self::unit(objects[start], time0, time1)
        } else if object_span == 2 {
            let left = Box::new(Self::unit(objects[start], time0, time1));
            let right = Box::new(Self::unit(objects[start + 1], time0, time1));

            let bounds = left.get_box().surround_with(right.get_box());
            Self::Branch {
                left, right, bounds
            }
        } else {
            let mid = start + object_span / 2;
            let left = Self::recursive_build(objects, start, mid, time0, time1);
            let right = Self::recursive_build(objects, mid + 1, end, time0, time1);
            let bounds = left.get_box().surround_with(right.get_box());

            Self::Branch {
                left: Box::new(left),
                right: Box::new(right),
                bounds
            }
        }
    }

    fn get_box(&self) -> &AABB {
        match self {
            BVH::Leaf(bounds, ..) => bounds,
            BVH::Branch { bounds, .. } => bounds
        }
    }
}

impl Hittable for BVH<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.get_box().hit(ray, t_min, t_max) {
            None
        } else {
            match self {
                BVH::Leaf(_, hittable) => hittable.hit(ray, t_min, t_max),
                BVH::Branch {
                    left, right, ..
                } => {
                    let left_hit = left.hit(ray, t_min, t_max);
                    let right_hit =
                        right.hit(ray, t_min, left_hit.as_ref().map_or(t_max, |x| x.t));

                    left_hit.or(right_hit)
                }
            }
        }

    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.get_box().clone())
    }
}
