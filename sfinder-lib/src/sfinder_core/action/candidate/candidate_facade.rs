use super::{candidate::ILockedCandidate, locked_candidate::LockedCandidate};
use crate::sfinder_core::{
    action::candidate::locked_180_candidate::Locked180Candidate,
    mino::{mino_factory::MinoFactory, mino_shifter::MinoShifter},
    srs::mino_rotation::MinoRotation,
};

pub fn create_90_locked<'a>(
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a MinoShifter,
    mino_rotation: &'a dyn MinoRotation,
    max_y: u8,
) -> Box<dyn ILockedCandidate + 'a> {
    create_locked(mino_factory, mino_shifter, mino_rotation, max_y, false)
}

pub fn create_180_locked<'a>(
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a MinoShifter,
    mino_rotation: &'a dyn MinoRotation,
    max_y: u8,
) -> Box<dyn ILockedCandidate + 'a> {
    create_locked(mino_factory, mino_shifter, mino_rotation, max_y, true)
}

pub fn create_locked<'a>(
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a MinoShifter,
    mino_rotation: &'a dyn MinoRotation,
    max_y: u8,
    use_180_rotation: bool,
) -> Box<dyn ILockedCandidate + 'a> {
    if use_180_rotation {
        Box::new(Locked180Candidate::new(
            mino_factory,
            mino_shifter,
            mino_rotation,
            max_y,
        ))
    } else {
        Box::new(LockedCandidate::new(
            mino_factory,
            mino_shifter,
            mino_rotation,
            max_y,
        ))
    }
}
