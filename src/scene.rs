use std::f64::INFINITY;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle, ProgressIterator};
use rayon::prelude::*;

use crate::camera::Camera;
use crate::color::Color3d;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ppm::PPMFile;
use crate::ray::Ray;
use crate::util::random_double;

pub struct Scene<T>
    where T: Hittable + Send + Sync {
    pub height: usize,
    pub width: usize,
    pub world: T,
    pub camera: Camera,
    pub spp: usize,
}

impl<T> Scene<T>
    where T: Hittable + Send + Sync {
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
        let pb = self.get_progress_bar();
        let start_time = std::time::Instant::now();

        for j in (0..self.height).progress_with(pb) {
            for i in 0..self.width {
                let mut color = Color3d::zero();
                for _ in 0..self.spp {
                    color += self.render_single(i, j);
                }

                buf[self.get_pixel_index(i, j)] = color;
            }
        }

        println!("\nTracing ({}*{}, spp={}) finished in {}.",
                 self.width, self.height, self.spp,
                 indicatif::HumanDuration(start_time.elapsed()));

        buf
    }

    pub fn render_parallel(&self) -> Vec<Color3d> {
        let start_time = std::time::Instant::now();
        let pb = self.get_progress_bar();
        let result = (0..self.height).into_par_iter().progress_with(pb).flat_map(|j| {
            (0..self.width).into_par_iter().map(move |i| {
                (0..self.spp).map(|_| self.render_single(i, j)).sum::<Color3d>()
            })
        }).collect();

        println!("\nTracing ({}*{}, spp={}) finished in {}.",
                 self.width, self.height, self.spp,
                 indicatif::HumanDuration(start_time.elapsed()));
        result
    }

    fn get_progress_bar(&self) -> ProgressBar {
        ProgressBar::new(self.height as u64)
            .with_style(
                ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>4}/{len:4} ({eta})")
                .progress_chars("#>-"))
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

    pub fn get_ppm_file_parallel(&self) -> PPMFile {
        PPMFile::create(
            self.height, self.width, self.spp, self.render_parallel()
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
