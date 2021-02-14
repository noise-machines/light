use nanorand::RNG;

pub struct Rand {
    rng: nanorand::WyRand,
}

impl Rand {
    pub fn from_seed(seed: u64) -> Rand {
        let rng = nanorand::WyRand::new_seed(seed);
        Rand { rng }
    }
    pub fn generate(&mut self) -> f32 {
        let random_number = self.rng.generate::<u16>();
        (random_number as f32) / (u16::MAX as f32)
    }
}
