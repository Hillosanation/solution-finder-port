use crate::sfinder_core::{mino::piece::Piece, srs::rotate::Rotate};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct KickType {
    pub piece: Piece,
    pub from: Rotate,
    pub to: Rotate,
}
