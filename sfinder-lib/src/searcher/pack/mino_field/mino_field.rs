use crate::{
    common::datastore::operation_with_key::OperationWithKey,
    searcher::pack::separable_mino::separable_mino::SeparableMino,
    sfinder_core::column_field::column_small_field::ColumnSmallField,
};

pub trait MinoField {
    fn get_outer_field(&self) -> ColumnSmallField;

    fn get_piece_counter(&self) -> !;

    fn get_max_index(&self) -> u8;

    // fn get_operations_stream(&self) -> impl Iterator<Item = &dyn OperationWithKey>;

    // fn get_separable_mino_stream(&self) -> impl Iterator<Item = &dyn SeparableMino>;
}
