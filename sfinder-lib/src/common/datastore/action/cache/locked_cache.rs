use super::memory::{array_memory::ArrayMemory, memory::Memory};
use crate::{
    common::datastore::action::cache::memory::field_memory::FieldMemory,
    sfinder_core::{
        field::{
            field_constants::BOARD_HEIGHT, large_field::LargeField, middle_field::MiddleField,
            small_field::SmallField,
        },
        srs::rotate::Rotate,
    },
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

    // TODO: I think this is the only place FieldMemory is created, so the Field implementations have to be imported here
    fn create_memory(height: u8) -> Box<dyn Memory> {
        const MIDDLE_HEIGHT: u8 = BOARD_HEIGHT * 2;
        const LARGE_HEIGHT: u8 = BOARD_HEIGHT * 4;

        match height {
            ..=BOARD_HEIGHT => Box::new(FieldMemory::<SmallField>::new()),
            ..=MIDDLE_HEIGHT => Box::new(FieldMemory::<MiddleField>::new()),
            ..=LARGE_HEIGHT => Box::new(FieldMemory::<LargeField>::new()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_core::field::field_constants::FIELD_WIDTH;

    fn assert_cache(height: u8) {
        let mut cache = LockedCache::new(height);

        for y in 0..height {
            for x in 0..FIELD_WIDTH {
                for &rotate in Rotate::value_list() {
                    assert!(!cache.is_visited(x, y, rotate));
                    cache.visit(x, y, rotate);
                    assert!(cache.is_visited(x, y, rotate));
                    assert!(!cache.is_found(x, y, rotate));
                    cache.found(x, y, rotate);
                    assert!(cache.is_found(x, y, rotate));
                }
            }
        }

        cache.clear();

        for y in 0..height {
            for x in 0..FIELD_WIDTH {
                for &rotate in Rotate::value_list() {
                    assert!(!cache.is_visited(x, y, rotate));
                    assert!(!cache.is_found(x, y, rotate));
                }
            }
        }
    }

    #[test]
    fn test_locked_cache_6() {
        assert_cache(6);
    }

    #[test]
    fn test_locked_cache_12() {
        assert_cache(12);
    }

    #[test]
    fn test_locked_cache_24() {
        assert_cache(24);
    }

    #[test]
    fn test_locked_cache_48() {
        assert_cache(48);
    }
}
