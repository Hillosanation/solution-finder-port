use super::piece::{MinMaxBounds, Piece, Positions};
use crate::{
    common::datastore::coordinate::Coordinate, extras::hash_code::HashCode,
    sfinder_core::srs::rotate::Rotate,
};

/// Primitive for the shape of the tetromino.
/// Is bundled with information about the piece, rotation, min/max x/y, the filled positions of the shape, and the mask of the shape.
/// wirelyre's implementation uses a different offset from the actual rotation axis used here that allows for easier generation of pieces in bounds.
#[derive(Debug)]
pub struct Mino {
    piece: Piece,
    rotate: Rotate,
    mask: u64,
    /// Porting note: combined all bounds
    bounds: MinMaxBounds,
    /// TODO: this doesn't seem to be used in hotspots, prune from Mino and generate on demand?
    positions: Positions,
}

const MASK_CENTER_X: i8 = 4;
const MASK_CENTER_Y: i8 = 2;

impl Mino {
    pub fn new(piece: Piece, rotate: Rotate) -> Self {
        let positions = Self::calc_positions(piece, rotate);
        Self {
            piece,
            rotate,
            mask: Self::calc_mask(&positions),
            bounds: Self::calc_bounds(piece, rotate),
            positions,
        }
    }

    fn calc_mask(positions: &Positions) -> u64 {
        positions
            .iter()
            // TODO: replace 10 with FIELD_WIDTH in Field, or replace with Field::get_x_mask entirely
            .map(|position| 1 << ((MASK_CENTER_Y + position.y) * 10 + (MASK_CENTER_X + position.x)))
            .fold(0, core::ops::BitOr::bitor)
    }

    // Porting note: follows naming convention of RotateDirection
    fn rotate_cw(positions: &Positions) -> Positions {
        positions.map(|coord| Coordinate::new(coord.y, -coord.x))
    }

    fn rotate_ccw(positions: &Positions) -> Positions {
        positions.map(|coord| Coordinate::new(-coord.y, coord.x))
    }

    fn rotate_180(positions: &Positions) -> Positions {
        positions.map(|coord| Coordinate::new(coord.x, coord.y))
    }

    fn calc_positions(piece: Piece, rotate: Rotate) -> Positions {
        let positions = piece.get_positions();
        match rotate {
            Rotate::Spawn => positions,
            Rotate::Right => Self::rotate_cw(&positions),
            Rotate::Reverse => Self::rotate_180(&positions),
            Rotate::Left => Self::rotate_ccw(&positions),
        }
    }

    fn calc_bounds(piece: Piece, rotate: Rotate) -> MinMaxBounds {
        match rotate {
            Rotate::Spawn => MinMaxBounds {
                min_x: piece.min_x(),
                max_x: piece.max_x(),
                min_y: piece.min_y(),
                max_y: piece.max_y(),
            },
            Rotate::Right => MinMaxBounds {
                min_x: piece.min_y(),
                max_x: piece.max_y(),
                min_y: -piece.max_x(),
                max_y: -piece.min_x(),
            },
            Rotate::Reverse => MinMaxBounds {
                min_x: -piece.max_x(),
                max_x: -piece.min_x(),
                min_y: -piece.max_y(),
                max_y: -piece.min_y(),
            },
            Rotate::Left => MinMaxBounds {
                min_x: -piece.max_y(),
                max_x: -piece.min_y(),
                min_y: piece.min_x(),
                max_y: piece.max_x(),
            },
        }
    }

    pub fn get_piece(&self) -> Piece {
        self.piece
    }

    pub fn get_rotate(&self) -> Rotate {
        self.rotate
    }

    pub fn get_min_x(&self) -> i8 {
        self.bounds.min_x
    }

    pub fn get_max_x(&self) -> i8 {
        self.bounds.max_x
    }

    pub fn get_min_y(&self) -> i8 {
        self.bounds.min_y
    }

    pub fn get_max_y(&self) -> i8 {
        self.bounds.max_y
    }

    pub fn get_positions(&self) -> Positions {
        self.positions
    }

    pub fn get_mask(&self, x: u8, y: i8) -> u64 {
        assert!(x < 10);
        assert!(-4 < y && y < 8);

        let slide = (x as i8 - MASK_CENTER_X) + (y as i8 - MASK_CENTER_Y) * 10;
        if slide >= 0 {
            self.mask << slide
        } else {
            self.mask >> -slide
        }
    }
}

impl PartialEq for Mino {
    fn eq(&self, other: &Self) -> bool {
        // other data memebers are derived from piece and rotate
        self.piece == other.piece && self.rotate == other.rotate
    }
}

impl Eq for Mino {}

impl PartialOrd for Mino {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mino {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.piece
            .cmp(&other.piece)
            .then(self.rotate.cmp(&other.rotate))
    }
}

impl HashCode for Mino {
    type Output = u8;

    fn hash_code(&self) -> Self::Output {
        self.piece.hash_code() ^ self.rotate.hash_code()
    }
}

impl std::hash::Hash for Mino {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.hash_code())
    }
}

impl nohash::IsEnabled for Mino {}

#[cfg(test)]
mod tests {
    use super::*;

    // Todo, tests require FieldFactory
}
