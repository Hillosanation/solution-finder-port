use super::mino_mask::MinoMask;
use crate::sfinder_core::{
    field::{field::Field, field_factory, small_field::SmallField},
    mino::mino::Mino,
};

pub struct SmallMinoMask {
    low: u64,
    x: u8,
}

impl SmallMinoMask {
    pub fn new(mino: &Mino, y: u8, delete_key: u64) -> Self {
        let mut field = field_factory::create_small_field();

        let x = u8::try_from(-mino.get_min_x()).unwrap();
        field.put(mino, x, y);
        field.insert_blank_row_with_key(delete_key);

        Self {
            x,
            low: field.into(),
        }
    }
}

impl MinoMask for SmallMinoMask {
    fn get_mino_mask(&self, x: u8) -> Box<dyn Field> {
        assert!(self.x <= x);
        let slide = x - self.x;

        Box::new(SmallField::from(self.low << slide))
    }
}
