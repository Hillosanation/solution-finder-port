use crate::sfinder_core::{
    action::reachable::reachable_facade,
    field::field::Field,
    mino::{mino::Mino, mino_factory::MinoFactory, mino_shifter::MinoShifter},
    srs::mino_rotation::MinoRotation,
};

use super::reachable::{ILockedReachable, Reachable, ReachableForCover};

pub struct LastSoftdropReachable<'a> {
    reachable: Box<dyn Reachable>,
    locked_reachable: Box<dyn ILockedReachable + 'a>,
    allow_depth: u8,
}

impl<'a> LastSoftdropReachable<'a> {
    pub fn new(
        mino_factory: &'a MinoFactory,
        mino_shifter: &'a MinoShifter,
        reachable: Box<dyn Reachable>,
        mino_rotation: &'a dyn MinoRotation,
        max_y: u8,
        allow_depth: u8,
        use_180_rotation: bool,
    ) -> Self {
        assert_ne!(allow_depth, 0);

        Self {
            reachable,
            locked_reachable: reachable_facade::create_locked(
                mino_factory,
                mino_shifter,
                mino_rotation,
                max_y,
                use_180_rotation,
            ),
            allow_depth,
        }
    }
}

impl ReachableForCover for LastSoftdropReachable<'_> {
    fn checks(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
        remaining_depth: u8,
    ) -> bool {
        if remaining_depth <= self.allow_depth {
            self.locked_reachable
                .checks(field, mino, x, y, valid_height)
        } else {
            self.reachable.checks(field, mino, x, y, valid_height)
        }
    }
}
