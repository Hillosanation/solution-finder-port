#[repr(u8)]
#[derive(Clone, Copy)]
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
