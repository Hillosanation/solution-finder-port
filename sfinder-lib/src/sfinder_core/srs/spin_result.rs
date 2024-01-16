use super::{rotate::Rotate, rotate_direction::RotateDirection};
use crate::sfinder_core::mino::mino::Mino;

// Porting note: replaces SuccessSpinResult, Option<T> is used instead of NoneSpinResult
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct SpinResult {
    after: &'static Mino, // only Rotate is used
    pub x: u8,
    pub y: u8,
    pub test_pattern_index: u8,
    pub direction: RotateDirection,
    pub is_privilege_spins: bool,
}

impl SpinResult {
    pub fn new(
        after: &'static Mino,
        x: u8,
        y: u8,
        test_pattern_index: u8,
        direction: RotateDirection,
        is_privilege_spins: bool,
    ) -> Self {
        Self {
            after,
            x,
            y,
            test_pattern_index,
            direction,
            is_privilege_spins,
        }
    }

    pub fn get_to_rotate(&self) -> Rotate {
        self.after.get_rotate()
    }
}
