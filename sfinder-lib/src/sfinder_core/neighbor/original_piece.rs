use crate::{
    common::{
        datastore::{
            action::action::Action, full_operation_with_key::FullOperationWithKey,
            mino_operation::MinoOperation, mino_operation_with_key::MinoOperationWithKey,
            operation::Operation, operation_with_key::OperationWithKey,
        },
        parser::operation_transform,
    },
    extras::hash_code::HashCode,
    sfinder_core::{
        field::{field::Field, field_factory},
        mino::mino::Mino,
        srs::rotate::Rotate,
    },
};

// Porting note: EMPTY_COLLIDER_PIECE (and the empty constructor) was used as a null check and is removed.
#[derive(Debug)]
pub struct OriginalPiece<'m> {
    operation_with_key: FullOperationWithKey<'m>,
    harddrop_collider: Box<dyn Field>,
    mino_field: Box<dyn Field>,
}

impl OriginalPiece<'_> {
    fn create_mino_field(mino: &Mino, x: u8, y: u8) -> Box<dyn Field> {
        let mut field = field_factory::create_field((y as i8 + mino.get_max_y() + 1) as u8);
        field.put(mino, x, y);
        field
    }

    fn create_harddrop_collider(mino: &Mino, x: u8, y: u8, field_height: u8) -> Box<dyn Field> {
        let mut field = field_factory::create_field(field_height);
        for y_index in y..(field_height + (-mino.get_min_y()) as u8) {
            field.put(mino, x, y_index);
        }
        for y_index in field_height..(field.get_max_field_height()) {
            for x_index in 0..10 {
                field.remove_block(x_index, y_index);
            }
        }
        field
    }

    pub fn get_mino_field(&self) -> &dyn Field {
        self.mino_field.as_ref()
    }

    pub fn get_harddrop_collider(&self) -> &dyn Field {
        self.harddrop_collider.as_ref()
    }
}

impl<'a> OriginalPiece<'a> {
    pub fn new(mino: &'a Mino, x: u8, y: u8, field_height: u8) -> Self {
        Self {
            operation_with_key: operation_transform::to_full_operation_with_key(
                mino,
                x,
                y,
                0u64,
                field_height,
            ),
            mino_field: OriginalPiece::create_mino_field(mino, x, y),
            harddrop_collider: OriginalPiece::create_harddrop_collider(mino, x, y, field_height),
        }
    }
}

impl Action<u8> for OriginalPiece<'_> {
    fn get_x(&self) -> u8 {
        self.operation_with_key.get_x()
    }

    fn get_y(&self) -> u8 {
        self.operation_with_key.get_y()
    }

    fn get_rotate(&self) -> Rotate {
        self.operation_with_key.get_rotate()
    }
}

impl Operation<u8> for OriginalPiece<'_> {
    fn get_piece(&self) -> crate::sfinder_core::mino::piece::Piece {
        self.operation_with_key.get_piece()
    }
}

impl MinoOperation<u8> for OriginalPiece<'_> {
    fn get_mino(&self) -> &Mino {
        self.operation_with_key.get_mino()
    }
}

impl OperationWithKey<u8> for OriginalPiece<'_> {
    fn get_using_key(&self) -> u64 {
        self.operation_with_key.get_using_key()
    }

    fn get_need_deleted_key(&self) -> u64 {
        self.operation_with_key.get_need_deleted_key()
    }
}

impl MinoOperationWithKey for OriginalPiece<'_> {}

impl<'a> PartialEq for OriginalPiece<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.operation_with_key == other.operation_with_key
    }
}

impl HashCode for OriginalPiece<'_> {
    type Output = u32;

    fn hash_code(&self) -> Self::Output {
        self.operation_with_key.hash_code()
    }
}

#[cfg(test)]
use crate::sfinder_core::mino::mino_factory::MinoFactory;
#[cfg(test)]
pub fn create_all_pieces<'a>(
    mino_factory: &'a MinoFactory,
    field_height: u8,
) -> Vec<OriginalPiece<'a>> {
    // A reference to a Mino in MinoFactory is needed because OriginalPiece stores a reference of a Mino
    use crate::sfinder_core::{field::field::FIELD_WIDTH, mino::piece::Piece};

    Piece::value_list()
        .iter()
        .flat_map(move |piece| {
            Rotate::value_list().iter().flat_map(move |rotate| {
                let mino = mino_factory.get(*piece, *rotate);

                (-mino.get_min_y()..field_height as i8 - mino.get_max_y()).flat_map(move |y| {
                    (-mino.get_min_x()..FIELD_WIDTH as i8 - mino.get_max_x())
                        .map(move |x| OriginalPiece::new(mino, x as u8, y as u8, field_height))
                })
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_core::mino::piece::Piece;

    #[test]
    fn test_equals() {
        assert_eq!(
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10)
        );
    }

    #[test]
    fn test_equals_diff_block() {
        assert_ne!(
            OriginalPiece::new(&Mino::new(Piece::L, Rotate::Spawn), 3, 4, 10),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10)
        );
    }

    #[test]
    fn test_equals_diff_rotate() {
        assert_ne!(
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Left), 3, 4, 10),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10)
        );
    }

    #[test]
    fn test_equals_diff_x() {
        assert_ne!(
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Left), 2, 4, 10),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10)
        );
    }

    #[test]
    fn test_equals_diff_y() {
        assert_ne!(
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Left), 3, 8, 10),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10)
        );
    }

    #[test]
    fn test_equals_diff_field_height() {
        assert_eq!(
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 5),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10)
        );
    }

    #[test]
    fn test_hash_code() {
        assert_eq!(
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10).hash_code(),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10).hash_code()
        );
    }

    #[test]
    fn test_hash_code_diff_block() {
        assert_ne!(
            OriginalPiece::new(&Mino::new(Piece::L, Rotate::Spawn), 3, 4, 10).hash_code(),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10).hash_code()
        );
    }

    #[test]
    fn test_hash_code_diff_rotate() {
        assert_ne!(
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Left), 3, 4, 10).hash_code(),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10).hash_code()
        );
    }

    #[test]
    fn test_hash_code_diff_x() {
        assert_ne!(
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Left), 2, 4, 10).hash_code(),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10).hash_code()
        );
    }

    #[test]
    fn test_hash_code_diff_y() {
        assert_ne!(
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Left), 3, 8, 10).hash_code(),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10).hash_code()
        );
    }

    #[test]
    fn test_hash_code_diff_field_height() {
        assert_eq!(
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 5).hash_code(),
            OriginalPiece::new(&Mino::new(Piece::I, Rotate::Spawn), 3, 4, 10).hash_code()
        );
    }
}
