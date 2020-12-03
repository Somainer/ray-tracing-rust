use crate::scene::Scene;
use crate::hittable_list::HittableList;
use crate::vec3::{Point3d, Vec3d};
use crate::camera::Camera;
use crate::util::Angle;
use crate::color::Color3d;

#[macro_use]
mod util;
// Macro modules must appear before modules using its macros.
mod vec3;
mod ppm;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod material;
mod vec3d_extensions;
mod scene;
mod acceleration;
mod texture;
mod perlin;
mod image_texture;
mod rectangle;
mod transformations;
mod subsurface;
mod onb;
mod pdf;

fn get_scene() -> Scene {
    // let aspect_ratio = 16.0 / 9.0;
    let aspect_ratio = 1.0;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let samples_per_pixel = 100;

    // let world = HittableList::random();
    // let world = HittableList::perlin_noise();
    // let world = HittableList::earth();
    let world = HittableList::cornel_box();

    let aperture = 0.0;
    let dist_to_focus = 10.0;
    let look_from = Point3d::new(278.0, 278.0, -800.0);
    let look_at = Point3d::new(278.0, 278.0, 0.0);
    // let look_at = Point3d::zero();
    let camera = Camera::new_with_shutter(
        look_from,
        look_at,
        Vec3d::new(0.0, 1.0, 0.0),
        aspect_ratio,
        Angle::DegAngle(40.0),
        aperture,
        dist_to_focus,
        0.0, 1.0
    );

    // let background = Color3d::new(0.70, 0.80, 1.00);
    let background = Color3d::zero();

    let scene = Scene::new(
        image_height,
        image_width,
        world,
        camera,
        samples_per_pixel,
        background,
    );

    scene
}

fn main() {
    let path = "image.ppm";

    get_scene().get_ppm_file_parallel().write_to(path.to_string()).unwrap()
}
