//! A wrapped function to filter a MinoFields struct.

use super::mino_fields::MinoFields;
use crate::searcher::pack::{
    memento::solution_filter::SolutionFilter, mino_field::mino_field::MinoField,
};

// TODO: This struct should be deprecated in favor of using .filter() on iterators directly.
pub struct FilteredMinoFields {
    mino_fields: Box<dyn MinoFields>,
    filter: Box<dyn SolutionFilter>,
}

impl FilteredMinoFields {
    pub fn new(
        mino_fields: Box<dyn MinoFields>,
        filter: Box<dyn SolutionFilter>,
    ) -> FilteredMinoFields {
        FilteredMinoFields {
            mino_fields,
            filter,
        }
    }
}

impl MinoFields for FilteredMinoFields {
    fn stream(&self) -> Box<dyn Iterator<Item = Box<dyn MinoField + '_>> + '_> {
        Box::new(
            self.mino_fields
                .stream()
                .filter(|mino_field| self.filter.test_mino_field(mino_field.as_ref())),
        )
    }
}
