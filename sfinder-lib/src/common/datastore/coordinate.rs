// TODO: use noHash when implementing Hash
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}

impl Coordinate {
    pub const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
}
