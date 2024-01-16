use super::{
    mino_rotation::MinoRotation, pattern::Pattern, rotate::Rotate,
    rotate_direction::RotateDirection,
};
use crate::sfinder_core::mino::piece::Piece;

pub struct MinoRotationImpl {
    cw_map: [Pattern; Piece::get_size() * Rotate::get_size()],
    ccw_map: [Pattern; Piece::get_size() * Rotate::get_size()],
    rotate_180_map: [Pattern; Piece::get_size() * Rotate::get_size()],
}

impl MinoRotationImpl {
    pub fn new(
        cw_map: [Pattern; Piece::get_size() * Rotate::get_size()],
        ccw_map: [Pattern; Piece::get_size() * Rotate::get_size()],
        rotate_180_map: [Pattern; Piece::get_size() * Rotate::get_size()],
    ) -> Self {
        Self {
            cw_map,
            ccw_map,
            rotate_180_map,
        }
    }
}

impl MinoRotation for MinoRotationImpl {
    fn get_map(&self, direction: RotateDirection) -> &[Pattern] {
        match direction {
            RotateDirection::Clockwise => &self.cw_map,
            RotateDirection::CounterClockwise => &self.ccw_map,
            RotateDirection::Rotate180 => &self.rotate_180_map,
        }
    }

    fn supports_180(&self) -> bool {
        true
    }
}
