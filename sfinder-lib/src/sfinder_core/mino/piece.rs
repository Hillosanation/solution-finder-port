//! Porting note: min/max functions used in the original function are now precalculated. Their validity is checked in test cases, and they look the same outside the module.
//! Items marked as pub(super) are shared with mino.rs

use crate::common::datastore::coordinate::Coordinate;
use crate::extras::hash_code::HashCode;
use std::fmt::Display;
use std::str::FromStr;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    T = 0,
    I,
    L,
    J,
    S,
    Z,
    O,
}

#[derive(Debug)]
pub(super) struct MinMaxBounds {
    pub(super) min_x: i8,
    pub(super) max_x: i8,
    pub(super) min_y: i8,
    pub(super) max_y: i8,
}

const PIECE_COUNT: usize = 7;
const CELL_COUNT: usize = 4;

pub(super) type Positions = [Coordinate; CELL_COUNT];

/// Indexed by Piece
#[rustfmt::skip]
const POSITION_MAP: [Positions; PIECE_COUNT] = [
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

    pub const fn value_list() -> &'static [Piece] {
        &VALUE_LIST
    }

    pub const fn get_size() -> usize {
        PIECE_COUNT
    }

    pub fn get_positions(self) -> Positions {
        POSITION_MAP[self as usize]
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

    pub const fn mirror(self) -> Self {
        match self {
            Piece::J => Piece::L,
            Piece::L => Piece::J,
            Piece::S => Piece::Z,
            Piece::Z => Piece::S,
            _ => self,
        }
    }
}

impl HashCode for Piece {
    type Output = u8;

    fn hash_code(&self) -> Self::Output {
        *self as u8
    }
}

impl std::hash::Hash for Piece {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.hash_code())
    }
}

impl nohash::IsEnabled for Piece {}

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

impl FromStr for Piece {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T" => Ok(Piece::T),
            "I" => Ok(Piece::I),
            "L" => Ok(Piece::L),
            "J" => Ok(Piece::J),
            "S" => Ok(Piece::S),
            "Z" => Ok(Piece::Z),
            "O" => Ok(Piece::O),
            _ => Err(format!("Invalid piece: {s}")),
        }
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

    // tests retrieved from StringEnumTransformTest.java
    #[test]
    fn to_block_string() {
        assert_eq!(
            ["T", "I", "L", "J", "S", "Z", "O"]
                .iter()
                .map(|s| Piece::from_str(s).unwrap())
                .collect::<Vec<_>>(),
            Piece::value_list()
        );
    }
}
