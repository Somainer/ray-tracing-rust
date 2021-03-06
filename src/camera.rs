use crate::vec3::{Point3d, Vec3d, Vec3};
use crate::ray::Ray;
use crate::util::{deg_to_rad, Angle, random_range};

pub struct Camera {
    origin: Point3d,
    lower_left_corner: Point3d,
    horizontal: Vec3d,
    vertical: Vec3d,
    u: Vec3d, v: Vec3d, w: Vec3d,
    lens_radius: f64,
    pub shutter_open: f64, pub shutter_close: f64 // Shutter open/close time
}

impl Camera {
    #[inline]
    pub fn new(
        look_from: Point3d,
        look_at: Point3d,
        vup: Vec3d,
        aspect_ratio: f64,
        fov: Angle,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        Self::new_with_shutter(
            look_from,
            look_at,
            vup,
            aspect_ratio,
            fov,
            aperture,
            focus_dist, 0.0, 0.0
        )
    }

    pub fn new_with_shutter(
        look_from: Point3d,
        look_at: Point3d,
        vup: Vec3d,
        aspect_ratio: f64,
        fov: Angle,
        aperture: f64,
        focus_dist: f64,
        shutter_open: f64,
        shutter_close: f64
    ) -> Self {
        let theta = fov.rad();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalized();
        let u = vup.cross(&w).normalized();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u, v, w,
            lens_radius,
            shutter_open, shutter_close
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let radius = self.lens_radius * Vec3d::random_in_unit_disk();
        let offset = self.u * radius.x + self.v * radius.y;

        Ray::new_with_time(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            random_range(self.shutter_open, self.shutter_close)
        )
    }
}
