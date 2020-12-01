use crate::vec3::{Vec3, Point3d};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Point3d,
    direction: Vec3<f64>,
    time: f64
}

impl Ray {
    #[inline]
    pub fn new(origin: Point3d, direction: Vec3<f64>) -> Ray {
        Self::new_with_time(origin, direction, 0.0)
    }

    #[inline]
    pub fn new_with_time(origin: Point3d, direction: Vec3<f64>, time: f64) -> Self {
        Self { origin, direction, time }
    }

    #[inline]
    pub fn origin(&self) -> Point3d {
        self.origin
    }

    #[inline]
    pub fn direction(&self) -> Vec3<f64> {
        self.direction
    }

    property! { time: f64 }

    #[inline]
    pub fn at(&self, time: f64) -> Vec3<f64> {
        self.origin + self.direction * time
    }
}