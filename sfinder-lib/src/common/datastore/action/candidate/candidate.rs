use crate::{
    common::datastore::action::action::Action,
    sfinder_core::{field::field::Field, mino::piece::Piece},
};
use std::collections::HashSet;

pub trait Candidate<T = Box<dyn Action>> {
    fn search(&self, field: &dyn Field, piece: Piece, valid_height: u8) -> HashSet<T>;
}

pub trait ILockedCandidate: Candidate<Box<dyn Action>> {}
