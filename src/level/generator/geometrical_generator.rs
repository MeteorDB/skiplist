use crate::level::LevelGenerator;
use rand::prelude::*;

pub struct GeometricalGenerator {
    total: usize,
    p: f64,
    rng: SmallRng,
}

impl GeometricalGenerator {
    pub fn new(total: usize, p: f64) -> Self {
        assert!(total > 0, "total must be greater than 0.");
        assert!(p > 0.0 && p < 1.0, "p must be in (0, 1).");
        GeometricalGenerator {
            total,
            p,
            rng: SmallRng::from_rng(thread_rng()).unwrap(),
        }
    }
}

impl LevelGenerator for GeometricalGenerator {
    fn total(&self) -> usize {
        self.total
    }

    fn random(&mut self) -> usize {
        let mut h = 0;
        let mut x = self.p;
        let f = 1.0 - self.rng.gen::<f64>();
        while x > f && h + 1 < self.total {
            h += 1;
            x *= self.p
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::GeometricalGenerator;

    #[test]
    #[should_panic]
    fn invalid_total() {
        GeometricalGenerator::new(0, 0.5);
    }

    #[test]
    #[should_panic]
    fn invalid_p_0() {
        GeometricalGenerator::new(1, 0.0);
    }

    #[test]
    #[should_panic]
    fn invalid_p_1() {
        GeometricalGenerator::new(1, 1.0);
    }

    #[test]
    fn new() {
        GeometricalGenerator::new(1, 0.5);
    }
}
