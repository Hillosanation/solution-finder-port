use crate::sfinder_core::field::{bit_operators, field_constants::BOARD_HEIGHT};

use super::memory::Memory;

pub struct MiddleMemory(u64, u64);

impl MiddleMemory {
    pub fn new() -> Self {
        Self(0, 0)
    }
}

impl Memory for MiddleMemory {
    fn get(&self, x: u8, y: u8) -> bool {
        match y {
            ..BOARD_HEIGHT => self.0 & bit_operators::get_x_mask(x, y) != 0,
            _ => self.1 & bit_operators::get_x_mask(x, y - BOARD_HEIGHT) != 0,
        }
    }

    fn set(&mut self, x: u8, y: u8) {
        match y {
            ..BOARD_HEIGHT => self.0 |= bit_operators::get_x_mask(x, y),
            _ => self.1 |= bit_operators::get_x_mask(x, y - BOARD_HEIGHT),
        }
    }

    fn clear(&mut self) {
        self.0 = 0;
        self.1 = 0;
    }
}
