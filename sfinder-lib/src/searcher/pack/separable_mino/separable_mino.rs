use crate::{
    common::datastore::mino_operation_with_key::MinoOperationWithKey,
    sfinder_core::{column_field::column_small_field::ColumnSmallField, field::field::Field},
};

pub trait SeparableMino {
    fn get_lower_y(&self) -> u8;

    fn to_mino_operation_with_key(&self) -> &dyn MinoOperationWithKey;

    // TODO: move around these methods to other trait, since SlideXSeparableMino explicitly does not implement them

    fn get_column_field(&self) -> &ColumnSmallField;

    fn get_field(&self) -> &dyn Field;
}
