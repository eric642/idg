use std::sync::atomic::{AtomicU64, Ordering};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {}

pub trait Generator {
    fn generator_id(&self) -> Result<u64, GeneratorError> {
        return Ok(1);
    }
}

struct Section(u64, u64);

impl Section {
    fn new(start: u64, end: u64) -> Self {
        Section(start, end)
    }
}

pub trait Allocator<T: Into<GeneratorError>> {
    fn allocator(step_size: u32) -> Result<Section, T>;
}

struct generator {
    now_id: AtomicU64,
    max_id: u64,
}

impl Generator for generator {
    fn generator_id(&self) -> Result<u64, GeneratorError> {
        let v = self.now_id.fetch_add(1, Ordering::Relaxed) + 1;
        if v < self.max {
            return Ok(v);
        }

        Ok(1)
    }
}
