use crate::sfinder_core::{
    field::{field::Field, field_factory, middle_field::MiddleField},
    mino::mino::Mino,
};

use super::mino_mask::MinoMask;

pub struct MiddleMinoMask {
    low: u64,
    high: u64,
    x: u8,
}

impl MiddleMinoMask {
    pub fn new(mino: &Mino, y: u8, delete_key: u64) -> Self {
        let mut field = field_factory::create_middle_field();

        let x = u8::try_from(-mino.get_min_x()).unwrap();
        field.put(mino, x, y);
        field.insert_blank_row_with_key(delete_key);

        let (low, high) = field.into();

        Self { low, high, x }
    }
}

impl MinoMask for MiddleMinoMask {
    fn get_mino_mask(&self, x: u8) -> Box<dyn Field> {
        assert!(self.x <= x);
        let slide = x - self.x;

        Box::new(MiddleField::from_parts(
            self.low << slide,
            self.high << slide,
        ))
    }
}
