/// Porting note: changed enum naming to make the rotation more explicit
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RotateDirection {
    Clockwise,
    CounterClockwise,
    Rotate180,
}

const VALUES_NO_180: [RotateDirection; 2] = [
    RotateDirection::Clockwise,
    RotateDirection::CounterClockwise,
];

const VALUES_WITH_180: [RotateDirection; 3] = [
    RotateDirection::Clockwise,
    RotateDirection::CounterClockwise,
    RotateDirection::Rotate180,
];

impl RotateDirection {
    pub fn reverse(self) -> Self {
        match self {
            RotateDirection::Clockwise => RotateDirection::CounterClockwise,
            RotateDirection::CounterClockwise => RotateDirection::Clockwise,
            RotateDirection::Rotate180 => RotateDirection::Rotate180,
        }
    }

    pub fn values_no_180() -> &'static [RotateDirection] {
        &VALUES_NO_180
    }

    pub fn values_with_180() -> &'static [RotateDirection] {
        &VALUES_WITH_180
    }
}
