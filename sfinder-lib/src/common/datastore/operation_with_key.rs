use super::operation::Operation;
use crate::sfinder_core::field::key_operators;
use std::fmt::Display;

pub trait OperationWithKey: Operation {
    fn get_using_key(&self) -> u64;

    fn get_need_deleted_key(&self) -> u64;

    // Porting note: renamed to avoid shadowing the trait method
    // Use the qualified name to access the other toUniqueKey functions.
    fn to_unique_key_with_delete_key(&self) -> u64 {
        const MASK_LOW: u64 = (1 << 30) - 1;
        const MASK_HIGH: u64 = MASK_LOW << 30;

        let need_deleted_key = self.get_need_deleted_key();
        let unique_deleted_key =
            (need_deleted_key & MASK_HIGH) | (need_deleted_key & MASK_LOW) << 35;

        unique_deleted_key + self.to_unique_key()
    }
}

impl PartialEq for dyn OperationWithKey + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.get_piece() == other.get_piece()
            && self.get_rotate() == other.get_rotate()
            && self.get_x() == other.get_x()
            && self.get_y() == other.get_y()
            && self.get_need_deleted_key() == other.get_need_deleted_key()
    }
}

impl PartialOrd for dyn OperationWithKey + '_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_piece().cmp(&other.get_piece()) {
            std::cmp::Ordering::Equal => {}
            ordering => return Some(ordering),
        }

        match self.get_rotate().cmp(&other.get_rotate()) {
            std::cmp::Ordering::Equal => {}
            ordering => return Some(ordering),
        }

        match self.get_x().cmp(&other.get_x()) {
            std::cmp::Ordering::Equal => {}
            ordering => return Some(ordering),
        }

        match self.get_y().cmp(&other.get_y()) {
            std::cmp::Ordering::Equal => {}
            ordering => return Some(ordering),
        }

        self.get_need_deleted_key()
            .partial_cmp(&other.get_need_deleted_key())
    }
}

impl Display for dyn OperationWithKey + '_ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            // Porting note: the alternate formatting replaces parseToStringSimple
            write!(
                f,
                "{},{},{},{}",
                self.get_rotate(),
                self.get_x(),
                self.get_y(),
                key_operators::to_column_key(self.get_using_key())
            )
        } else {
            // Porting note: the default formatting replaces parseToString
            write!(
                f,
                "{},{},{},{},{},{}",
                self.get_piece(),
                self.get_rotate(),
                self.get_x(),
                self.get_y(),
                key_operators::to_column_key(self.get_need_deleted_key()),
                key_operators::to_column_key(self.get_using_key())
            )
        }
    }
}
