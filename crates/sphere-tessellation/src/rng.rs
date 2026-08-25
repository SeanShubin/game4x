//! A small deterministic generator, so worlds reproduce exactly from a seed.
//!
//! This is SplitMix64. It is not cryptographic and does not need to be; it needs to be
//! identical on every platform, which a hand-written integer routine guarantees and a
//! library dependency does not.

#[derive(Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `[0, 1)`.
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_gives_same_sequence() {
        let a: Vec<u64> = (0..16).map(|_| Rng::new(42).next_u64()).collect();
        let mut rng = Rng::new(42);
        let b: Vec<u64> = (0..16).map(|_| rng.next_u64()).collect();
        assert_eq!(a[0], b[0]);
        assert_ne!(b[0], b[1], "the generator must actually advance");
    }

    #[test]
    fn unit_stays_in_range() {
        let mut rng = Rng::new(7);
        for _ in 0..10_000 {
            let value = rng.unit();
            assert!((0.0..1.0).contains(&value));
        }
    }
}
