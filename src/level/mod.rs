pub mod generator;

pub trait LevelGenerator {
    fn total(&self) -> usize;

    fn random(&mut self) -> usize;
}
