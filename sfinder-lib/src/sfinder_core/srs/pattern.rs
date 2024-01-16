use crate::{common::datastore::coordinate::Coordinate, extras::hash_code::HashCode};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    // each item is (offset, is_privilege_spins)
    // offset: テストパターンごとに、ミノの移動量を表す配列（[x, y]）
    // is_privilege_spins: テストパターンごとで、スピンをMiniからRegularに昇格するパターンを`true`で表す配列
    checks: Vec<(Coordinate, bool)>,
}

impl Pattern {
    // Porting note: replaces noPrivilegeSpins
    pub fn with_no_privilege_spins(offsets: Vec<Coordinate>) -> Self {
        Self {
            checks: offsets.into_iter().map(|offset| (offset, false)).collect(),
        }
    }

    pub fn new(checks: Vec<(Coordinate, bool)>) -> Self {
        Self { checks }
    }

    pub fn get_checks(&self) -> &[(Coordinate, bool)] {
        &self.checks
    }

    pub fn get_offsets(&self) -> impl Iterator<Item = &Coordinate> {
        self.checks.iter().map(|(offset, _)| offset)
    }

    pub fn is_privilege_spins_at(&self, index: u8) -> bool {
        self.checks[index as usize].1
    }
}

impl HashCode for Pattern {
    type Output = u64;

    fn hash_code(&self) -> Self::Output {
        todo!("I don't think this hash is used")
    }
}
