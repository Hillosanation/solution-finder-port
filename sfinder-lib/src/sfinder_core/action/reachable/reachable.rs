use crate::sfinder_core::{field::field::Field, mino::mino::Mino};

pub trait Reachable {
    // checksを呼び出す前に、Field.cansPutの確認を必ずしていること
    // this will be caught in debug mode
    fn checks(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
    ) -> bool;

    // Porting note: this is used when you want to check the exact action, instead of checking all congruent actions
    fn check(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
    ) -> bool;
}

pub trait ILockedReachable: Reachable {}

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
