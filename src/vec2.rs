use rand::distributions::uniform::SampleUniform;
use rand::distributions::uniform::UniformSampler;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Add)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}
pub type Vec2f = Vec2<f32>;

pub struct UniformVec2<T: SampleUniform> {
    x: Uniform<T>,
    y: Uniform<T>,
}

impl UniformSampler for UniformVec2<f32> {
    type X = Vec2f;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        let lx = low.borrow().x;
        let hx = high.borrow().x;
        let ly = low.borrow().y;
        let hy = high.borrow().y;

        Self {
            x: rand::distributions::Uniform::new(lx.min(hx), lx.max(hx)),
            y: rand::distributions::Uniform::new(ly.min(hy), ly.max(hy)),
        }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        let lx = low.borrow().x;
        let hx = high.borrow().x;
        let ly = low.borrow().y;
        let hy = high.borrow().y;
        Self {
            x: rand::distributions::Uniform::new_inclusive(lx.min(hx), lx.max(hx)),
            y: rand::distributions::Uniform::new_inclusive(ly.min(hy), ly.max(hy)),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Self::X {
            x: self.x.sample(rng),
            y: self.y.sample(rng),
        }
    }
}

impl SampleUniform for Vec2f {
    type Sampler = UniformVec2<f32>;
}