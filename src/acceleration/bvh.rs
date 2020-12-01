use crate::acceleration::aabb::AABB;
use crate::hittable_list::HittableList;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use std::borrow::Borrow;
use crate::util::random_in_range;
use std::cmp::Ordering;

pub struct BVH<'a> {
    objects: &'a [Box<dyn Hittable + Send + Sync>],
    root: Box<BVHTree>
}

impl<'a> BVH<'a> {
    pub fn new(list: &'a [Box<dyn Hittable + Send + Sync>], t_min: f64, t_max: f64) -> Self {
        let leaves = Self::get_leaves(list, t_min, t_max);
        let root = Self::recursive_build(&leaves);

        BVH {
            objects: list,
            root
        }
    }

    fn get_leaves(list: &[Box<dyn Hittable + Send + Sync>], t_min: f64, t_max: f64) -> Vec<BVHTree> {
        let mut leaves: Vec<_> = list.iter().enumerate().map(|(i, obj)| {
            BVHTree::Leaf(obj.bounding_box(t_min, t_max).unwrap().clone(), i)
        }).collect();
        let axis = random_in_range(0, 2);
        leaves.sort_unstable_by(|a, b| {
            let box_a = a.bounds();
            let box_b = b.bounds();

            box_a.minimum[axis].partial_cmp(&box_b.minimum[axis])
                .unwrap_or(Ordering::Equal)
        });

        leaves
    }

    fn recursive_build(list: &[BVHTree]) -> Box<BVHTree> {
        let object_span = list.len();
        if object_span == 1 {
            Box::new(list[0].clone())
        } else {
            let mid = list.len() >> 1;
            let left = Self::recursive_build(&list[..mid]);
            let right = Self::recursive_build(&list[mid..]);
            let bounds = left.bounds().surround_with(right.bounds());

            Box::new(BVHTree::Branch {
                left, right, bounds
            })
        }
    }

    fn merge_hits(ray: &Ray, objects: &'a [Box<dyn Hittable + Send + Sync>], candidates: &[usize], t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        candidates.iter().flat_map(|&index| {
            objects[index].hit(ray, t_min, t_max)
        }).min_by(|hit1, hit2| {
            hit1.t.partial_cmp(&hit2.t).unwrap_or(Ordering::Equal)
        })
    }
}

#[derive(Clone)]
enum BVHTree {
    Leaf(AABB, usize),
    Branch {
        bounds: AABB,
        left: Box<BVHTree>,
        right: Box<BVHTree>
    }
}

impl BVHTree {
    fn bounds(&self) -> &AABB {
        match self {
            Self::Leaf(bounds, ..) => bounds,
            Self::Branch { bounds, .. } => bounds
        }
    }

    fn get_intersect_candidates(&self, ray: &Ray, t_min: f64, t_max: f64, candidates: &mut Vec<usize>) {
        match self {
            Self::Leaf(bounds, index) => {
                if bounds.hit(ray, t_min, t_max) {
                    candidates.push(*index)
                }
            },
            Self::Branch { bounds, left, right } => {
                if bounds.hit(ray, t_min, t_max) {
                    left.get_intersect_candidates(ray, t_min, t_max, candidates);
                    right.get_intersect_candidates(ray, t_min, t_max, candidates);
                }
            }
        }
    }
}

impl<'a> Hittable for BVH<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut candidates = vec![];
        self.root.get_intersect_candidates(ray, t_min, t_max, &mut candidates);

        Self::merge_hits(ray, self.objects, &candidates, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(self.root.bounds().clone())
    }
}

pub struct OwnedBVH {
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
    root: Box<BVHTree>
}

impl OwnedBVH {
    pub fn new(objects: Vec<Box<dyn Hittable + Send + Sync>>, t_min: f64, t_max: f64) -> Self {
        let leaves = BVH::get_leaves(&objects, t_min, t_max);
        let root = BVH::recursive_build(&leaves);

        OwnedBVH {
            objects,
            root
        }
    }
}

impl Hittable for OwnedBVH {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut candidates = vec![];
        self.root.get_intersect_candidates(ray, t_min, t_max, &mut candidates);

        BVH::merge_hits(ray, &self.objects, &candidates, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(self.root.bounds().clone())
    }
}
