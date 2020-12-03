use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::{Diffuse, Metal, Dielectric, DiffuseLight};
use crate::color::Color3d;
use crate::sphere::{Sphere, MovingSphere};
use crate::vec3::{Point3d, Vec3d};
use crate::util::{random_double, random_range, Angle, random_in_range};
use crate::vec3d_extensions::RandomGen;
use crate::acceleration::aabb::AABB;
use crate::texture::{CheckerTexture, NoiseTexture, SolidColor};
use crate::image_texture::ImageTexture;
use crate::rectangle::{XYRect, YZRect, XZRect, RectBox};
use crate::transformations::Transformable;
use crate::subsurface::ConstantMedium;
use crate::acceleration::bvh::OwnedBVH;

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
            RectBox::new(Point3d::new(0.0, 0.0, 0.0), Point3d::new(165.0, 330.0, 165.0), Box::new(white()))
                .rotate_y(Angle::DegAngle(15.0))
                .translate(Vec3d::new(265.0, 0.0, 295.0))
        ));
        world.add(Box::new(
            RectBox::new(Point3d::new(0.0, 0.0, 0.0), Point3d::new(165.0, 165.0, 165.0), Box::new(white()))
                .rotate_y(Angle::DegAngle(-18.0))
                .translate(Vec3d::new(130.0, 0.0, 65.0))
        ));

        world
    }

    pub fn cornel_smoke() -> Self {
        let mut world = Self::cornel_box();
        let box2 = world.objects.pop().unwrap();
        let box1 = world.objects.pop().unwrap();

        world.add(Box::new(ConstantMedium::for_color(box1, 0.01, Color3d::zero())));
        world.add(Box::new(ConstantMedium::for_color(box2, 0.01, Color3d::one())));

        world
    }

    pub fn all_feature_box() -> LightedWorld {
        let mut boxes1 = Self::new();
        let ground = Diffuse::for_color(Color3d::new(0.48, 0.82, 0.53));
        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = random_range(1.0, 101.0);
                let z1 = z0 + w;
                boxes1.add(Box::new(
                    RectBox::new(Point3d::new(x0, y0, z0), Point3d::new(x1, y1, z1), ground.clone())
                ));
            }
        }
        let mut objects = LightedWorld::new();

        objects.add(Box::new(OwnedBVH::new(boxes1.objects, 0.0, 1.0)));

        let light = DiffuseLight::new(SolidColor::new(Color3d::only(7.0)));
        objects.add_light(Box::new(
            XZRect::new((123.0, 147.0), (423.0, 412.0), 554.0, light)
        ));

        let center1 = Point3d::new(400.0, 400.0, 200.0);
        let center2 = center1 + Vec3d::new(30.0, 0.0, 0.0);
        let moving_sphere_material = Diffuse::for_color(Color3d::new(0.7, 0.3, 0.1));
        objects.add(Box::new(MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material)));
        objects.add(Box::new(
            Sphere::new(Point3d::new(260.0, 150.0, 45.0), 50.0, Dielectric { index_refraction: 1.5 })
        ));
        objects.add(Box::new(
            Sphere::new(Point3d::new(0.0, 150.0, 145.0), 50.0, Metal { albedo: Color3d::new(0.8, 0.8, 0.9), fuzz: 1.0 })
        ));

        let boundary = Sphere::new(Point3d::new(360.0, 150.0, 145.0), 70.0, Dielectric { index_refraction: 1.5 });
        objects.add(Box::new(boundary.clone()));
        objects.add(Box::new(ConstantMedium::for_color(boundary.clone(), 0.2, Color3d::new(0.2, 0.4, 0.9))));
        objects.add(Box::new(
            ConstantMedium::for_color(
                Sphere::new(Point3d::zero(), 5000.0, Dielectric{index_refraction: 1.5}),
                0.0001,
                Color3d::one()
            )));

        let earth_texture =
            ImageTexture::from_file("earthmap.jpg".to_string()).unwrap();
        let material = Diffuse::new(earth_texture);
        objects.add(Box::new(
            Sphere::new(Point3d::new(400.0, 200.0, 400.0), 100.0, material)
        ));

        let perlin_texture = NoiseTexture::new(0.1);
        objects.add(Box::new(
            Sphere::new(Point3d::new(220.0, 280.0, 300.0), 80.0, Box::new(Diffuse::new(perlin_texture)))
        ));

        let mut boxes2 = Self::new();
        let white = Diffuse::for_color(Color3d::only(0.73));
        for _ in 0..1000 {
            boxes2.add(Box::new(Sphere::new(Point3d::random_range(0.0, 165.0), 10.0, white.clone())));
        }
        let node = OwnedBVH::new(boxes2.objects, 0.0, 1.0)
            .rotate_y(Angle::DegAngle(15.0))
            .translate(Vec3d::new(-100.0, 270.0, 395.0));

        objects.add(Box::new(node));

        objects
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

    fn pdf_value(&self, origin: Point3d, v: Vec3d) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;

        self.objects.iter().map(|object| {
            weight * object.pdf_value(origin, v)
        }).sum()
    }

    fn random(&self, origin: Point3d) -> Vec3d {
        self.objects[random_in_range(0, self.objects.len())]
            .random(origin)
    }
}

pub struct LightedWorld {
    pub objects: HittableList,
    pub lights: HittableList
}

impl LightedWorld {
    pub fn new() -> Self {
        Self {
            objects: HittableList::new(),
            lights: HittableList::new()
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Send + Sync>) {
        self.objects.add(object);
    }

    pub fn add_light(&mut self, object: Box<dyn Hittable + Send + Sync>) {
        self.lights.add(object);
    }
}

impl Hittable for LightedWorld {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.objects.bounding_box(time0, time1)
    }

    fn pdf_value(&self, origin: Point3d, v: Vec3d) -> f64 {
        self.objects.pdf_value(origin, v)
    }

    fn random(&self, origin: Point3d) -> Vec3d {
        self.objects.random(origin)
    }
}

impl From<HittableList> for LightedWorld {
    fn from(list: HittableList) -> Self {
        Self {
            objects: list,
            lights: HittableList::new()
        }
    }
}
