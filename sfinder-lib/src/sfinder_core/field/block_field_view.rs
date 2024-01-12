use super::field_constants::FIELD_WIDTH;
use crate::common::datastore::block_field::BlockField;

const EMPTY_CHAR: char = '_';

pub fn to_string(block_field: &BlockField) -> String {
    to_string_with_height(block_field, block_field.get_height())
}

pub fn to_string_with_height(block_field: &BlockField, max_height: u8) -> String {
    let mut result = String::new();
    for y in (0..max_height).rev() {
        for x in 0..FIELD_WIDTH {
            if let Some(piece) = block_field.get_piece_of_block(x, y) {
                result += &piece.to_string();
            } else {
                result.push(EMPTY_CHAR);
            }
        }
        result.push('\n');
    }
    result
}
