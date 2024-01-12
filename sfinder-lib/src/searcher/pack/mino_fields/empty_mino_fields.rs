use super::mino_fields::MinoFields;
use crate::searcher::pack::mino_field::mino_field::MinoField;

pub struct EmptyMinoFields {}

impl EmptyMinoFields {
    pub fn new() -> EmptyMinoFields {
        EmptyMinoFields {}
    }
}

impl MinoFields for EmptyMinoFields {
    fn stream(&self) -> Box<dyn Iterator<Item = Box<dyn MinoField + '_>> + '_> {
        Box::new(std::iter::empty())
    }
}
