use super::memory::Memory;
use crate::sfinder_core::field::{bit_operators, key_operators};

pub struct SmallMemory(u64);

impl SmallMemory {
    pub fn new() -> Self {
        Self(0)
    }
}

impl Memory for SmallMemory {
    fn get(&self, x: u8, y: u8) -> bool {
        self.0 & bit_operators::get_x_mask(x, y) != 0
    }

    fn set(&mut self, x: u8, y: u8) {
        self.0 |= bit_operators::get_x_mask(x, y);
    }

    fn clear(&mut self) {
        self.0 = 0;
    }
}
