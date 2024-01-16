use super::{
    mino_rotation::MinoRotation, pattern::Pattern, rotate::Rotate,
    rotate_direction::RotateDirection,
};
use crate::sfinder_core::mino::piece::Piece;

pub struct MinoRotationNo180Impl {
    cw_map: [Pattern; Piece::get_size() * Rotate::get_size()],
    ccw_map: [Pattern; Piece::get_size() * Rotate::get_size()],
}

impl MinoRotationNo180Impl {
    pub fn new(
        cw_map: [Pattern; Piece::get_size() * Rotate::get_size()],
        ccw_map: [Pattern; Piece::get_size() * Rotate::get_size()],
    ) -> Self {
        Self { cw_map, ccw_map }
    }
}

impl MinoRotation for MinoRotationNo180Impl {
    fn get_map(&self, direction: RotateDirection) -> &[Pattern] {
        match direction {
            RotateDirection::Clockwise => &self.cw_map,
            RotateDirection::CounterClockwise => &self.ccw_map,
            RotateDirection::Rotate180 => panic!("180 rotation is not supported"),
        }
    }

    fn supports_180(&self) -> bool {
        false
    }
}
