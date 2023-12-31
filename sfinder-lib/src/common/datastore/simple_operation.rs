use super::{action::action::Action, operation::Operation};
use crate::{
    extras::hash_code::HashCode,
    sfinder_core::{mino::piece::Piece, srs::rotate::Rotate},
};
use std::{convert::Infallible, fmt::Display, str::FromStr};

// TODO: merge with MinoOperation?
// Porting note: This doesn't check if the operation is valid.
#[derive(Clone, PartialEq, Debug)]
pub struct SimpleOperation {
    piece: Piece,
    rotate: Rotate,
    x: u8,
    y: u8,
}

impl SimpleOperation {
    pub fn new(piece: Piece, rotate: Rotate, x: u8, y: u8) -> Self {
        Self {
            piece,
            rotate,
            x,
            y,
        }
    }
}

impl Action<u8> for SimpleOperation {
    fn get_x(&self) -> u8 {
        self.x
    }

    fn get_y(&self) -> u8 {
        self.y
    }

    fn get_rotate(&self) -> Rotate {
        self.rotate
    }
}

impl Operation<u8> for SimpleOperation {
    fn get_piece(&self) -> Piece {
        self.piece
    }
}

impl HashCode for SimpleOperation {
    type Output = u32;

    fn hash_code(&self) -> Self::Output {
        Operation::default_hash_code(self)
    }
}

impl PartialOrd for SimpleOperation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self as &dyn Operation<u8>).partial_cmp(other)
    }
}

// Porting note: moved from OperationInterpreter
impl FromStr for SimpleOperation {
    type Err = Infallible;

    // Used mainly for testing
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(',').collect::<Vec<_>>();
        assert_eq!(split.len(), 4);

        Ok(SimpleOperation::new(
            split[0].parse().unwrap(),
            split[1].parse().unwrap(),
            split[2].parse().unwrap(),
            split[3].parse().unwrap(),
        ))
    }
}

impl Display for SimpleOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self as &dyn Operation<u8>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getter() {
        let operation = SimpleOperation::new(Piece::T, Rotate::Spawn, 4, 5);

        assert_eq!(operation.get_piece(), Piece::T);
        assert_eq!(operation.get_rotate(), Rotate::Spawn);
        assert_eq!(operation.get_x(), 4);
        assert_eq!(operation.get_y(), 5);
    }

    #[test]
    #[rustfmt::skip]
    fn test_equal() {
        let operation = SimpleOperation::new(Piece::T, Rotate::Spawn, 4, 5);
        assert_eq!(operation, SimpleOperation::new(Piece::T, Rotate::Spawn, 4, 5));
        assert_ne!(operation, SimpleOperation::new(Piece::L, Rotate::Spawn, 4, 5));
        assert_ne!(operation, SimpleOperation::new(Piece::T, Rotate::Left, 4, 5));
        assert_ne!(operation, SimpleOperation::new(Piece::T, Rotate::Spawn, 3, 5));
        assert_ne!(operation, SimpleOperation::new(Piece::T, Rotate::Spawn, 4, 6));
    }

    #[test]
    #[rustfmt::skip]
    fn test_hash_code() {
        let operation = SimpleOperation::new(Piece::T, Rotate::Spawn, 4, 5);
        assert_eq!(operation.hash_code(), SimpleOperation::new(Piece::T, Rotate::Spawn, 4, 5).hash_code());
        assert_ne!(operation.hash_code(), SimpleOperation::new(Piece::L, Rotate::Spawn, 4, 5).hash_code());
        assert_ne!(operation.hash_code(), SimpleOperation::new(Piece::T, Rotate::Left, 4, 5).hash_code());
        assert_ne!(operation.hash_code(), SimpleOperation::new(Piece::T, Rotate::Spawn, 3, 5).hash_code());
        assert_ne!(operation.hash_code(), SimpleOperation::new(Piece::T, Rotate::Spawn, 4, 6).hash_code());
    }

    #[test]
    #[rustfmt::skip]
    fn test_compare_to() {
        let binding = Box::new(SimpleOperation::new(Piece::T, Rotate::Spawn, 4, 5)) as Box<dyn Operation<u8>>;
        let operation1 = binding.as_ref();
        let operation2 = &SimpleOperation::new(Piece::T, Rotate::Spawn, 4, 5) as &dyn Operation<u8>;
        let operation3 = &SimpleOperation::new(Piece::T, Rotate::Spawn, 4, 13) as &dyn Operation<u8>;
        let operation4 = &SimpleOperation::new(Piece::T, Rotate::Spawn, 5, 13) as &dyn Operation<u8>;

        assert_eq!(operation1, operation2);
        assert_ne!(operation1, operation3);
        assert_ne!(operation1, operation4);
        assert_ne!(operation3, operation4);

        assert!(operation1 < operation3);
        assert!(operation3 < operation4);
    }
}
