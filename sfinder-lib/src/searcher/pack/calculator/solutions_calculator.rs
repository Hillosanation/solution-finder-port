use crate::{
    searcher::pack::{
        mino_fields::recursive_mino_fields::RecursiveMinoFields, separable_minos::SeparableMinos,
    },
    sfinder_core::{column_field::column_small_field::ColumnSmallField, field::field::Field},
};

// TODO: check if ownership is necessary
pub trait SolutionsCalculator {
    fn get_height(&self) -> u8;

    fn is_filled(&self, column_field: &ColumnSmallField) -> bool;

    fn get_connections(&self, column_field: &ColumnSmallField) -> ColumnFieldConnections;

    fn get_inverted_outer_field(&self, outer_column_field: &ColumnSmallField) -> Box<dyn Field>;

    fn get_separable_minos(&self) -> SeparableMinos<'_>;

    fn get_recursive_mino_fields(&self) -> Box<dyn RecursiveMinoFields>;
}

struct ColumnFieldConnections {}
