//! Helper struct used only Field4x10MinoPackingHelper
//! Porting note: moved from Field4x10MinoPackingHelper

use super::mino_field::{MinoField, SeparableMinoTraverser, Traversable};
use crate::{
    common::datastore::{
        full_operation_with_key::FullOperationWithKey,
        mino_operation_with_key::MinoOperationWithKey, piece_counter::PieceCounter,
    },
    searcher::pack::separable_mino::separable_mino::SeparableMino,
    sfinder_core::{
        column_field::column_small_field::ColumnSmallField,
        field::field::Field,
        mino::{mino_factory::MinoFactory, piece::Piece},
        srs::rotate::Rotate,
    },
};

const LAST_OPERATION: FullOperationWithKey = FullOperationWithKey::new(
    MinoFactory::new().get(Piece::I, Rotate::Spawn),
    0,
    0,
    0b1000000000100000000010000000001,
    0,
);
const LAST_SEPARABLE_MINO: LastSeparableMino = LastSeparableMino {};
const EMPTY_COLUMN_FIELD: ColumnSmallField = ColumnSmallField::new();

struct LastSeparableMino {}

impl SeparableMino for LastSeparableMino {
    fn get_lower_y(&self) -> u8 {
        0
    }

    fn get_mino_operation_with_key(&self) -> &dyn MinoOperationWithKey {
        &LAST_OPERATION
    }

    fn get_column_field(&self) -> &ColumnSmallField {
        unimplemented!()
    }

    fn get_field(&self) -> &dyn Field {
        unimplemented!()
    }
}

pub struct IOnlyMinoField {}

impl MinoField for IOnlyMinoField {
    fn get_outer_field(&self) -> &ColumnSmallField {
        &EMPTY_COLUMN_FIELD
    }

    fn get_piece_counter(&self) -> PieceCounter {
        PieceCounter::with_single_piece(Piece::I)
    }

    fn get_max_separable_mino(&self) -> &dyn SeparableMino {
        unimplemented!()
    }

    fn get_separable_mino_stream(&self) -> SeparableMinoTraverser {
        SeparableMinoTraverser::new(self)
    }
}

impl Traversable for IOnlyMinoField {
    fn get_separable_mino(&self) -> &dyn SeparableMino {
        &LAST_SEPARABLE_MINO
    }

    fn get_next(&self) -> Option<&dyn MinoField> {
        None
    }
}
