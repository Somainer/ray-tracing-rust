use crate::scene::Scene;
use crate::hittable_list::HittableList;
use crate::vec3::{Point3d, Vec3d};
use crate::camera::Camera;
use crate::util::Angle;

#[macro_use]
mod util;
// Macro modules must appear before modules using its macros.
mod vec3;
mod ppm;
mod color;
mod progress;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod material;
mod vec3d_extensions;
mod scene;

fn get_scene() -> Scene<HittableList> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let samples_per_pixel = 100;

    let world = HittableList::random();

    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let look_from = Point3d::new(13.0, 2.0, 3.0);
    let look_at = Point3d::new(0.0, 0.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3d::new(0.0, 1.0, 0.0),
        aspect_ratio,
        Angle::DegAngle(20.0),
        aperture,
        dist_to_focus,
    );

    let scene = Scene::new(
        image_height,
        image_width,
        world,
        camera,
        samples_per_pixel,
    );

    scene
}

fn main() {
    let path = "image.ppm";

    get_scene().get_ppm_file_parallel().write_to(path.to_string()).unwrap()
}
