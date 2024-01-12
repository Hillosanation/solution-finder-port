use crate::sfinder_core::mino::piece::Piece;
use std::fmt::Display;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum ColorType {
    Empty = 0,
    I,
    L,
    O,
    Z,
    T,
    J,
    S,
    Gray,
}

const VALUE_LIST: [ColorType; 9] = [
    ColorType::Empty,
    ColorType::I,
    ColorType::L,
    ColorType::O,
    ColorType::Z,
    ColorType::T,
    ColorType::J,
    ColorType::S,
    ColorType::Gray,
];

impl ColorType {
    // Porting note: cast to u8 for getNumber

    // Panics if value is invalid.
    pub fn new(value: u8) -> ColorType {
        VALUE_LIST[value as usize]
    }

    pub fn is_mino_block(&self) -> bool {
        !matches!(self, ColorType::Empty | ColorType::Gray)
    }
}

impl TryFrom<u8> for ColorType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        VALUE_LIST
            .get(value as usize)
            .copied()
            .ok_or("Invalid ColorType value".to_owned())
    }
}

impl From<Piece> for ColorType {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::I => ColorType::I,
            Piece::L => ColorType::L,
            Piece::O => ColorType::O,
            Piece::Z => ColorType::Z,
            Piece::T => ColorType::T,
            Piece::J => ColorType::J,
            Piece::S => ColorType::S,
        }
    }
}

impl TryFrom<ColorType> for Piece {
    type Error = String;

    fn try_from(value: ColorType) -> Result<Self, Self::Error> {
        match value {
            ColorType::I => Ok(Piece::I),
            ColorType::L => Ok(Piece::L),
            ColorType::O => Ok(Piece::O),
            ColorType::Z => Ok(Piece::Z),
            ColorType::T => Ok(Piece::T),
            ColorType::J => Ok(Piece::J),
            ColorType::S => Ok(Piece::S),
            _ => Err("Cannot convert this color to Piece".to_owned()),
        }
    }
}

// Porting note: moved from ColoredFieldView
impl Display for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            ColorType::Empty => '_',
            ColorType::I => 'I',
            ColorType::L => 'L',
            ColorType::O => 'O',
            ColorType::Z => 'Z',
            ColorType::T => 'T',
            ColorType::J => 'J',
            ColorType::S => 'S',
            ColorType::Gray => 'X',
        };
        write!(f, "{c}")
    }
}
