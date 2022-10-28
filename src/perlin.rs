use rand::seq::SliceRandom;
use rand_distr::{Distribution, UnitSphere};

use crate::{
    common::TRng,
    vec3::{Dir3, Point3},
};

#[derive(Debug, Clone)]
pub struct Perlin {
    ranvec: Vec<Dir3>,
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
    bits: usize,
}

impl Perlin {
    pub fn new(bits: usize, rng: &mut TRng) -> Self {
        let mut ranvec = Vec::<Dir3>::new();

        for _ in 0..(1 << bits) {
            ranvec.push(Dir3::new_from_arr(UnitSphere.sample(rng)));
        }
        let perm_x = Self::generate_per(bits, rng);
        let perm_y = Self::generate_per(bits, rng);
        let perm_z = Self::generate_per(bits, rng);

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
            bits,
        }
    }
    pub fn turbulence(&self, mut p: Point3, depth: i32, fall_off: f32) -> f32 {
        let mut accum = 0.0;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= fall_off;
            p = Point3(p.0 * 2.0);
        }

        accum.abs()
    }
    pub fn noise(&self, p: Point3) -> f32 {
        let mask = ((1 << self.bits) - 1) as i32;
        let u = p.0.e[0] - p.0.e[0].floor();
        let v = p.0.e[1] - p.0.e[1].floor();
        let w = p.0.e[2] - p.0.e[2].floor();
        let i = p.0.e[0].floor() as i32;
        let j = p.0.e[1].floor() as i32;
        let k = p.0.e[2].floor() as i32;

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut c = [[[Dir3::ZERO; 2]; 2]; 2];
        for di in 0i32..2 {
            for dj in 0i32..2 {
                for dk in 0i32..2 {
                    let r = (self.perm_x[((i + di) & mask) as usize]
                        ^ self.perm_y[((j + dj) & mask) as usize]
                        ^ self.perm_z[((k + dk) & mask) as usize])
                        as usize;
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[r];
                }
            }
        }

        let mut accum = 0.0;
        for i in 0..2 {
            let fi = i as f32;
            for j in 0..2 {
                let fj = j as f32;
                for k in 0..2 {
                    let fk = k as f32;
                    let weight = Dir3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (fi * uu + (1.0 - fi) * (1.0 - uu))
                        * (fj * vv + (1.0 - fj) * (1.0 - vv))
                        * (fk * ww + (1.0 - fk) * (1.0 - ww))
                        * Dir3::dot(c[i][j][k], weight);
                }
            }
        }

        accum
    }

    fn generate_per(bits: usize, rng: &mut TRng) -> Vec<u32> {
        let mut result = (0..(1 << bits) as u32)
            .into_iter()
            .collect::<Vec<_>>();
        result.shuffle(rng);
        result
    }
}
