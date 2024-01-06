use crate::sfinder_core::field::field::Field;

pub trait MinoMask {
    fn get_mino_mask(&self, x: u8) -> Box<dyn Field>;
}
