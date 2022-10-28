use rand::{Rng, seq::SliceRandom};

use crate::{common::TRng, vec3::Point3};

#[derive(Debug, Clone)]
pub struct Perlin {
    ranfloat : Vec<f32>,
    perm_x : Vec<u32>,
    perm_y : Vec<u32>,
    perm_z : Vec<u32>,
    bits : usize,
}

impl Perlin {
    pub fn new(bits : usize, rng : &mut TRng) -> Self {
        let mut ranfloat = Vec::<f32>::new();

        for _ in 0..(1<<bits) {
            ranfloat.push(rng.gen())
        }
        let perm_x = Self::generate_per(bits, rng);
        let perm_y = Self::generate_per(bits, rng);
        let perm_z = Self::generate_per(bits, rng);

        Self { ranfloat, perm_x, perm_y, perm_z, bits}
    }
    pub fn sample(&self, p : Point3) -> f32 {
        let mask = ((1<<self.bits) - 1) as i32;
        let i = ((p.0.e[0] * 4.0) as i32 & mask) as usize;
        let j = ((p.0.e[1] * 4.0) as i32 & mask) as usize;
        let k = ((p.0.e[2] * 4.0) as i32 & mask) as usize;
        let r = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;
        self.ranfloat[r]
    }
    fn generate_per(bits : usize, rng : &mut TRng) -> Vec::<u32> {
        let mut result = (0 as u32..(1<<bits) as u32).into_iter().collect::<Vec<_>>();
        result.shuffle(rng);
        result
    }
}