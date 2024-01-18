use crate::sfinder_core::{
    field::{
        large_field::{LargeField, MAX_FIELD_HEIGHT as LARGE_MAX},
        middle_field::{MiddleField, MAX_FIELD_HEIGHT as MIDDLE_MAX},
        small_field::{SmallField, MAX_FIELD_HEIGHT as SMALL_MAX},
    },
    srs::rotate::Rotate,
};

use super::memory::{array_memory::ArrayMemory, field_memory::FieldMemory, memory::Memory};

pub struct MinimalLockedCache {
    visited_map: [Box<dyn Memory>; Rotate::get_size()],
}

impl MinimalLockedCache {
    pub fn new(height: u8) -> Self {
        Self {
            visited_map: std::array::from_fn(|_| Self::create_memory(height)),
        }
    }

    // TODO: I think this is the only place FieldMemory is created, so the Field implementations have to be imported here
    fn create_memory(height: u8) -> Box<dyn Memory> {
        match height {
            ..=SMALL_MAX => Box::new(FieldMemory::<SmallField>::new()),
            ..=MIDDLE_MAX => Box::new(FieldMemory::<MiddleField>::new()),
            ..=LARGE_MAX => Box::new(FieldMemory::<LargeField>::new()),
            _ => Box::new(ArrayMemory::new(height)),
        }
    }

    pub fn clear(&mut self) {
        for memory in self.visited_map.iter_mut() {
            memory.clear();
        }
    }

    pub fn visit(&mut self, x: u8, y: u8, rotate: Rotate) {
        self.visited_map[rotate as usize].set(x, y);
    }

    pub fn is_visited(&self, x: u8, y: u8, rotate: Rotate) -> bool {
        self.visited_map[rotate as usize].get(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_core::field::field_constants::FIELD_WIDTH;

    fn assert_cache(height: u8) {
        let mut cache = MinimalLockedCache::new(height);

        for y in 0..height {
            for x in 0..FIELD_WIDTH {
                for &rotate in Rotate::value_list() {
                    assert!(!cache.is_visited(x, y, rotate));
                    cache.visit(x, y, rotate);
                    assert!(cache.is_visited(x, y, rotate));
                }
            }
        }

        cache.clear();

        for y in 0..height {
            for x in 0..FIELD_WIDTH {
                for &rotate in Rotate::value_list() {
                    assert!(!cache.is_visited(x, y, rotate));
                }
            }
        }
    }

    #[test]
    fn test_locked_cache6() {
        assert_cache(6);
    }

    #[test]
    fn test_locked_cache12() {
        assert_cache(12);
    }

    #[test]
    fn test_locked_cache24() {
        assert_cache(24);
    }

    #[test]
    fn test_locked_cache48() {
        assert_cache(48);
    }
}
