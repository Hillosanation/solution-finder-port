use super::operation::Operation;
use crate::sfinder_core::mino::mino::Mino;

pub trait MinoOperation<Coord>: Operation<Coord>
where
    u32: From<Coord>,
    u64: From<Coord>,
{
    fn get_mino(&self) -> &Mino;
}
