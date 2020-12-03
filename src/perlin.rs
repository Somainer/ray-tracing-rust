use crate::util::random_in_range;
use crate::vec3::{Point3d, Vec3d};
use crate::vec3d_extensions::RandomGen;

#[derive(Clone)]
pub struct Perlin {
    random_vector: Vec<Vec3d>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let random_vector =
            (0..Self::POINT_COUNT)
                .map(|_| Vec3d::random_range(-1.0, 1.0).normalized())
                .collect();
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            random_vector, perm_x, perm_y, perm_z
        }
    }

    pub fn noise(&self, p: Point3d) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3d::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.random_vector[
                        (self.perm_x[((i + di) & 255) as usize] ^
                        self.perm_y[((j + dj) & 255) as usize] ^
                        self.perm_z[((k + dk) & 255) as usize]) as usize
                    ]
                }
            }
        }

        Self::perlin_interpolation(&c, u, v, w)
    }

    pub fn turbulence(&self, p: Point3d, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = (0..Self::POINT_COUNT as i32).collect();
        Self::permute(&mut p);

        p
    }

    fn permute(p: &mut Vec<i32>) {
        for i in (0..p.len()).rev() {
            let target = random_in_range(0, i);
            p.swap(i, target);
        }
    }

    fn perlin_interpolation(c: &[[[Vec3d; 2]; 2]; 2], u1: f64, v1: f64, w1: f64) -> f64 {
        let u = u1 * u1 * (3.0 - 2.0 * u1);
        let v = v1 * v1 * (3.0 - 2.0 * v1);
        let w = w1 * w1 * (3.0 - 2.0 * w1);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v =
                        Vec3d::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u)) *
                        (j as f64 * v + (1 - j) as f64 * (1.0 - v)) *
                        (k as f64 * w + (1 - k) as f64 * (1.0 - w)) *
                        c[i as usize][j as usize][k as usize].dot(&weight_v);
                }
            }
        }

        accum
    }

    #[allow(unused)]
    fn tri_linear_interpolation(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u)) *
                        (j as f64 * v + (1 - j) as f64 * (1.0 - v)) *
                        (k as f64 * w + (1 - k) as f64 * (1.0 - w)) *
                        c[i as usize][j as usize][k as usize];
                }
            }
        }

        accum
    }
}
