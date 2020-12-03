use crate::vec3::{Vec3, Vec3d};

pub struct OrthonormalBasis {
    pub axis: Vec3<Vec3d>
}

macro_rules! map_axis {
    ($from: ident -> $to: ident) => {
        #[inline]
        pub fn $to(&self) -> Vec3d {
            self.axis.$from
        }
    };
}

impl OrthonormalBasis {
    map_axis! { x -> u }
    map_axis! { y -> v }
    map_axis! { z -> w }

    #[inline]
    pub fn local(&self, a: Vec3d) -> Vec3d {
        a.x * self.u() +
            a.y * self.v() +
            a.z * self.w()
    }

    pub fn from_w(n: Vec3d) -> Self {
        let z = n.normalized();
        let a =
            if z.x.abs() > 0.9 {
                Vec3d::new(0.0, 1.0, 0.0)
            } else {
                Vec3d::new(1.0, 0.0, 0.0)
            };
        let y = z.cross(&a).normalized();
        let x = z.cross(&y);

        Self {
            axis: Vec3::new(x, y, z)
        }
    }
}
