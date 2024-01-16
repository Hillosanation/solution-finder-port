use crate::{common::datastore::coordinate::Coordinate, extras::hash_code::HashCode};

#[deprecated]
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    offsets: Vec<Coordinate>,
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

    // Used in MinoRotation implementors
    pub fn get_offsets(&self) -> &[Coordinate] {
        &self.offsets
    }

    // Used in MinoRotation implementors
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

#[derive(Debug, Clone, PartialEq)]
pub struct _Pattern {
    // offset: テストパターンごとに、ミノの移動量を表す配列（[x, y]）
    // is_privilege_spins: テストパターンごとで、スピンをMiniからRegularに昇格するパターンを`true`で表す配列
    checks: Vec<(Coordinate, bool)>,
}

impl _Pattern {
    // Porting note: replaces noPrivilegeSpins
    pub fn with_no_privilege_spins(offsets: Vec<Coordinate>) -> Self {
        Self {
            checks: offsets.into_iter().map(|offset| (offset, false)).collect(),
        }
    }

    pub fn new(checks: Vec<(Coordinate, bool)>) -> Self {
        Self { checks }
    }

    pub fn get_offsets(&self) -> impl Iterator<Item = &Coordinate> {
        self.checks.iter().map(|(offset, _)| offset)
    }

    // Porting note: replaces getPrivilegeSpinsAt
    // TODO: move MinoRotation functions down to Pattern
    pub fn get_checks(&self) -> &[(Coordinate, bool)] {
        &self.checks
    }
}

impl HashCode for _Pattern {
    type Output = u64;

    fn hash_code(&self) -> Self::Output {
        todo!("I don't think this hash is used")
    }
}
