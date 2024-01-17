use crate::sfinder_core::{field::field::Field, mino::mino::Mino};

pub trait Reachable {
    // checksを呼び出す前に、Field.cansPutの確認を必ずしていること
    fn checks(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
    ) -> bool;
}

pub trait ILockedReachable: Reachable {}
