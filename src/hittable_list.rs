use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::{Diffuse, Metal, Dielectric, DiffuseLight};
use crate::color::Color3d;
use crate::sphere::{Sphere, MovingSphere};
use crate::vec3::{Point3d, Vec3d};
use crate::util::{random_double, random_range};
use crate::vec3d_extensions::RandomGen;
use crate::acceleration::aabb::AABB;
use crate::texture::{CheckerTexture, NoiseTexture, SolidColor};
use crate::image_texture::ImageTexture;
use crate::rectangle::{XYRect, YZRect, XZRect, RectBox};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Send + Sync>>
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }

    pub fn random() -> Self {
        let mut world = Self::new();
        // let ground_material = Box::new(Diffuse::for_color(Color3d::only(0.5)));
        let ground_material = Box::new(Diffuse::new(Box::new(
            CheckerTexture::for_two_color(
                Color3d::new(0.2, 0.3, 0.1),
                Color3d::only(0.9)
            ))
        ));

        world.add(Box::new(
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
                        let material = Box::new(Diffuse::for_color(albedo));
                        if random_double() < 0.5 {
                            let sphere = Sphere::new(
                                center,
                                0.2,
                                material
                            );
                            world.add(Box::new(sphere));
                        } else {
                            let sphere = MovingSphere::new(
                                center, center + Vec3d::new(0.0, random_range(0.0, 0.5), 0.0),
                                0.0, 1.0, 0.2, material
                            );
                            world.add(Box::new(sphere));
                        }
                    } else if choose_material < 0.95 {
                        let albedo = Color3d::random_range(0.5, 1.0);
                        let fuzz = random_range(0.0, 0.5);
                        let material = Box::new(Metal { albedo, fuzz });
                        world.add(Box::new(
                            Sphere::new(
                                center,
                                0.2,
                                material
                            )
                        ));
                    } else {
                        let material = Box::new(Dielectric { index_refraction: 1.5 });
                        world.add(Box::new(
                            Sphere::new(
                                center, 0.2, material
                            )
                        ));
                    }
                }
            }
        }

        world.add(Box::new(Sphere::new(
            Point3d::new(0.0, 1.0, 0.0), 1.0,
            Box::new(Dielectric { index_refraction: 1.5 })
        )));
        world.add(Box::new(Sphere::new(
            Point3d::new(-4.0, 1.0, 0.0), 1.0,
            Box::new(Diffuse::for_color(Color3d::new(0.7, 0.6, 0.5))))));
        world.add(Box::new(Sphere::new(
            Point3d::new(4.0, 1.0, 0.0), 1.0,
            Box::new(Metal { albedo: Color3d::new(0.7, 0.6, 0.5), fuzz: 0.0 }))));
        world
    }

    pub fn perlin_noise() -> Self {

        let mut world = Self::new();
        // let ground_material = Box::new(Diffuse::for_color(Color3d::only(0.5)));
        let ground_material = Box::new(Diffuse::new(Box::new(
            NoiseTexture::new(4.0)
        )));

        world.add(Box::new(
            Sphere::new(
                Point3d::new(0.0, -1000.0, 0.0),
                1000.0,
                ground_material)));
        world.add(Box::new(Sphere::new(
            Point3d::new(4.0, 1.0, 0.0), 1.0,
            Box::new(Diffuse::new(Box::new(NoiseTexture::new(4.0)))))));

        world
    }

    pub fn earth() -> Self {
        let mut world = Self::new();
        let earth_texture =
            ImageTexture::from_file("earthmap.jpg".to_string()).unwrap();
        let material = Diffuse::new(Box::new(earth_texture));

        world.add(Box::new(
            Sphere::new(
                Point3d::zero(), 2.0, Box::new(material)
            )
        ));

        world
    }

    pub fn sample_light() -> Self {
        let mut world = Self::perlin_noise();

        let light = DiffuseLight::new(Box::new(SolidColor::new(Color3d::only(4.0))));
        let light2 = DiffuseLight::new(Box::new(SolidColor::new(Color3d::only(4.0))));
        world.add(Box::new(XYRect::new((3.0, 1.0), (5.0, 3.0), -2.0, Box::new(light))));
        world.add(Box::new(
            Sphere::new(Point3d::new(0.0, 7.0, 0.0), 2.0, Box::new(light2))
        ));

        world
    }

    pub fn cornel_box() -> Self {
        let mut world = Self::new();

        let red = Diffuse::for_color(Color3d::new(0.65, 0.05, 0.05));
        let white = || Diffuse::for_color(Color3d::only(0.73));
        let green = Diffuse::for_color(Color3d::new(0.12, 0.45, 0.15));
        let light = DiffuseLight::new(Box::new(SolidColor::new(Color3d::only(15.0))));

        world.add(Box::new(
            YZRect::new((0.0, 0.0), (555.0, 555.0), 555.0, Box::new(green))
        ));
        world.add(Box::new(
            YZRect::new((0.0, 0.0), (555.0, 555.0), 0.0, Box::new(red))
        ));
        world.add(Box::new(
            XZRect::new((0.0, 0.0), (555.0, 555.0), 0.0, Box::new(white()))
        ));
        world.add(Box::new(
            XZRect::new((0.0, 0.0), (555.0, 555.0), 555.0, Box::new(white()))
        ));
        world.add(Box::new(
            XYRect::new((0.0, 0.0), (555.0, 555.0), 555.0, Box::new(white()))
        ));
        world.add(Box::new(
            XZRect::new((213.0, 227.0), (343.0, 332.0), 554.0, Box::new(light))
        ));

        world.add(Box::new(
            RectBox::new(Point3d::new(130.0, 0.0, 65.0), Point3d::new(295.0, 165.0, 230.0), Box::new(white()))
        ));
        world.add(Box::new(
            RectBox::new(Point3d::new(265.0, 0.0, 295.0), Point3d::new(430.0, 330.0, 460.0), Box::new(white()))
        ));

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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let mut result_box: Option<AABB> = None;
        for object in &self.objects {
            if let Some(bounding_box) = object.bounding_box(time0, time1) {
                if let Some(old_box) = result_box.take() {
                    result_box.replace(AABB::surround(old_box, bounding_box));
                } else {
                    result_box.replace(bounding_box);
                }
            } else {
                return None
            }
        }

        result_box
    }
}
