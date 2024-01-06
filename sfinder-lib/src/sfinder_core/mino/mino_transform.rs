//! Helper struct for MinoShifter

use super::mino_factory::MinoFactory;
use crate::{
    common::datastore::{
        action::minimal_action::MinimalAction, coordinate::Coordinate,
        mino_operation::MinoOperation, simple_mino_operation::SimpleMinoOperation,
    },
    sfinder_core::{field::field_constants::FIELD_WIDTH, srs::rotate::Rotate},
};

#[derive(Debug)]
pub struct MinoTransform {
    offsets: Vec<Coordinate>,
    rotates: Vec<Rotate>,
    reverse_map: Vec<Vec<Rotate>>,
}

impl MinoTransform {
    pub fn new() -> Self {
        let mut this = Self {
            offsets: vec![Coordinate::new(0, 0); Rotate::get_size()],
            rotates: Rotate::value_list().to_vec(),
            reverse_map: vec![Vec::new(); Rotate::get_size()],
        };

        this.refresh();

        this
    }

    pub fn set(&mut self, rotate: Rotate, offset_x: i8, offset_y: i8, new_rotate: Rotate) {
        let index = rotate as usize;
        assert_eq!(self.rotates.len(), index);

        self.offsets[index] = Coordinate::new(offset_x, offset_y);
        self.rotates[index] = new_rotate;
        self.refresh();
    }

    fn refresh(&mut self) {
        for reverse in self.reverse_map.iter_mut() {
            reverse.clear();
        }

        for &rotate in Rotate::value_list() {
            let index = rotate as usize;
            if let Some(&new_rotate) = self.rotates.get(index) {
                if rotate != new_rotate {
                    // 変換後の回転が同じになる、他の回転とも関連づける
                    for r in self.reverse_map[new_rotate as usize].clone() {
                        self.reverse_map[r as usize].push(rotate);
                        self.reverse_map[rotate as usize].push(r);
                    }

                    // 変換前と変換後を関連づける
                    self.reverse_map[new_rotate as usize].push(rotate);
                    self.reverse_map[rotate as usize].push(new_rotate);
                }
            }
        }
    }

    pub fn transform(&self, x: u8, y: u8, rotate: Rotate) -> MinimalAction {
        let index = rotate as usize;
        MinimalAction::new(
            u8::try_from(x as i8 + self.offsets[index].x).unwrap(),
            u8::try_from(y as i8 + self.offsets[index].y).unwrap(),
            self.rotates[index],
        )
    }

    pub fn enumerate_others(&self, x: u8, y: u8, rotate: Rotate) -> Vec<MinimalAction> {
        let index = rotate as usize;
        let new_x = x as i8 + self.offsets[index].x;
        let new_y = y as i8 + self.offsets[index].y;

        self.reverse_map[index]
            .iter()
            .copied()
            .map(|prev_rotate| {
                let prev_index = prev_rotate as usize;
                MinimalAction::new(
                    u8::try_from(new_x - self.offsets[prev_index].x).unwrap(),
                    u8::try_from(new_y - self.offsets[prev_index].y).unwrap(),
                    prev_rotate,
                )
            })
            .collect()
    }

    pub fn transform_rotate(&self, rotate: Rotate) -> Rotate {
        self.rotates[rotate as usize]
    }

    // TODO: move to MinoOperation module to make this struct exclusively for MinoShifter
    pub fn mirror<'a>(
        mino_factory: &'a MinoFactory,
        mino_operation: &dyn MinoOperation,
    ) -> SimpleMinoOperation<'a> {
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

        SimpleMinoOperation::new(
            &mirror_mino,
            u8::try_from(lx as i8 - mirror_mino.get_min_x()).unwrap(),
            u8::try_from(by as i8 - mirror_mino.get_min_y()).unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::datastore::simple_mino_operation::SimpleMinoOperation,
        sfinder_core::mino::piece::Piece,
    };

    fn assert_operations(
        mino_factory: &MinoFactory,
        pairs: Vec<(SimpleMinoOperation, SimpleMinoOperation)>,
    ) {
        for (a, b) in pairs {
            assert_eq!(MinoTransform::mirror(&mino_factory, &a), b);
            assert_eq!(MinoTransform::mirror(&mino_factory, &b), a);
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
