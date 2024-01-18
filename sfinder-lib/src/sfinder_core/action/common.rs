//! Refactors common logic used in Candidate and Reachable
//! TODO: use interior mutability to avoid &mut self in checks/search?
//! TODO: replace early returns with chaining boolean expressions in Candidate/Reachable

use crate::sfinder_core::{
    field::{field::Field, field_constants::FIELD_WIDTH},
    mino::mino::Mino,
};

#[derive(Debug, PartialEq)]
pub enum FromDirection {
    None,
    Left,
    Right,
}

pub fn can_put_mino_in_field(field: &dyn Field, mino: &Mino, x: u8, y: u8) -> bool {
    -mino.get_min_x() as u8 <= x
        && x < FIELD_WIDTH - mino.get_max_x() as u8
        && -mino.get_min_y() as u8 <= y
        // casts guarded by previous checks
        && field.can_put(mino, x, y)
}
