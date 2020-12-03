use crate::vec3::{Vec3d, Point3d};
use crate::onb::OrthonormalBasis;
use std::f64::consts::PI;
use crate::hittable::Hittable;
use crate::util::random_double;

pub trait PDF: Send + Sync {
    fn value(&self, direction: Vec3d) -> f64;
    fn generate(&self) -> Vec3d;
}

#[derive(Copy, Clone)]
pub struct NoPDF;
impl PDF for NoPDF {
    fn value(&self, _direction: Vec3d) -> f64 {
        Default::default()
    }
    fn generate(&self) -> Vec3d {
        Default::default()
    }
}

pub trait PDFExtensions
where
    Self: PDF + Sized {
    fn mix<P: PDF + Sized>(self, that: P) -> MixturePDF<Self, P> {
        MixturePDF(self, that)
    }
}

impl <T: PDF + Sized> PDFExtensions for T {}

impl PDF for Box<dyn PDF> {
    fn value(&self, direction: Vec3d) -> f64 {
        self.as_ref().value(direction)
    }

    fn generate(&self) -> Vec3d {
        self.as_ref().generate()
    }
}

pub struct CosinePDF {
    uvw: OrthonormalBasis
}

impl CosinePDF {
    #[inline]
    pub fn new(w: Vec3d) -> Self {
        Self {
            uvw: OrthonormalBasis::from_w(w)
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3d) -> f64 {
        let cosine = direction.normalized().dot(&self.uvw.w());

        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3d {
        self.uvw.local(Vec3d::random_cosine_direction())
    }
}

pub struct HittablePDF<H: Hittable + Send + Sync> {
    origin: Point3d,
    hittable: H
}

impl<H: Hittable + Send + Sync> HittablePDF<H> {
    pub fn new(hittable: H, origin: Point3d) -> Self {
        Self {
            hittable, origin
        }
    }
}

impl<H: Hittable + Send + Sync> PDF for HittablePDF<H> {
    fn value(&self, direction: Vec3d) -> f64 {
        self.hittable.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3d {
        self.hittable.random(self.origin)
    }
}

impl<P: PDF> PDF for &P {
    fn value(&self, direction: Vec3d) -> f64 {
        (*self).value(direction)
    }

    fn generate(&self) -> Vec3d {
        (*self).generate()
    }
}

pub struct MixturePDF<P1, P2>(P1, P2)
where P1: PDF, P2: PDF;

impl<P1, P2> PDF for MixturePDF<P1, P2>
where
    P1: PDF,
    P2: PDF {
    fn value(&self, direction: Vec3d) -> f64 {
        0.5 * self.0.value(direction) +
            0.5 * self.1.value(direction)
    }

    fn generate(&self) -> Vec3d {
        if random_double() < 0.5 {
            self.0.generate()
        } else {
            self.1.generate()
        }
    }
}
