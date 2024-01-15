//! Kick table defined for SRS.

use crate::common::datastore::coordinate::Coordinate;

use super::{offset::Offset, pattern::Pattern, rotate::Rotate};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OffsetType {
    I,
    O,
    T,
    Other,
}

pub struct OffsetDefine {
    offset_type: OffsetType,
    rotate_map: [Offset; Rotate::get_size()],
}

impl OffsetDefine {
    pub fn new(offset_type: OffsetType) -> Self {
        let rotate_map = match offset_type {
            OffsetType::I => [
                Offset::new(vec![
                    Coordinate::new(0, 0),
                    Coordinate::new(-1, 0),
                    Coordinate::new(2, 0),
                    Coordinate::new(-1, 0),
                    Coordinate::new(2, 0),
                ]),
                Offset::new(vec![
                    Coordinate::new(-1, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 1),
                    Coordinate::new(0, -2),
                ]),
                Offset::new(vec![
                    Coordinate::new(-1, 1),
                    Coordinate::new(1, 1),
                    Coordinate::new(-2, 1),
                    Coordinate::new(1, 0),
                    Coordinate::new(-2, 0),
                ]),
                Offset::new(vec![
                    Coordinate::new(0, 1),
                    Coordinate::new(0, 1),
                    Coordinate::new(0, 1),
                    Coordinate::new(0, -1),
                    Coordinate::new(0, 2),
                ]),
            ],
            OffsetType::O => [
                Offset::new(vec![Coordinate::new(0, 0)]),
                Offset::new(vec![Coordinate::new(0, -1)]),
                Offset::new(vec![Coordinate::new(-1, -1)]),
                Offset::new(vec![Coordinate::new(-1, 0)]),
            ],
            OffsetType::T => [
                Offset::new(vec![
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                ]),
                Offset::new(vec![
                    Coordinate::new(0, 0),
                    Coordinate::new(1, 0),
                    Coordinate::new(1, -1),
                    Coordinate::new(0, 2),
                    Coordinate::new(1, 2),
                ]),
                Offset::new(vec![
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                ]),
                Offset::new(vec![
                    Coordinate::new(0, 0),
                    Coordinate::new(-1, 0),
                    Coordinate::new(-1, -1),
                    Coordinate::new(0, 2),
                    Coordinate::new(-1, 2),
                ]),
            ],
            OffsetType::Other => [
                Offset::new(vec![
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                ]),
                Offset::new(vec![
                    Coordinate::new(0, 0),
                    Coordinate::new(1, 0),
                    Coordinate::new(1, -1),
                    Coordinate::new(0, 2),
                    Coordinate::new(1, 2),
                ]),
                Offset::new(vec![
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                    Coordinate::new(0, 0),
                ]),
                Offset::new(vec![
                    Coordinate::new(0, 0),
                    Coordinate::new(-1, 0),
                    Coordinate::new(-1, -1),
                    Coordinate::new(0, 2),
                    Coordinate::new(-1, 2),
                ]),
            ],
        };

        Self {
            offset_type,
            rotate_map,
        }
    }

    // Porting note: replaces getPattern
    pub fn create_pattern(&self, current: Rotate, next: Rotate) -> Pattern {
        // SRSでは、TSTフォームはRegular判定にする
        if self.offset_type == OffsetType::T && matches!(next, Rotate::Right | Rotate::Left) {
            // 「接着時にTが横向き and 回転テストパターンが最後(4)のケース」を格上げする
            self.rotate_map[current as usize]
                .to_pattern_with_privilege_spin(&self.rotate_map[next as usize], 4)
        } else {
            self.rotate_map[current as usize].to_pattern(&self.rotate_map[next as usize])
        }
    }
}
