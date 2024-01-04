use super::operation_with_key::OperationWithKey;
use crate::{
    common::datastore::mino_operation::MinoOperation,
    extras::hash_code::HashCode,
    sfinder_core::field::{field::Field, field_constants::FIELD_WIDTH, field_factory},
};

pub trait MinoOperationWithKey: OperationWithKey + MinoOperation {
    fn default_hash(&self) -> u32 {
        let mut result = u32::from(self.get_y());
        result = FIELD_WIDTH as u32 * result + u32::from(self.get_x());
        result = 31 * result + self.get_mino().hash_code() as u32;

        let need_deleted_key = self.get_need_deleted_key();
        result = 31 * result + (need_deleted_key ^ need_deleted_key >> 32) as u32;

        result
    }

    fn create_mino_field(&self, max_height: u8) -> Box<dyn Field> {
        let mut field = field_factory::create_field(max_height);
        field.put(self.get_mino(), self.get_x(), self.get_y());
        field.insert_blank_row_with_key(self.get_need_deleted_key());
        field
    }
}

impl<'a> PartialEq for dyn MinoOperationWithKey + 'a {
    fn eq(&self, other: &Self) -> bool {
        self.get_x() == other.get_x()
            && self.get_y() == other.get_y()
            && self.get_need_deleted_key() == other.get_need_deleted_key()
            && self.get_mino() == other.get_mino()
    }
}
