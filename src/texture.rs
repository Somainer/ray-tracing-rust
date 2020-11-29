use crate::vec3::Point3d;
use crate::color::Color3d;
use crate::perlin::Perlin;

pub trait Texture: Send + Sync {
    fn eval(&self, u: f64, v: f64, p: Point3d) -> Color3d;
}

#[derive(Clone)]
pub struct SolidColor {
    pub color: Color3d
}

impl SolidColor {
    #[inline]
    pub fn new(color: Color3d) -> Self {
        Self {
            color
        }
    }
}

impl Texture for SolidColor {
    fn eval(&self, _u: f64, _v: f64, _p: Point3d) -> Color3d {
        self.color
    }
}

pub struct CheckerTexture {
    pub even: Box<dyn Texture>,
    pub odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self { odd, even }
    }

    pub fn for_two_color(c1: Color3d, c2: Color3d) -> Self {
        Self::new(
            Box::new(SolidColor::new(c1)),
            Box::new(SolidColor::new(c2))
        )
    }
}

impl Texture for CheckerTexture {
    fn eval(&self, u: f64, v: f64, p: Point3d) -> Color3d {
        let sines = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();

        if sines < 0.0 {
            self.odd.eval(u, v, p)
        } else {
            self.even.eval(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale
        }
    }
}

impl Texture for NoiseTexture {
    fn eval(&self, _u: f64, _v: f64, p: Point3d) -> Color3d {
        (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turbulence(p * self.scale, 7))) *
            Color3d::one() * 0.5
    }
}
