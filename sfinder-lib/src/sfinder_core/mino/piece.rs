//! Porting note: min/max functions used in the original function are now precalculated. Their validity is checked in test cases, and they look the same outside the module.
use crate::common::datastore::coordinate::Coordinate;
use std::fmt::Display;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    T = 0,
    I,
    L,
    J,
    S,
    Z,
    O,
}

struct MinMaxBounds {
    min_x: i8,
    max_x: i8,
    min_y: i8,
    max_y: i8,
}

const PIECE_COUNT: usize = 7;
const CELL_COUNT: usize = 4;

/// Indexed by Piece
#[rustfmt::skip]
const POSITION_MAP: [[Coordinate; CELL_COUNT]; PIECE_COUNT] = [
    // Porting note: Reuse Coordinate here to make it more explicit
    [Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(1, 0), Coordinate::new(0, 1)],
    [Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(1, 0), Coordinate::new(2, 0)],
    [Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(1, 0), Coordinate::new(1, 1)],
    [Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(1, 0), Coordinate::new(-1, 1)],
    [Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(0, 1), Coordinate::new(1, 1)],
    [Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(0, 1), Coordinate::new(-1, 1)],
    [Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(0, 1), Coordinate::new(1, 1)],
];

#[rustfmt::skip]
const MINMAX_MAP: [MinMaxBounds; PIECE_COUNT] = [
    MinMaxBounds { min_x: -1, max_x: 1, min_y: 0, max_y: 1 },
    MinMaxBounds { min_x: -1, max_x: 2, min_y: 0, max_y: 0 },
    MinMaxBounds { min_x: -1, max_x: 1, min_y: 0, max_y: 1 },
    MinMaxBounds { min_x: -1, max_x: 1, min_y: 0, max_y: 1 },
    MinMaxBounds { min_x: -1, max_x: 1, min_y: 0, max_y: 1 },
    MinMaxBounds { min_x: -1, max_x: 1, min_y: 0, max_y: 1 },
    MinMaxBounds { min_x: 0, max_x: 1, min_y: 0, max_y: 1 },
];

const VALUE_LIST: [Piece; PIECE_COUNT] = [
    Piece::T,
    Piece::I,
    Piece::L,
    Piece::J,
    Piece::S,
    Piece::Z,
    Piece::O,
];

/// Porting note: casting replaces getNumber
impl Piece {
    /// Porting note: replaces getBlock
    /// Panics if number is out of range.
    pub fn new(number: u8) -> Self {
        VALUE_LIST[number as usize]
    }

    pub fn value_list() -> &'static [Piece] {
        &VALUE_LIST
    }

    pub fn get_size() -> usize {
        PIECE_COUNT
    }

    pub fn min_x(self) -> i8 {
        MINMAX_MAP[self as usize].min_x
    }

    pub fn max_x(self) -> i8 {
        MINMAX_MAP[self as usize].max_x
    }

    pub fn min_y(self) -> i8 {
        MINMAX_MAP[self as usize].min_y
    }

    pub fn max_y(self) -> i8 {
        MINMAX_MAP[self as usize].max_y
    }
}

/// Porting note: This replaces getName
impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self {
            Piece::T => "T",
            Piece::I => "I",
            Piece::L => "L",
            Piece::J => "J",
            Piece::S => "S",
            Piece::Z => "Z",
            Piece::O => "O",
        };
        write!(f, "{piece}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod precompute_bounds {
        use super::*;

        #[test]
        fn min_x() {
            for piece in Piece::value_list().into_iter().copied() {
                let computed = POSITION_MAP[piece as usize]
                    .iter()
                    .map(|c| c.x)
                    .min()
                    .unwrap();
                assert_eq!(piece.min_x(), computed);
            }
        }

        #[test]
        fn max_x() {
            for piece in Piece::value_list().into_iter().copied() {
                let computed = POSITION_MAP[piece as usize]
                    .iter()
                    .map(|c| c.x)
                    .max()
                    .unwrap();
                assert_eq!(piece.max_x(), computed);
            }
        }

        #[test]
        fn min_y() {
            for piece in Piece::value_list().into_iter().copied() {
                let computed = POSITION_MAP[piece as usize]
                    .iter()
                    .map(|c| c.y)
                    .min()
                    .unwrap();
                assert_eq!(piece.min_y(), computed);
            }
        }

        #[test]
        fn max_y() {
            for piece in Piece::value_list().into_iter().copied() {
                let computed = POSITION_MAP[piece as usize]
                    .iter()
                    .map(|c| c.y)
                    .max()
                    .unwrap();
                assert_eq!(piece.max_y(), computed);
            }
        }
    }

    #[test]
    fn get_block() {
        assert_eq!(Piece::new(0), Piece::T);
        assert_eq!(Piece::new(1), Piece::I);
        assert_eq!(Piece::new(2), Piece::L);
        assert_eq!(Piece::new(3), Piece::J);
        assert_eq!(Piece::new(4), Piece::S);
        assert_eq!(Piece::new(5), Piece::Z);
        assert_eq!(Piece::new(6), Piece::O);
    }

    #[test]
    fn value_list() {
        assert_eq!(
            Piece::value_list(),
            &[
                Piece::T,
                Piece::I,
                Piece::L,
                Piece::J,
                Piece::S,
                Piece::Z,
                Piece::O
            ]
        );
    }

    #[test]
    fn get_size() {
        assert_eq!(Piece::get_size(), 7);
    }

    #[test]
    fn get_number() {
        assert_eq!(Piece::T as u8, 0);
        assert_eq!(Piece::I as u8, 1);
        assert_eq!(Piece::L as u8, 2);
        assert_eq!(Piece::J as u8, 3);
        assert_eq!(Piece::S as u8, 4);
        assert_eq!(Piece::Z as u8, 5);
        assert_eq!(Piece::O as u8, 6);
    }

    #[test]
    fn get_name() {
        assert_eq!(Piece::T.to_string(), "T");
        assert_eq!(Piece::I.to_string(), "I");
        assert_eq!(Piece::L.to_string(), "L");
        assert_eq!(Piece::J.to_string(), "J");
        assert_eq!(Piece::S.to_string(), "S");
        assert_eq!(Piece::Z.to_string(), "Z");
        assert_eq!(Piece::O.to_string(), "O");
    }
}
