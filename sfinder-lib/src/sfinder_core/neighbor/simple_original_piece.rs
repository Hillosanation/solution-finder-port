//! OriginalPiece without computing the harddrop collider.

use crate::{
    common::datastore::{
        action::action::Action, full_operation_with_key::FullOperationWithKey,
        mino_operation::MinoOperation, mino_operation_with_key::MinoOperationWithKey,
        operation::Operation, operation_with_key::OperationWithKey,
    },
    extras::hash_code::HashCode,
    sfinder_core::{
        field::field::Field,
        mino::{mino::Mino, piece::Piece},
        srs::rotate::Rotate,
    },
};

#[derive(Debug)]
pub struct SimpleOriginalPiece {
    operation_with_key: FullOperationWithKey,
    mino_field: Box<dyn Field>,
}

impl SimpleOriginalPiece {
    pub fn new(operation_with_key: FullOperationWithKey, field_height: u8) -> Self {
        Self {
            mino_field: operation_with_key.create_mino_field(field_height),
            operation_with_key,
        }
    }

    pub fn get_mino_field(&self) -> &dyn Field {
        self.mino_field.as_ref()
    }
}

impl Action for SimpleOriginalPiece {
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

impl Operation for SimpleOriginalPiece {
    fn get_piece(&self) -> Piece {
        self.operation_with_key.get_piece()
    }
}

impl MinoOperation for SimpleOriginalPiece {
    fn get_mino(&self) -> &Mino {
        self.operation_with_key.get_mino()
    }
}

impl OperationWithKey for SimpleOriginalPiece {
    fn get_using_key(&self) -> u64 {
        self.operation_with_key.get_using_key()
    }

    fn get_need_deleted_key(&self) -> u64 {
        self.operation_with_key.get_need_deleted_key()
    }
}

impl MinoOperationWithKey for SimpleOriginalPiece {}

impl PartialEq for SimpleOriginalPiece {
    fn eq(&self, other: &Self) -> bool {
        self.operation_with_key == other.operation_with_key
    }
}

impl HashCode for SimpleOriginalPiece {
    type Output = u64;

    fn hash_code(&self) -> Self::Output {
        self.operation_with_key.hash_code()
    }
}
