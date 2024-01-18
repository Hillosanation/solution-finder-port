use crate::sfinder_core::srs::{rotate::Rotate, rotate_direction::RotateDirection};

#[derive(PartialEq, PartialOrd)]
pub struct Spin {
    cleared_rows: ClearedRows,
    spin: TSpins,
    name: TSpinNames,
}

impl Spin {
    pub fn new(spin: TSpins, name: TSpinNames, cleared_rows: u8) -> Self {
        debug_assert!(
            !((spin == TSpins::Regular && name == TSpinNames::Neo)
                || (spin == TSpins::Mini && matches!(name, TSpinNames::Iso | TSpinNames::Fin))),
            "invalid spin: spin={spin:?}, name={name:?}"
        );
        Self {
            spin,
            name,
            cleared_rows: ClearedRows::from(cleared_rows),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TSpins {
    Regular,
    Mini,
}

impl TSpins {
    pub fn new(filled_t_front: bool, is_privilege_spins: bool) -> Self {
        // 前提: Tスピンとなる条件（Tの隅に3つ以上ブロックが存在している）はこの時点で満たしている

        // Tの凸側のブロックが両方揃っている
        // or
        // TSTフォームのような特権がある場合はRegularと判定する
        // e.g. SRSでは「接着時にTが横向き and 回転テストパターンが最後のケース」の場合はRegular
        if filled_t_front || is_privilege_spins {
            TSpins::Regular
        } else {
            TSpins::Mini
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TSpinNames {
    NoName,
    Fin,
    Iso,
    Neo,
}

impl TSpinNames {
    pub fn new(
        to_rotate: Rotate,
        test_pattern_index: u8,
        filled_t_front: bool,
        direction: RotateDirection,
    ) -> Self {
        if (direction == RotateDirection::CounterClockwise && to_rotate == Rotate::Right)
            || (direction == RotateDirection::Clockwise && to_rotate == Rotate::Left)
        {
            match test_pattern_index {
                // 正面側に2つブロックがある
                3 if filled_t_front => TSpinNames::Iso,
                3 => TSpinNames::Neo,
                4 => TSpinNames::Fin,
                _ => TSpinNames::NoName,
            }
        } else {
            TSpinNames::NoName
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ClearedRows {
    Zero = 0,
    Single,
    Double,
    Triple,
}

impl From<u8> for ClearedRows {
    fn from(value: u8) -> Self {
        match value {
            0 => ClearedRows::Zero,
            1 => ClearedRows::Single,
            2 => ClearedRows::Double,
            3 => ClearedRows::Triple,
            _ => panic!("invalid number of cleared rows: {value}"),
        }
    }
}
