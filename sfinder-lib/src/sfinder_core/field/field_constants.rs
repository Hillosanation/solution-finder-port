//! Constants that are commonly used when performing operations related to a field.

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BoardCount {
    Small = 1,
    Middle = 2,
    Large = 4,
}

pub const FIELD_WIDTH: u8 = 10;
pub const BOARD_HEIGHT: u8 = 6;
pub const VALID_BOARD_RANGE: u64 = (1 << (FIELD_WIDTH * BOARD_HEIGHT)) - 1;
