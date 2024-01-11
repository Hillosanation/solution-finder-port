use super::{
    mino::{Mino, MINOS},
    piece::Piece,
};
use crate::sfinder_core::srs::rotate::Rotate;

pub struct MinoFactory {}

impl MinoFactory {
    const fn into_val(piece: Piece, rotate: Rotate) -> usize {
        piece as usize * Rotate::get_size() + rotate as usize
    }

    pub const fn new() -> Self {
        Self {}
    }

    // TODO: move get to module level and remove MinoFactory struct
    pub const fn get(&self, piece: Piece, rotate: Rotate) -> &'static Mino {
        &MINOS[MinoFactory::into_val(piece, rotate)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn legacy_new() -> [Mino; Piece::get_size() * Rotate::get_size()] {
        std::array::from_fn(|i| {
            Mino::new(
                Piece::new((i / Rotate::get_size()) as _),
                Rotate::new((i % Rotate::get_size()) as _),
            )
        })
    }

    #[test]
    fn const_eval() {
        assert_eq!(legacy_new(), MINOS);
    }

    #[test]
    fn size() {
        assert_eq!(legacy_new().len(), Piece::get_size() * Rotate::get_size());
    }

    #[test]
    fn create() {
        let mino_factory = MinoFactory::new();

        for &piece in Piece::value_list() {
            for &rotate in Rotate::value_list() {
                let mino = mino_factory.get(piece, rotate);
                for _ in 0..100 {
                    // check for consistency
                    // 同じインスタンスであること
                    assert_eq!(mino, mino_factory.get(piece, rotate));
                }
            }
        }
    }
}
