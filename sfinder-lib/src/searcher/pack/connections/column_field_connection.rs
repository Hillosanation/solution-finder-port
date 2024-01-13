use crate::{
    searcher::pack::{separable_mino::separable_mino::SeparableMino, sized_bit::SizedBit},
    sfinder_core::column_field::{
        column_field::ColumnField, column_field_factory, column_small_field::ColumnSmallField,
    },
};

pub struct ColumnFieldConnection<'a> {
    mino: &'a dyn SeparableMino,
    inner_field: ColumnSmallField,
    outer_field: ColumnSmallField,
}

impl<'a> ColumnFieldConnection<'a> {
    pub fn new(
        mino: &'a dyn SeparableMino,
        freeze: &ColumnSmallField,
        sized_bit: &SizedBit,
    ) -> Self {
        assert!(sized_bit.height <= 10);
        let fill_board = sized_bit.fill_board;

        let board = freeze.get_board(0);
        let inner_field = column_field_factory::create_small_field_from_inner(board & fill_board);
        let outer_field = column_field_factory::create_small_field_from_inner(board & !fill_board);

        Self {
            mino,
            inner_field,
            outer_field,
        }
    }

    pub fn get_mino(&self) -> &dyn SeparableMino {
        self.mino
    }

    pub fn get_inner_field(&self) -> &ColumnSmallField {
        &self.inner_field
    }

    pub fn get_outer_field(&self) -> &ColumnSmallField {
        &self.outer_field
    }
}
