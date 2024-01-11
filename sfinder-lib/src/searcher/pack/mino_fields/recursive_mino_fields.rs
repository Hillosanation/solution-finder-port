//! The items are guarenteed to be RecursiveMinoField, compared to MinoFields.

use crate::searcher::pack::mino_field::recursive_mino_field::RecursiveMinoField;

pub trait RecursiveMinoFields {
    fn recursive_stream(&self) -> Box<dyn Iterator<Item = RecursiveMinoField> + '_>;
}
