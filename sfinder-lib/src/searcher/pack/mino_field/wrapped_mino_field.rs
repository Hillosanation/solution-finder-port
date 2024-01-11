use super::mino_field::{MinoField, SeparableMinoTraverser, Traversable};
use crate::{
    common::datastore::piece_counter::PieceCounter,
    searcher::pack::{
        separable_mino::separable_mino::SeparableMino, separable_minos::SeparableMinos,
    },
    sfinder_core::column_field::column_small_field::ColumnSmallField,
};

pub struct WrappedMinoField<'a> {
    // Porting note: mino_field needs to be borrowed to allow max_separable_mino to reference it while not being self-referential
    mino_field: &'a dyn MinoField,
    outer_field: ColumnSmallField,
    separable_mino: &'a dyn SeparableMino,
    max_separable_mino: &'a dyn SeparableMino, // hmm, self-referentials again?
}

impl<'a> WrappedMinoField<'a> {
    pub fn new(
        separable_mino: &'a dyn SeparableMino,
        mino_field: &'a dyn MinoField,
        outer_field: ColumnSmallField,
    ) -> Self {
        let prev_max = mino_field.get_max_separable_mino();

        let max = match SeparableMinos::compare_index(separable_mino, prev_max) {
            std::cmp::Ordering::Less => prev_max,
            _ => separable_mino,
        };

        Self {
            mino_field,
            outer_field,
            separable_mino,
            max_separable_mino: max,
        }
    }
}

impl MinoField for WrappedMinoField<'_> {
    fn get_outer_field(&self) -> &ColumnSmallField {
        &self.outer_field
    }

    fn get_piece_counter(&self) -> PieceCounter {
        self.mino_field.get_piece_counter().add_piece(
            self.separable_mino
                .get_mino_operation_with_key()
                .get_piece(),
        )
    }

    fn get_max_separable_mino(&self) -> &dyn SeparableMino {
        self.max_separable_mino
    }

    fn get_separable_mino_stream(&self) -> SeparableMinoTraverser {
        SeparableMinoTraverser::new(self)
    }
}

impl Traversable for WrappedMinoField<'_> {
    fn get_separable_mino(&self) -> &dyn SeparableMino {
        self.separable_mino
    }

    fn get_next(&self) -> Option<&dyn MinoField> {
        Some(self.mino_field)
    }
}
