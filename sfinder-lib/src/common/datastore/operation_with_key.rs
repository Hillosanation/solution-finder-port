use super::operation::Operation;
use crate::sfinder_core::field::key_operators;
use std::fmt::Display;

pub trait OperationWithKey<Coord>: Operation<Coord>
where
    u32: From<Coord>,
    u64: From<Coord>,
    Coord: Display,
{
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

    // Porting note: Because we need multiple ways of converting to a string, and because of lifetime stuff,
    // it's easier to implement the parsing functions here.

    fn parse_to_string(&self) -> String {
        format!(
            "{},{:?},{},{},{},{}",
            self.get_piece(),
            self.get_rotate(),
            self.get_x(),
            self.get_y(),
            key_operators::to_column_key(self.get_need_deleted_key()),
            key_operators::to_column_key(self.get_using_key())
        )
    }

    fn parse_to_string_simple(&self) -> String {
        format!(
            "{:?},{},{},{}",
            self.get_rotate(),
            self.get_x(),
            self.get_y(),
            key_operators::to_column_key(self.get_using_key())
        )
    }
}
