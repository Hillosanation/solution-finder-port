use crate::{
    extras::callable::Callable,
    searcher::pack::{
        calculator::connections_to_list_callable::ConnectionsToListCallable,
        mino_field::recursive_mino_field::RecursiveMinoField,
    },
};
use std::cell::OnceCell;

use super::recursive_mino_fields::RecursiveMinoFields;

pub struct MemorizedRecursiveMinoFields<'a> {
    callable: ConnectionsToListCallable,
    result: OnceCell<Vec<RecursiveMinoField<'a>>>,
}

impl<'a> MemorizedRecursiveMinoFields<'a> {
    pub fn new(callable: ConnectionsToListCallable) -> Self {
        Self {
            callable,
            result: OnceCell::new(),
        }
    }
}

impl RecursiveMinoFields for MemorizedRecursiveMinoFields<'_> {
    // TODO: because this struct retains ownership of RecursiveMinoField, we need to clone to return the iterator with the right signature
    // try to see of modifying RecursiveMinoFields is possible
    fn recursive_stream(&self) -> Box<dyn Iterator<Item = RecursiveMinoField> + '_> {
        Box::new(
            self.result
                .get_or_init(|| self.callable.call())
                .to_owned()
                .into_iter(),
        )
    }
}
