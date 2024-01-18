use crate::sfinder_core::{
    action::reachable::reachable::Reachable, field::field::Field, mino::mino::Mino,
};

pub trait ReachableForCover {
    fn checks(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
        remaining_depth: u8,
    ) -> bool;
}

// Porting note: replaces ReachableForCoverWrapper
impl ReachableForCover for dyn Reachable {
    fn checks(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
        _remaining_depth: u8,
    ) -> bool {
        <dyn Reachable>::checks(self, field, mino, x, y, valid_height)
    }
}
