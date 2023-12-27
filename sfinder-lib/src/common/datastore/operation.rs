use super::action::action::Action;
use crate::sfinder_core::mino::piece::Piece;

pub trait Operation<Coord>: Action<Coord> + std::fmt::Debug
where
    u32: From<Coord>,
    u64: From<Coord>,
{
    fn get_piece(&self) -> Piece;

    fn default_hash_code(&self) -> u32 {
        let mut result = u32::from(self.get_y());
        result = 10 * result + u32::from(self.get_x());
        result = 7 * result + self.get_piece() as u32;
        result = 4 * result + self.get_rotate() as u32;

        result
    }

    fn to_unique_key(&self) -> u64 {
        self.get_piece() as u64 * 4 * 24 * 10
            + self.get_rotate() as u64 * 24 * 10
            + u64::from(self.get_y()) * 10
            + u64::from(self.get_x())
    }
}

impl<Coord> PartialEq for dyn Operation<Coord> + '_
where
    u32: From<Coord>,
    u64: From<Coord>,
    Coord: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.get_piece() == other.get_piece()
            && self.get_rotate() == other.get_rotate()
            && self.get_x() == other.get_x()
            && self.get_y() == other.get_y()
    }
}

impl<Coord> PartialOrd for dyn Operation<Coord> + '_
where
    u32: From<Coord>,
    u64: From<Coord>,
    Coord: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.get_piece().cmp(&other.get_piece()).then_with(|| {
            self.get_rotate().cmp(&other.get_rotate()).then_with(|| {
                self.get_x()
                    .cmp(&other.get_x())
                    .then_with(|| self.get_y().cmp(&other.get_y()))
            })
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::datastore::simple_operation::SimpleOperation, sfinder_core::srs::rotate::Rotate,
    };

    fn test_operations(
        operation1: &dyn Operation<u8>,
        operation2: &dyn Operation<u8>,
    ) -> Option<std::cmp::Ordering> {
        operation1.partial_cmp(operation2)
    }

    fn test_symmetry(operation1: &dyn Operation<u8>, operation2: &dyn Operation<u8>) {
        match test_operations(operation1, operation2) {
            Some(std::cmp::Ordering::Greater) => assert_eq!(
                test_operations(operation2, operation1),
                Some(std::cmp::Ordering::Less)
            ),
            Some(std::cmp::Ordering::Less) => assert_eq!(
                test_operations(operation2, operation1),
                Some(std::cmp::Ordering::Greater)
            ),
            _ => panic!("invalid test input"),
        }
    }

    #[test]
    fn compare() {
        assert_eq!(
            test_operations(
                &SimpleOperation::new(Piece::I, Rotate::Spawn, 0, 1),
                &SimpleOperation::new(Piece::I, Rotate::Spawn, 0, 1)
            ),
            Some(std::cmp::Ordering::Equal)
        );
    }

    #[test]
    fn compare_diff_block() {
        test_symmetry(
            &SimpleOperation::new(Piece::S, Rotate::Spawn, 0, 1),
            &SimpleOperation::new(Piece::J, Rotate::Spawn, 7, 1),
        );
    }

    #[test]
    fn compare_diff_rotate() {
        test_symmetry(
            &SimpleOperation::new(Piece::S, Rotate::Left, 0, 1),
            &SimpleOperation::new(Piece::J, Rotate::Right, 7, 1),
        );
    }

    #[test]
    fn compare_diff_x() {
        test_symmetry(
            &SimpleOperation::new(Piece::I, Rotate::Spawn, 0, 1),
            &SimpleOperation::new(Piece::I, Rotate::Spawn, 7, 1),
        );
    }

    #[test]
    fn compare_diff_y() {
        test_symmetry(
            &SimpleOperation::new(Piece::I, Rotate::Spawn, 0, 1),
            &SimpleOperation::new(Piece::I, Rotate::Spawn, 0, 4),
        );
    }
}
