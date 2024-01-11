use super::solutions_calculator::SolutionsCalculator;
use crate::{
    extras::callable::Callable,
    searcher::pack::mino_field::recursive_mino_field::RecursiveMinoField,
    sfinder_core::column_field::column_small_field::ColumnSmallField,
};

pub struct ConnectionsToStreamCallable {}

impl ConnectionsToStreamCallable {
    pub fn new(
        calculator: Box<dyn SolutionsCalculator>,
        init_column_field: ColumnSmallField,
        outer_column_field: ColumnSmallField,
        limit_outer_field: ColumnSmallField,
    ) -> Self {
        todo!()
    }
}

impl<'a> Callable<Box<dyn Iterator<Item = RecursiveMinoField<'a>>>>
    for ConnectionsToStreamCallable
{
    fn call(&self) -> Box<dyn Iterator<Item = RecursiveMinoField<'a>>> {
        todo!()
    }
}
