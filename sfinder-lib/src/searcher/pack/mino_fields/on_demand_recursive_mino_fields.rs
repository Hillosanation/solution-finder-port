use crate::{
    extras::callable::Callable,
    searcher::pack::{
        calculator::connections_to_stream_callable::ConnectionsToStreamCallable,
        mino_field::recursive_mino_field::RecursiveMinoField,
    },
};

use super::recursive_mino_fields::RecursiveMinoFields;

pub struct OnDemandRecursiveMinoFields {
    callable: ConnectionsToStreamCallable,
}

impl OnDemandRecursiveMinoFields {
    pub fn new(callable: ConnectionsToStreamCallable) -> Self {
        Self { callable }
    }
}

impl RecursiveMinoFields for OnDemandRecursiveMinoFields {
    fn recursive_stream(&self) -> Box<dyn Iterator<Item = RecursiveMinoField> + '_> {
        self.callable.call()
    }
}
