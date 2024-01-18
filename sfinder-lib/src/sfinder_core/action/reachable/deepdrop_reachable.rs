//! Only used by SequenceFumenParser

use super::reachable::Reachable;
use crate::sfinder_core::{field::field::Field, mino::mino::Mino};

pub struct HarddropReachable {}

impl Reachable for HarddropReachable {
    fn checks(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
    ) -> bool {
        self.check(field, mino, x, y, valid_height)
    }

    fn check(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
    ) -> bool {
        // Porting note: this should already be guarenteed by the caller per Reachable
        debug_assert!(field.can_put(mino, x, y));

        y as i8 + mino.get_max_y() < valid_height as i8
    }
}
