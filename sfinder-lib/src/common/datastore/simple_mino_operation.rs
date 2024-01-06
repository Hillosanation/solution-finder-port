use super::{action::action::Action, mino_operation::MinoOperation, operation::Operation};
use crate::{
    extras::hash_code::HashCode,
    sfinder_core::{mino::mino::Mino, srs::rotate::Rotate},
};

#[derive(Debug)]
pub struct SimpleMinoOperation<'m> {
    mino: &'m Mino,
    x: u8,
    y: u8,
}

impl<'a> SimpleMinoOperation<'a> {
    pub fn new(mino: &'a Mino, x: u8, y: u8) -> Self {
        Self { mino, x, y }
    }
}

impl Action for SimpleMinoOperation<'_> {
    fn get_x(&self) -> u8 {
        self.x
    }

    fn get_y(&self) -> u8 {
        self.y
    }

    fn get_rotate(&self) -> Rotate {
        self.mino.get_rotate()
    }
}

impl Operation for SimpleMinoOperation<'_> {
    fn get_piece(&self) -> crate::sfinder_core::mino::piece::Piece {
        self.mino.get_piece()
    }
}

impl MinoOperation for SimpleMinoOperation<'_> {
    fn get_mino(&self) -> &Mino {
        self.mino
    }
}

impl PartialEq for SimpleMinoOperation<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.mino == other.mino && self.x == other.x && self.y == other.y
    }
}

impl HashCode for SimpleMinoOperation<'_> {
    type Output = u32;

    fn hash_code(&self) -> Self::Output {
        <dyn Operation>::default_hash_code(self)
    }
}
