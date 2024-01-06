use super::{action::action::Action, mino_operation::MinoOperation, operation::Operation};
use crate::{
    extras::hash_code::HashCode,
    sfinder_core::{
        field::field_constants::FIELD_WIDTH,
        mino::{mino::Mino, mino_factory::MinoFactory},
        srs::rotate::Rotate,
    },
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

    // Porting note: moved from MinoTransform
    pub fn as_mirror(mino_factory: &'a MinoFactory, mino_operation: &dyn MinoOperation) -> Self {
        let piece = mino_operation.get_piece();
        let rotate = mino_operation.get_rotate();
        let x = mino_operation.get_x();
        let y = mino_operation.get_y();

        // TODO: technically you don't need to query mino_factory here to get this information, since you can recover them from the mirrored mino
        let mino = mino_factory.get(piece, rotate);
        let rx = u8::try_from(x as i8 + mino.get_max_x()).unwrap();
        let by = u8::try_from(y as i8 + mino.get_min_y()).unwrap();

        let mirror_piece = piece.mirror();
        let mirror_rotate = rotate.mirror();
        let mirror_mino = mino_factory.get(mirror_piece, mirror_rotate);
        let lx = (FIELD_WIDTH - 1) - rx;

        Self::new(
            &mirror_mino,
            u8::try_from(lx as i8 - mirror_mino.get_min_x()).unwrap(),
            u8::try_from(by as i8 - mirror_mino.get_min_y()).unwrap(),
        )
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::datastore::simple_mino_operation::SimpleMinoOperation,
        sfinder_core::mino::piece::Piece,
    };

    // Porting note: tests retrieved from MinoTransformTest

    fn assert_operations(
        mino_factory: &MinoFactory,
        pairs: Vec<(SimpleMinoOperation, SimpleMinoOperation)>,
    ) {
        for (a, b) in pairs {
            assert_eq!(SimpleMinoOperation::as_mirror(&mino_factory, &a), b);
            assert_eq!(SimpleMinoOperation::as_mirror(&mino_factory, &b), a);
        }
    }

    fn create_simple_operation(
        mino_factory: &MinoFactory,
        piece: Piece,
        rotate: Rotate,
        x: u8,
        y: u8,
    ) -> SimpleMinoOperation {
        SimpleMinoOperation::new(&mino_factory.get(piece, rotate), x, y)
    }

    #[test]
    fn mirror_i() {
        let mino_factory = MinoFactory::new();
        let piece = Piece::I;
        let pairs = vec![
            (
                create_simple_operation(&mino_factory, piece, Rotate::Spawn, 1, 0),
                create_simple_operation(&mino_factory, piece, Rotate::Spawn, 7, 0),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Reverse, 2, 0),
                create_simple_operation(&mino_factory, piece, Rotate::Reverse, 8, 0),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Right, 0, 2),
                create_simple_operation(&mino_factory, piece, Rotate::Left, 9, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Left, 0, 1),
                create_simple_operation(&mino_factory, piece, Rotate::Right, 9, 2),
            ),
        ];

        assert_operations(&mino_factory, pairs);
    }

    #[test]
    fn mirror_t() {
        let mino_factory = MinoFactory::new();
        let piece = Piece::T;
        let pairs = vec![
            (
                create_simple_operation(&mino_factory, piece, Rotate::Spawn, 1, 1),
                create_simple_operation(&mino_factory, piece, Rotate::Spawn, 8, 1),
            ),
            (
                // Porting note: the original test had unplacable minos, which I modified to now be placable
                create_simple_operation(&mino_factory, piece, Rotate::Reverse, 1, 1),
                create_simple_operation(&mino_factory, piece, Rotate::Reverse, 8, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Right, 0, 1),
                create_simple_operation(&mino_factory, piece, Rotate::Left, 9, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Left, 1, 1),
                create_simple_operation(&mino_factory, piece, Rotate::Right, 8, 1),
            ),
        ];

        assert_operations(&mino_factory, pairs);
    }

    #[test]
    fn mirror_o() {
        let mino_factory = MinoFactory::new();
        let piece = Piece::O;
        let pairs = vec![
            (
                create_simple_operation(&mino_factory, piece, Rotate::Spawn, 0, 0),
                create_simple_operation(&mino_factory, piece, Rotate::Spawn, 8, 0),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Reverse, 1, 1),
                create_simple_operation(&mino_factory, piece, Rotate::Reverse, 9, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Left, 1, 0),
                create_simple_operation(&mino_factory, piece, Rotate::Right, 8, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Right, 0, 1),
                create_simple_operation(&mino_factory, piece, Rotate::Left, 9, 0),
            ),
        ];

        assert_operations(&mino_factory, pairs);
    }

    #[test]
    fn mirror_s() {
        let mino_factory = MinoFactory::new();
        let piece = Piece::S;
        let pairs = vec![
            (
                create_simple_operation(&mino_factory, piece, Rotate::Spawn, 1, 0),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Spawn, 8, 0),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Reverse, 1, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Reverse, 8, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Left, 1, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Right, 8, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Right, 0, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Left, 9, 1),
            ),
        ];

        assert_operations(&mino_factory, pairs);
    }

    #[test]
    fn mirror_z() {
        let mino_factory = MinoFactory::new();
        let piece = Piece::Z;
        let pairs = vec![
            (
                create_simple_operation(&mino_factory, piece, Rotate::Spawn, 1, 0),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Spawn, 8, 0),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Reverse, 1, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Reverse, 8, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Left, 1, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Right, 8, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Right, 0, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Left, 9, 1),
            ),
        ];

        assert_operations(&mino_factory, pairs);
    }

    #[test]
    fn mirror_l() {
        let mino_factory = MinoFactory::new();
        let piece = Piece::L;
        let pairs = vec![
            (
                create_simple_operation(&mino_factory, piece, Rotate::Spawn, 1, 0),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Spawn, 8, 0),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Reverse, 1, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Reverse, 8, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Left, 1, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Right, 8, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Right, 0, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Left, 9, 1),
            ),
        ];

        assert_operations(&mino_factory, pairs);
    }

    #[test]
    fn mirror_j() {
        let mino_factory = MinoFactory::new();
        let piece = Piece::J;
        let pairs = vec![
            (
                create_simple_operation(&mino_factory, piece, Rotate::Spawn, 1, 0),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Spawn, 8, 0),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Reverse, 1, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Reverse, 8, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Left, 1, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Right, 8, 1),
            ),
            (
                create_simple_operation(&mino_factory, piece, Rotate::Right, 0, 1),
                create_simple_operation(&mino_factory, piece.mirror(), Rotate::Left, 9, 1),
            ),
        ];

        assert_operations(&mino_factory, pairs);
    }
}
