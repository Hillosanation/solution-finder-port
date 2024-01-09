use super::{mask::mino_mask_factory, separable_mino::SeparableMino};
use crate::{
    common::datastore::{
        action::action::Action, full_operation_with_key::FullOperationWithKey,
        mino_operation::MinoOperation, mino_operation_with_key::MinoOperationWithKey,
        operation_with_key::OperationWithKey,
    },
    sfinder_core::{
        column_field::{
            column_field::ColumnField, column_field_factory, column_small_field::ColumnSmallField,
        },
        field::field::Field,
    },
};

#[derive(Debug)]
pub struct FullOperationSeparableMino<'m> {
    operation: FullOperationWithKey<'m>,
    column_field: ColumnSmallField,
    lower_y: u8,
    field: Box<dyn Field>,
}

impl<'m> FullOperationSeparableMino<'m> {
    pub fn new(
        operation_with_key: FullOperationWithKey<'m>,
        upper_y: u8,
        field_height: u8,
    ) -> Self {
        assert!(upper_y <= 10);

        let x = operation_with_key.get_x();
        let y = operation_with_key.get_y();
        let mino = operation_with_key.get_mino();
        let delete_key = operation_with_key.get_need_deleted_key();

        let mino_mask = mino_mask_factory::create(field_height, mino, y, delete_key);
        let mask = mino_mask.get_mino_mask(x);

        let mut column_small_field = column_field_factory::create_small_field();
        let lower_y = u8::try_from(y as i8 + mino.get_min_y()).unwrap();
        for ny in lower_y..=upper_y {
            for nx in u8::try_from(x as i8 + mino.get_min_x()).unwrap()
                ..=u8::try_from(x as i8 + mino.get_max_x()).unwrap()
            {
                if mask.exists_block(x, y) {
                    column_small_field.set_block(nx, ny, field_height);
                }
            }
        }

        let field = operation_with_key.create_mino_field(field_height);

        Self {
            operation: operation_with_key,
            column_field: column_small_field,
            lower_y,
            field,
        }
    }

    // Porting note: used in SeparableMinos to destructure the MinoOperationWithKey from the SeparableMino.
    // TODO: Determine if this can replace get_mino_operation_with_key in SeparableMino
    pub fn to_mino_operation_with_key(self) -> Box<dyn MinoOperationWithKey + 'm> {
        Box::new(self.operation)
    }
}

impl SeparableMino for FullOperationSeparableMino<'_> {
    fn get_lower_y(&self) -> u8 {
        self.lower_y
    }

    fn get_mino_operation_with_key(&self) -> &dyn MinoOperationWithKey {
        &self.operation
    }

    fn get_column_field(&self) -> &ColumnSmallField {
        &self.column_field
    }

    fn get_field(&self) -> &dyn Field {
        self.field.as_ref()
    }
}

impl PartialEq for FullOperationSeparableMino<'_> {
    fn eq(&self, other: &Self) -> bool {
        <dyn OperationWithKey>::eq(&self.operation, &other.operation)
    }
}

impl PartialOrd for FullOperationSeparableMino<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        <dyn OperationWithKey>::partial_cmp(&self.operation, &other.operation)
    }
}
