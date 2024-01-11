use super::mino_field::{MinoField, SeparableMinoTraverser, Traversable};
use crate::{
    common::datastore::piece_counter::PieceCounter,
    searcher::pack::{
        separable_mino::separable_mino::SeparableMino, separable_minos::SeparableMinos,
    },
    sfinder_core::column_field::column_small_field::ColumnSmallField,
};

pub struct RecursiveMinoField<'a> {
    separable_mino: &'a dyn SeparableMino,
    mino_field: Option<&'a dyn MinoField>,
    outer_field: ColumnSmallField,
    piece_counter: PieceCounter,
    max_separable_mino: &'a dyn SeparableMino,
}

impl<'a> RecursiveMinoField<'a> {
    pub fn new(
        separable_mino: &'a dyn SeparableMino,
        mino_field: Option<&'a dyn MinoField>,
        outer_field: ColumnSmallField,
    ) -> Self {
        let (max_separable_mino, piece_counter) = if let Some(prev_field) = mino_field {
            let prev_max = prev_field.get_max_separable_mino();

            (
                match SeparableMinos::compare_index(separable_mino, prev_max) {
                    std::cmp::Ordering::Less => prev_max,
                    _ => separable_mino,
                },
                prev_field
                    .get_piece_counter()
                    .add_piece(separable_mino.get_mino_operation_with_key().get_piece()),
            )
        } else {
            (
                separable_mino,
                PieceCounter::with_single_piece(
                    separable_mino.get_mino_operation_with_key().get_piece(),
                ),
            )
        };

        Self {
            separable_mino,
            mino_field,
            outer_field,
            piece_counter,
            max_separable_mino,
        }
    }
}

impl MinoField for RecursiveMinoField<'_> {
    fn get_outer_field(&self) -> &ColumnSmallField {
        &self.outer_field
    }

    fn get_piece_counter(&self) -> PieceCounter {
        self.piece_counter.clone()
    }

    fn get_max_separable_mino(&self) -> &dyn SeparableMino {
        self.max_separable_mino
    }

    fn get_separable_mino_stream(&self) -> SeparableMinoTraverser {
        SeparableMinoTraverser::new(self)
    }
}

impl Traversable for RecursiveMinoField<'_> {
    fn get_separable_mino(&self) -> &dyn SeparableMino {
        self.separable_mino
    }

    fn get_next(&self) -> Option<&dyn MinoField> {
        self.mino_field
    }
}

impl PartialEq for RecursiveMinoField<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.mino_field == other.mino_field
    }
}
