//! The items are guarenteed to be RecursiveMinoField, compared to MinoFields.

use super::mino_fields::MinoFields;
use crate::searcher::pack::mino_field::{
    mino_field::MinoField, recursive_mino_field::RecursiveMinoField,
};

pub trait RecursiveMinoFields {
    fn recursive_stream(&self) -> Box<dyn Iterator<Item = RecursiveMinoField> + '_>;
}

impl MinoFields for dyn RecursiveMinoFields + '_ {
    // Porting note: refactored implementation here, since all implementors directly call recursiveStream anyways
    // TODO: is this needed? Can I just use RecursiveMinoFields directly to avoid the boxing?
    fn stream(&self) -> Box<dyn Iterator<Item = Box<dyn MinoField + '_>> + '_> {
        Box::new(
            self.recursive_stream()
                .map(|mino_field| Box::new(mino_field) as _),
        )
    }
}
