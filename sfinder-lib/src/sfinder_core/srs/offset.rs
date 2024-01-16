use super::pattern::_Pattern;
use crate::common::datastore::coordinate::Coordinate;

pub struct Offset {
    offsets: Vec<Coordinate>,
}

impl Offset {
    pub fn new(offsets: Vec<Coordinate>) -> Self {
        Self { offsets }
    }

    pub fn to_pattern(&self, other: &Self) -> _Pattern {
        _Pattern::with_no_privilege_spins(self.create_pattern_array(other))
    }

    pub fn to_pattern_with_privilege_spin(
        &self,
        other: &Self,
        privilege_spin_index: u8,
    ) -> _Pattern {
        _Pattern::new(
            self.create_pattern_array(other)
                .into_iter()
                .enumerate()
                .map(|(i, offset)| (offset, i == privilege_spin_index as usize))
                .collect(),
        )
    }

    fn create_pattern_array(&self, other: &Self) -> Vec<Coordinate> {
        self.offsets
            .iter()
            .zip(other.offsets.iter())
            .map(|(a, b)| Coordinate::new(a.x - b.x, a.y - b.y))
            .collect()
    }
}
