use crate::sfinder_core::field::{bit_operators, field_constants::BOARD_HEIGHT};

use super::memory::Memory;

const BORDER_LOWER: u8 = BOARD_HEIGHT;
const BORDER_MID: u8 = BOARD_HEIGHT * 2;
const BORDER_HIGH: u8 = BOARD_HEIGHT * 3;

pub struct LargeMemory(u64, u64, u64, u64);

impl LargeMemory {
    pub fn new() -> Self {
        Self(0, 0, 0, 0)
    }
}

impl Memory for LargeMemory {
    fn get(&self, x: u8, y: u8) -> bool {
        match y {
            ..BORDER_LOWER => self.0 & bit_operators::get_x_mask(x, y) != 0,
            ..BORDER_MID => self.1 & bit_operators::get_x_mask(x, y - BORDER_LOWER) != 0,
            ..BORDER_HIGH => self.2 & bit_operators::get_x_mask(x, y - BORDER_MID) != 0,
            _ => self.3 & bit_operators::get_x_mask(x, y - BORDER_HIGH) != 0,
        }
    }

    fn set(&mut self, x: u8, y: u8) {
        match y {
            ..BORDER_LOWER => self.0 |= bit_operators::get_x_mask(x, y),
            ..BORDER_MID => self.1 |= bit_operators::get_x_mask(x, y - BORDER_LOWER),
            ..BORDER_HIGH => self.2 |= bit_operators::get_x_mask(x, y - BORDER_MID),
            _ => self.3 |= bit_operators::get_x_mask(x, y - BORDER_HIGH),
        }
    }

    fn clear(&mut self) {
        self.0 = 0;
        self.1 = 0;
        self.2 = 0;
        self.3 = 0;
    }
}
