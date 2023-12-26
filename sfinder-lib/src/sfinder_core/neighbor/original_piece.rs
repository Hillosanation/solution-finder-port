use crate::{
    common::datastore::{
        action::action::Action, full_operation_with_key::FullOperationWithKey,
        mino_operation::MinoOperation, mino_operation_with_key::MinoOperationWithKey,
        operation::Operation, operation_with_key::OperationWithKey,
    },
    sfinder_core::{field::field::Field, mino::mino::Mino, srs::rotate::Rotate},
};

// Porting note: EMPTY_COLLIDER_PIECE, (and the empty constructor) was used as a null check and is removed.
pub struct OriginalPiece<'m> {
    operation_with_key: FullOperationWithKey<'m>,
    harddrop_collider: Box<dyn Field>,
    mino_field: Box<dyn Field>,
}

impl OriginalPiece<'_> {
    fn create_mino_field(mino: &Mino, x: u8, y: u8) -> Box<dyn Field> {
        todo!("FieldFactory");
    }

    fn create_harddrop_collider(mino: &Mino, x: u8, y: u8) -> Box<dyn Field> {
        todo!("FieldFactory");
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
            operation_with_key: todo!("OperationTransform"),
            mino_field: OriginalPiece::create_mino_field(mino, x, y),
            harddrop_collider: OriginalPiece::create_harddrop_collider(mino, x, y),
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

impl MinoOperationWithKey<u8> for OriginalPiece<'_> {}

impl<'a> PartialEq for OriginalPiece<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.operation_with_key == other.operation_with_key
    }
}
