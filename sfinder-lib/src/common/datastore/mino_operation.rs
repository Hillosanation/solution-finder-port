use super::operation::Operation;
use crate::sfinder_core::mino::mino::Mino;

pub trait MinoOperation: Operation {
    fn get_mino(&self) -> &Mino;
}
