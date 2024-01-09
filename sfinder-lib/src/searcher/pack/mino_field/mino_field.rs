use crate::{
    common::datastore::{
        mino_operation_with_key::MinoOperationWithKey, piece_counter::PieceCounter,
    },
    searcher::pack::separable_mino::separable_mino::SeparableMino,
    sfinder_core::column_field::column_small_field::ColumnSmallField,
};
use std::iter::FusedIterator;

pub trait MinoField: Traversable {
    fn get_outer_field(&self) -> ColumnSmallField;

    fn get_piece_counter(&self) -> PieceCounter;

    fn get_max_index(&self) -> u8;

    // Porting note: extra structs are used to avoid using return position impl Trait in a trait method
    fn get_separable_mino_stream(&self) -> SeparableMinoTraverser
    where
        Self: Sized,
    {
        SeparableMinoTraverser {
            current: Some(self),
        }
    }

    fn get_operations_stream(&self) -> MinoOperationWithKeyTraverser
    where
        Self: Sized,
    {
        MinoOperationWithKeyTraverser {
            current: self.get_separable_mino_stream(),
        }
    }
}

pub trait Traversable {
    fn get_separable_mino(&self) -> &dyn SeparableMino;

    // TODO: figure out the structuere of the links. Is it just a linked list, or a tree?
    fn get_next(&self) -> Option<&dyn MinoField>;
}

pub struct SeparableMinoTraverser<'a> {
    current: Option<&'a dyn MinoField>,
}

impl<'a> Iterator for SeparableMinoTraverser<'a> {
    type Item = &'a dyn SeparableMino;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.current?;
        self.current = prev.get_next();
        Some(prev.get_separable_mino())
    }
}

impl FusedIterator for SeparableMinoTraverser<'_> {}

pub struct MinoOperationWithKeyTraverser<'a> {
    current: SeparableMinoTraverser<'a>,
}

impl<'a> Iterator for MinoOperationWithKeyTraverser<'a> {
    // returns the subtrait MinosOperationWithKey to avoid upcasting to OperationWithKey
    type Item = &'a dyn MinoOperationWithKey;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.current.next()?.get_mino_operation_with_key())
    }
}
