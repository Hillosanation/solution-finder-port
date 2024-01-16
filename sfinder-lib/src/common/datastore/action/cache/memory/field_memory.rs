use super::memory::Memory;
use crate::sfinder_core::field::field::Field;

pub struct FieldMemory<F: Field>(F);

impl<F: Field> FieldMemory<F> {
    pub fn new() -> Self {
        Self(F::new())
    }
}

impl<F: Field> Memory for FieldMemory<F> {
    fn get(&self, x: u8, y: u8) -> bool {
        !self.0.is_empty_block(x, y)
    }

    fn set(&mut self, x: u8, y: u8) {
        self.0.set_block(x, y);
    }

    fn clear(&mut self) {
        self.0.clear_all();
    }
}
