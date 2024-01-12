//! Helper struct used only by SpinMaskFields

use crate::sfinder_core::field::field::Field;

// Porting note: Made struct public for ergonomics, this stores a result, so it shouldn't be mutated
pub struct MaskField {
    pub rest: Box<dyn Field>,
    pub not_allowed: Box<dyn Field>,
}

impl MaskField {
    pub fn new(rest: Box<dyn Field>, not_allowed: Box<dyn Field>) -> Self {
        Self { rest, not_allowed }
    }
}
