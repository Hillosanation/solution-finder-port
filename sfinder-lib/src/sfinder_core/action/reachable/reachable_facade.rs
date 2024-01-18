use crate::sfinder_core::{
    mino::{mino_factory::MinoFactory, mino_shifter::IMinoShifter},
    srs::mino_rotation::MinoRotation,
};

use super::{
    locked_180_reachable::Locked180Reachable, locked_reachable::LockedReachable,
    reachable::ILockedReachable,
};

pub fn create_90_locked<'a>(
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a dyn IMinoShifter,
    mino_rotation: &'a dyn MinoRotation,
    max_y: u8,
) -> Box<dyn ILockedReachable + 'a> {
    create_locked(mino_factory, mino_shifter, mino_rotation, max_y, false)
}

pub fn create_180_locked<'a>(
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a dyn IMinoShifter,
    mino_rotation: &'a dyn MinoRotation,
    max_y: u8,
) -> Box<dyn ILockedReachable + 'a> {
    create_locked(mino_factory, mino_shifter, mino_rotation, max_y, true)
}

pub fn create_locked<'a>(
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a dyn IMinoShifter,
    mino_rotation: &'a dyn MinoRotation,
    max_y: u8,
    use_180_rotation: bool,
) -> Box<dyn ILockedReachable + 'a> {
    if use_180_rotation {
        Box::new(Locked180Reachable::new(
            mino_factory,
            mino_shifter,
            mino_rotation,
            max_y,
        ))
    } else {
        Box::new(LockedReachable::new(
            mino_factory,
            mino_shifter,
            mino_rotation,
            max_y,
        ))
    }
}
