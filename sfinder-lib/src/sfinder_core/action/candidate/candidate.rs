use crate::{
    common::datastore::action::minimal_action::MinimalAction,
    sfinder_core::{field::field::Field, mino::piece::Piece},
};
use nohash::IntSet;

pub trait Candidate<T = MinimalAction> {
    /// Porting note: Only canonical actions are returned in the set, but congruent actions are not equal to the canonical.
    fn search(&mut self, field: &dyn Field, piece: Piece, valid_height: u8) -> IntSet<T>;
}

pub trait ILockedCandidate: Candidate<MinimalAction> {}
