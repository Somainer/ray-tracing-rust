use std::f64::consts::PI;
use rand::distributions::Uniform;
use rand::prelude::*;
use crate::vec3d_extensions::RandomGen;
#[macro_export]
macro_rules! property {
    ($($name: ident : $type: ty)+) => ($(
        #[inline]
        pub fn $name(&self) -> $type {
            self.$name
        }
    )+);
}

pub fn deg_to_rad(degree: f64) -> f64 {
    degree * PI / 180.0
}
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / PI
}

pub fn random_double() -> f64 {
    lazy_static::lazy_static!{
        static ref distribution: Uniform<f64> = Uniform::new(0.0, 1.0);
    }
    let mut rng = rand::thread_rng();
    rng.sample(*distribution)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

#[inline]
pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min { min }
    else if x > max { max }
    else { x }
}

pub enum Angle {
    RadAngle(f64),
    DegAngle(f64)
}

impl Angle {
    #[inline]
    pub fn rad(&self) -> f64 {
        match self {
            Angle::RadAngle(rad) => *rad,
            Angle::DegAngle(deg) => deg_to_rad(*deg)
        }
    }

    #[inline]
    pub fn deg(&self) -> f64 {
        match self {
            Angle::DegAngle(deg) => *deg,
            Angle::RadAngle(rad) => rad_to_deg(*rad)
        }
    }
}
