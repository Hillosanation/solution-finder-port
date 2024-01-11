use crate::{
    extras::callable::Callable,
    searcher::pack::mino_field::recursive_mino_field::RecursiveMinoField,
};

use super::connections_to_stream_callable::ConnectionsToStreamCallable;

pub struct ConnectionsToListCallable {
    callable: ConnectionsToStreamCallable,
}

impl<'a> Callable<Vec<RecursiveMinoField<'a>>> for ConnectionsToListCallable {
    fn call(&self) -> Vec<RecursiveMinoField<'a>> {
        self.callable.call().collect()
    }
}
