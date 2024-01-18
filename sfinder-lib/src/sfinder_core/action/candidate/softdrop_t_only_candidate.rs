use super::{
    candidate::Candidate, harddrop_candidate::HarddropCandidate, locked_candidate::LockedCandidate,
};
use crate::{
    common::datastore::action::minimal_action::MinimalAction,
    sfinder_core::{field::field::Field, mino::piece::Piece},
};
use nohash::IntSet;

pub struct SoftdropTOnlyCandidate<'a> {
    harddrop_candidate: HarddropCandidate<'a>,
    locked_candidate: LockedCandidate<'a>,
}

impl<'a> SoftdropTOnlyCandidate<'a> {
    pub fn new(
        harddrop_candidate: HarddropCandidate<'a>,
        locked_candidate: LockedCandidate<'a>,
    ) -> Self {
        Self {
            harddrop_candidate,
            locked_candidate,
        }
    }
}

impl Candidate for SoftdropTOnlyCandidate<'_> {
    fn search(
        &mut self,
        field: &dyn Field,
        piece: Piece,
        valid_height: u8,
    ) -> IntSet<MinimalAction> {
        if piece == Piece::T {
            self.locked_candidate.search(field, piece, valid_height)
        } else {
            self.harddrop_candidate.search(field, piece, valid_height)
        }
    }
}
