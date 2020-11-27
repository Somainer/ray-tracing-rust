use std::rc::Rc;

use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::{Diffuse, Metal, Dielectric};
use crate::color::Color3d;
use crate::sphere::Sphere;
use crate::vec3::Point3d;
use crate::util::{random_double, random_range};
use crate::vec3d_extensions::RandomGen;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn random() -> Self {
        let mut world = Self::new();
        let ground_material = Rc::new(Diffuse { albedo: Color3d::only(0.5) });
        world.add(Rc::new(
            Sphere::new(
                Point3d::new(0.0, -1000.0, 0.0),
                1000.0,
                ground_material)));

        for a in -11..11 {
            for b in -11..11 {
                let choose_material = random_double();
                let center = Point3d::new(a as f64 + 0.9 * random_double(), 0.2,  b as f64 + 0.9 * random_double());

                if (center - Point3d::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                    if choose_material < 0.8 {
                        let albedo = Color3d::random() * Color3d::random();
                        world.add(Rc::new(Sphere::new(
                            center,
                            0.2,
                            Rc::new(Diffuse { albedo })
                        )));
                    } else if choose_material < 0.95 {
                        let albedo = Color3d::random_range(0.5, 1.0);
                        let fuzz = random_range(0.0, 0.5);
                        let material = Rc::new(Metal { albedo, fuzz });
                        world.add(Rc::new(
                            Sphere::new(
                                center,
                                0.2,
                                material
                            )
                        ));
                    } else {
                        let material = Rc::new(Dielectric { index_refraction: 1.5 });
                        world.add(Rc::new(
                            Sphere::new(
                                center, 0.2, material
                            )
                        ));
                    }
                }
            }
        }

        world.add(Rc::new(Sphere::new(
            Point3d::new(0.0, 1.0, 0.0), 1.0,
            Rc::new(Dielectric { index_refraction: 1.5 })
        )));
        world.add(Rc::new(Sphere::new(
            Point3d::new(-4.0, 1.0, 0.0), 1.0,
            Rc::new(Diffuse { albedo: Color3d::new(0.7, 0.6, 0.5) }))));
        world.add(Rc::new(Sphere::new(
            Point3d::new(4.0, 1.0, 0.0), 1.0,
            Rc::new(Metal { albedo: Color3d::new(0.7, 0.6, 0.5), fuzz: 0.0 }))));
        world
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        let mut closest = t_max;

        for object in self.objects.iter() {
            if let Some(result) = object.hit(&ray, t_min, closest) {
                closest = result.t;
                hit = Some(result);
            }
        }

        hit
    }
}