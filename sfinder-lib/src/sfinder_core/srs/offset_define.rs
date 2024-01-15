//! Kick table defined for SRS.

use crate::{common::datastore::coordinate::Coordinate, sfinder_core::mino::piece::Piece};

use super::{offset::Offset, pattern::Pattern, rotate::Rotate};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OffsetType {
    I,
    O,
    T,
    Other,
}

impl From<Piece> for OffsetType {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::I => Self::I,
            Piece::O => Self::O,
            Piece::T => Self::T,
            _ => Self::Other,
        }
    }
}

#[rustfmt::skip]
fn get_rotate_map(offset_type: OffsetType) -> [Offset; Rotate::get_size()] {
    match offset_type {
        OffsetType::I => [
            Offset::new(vec![Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(2, 0), Coordinate::new(-1, 0), Coordinate::new(2, 0)]),
            Offset::new(vec![Coordinate::new(-1, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, -2)]),
            Offset::new(vec![Coordinate::new(-1, 1), Coordinate::new(1, 1), Coordinate::new(-2, 1), Coordinate::new(1, 0), Coordinate::new(-2, 0)]),
            Offset::new(vec![Coordinate::new(0, 1), Coordinate::new(0, 1), Coordinate::new(0, 1), Coordinate::new(0, -1), Coordinate::new(0, 2)]),
        ],
        OffsetType::O => [
            Offset::new(vec![Coordinate::new(0, 0)]),
            Offset::new(vec![Coordinate::new(0, -1)]),
            Offset::new(vec![Coordinate::new(-1, -1)]),
            Offset::new(vec![Coordinate::new(-1, 0)]),
        ],
        OffsetType::T => [
            Offset::new(vec![Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0)]),
            Offset::new(vec![Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(1, -1), Coordinate::new(0, 2), Coordinate::new(1, 2)]),
            Offset::new(vec![Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0)]),
            Offset::new(vec![Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(-1, -1), Coordinate::new(0, 2), Coordinate::new(-1, 2)]),
        ],
        OffsetType::Other => [
            Offset::new(vec![Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0)]),
            Offset::new(vec![Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(1, -1), Coordinate::new(0, 2), Coordinate::new(1, 2)]),
            Offset::new(vec![Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0)]),
            Offset::new(vec![Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(-1, -1), Coordinate::new(0, 2), Coordinate::new(-1, 2)]),
        ],
    }
}

// Porting note: replaces getPattern
pub fn create_pattern(offset_type: OffsetType, current: Rotate, next: Rotate) -> Pattern {
    let rotate_map = get_rotate_map(offset_type);

    // SRSでは、TSTフォームはRegular判定にする
    if offset_type == OffsetType::T && matches!(next, Rotate::Right | Rotate::Left) {
        // 「接着時にTが横向き and 回転テストパターンが最後(4)のケース」を格上げする
        rotate_map[current as usize].to_pattern_with_privilege_spin(&rotate_map[next as usize], 4)
    } else {
        rotate_map[current as usize].to_pattern(&rotate_map[next as usize])
    }
}
