use crate::{common::datastore::coordinate::Coordinate, extras::hash_code::HashCode};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    // テストパターンごとに、ミノの移動量を表す配列（[x, y]）
    offsets: Vec<Coordinate>,
    // テストパターンごとで、スピンをMiniからRegularに昇格するパターンを`true`で表す配列
    privilege_spins: Vec<bool>,
}

impl Pattern {
    // Porting note: replaces noPrivilegeSpins
    pub fn with_no_privilege_spins(offsets: Vec<Coordinate>) -> Self {
        let len = offsets.len();
        Self {
            offsets,
            privilege_spins: vec![false; len],
        }
    }

    pub fn new(offsets: Vec<Coordinate>, privilege_spins: Vec<bool>) -> Self {
        assert_eq!(offsets.len(), privilege_spins.len());
        Self {
            offsets,
            privilege_spins,
        }
    }

    pub fn get_offsets(&self) -> &[Coordinate] {
        &self.offsets
    }

    pub fn is_privilege_spins_at(&self, index: u8) -> bool {
        self.privilege_spins[index as usize]
    }
}

impl HashCode for Pattern {
    type Output = u64;

    fn hash_code(&self) -> Self::Output {
        todo!("I don't think this hash is used")
    }
}
