use crate::{common::datastore::coordinate::Coordinate, extras::hash_code::HashCode};

const PATTERN_SIZE: usize = 5;

#[derive(PartialEq)]
pub struct Pattern {
    // テストパターンごとに、ミノの移動量を表す配列（[x, y]）
    offsets: [Coordinate; PATTERN_SIZE],
    // テストパターンごとで、スピンをMiniからRegularに昇格するパターンを`true`で表す配列
    privilege_spins: [bool; PATTERN_SIZE],
}

impl Pattern {
    // Porting note: replaces noPrivilegeSpins
    pub const fn with_no_privilege_spins(offsets: [Coordinate; PATTERN_SIZE]) -> Self {
        Self {
            offsets,
            privilege_spins: [false; PATTERN_SIZE],
        }
    }

    pub const fn new(
        offsets: [Coordinate; PATTERN_SIZE],
        privilege_spins: [bool; PATTERN_SIZE],
    ) -> Self {
        Self {
            offsets,
            privilege_spins,
        }
    }

    pub const fn get_offsets(&self) -> &[Coordinate; 5] {
        &self.offsets
    }

    pub const fn is_privilege_spins_at(&self, index: u8) -> bool {
        self.privilege_spins[index as usize]
    }
}

impl HashCode for Pattern {
    type Output = u64;

    fn hash_code(&self) -> Self::Output {
        todo!("I don't think this hash is used")
    }
}
