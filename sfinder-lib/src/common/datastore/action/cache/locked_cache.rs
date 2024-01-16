use super::memory::{memory::Memory, small_memory::SmallMemory};
use crate::{
    common::datastore::action::cache::memory::{
        array_memory::ArrayMemory, large_memory::LargeMemory, middle_memory::MiddleMemory,
    },
    sfinder_core::{field::field_constants::BOARD_HEIGHT, srs::rotate::Rotate},
};

pub struct LockedCache {
    visited_map: [Box<dyn Memory>; Rotate::get_size()],
    found_map: [Box<dyn Memory>; Rotate::get_size()],
}

impl LockedCache {
    pub fn new(height: u8) -> Self {
        Self {
            visited_map: std::array::from_fn(|_| Self::create_memory(height)),
            found_map: std::array::from_fn(|_| Self::create_memory(height)),
        }
    }

    fn create_memory(height: u8) -> Box<dyn Memory> {
        const MIDDLE_HEIGHT: u8 = BOARD_HEIGHT * 2;
        const LARGE_HEIGHT: u8 = BOARD_HEIGHT * 3;

        match height {
            ..=BOARD_HEIGHT => Box::new(SmallMemory::new()),
            ..=MIDDLE_HEIGHT => Box::new(MiddleMemory::new()),
            ..=LARGE_HEIGHT => Box::new(LargeMemory::new()),
            _ => Box::new(ArrayMemory::new(height)),
        }
    }

    pub fn clear(&mut self) {
        for memory in self.visited_map.iter_mut() {
            memory.clear();
        }
        for memory in self.found_map.iter_mut() {
            memory.clear();
        }
    }

    pub fn visit(&mut self, x: u8, y: u8, rotate: Rotate) {
        self.visited_map[rotate as usize].set(x, y);
    }

    pub fn found(&mut self, x: u8, y: u8, rotate: Rotate) {
        self.found_map[rotate as usize].set(x, y);
    }

    pub fn is_visited(&self, x: u8, y: u8, rotate: Rotate) -> bool {
        self.visited_map[rotate as usize].get(x, y)
    }

    pub fn is_found(&self, x: u8, y: u8, rotate: Rotate) -> bool {
        self.found_map[rotate as usize].get(x, y)
    }

    pub fn reset_trail(&mut self) {
        for memory in self.visited_map.iter_mut() {
            memory.clear();
        }
    }
}
