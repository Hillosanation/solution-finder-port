//! A helper wrapper struct used by RecursiveMinoFieldMemento

use super::separable_mino::SeparableMino;
use crate::{
    common::datastore::mino_operation_with_key::MinoOperationWithKey,
    searcher::pack::slide_x_operation_with_key::SlideXOperationWithKey,
    sfinder_core::{column_field::column_small_field::ColumnSmallField, field::field::Field},
};

pub struct SlideXSeparableMino<'sm> {
    separable_mino: &'sm dyn SeparableMino,
    // Porting note: the original recalculates the operation, but this isn't really possible in Rust.
    // The other implementor of SeparableMino provides a borrowed reference to the operation, so we match this
    // behavior here.
    operation: SlideXOperationWithKey<'sm>,
}

impl<'a> SlideXSeparableMino<'a> {
    pub fn new(separable_mino: &'a dyn SeparableMino, slide_x: u8) -> Self {
        Self {
            separable_mino,
            operation: SlideXOperationWithKey::new(
                separable_mino.get_mino_operation_with_key(),
                slide_x,
            ),
        }
    }
}

impl<'a> SeparableMino for SlideXSeparableMino<'a> {
    fn get_lower_y(&self) -> u8 {
        self.separable_mino.get_lower_y()
    }

    fn get_mino_operation_with_key(&self) -> &dyn MinoOperationWithKey {
        &self.operation
    }

    fn get_column_field(&self) -> &ColumnSmallField {
        unimplemented!()
    }

    fn get_field(&self) -> &dyn Field {
        unimplemented!()
    }
}
