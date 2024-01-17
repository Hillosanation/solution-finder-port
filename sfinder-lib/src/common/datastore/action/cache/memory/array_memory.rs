use super::memory::Memory;
use crate::sfinder_core::field::{bit_operators, field_constants::BOARD_HEIGHT};

// Porting note: I used bitboards here to reuse implementation
pub struct ArrayMemory {
    // TODO: smallvec optimization?
    data: Vec<u64>,
}

impl ArrayMemory {
    pub fn new(height: u8) -> Self {
        Self {
            data: vec![0; (height / BOARD_HEIGHT) as _],
        }
    }
}

impl Memory for ArrayMemory {
    fn get(&self, x: u8, y: u8) -> bool {
        self.data[(y / BOARD_HEIGHT) as usize] & bit_operators::get_x_mask(x, y % BOARD_HEIGHT) != 0
    }

    fn set(&mut self, x: u8, y: u8) {
        self.data[(y / BOARD_HEIGHT) as usize] |= bit_operators::get_x_mask(x, y % BOARD_HEIGHT);
    }

    fn clear(&mut self) {
        self.data.fill(0);
    }
}
