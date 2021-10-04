use std::cell::Cell;
use std::fmt;

#[derive(Default)]
pub struct Counter(Cell<u64>);

impl Counter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc(&self) {
        self.add(1)
    }

    pub fn add(&self, rhs: u64) {
        self.0.set(self.0.get() + rhs)
    }

    pub fn get(&self) -> u64 {
        self.0.get()
    }

    pub fn take(&self) -> u64 {
        self.0.take()
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.get().fmt(f)
    }
}
