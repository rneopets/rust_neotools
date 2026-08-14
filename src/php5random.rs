use php5rand::{Php5MtRandom, Php5Random};
use pyo3::prelude::*;

/// Python-exposed wrapper around `Php5Random` (PHP5's `rand()`/`srand()`).
#[pyclass(name = "Php5Random")]
pub struct PyPhp5Random(Php5Random);

#[pymethods]
impl PyPhp5Random {
    #[new]
    fn new(seed: u32) -> Self {
        PyPhp5Random(Php5Random::new(seed))
    }

    fn rand(&mut self) -> u32 {
        self.0.rand()
    }

    fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        self.0.rand_range(min, max)
    }
}

/// Python-exposed wrapper around `Php5MtRandom` (PHP5's `mt_rand()`/`mt_srand()`).
#[pyclass(name = "Php5MtRandom")]
pub struct PyPhp5MtRandom(Php5MtRandom);

#[pymethods]
impl PyPhp5MtRandom {
    #[new]
    fn new(seed: u32) -> Self {
        PyPhp5MtRandom(Php5MtRandom::new(seed))
    }

    fn rand(&mut self) -> u32 {
        self.0.rand()
    }

    fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        self.0.rand_range(min, max)
    }
}
