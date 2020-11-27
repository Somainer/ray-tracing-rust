use std::ops::*;
use std::fmt::{Display, Formatter};

extern crate num_traits;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.x)
            .chain(std::iter::once(&self.y))
            .chain(std::iter::once(&self.z))
    }
}

impl<T: Copy> Vec3<T> {
    pub fn only(value: T) -> Self {
        Self::new(value, value, value)
    }

    pub fn values(&self) -> impl Iterator<Item = T> {
        std::iter::once(self.x)
            .chain(std::iter::once(self.y))
            .chain(std::iter::once(self.z))
    }
}

impl <T: num_traits::Num + Copy> Vec3<T> {
    pub fn one() -> Self {
        Self::new(
            T::one(),
            T::one(),
            T::one(),
        )
    }

    pub fn zero() -> Self {
        Self::new(
            T::zero(),
            T::zero(),
            T::zero(),
        )
    }

    pub fn linear_interpolation(start: Self, end: Self, value: T) -> Self {
        start * (T::one() - value) + end * value
    }
}

macro_rules! impl_bin_op {
    ($t: ident :: $name: ident) => {
        impl <T: $t<Output = T>> $t for Vec3<T> {
            type Output = Self;

            fn $name(self, rhs: Self) -> Self::Output {
                Self {
                    x: $t::$name(self.x, rhs.x),
                    y: $t::$name(self.y, rhs.y),
                    z: $t::$name(self.z, rhs.z)
                }
            }
        }

        impl <T: $t<Output = T> + Copy> $t<T> for Vec3<T> {
            type Output = Vec3<T>;

            fn $name(self, rhs: T) -> Self::Output {
                Self::Output {
                    x: $t::$name(self.x, rhs),
                    y: $t::$name(self.y, rhs),
                    z: $t::$name(self.z, rhs),
                }
            }
        }
    };
}

macro_rules! impl_bin_assign_op {
    ($t: ident :: $method: ident) => {
        impl<T: $t> $t for Vec3<T> {
            fn $method(&mut self, rhs: Self) {
                $t::$method(&mut self.x, rhs.x);
                $t::$method(&mut self.y, rhs.y);
                $t::$method(&mut self.z, rhs.z);
            }
        }

        impl <T: $t + Copy> $t<T> for Vec3<T> {
            fn $method(&mut self, rhs: T) {
                $t::$method(&mut self.x, rhs);
                $t::$method(&mut self.y, rhs);
                $t::$method(&mut self.z, rhs);
            }
        }
    };
}

impl_bin_op!(Add::add);
impl_bin_op!(Sub::sub);
impl_bin_op!(Mul::mul);
impl_bin_assign_op!(AddAssign::add_assign);
impl_bin_assign_op!(SubAssign::sub_assign);

// impl <T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
//     type Output = Self;
//
//     fn mul(self, rhs: T) -> Self::Output {
//         Self {
//             x: self.x * rhs,
//             y: self.y * rhs,
//             z: self.z * rhs
//         }
//     }
// }

macro_rules! impl_mul_for {
    {$($type: ident)+} => ($(
        impl Mul<Vec3<$type>> for $type {
            type Output = Vec3<$type>;

            #[inline]
            fn mul(self, rhs: Vec3<$type>) -> Self::Output {
                rhs * self
            }
        }
    )+)
}

impl_mul_for!{ usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl <T: MulAssign + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl <T: num_traits::Float> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        self.mul(T::one() / rhs)
    }
}

impl <T: DivAssign + Copy> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg()
        }
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index {} out of bounds 0 - 2.", index)
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index {} out of bounds 0 - 2.", index)
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vec3<T> {
    pub fn norm_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: num_traits::Float> Vec3<T> {
    pub fn norm(&self) -> T {
        self.norm_squared().sqrt()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x
        )
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let norm = self.norm();

        self.mul(T::one() / norm)
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub type Point3d = Vec3<f64>;
pub type Vec3d = Vec3<f64>;
