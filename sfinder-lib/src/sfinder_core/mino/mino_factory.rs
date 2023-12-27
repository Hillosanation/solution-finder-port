use super::{mino::Mino, piece::Piece};
use crate::sfinder_core::srs::rotate::Rotate;
use nohash::IntMap;

pub struct MinoFactory {
    map: IntMap<u8, Mino>,
}

impl MinoFactory {
    const fn into_val(piece: Piece, rotate: Rotate) -> u8 {
        piece as u8 * Rotate::get_size() as u8 + rotate as u8
    }

    pub fn new() -> Self {
        let mut map = IntMap::default();
        for &piece in Piece::value_list() {
            for &rotate in Rotate::value_list() {
                let key = MinoFactory::into_val(piece, rotate);
                let mino = Mino::new(piece, rotate);
                map.insert(key, mino);
            }
        }

        Self { map }
    }

    pub fn get(&self, piece: Piece, rotate: Rotate) -> &Mino {
        &self.map[&MinoFactory::into_val(piece, rotate)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(
            MinoFactory::new().map.len(),
            Piece::get_size() * Rotate::get_size()
        );
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
