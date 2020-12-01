use crate::vec3::Point3d;
use crate::color::Color3d;
use crate::perlin::Perlin;

pub trait Texture: Send + Sync {
    fn eval(&self, u: f64, v: f64, p: Point3d) -> Color3d;
}

impl Texture for Box<dyn Texture> {
    fn eval(&self, u: f64, v: f64, p: Point3d) -> Color3d {
        self.as_ref().eval(u, v, p)
    }
}

impl<T: Texture> Texture for Box<T> {
    fn eval(&self, u: f64, v: f64, p: Point3d) -> Color3d {
        self.as_ref().eval(u, v, p)
    }
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

pub struct CheckerTexture<T1, T2>
where T1: Texture, T2: Texture {
    pub even: T1,
    pub odd: T2,
}

impl<T1: Texture, T2: Texture> CheckerTexture<T1, T2> {
    pub fn new(even: T1, odd: T2) -> Self {
        Self { odd, even }
    }
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn for_two_color(c1: Color3d, c2: Color3d) -> Self {
        Self::new(
            SolidColor::new(c1),
            SolidColor::new(c2)
        )
    }
}

impl<T1: Texture, T2: Texture> Texture for CheckerTexture<T1, T2> {
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
