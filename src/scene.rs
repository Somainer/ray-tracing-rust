use std::f64::INFINITY;

use crate::camera::Camera;
use crate::color::Color3d;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ppm::PPMFile;
use crate::progress::ProgressIterable;
use crate::ray::Ray;
use crate::util::random_double;

pub struct Scene<T>
    where T: Hittable {
    pub height: usize,
    pub width: usize,
    pub world: T,
    pub camera: Camera,
    pub spp: usize,
}

impl<T> Scene<T>
    where T: Hittable {
    pub fn new(height: usize, width: usize, world: T, camera: Camera, spp: usize) -> Self {
        Self {
            height,
            width,
            world,
            camera,
            spp,
        }
    }

    #[inline]
    fn render_single(&self, i: usize, j: usize) -> Color3d {
        let u = (i as f64 + random_double()) / (self.width - 1) as f64;
        let v = 1.0 - (j as f64 + random_double()) / (self.height - 1) as f64;
        let r = self.camera.get_ray(u, v);

        ray_color(&r, &self.world, 50)
    }

    pub fn render(&self) -> Vec<Color3d> {
        let mut buf = vec![Color3d::zero(); self.height * self.width];

        for j in (0..self.height).iter_progressed() {
            for i in 0..self.width {
                let mut color = Color3d::zero();
                for _ in 0..self.spp {
                    color += self.render_single(i, j);
                }

                buf[self.get_pixel_index(i, j)] = color;
            }
        }

        buf
    }

    #[inline]
    pub fn get_pixel_index(&self, i: usize, j: usize) -> usize {
        j * self.width + i
    }

    pub fn get_ppm_file(&self) -> PPMFile {
        PPMFile::create(
            self.height, self.width, self.spp, self.render(),
        )
    }
}

pub fn ray_color<H: Hittable>(ray: &Ray, world: &H, max_depth: i32) -> Color3d {
    if max_depth <= 0 {
        Color3d::zero()
    } else if let Some(hit) = world.hit(&ray, 0.001, INFINITY) {
        if let Some((color, scattered)) = hit.material.scatter(&ray, &hit) {
            color * ray_color(&scattered, world, max_depth - 1)
        } else {
            Color3d::zero()
        }
    } else {
        let unit_direction = ray.direction().normalized();
        let time = 0.5 * (unit_direction.y + 1.0);
        Color3d::linear_interpolation(
            Color3d::one(),
            Color3d::new(0.5, 0.7, 1.0),
            time,
        )
    }
}
