use super::field_constants::FIELD_WIDTH;
use crate::common::datastore::block_field::BlockField;

const EMPTY_CHAR: char = '_';

pub fn to_string(block_field: &BlockField) -> String {
    to_string_with_height(block_field, block_field.get_height())
}

pub fn to_string_with_height(block_field: &BlockField, max_height: u8) -> String {
    (0..max_height)
        .rev()
        .map(|y| {
            (0..FIELD_WIDTH)
                .map(|x| {
                    if let Some(piece) = block_field.get_piece_of_block(x, y) {
                        piece.to_string()
                    } else {
                        EMPTY_CHAR.to_string()
                    }
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
