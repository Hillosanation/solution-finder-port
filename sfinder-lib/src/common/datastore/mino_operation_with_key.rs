use super::operation_with_key::OperationWithKey;
use crate::{
    common::datastore::mino_operation::MinoOperation,
    extras::hash_code::HashCode,
    sfinder_core::field::{field::Field, field_constants::FIELD_WIDTH, field_factory},
};

pub trait MinoOperationWithKey: OperationWithKey + MinoOperation {
    fn default_hash(&self) -> u64 {
        let mut result = u64::from(self.get_y());
        result = FIELD_WIDTH as u64 * result + u64::from(self.get_x());
        result = 31 * result + self.get_mino().hash_code() as u64;

        let need_deleted_key = self.get_need_deleted_key();
        result = 31 * result + need_deleted_key ^ need_deleted_key >> 32;

        result
    }

    fn create_mino_field(&self, max_height: u8) -> Box<dyn Field> {
        let mut field = field_factory::create_field(max_height);
        field.put(self.get_mino(), self.get_x(), self.get_y());
        field.insert_blank_row_with_key(self.get_need_deleted_key());
        field
    }
}

// When downcasted to this trait, the only way to differentiate between structs are the functions that are available
// get_using_key should be determined by the other functions, so no need to check if using_key are equal
impl PartialEq for dyn MinoOperationWithKey + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.get_x() == other.get_x()
            && self.get_y() == other.get_y()
            && self.get_need_deleted_key() == other.get_need_deleted_key()
            && self.get_mino() == other.get_mino()
    }
}

impl Eq for dyn MinoOperationWithKey + '_ {}

impl PartialOrd for dyn MinoOperationWithKey + '_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for dyn MinoOperationWithKey + '_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.get_x().cmp(&other.get_x()))
            .then(self.get_y().cmp(&other.get_y()))
            .then(
                self.get_need_deleted_key()
                    .cmp(&other.get_need_deleted_key()),
            )
            .then(self.get_mino().cmp(&other.get_mino()))
    }
}
