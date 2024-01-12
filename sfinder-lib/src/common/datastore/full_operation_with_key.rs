use super::{
    action::action::Action, mino_operation::MinoOperation,
    mino_operation_with_key::MinoOperationWithKey, operation::Operation,
    operation_with_key::OperationWithKey,
};
use crate::{
    extras::hash_code::HashCode,
    sfinder_core::{
        field::field_factory,
        mino::{mino::Mino, piece::Piece},
        srs::rotate::Rotate,
    },
};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct FullOperationWithKey<'m> {
    mino: &'m Mino,
    x: u8,
    y: u8,
    need_deleted_key: u64,
    using_key: u64,
}

impl<'a> FullOperationWithKey<'a> {
    pub fn create(mino: &'a Mino, x: u8, y: u8, need_deleted_key: u64, field_height: u8) -> Self {
        let mut field = field_factory::create_field(field_height);
        field.put(mino, x, y);
        field.insert_blank_row_with_key(need_deleted_key);

        Self {
            mino,
            x,
            y,
            need_deleted_key,
            using_key: field.get_using_key(),
        }
    }

    // lowerY: 最も下にあるブロックのy座標
    pub fn new_with_lower_y(
        mino: &'a Mino,
        x: u8,
        need_deleted_key: u64,
        using_key: u64,
        lower_y: u8,
    ) -> Self {
        Self {
            mino,
            x,
            y: lower_y + (-mino.get_min_y()) as u8, // 回転軸のy座標 (ライン消去後のフィールドに対して置かれるべき位置)に直す
            need_deleted_key,
            using_key,
        }
    }

    pub const fn new(mino: &'a Mino, x: u8, y: u8, need_deleted_key: u64, using_key: u64) -> Self {
        Self {
            mino,
            x,
            y,
            need_deleted_key,
            using_key,
        }
    }
}

impl Action for FullOperationWithKey<'_> {
    fn get_x(&self) -> u8 {
        self.x
    }

    fn get_y(&self) -> u8 {
        self.y
    }

    fn get_rotate(&self) -> Rotate {
        self.get_mino().get_rotate()
    }
}

impl Operation for FullOperationWithKey<'_> {
    fn get_piece(&self) -> Piece {
        self.mino.get_piece()
    }
}

impl MinoOperation for FullOperationWithKey<'_> {
    fn get_mino(&self) -> &Mino {
        self.mino
    }
}

impl OperationWithKey for FullOperationWithKey<'_> {
    fn get_need_deleted_key(&self) -> u64 {
        self.need_deleted_key
    }

    fn get_using_key(&self) -> u64 {
        self.using_key
    }
}

impl MinoOperationWithKey for FullOperationWithKey<'_> {}

impl HashCode for FullOperationWithKey<'_> {
    type Output = u64;

    fn hash_code(&self) -> Self::Output {
        MinoOperationWithKey::default_hash(self)
    }
}

impl PartialEq for FullOperationWithKey<'_> {
    fn eq(&self, other: &Self) -> bool {
        self as &dyn MinoOperationWithKey == other as &_
    }
}

impl Display for FullOperationWithKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self as &dyn OperationWithKey)
    }
}
