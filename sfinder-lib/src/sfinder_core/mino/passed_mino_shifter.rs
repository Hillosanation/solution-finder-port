use super::piece::Piece;
use crate::{
    common::datastore::action::minimal_action::MinimalAction, sfinder_core::srs::rotate::Rotate,
};

pub struct PassedMinoShifter {}

impl PassedMinoShifter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_canonical_rotate(&self, _piece: Piece, rotate: Rotate) -> Rotate {
        rotate
    }

    pub fn create_canonical_action(
        &self,
        _piece: Piece,
        rotate: Rotate,
        x: u8,
        y: u8,
    ) -> MinimalAction {
        MinimalAction::new(x, y, rotate)
    }

    pub fn congruent_actions(
        &self,
        _piece: Piece,
        rotate: Rotate,
        x: u8,
        y: u8,
    ) -> Vec<MinimalAction> {
        vec![MinimalAction::new(x, y, rotate)]
    }

    pub fn get_unique_rotates(&self, piece: Piece) -> Vec<Rotate> {
        Rotate::value_list().to_vec()
    }
}
