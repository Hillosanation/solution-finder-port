//! See FilteredMinoFields

use super::recursive_mino_fields::RecursiveMinoFields;
use crate::searcher::pack::{
    memento::solution_filter::SolutionFilter, mino_field::recursive_mino_field::RecursiveMinoField,
};

pub struct FilteredRecursiveMinoFields {
    mino_fields: Box<dyn RecursiveMinoFields>,
    filter: Box<dyn SolutionFilter>,
}

impl FilteredRecursiveMinoFields {
    pub fn new(
        mino_fields: Box<dyn RecursiveMinoFields>,
        filter: Box<dyn SolutionFilter>,
    ) -> FilteredRecursiveMinoFields {
        FilteredRecursiveMinoFields {
            mino_fields,
            filter,
        }
    }
}

impl RecursiveMinoFields for FilteredRecursiveMinoFields {
    fn recursive_stream(&self) -> Box<dyn Iterator<Item = RecursiveMinoField> + '_> {
        Box::new(
            self.mino_fields
                .recursive_stream()
                .filter(|mino_field| self.filter.test_mino_field(mino_field)),
        )
    }
}
