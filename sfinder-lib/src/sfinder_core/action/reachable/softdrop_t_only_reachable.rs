use super::{
    harddrop_reachable::HarddropReachable,
    reachable::{ILockedReachable, Reachable},
    reachable_facade,
};
use crate::sfinder_core::{
    field::field::Field,
    mino::{mino::Mino, mino_factory::MinoFactory, mino_shifter::MinoShifter, piece::Piece},
    srs::mino_rotation::MinoRotation,
};

pub struct SoftdropTOnlyReachable<'a> {
    harddrop_reachable: HarddropReachable,
    locked_reachable: Box<dyn ILockedReachable + 'a>,
}

impl<'a> SoftdropTOnlyReachable<'a> {
    pub fn new(
        mino_factory: &'a MinoFactory,
        mino_shifter: &'a MinoShifter,
        mino_rotation: &'a dyn MinoRotation,
        max_y: u8,
        use_180_rotation: bool,
    ) -> Self {
        Self {
            harddrop_reachable: HarddropReachable::new(max_y),
            locked_reachable: reachable_facade::create_locked(
                mino_factory,
                mino_shifter,
                mino_rotation,
                max_y,
                use_180_rotation,
            ),
        }
    }
}

impl Reachable for SoftdropTOnlyReachable<'_> {
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
        if mino.get_piece() == Piece::T {
            self.locked_reachable.check(field, mino, x, y, valid_height)
        } else {
            self.harddrop_reachable
                .check(field, mino, x, y, valid_height)
        }
    }
}
