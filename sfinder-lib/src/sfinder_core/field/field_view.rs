use crate::sfinder_core::field::field_constants::{BOARD_HEIGHT, FIELD_WIDTH};

use super::field::Field;

const EXISTS: char = 'X';
const EMPTY: char = '_';

pub fn to_string(field: &dyn Field) -> String {
    to_string_with_height(field, field.get_max_field_height())
}

pub fn to_string_with_height(field: &dyn Field, max_field_height: u8) -> String {
    assert!(max_field_height <= field.get_board_count() as u8 * BOARD_HEIGHT);

    let mut result = String::new();
    for y in (0..max_field_height).rev() {
        for x in 0..FIELD_WIDTH {
            result.push(if field.is_empty_block(x, y) {
                EMPTY
            } else {
                EXISTS
            });
        }
        result.push('\n');
    }

    result
}

pub fn to_reduced_string(field: &dyn Field) -> String {
    let max_field_height = field.get_max_field_height();

    (0..max_field_height)
        .rev()
        .map(|y| {
            (0..FIELD_WIDTH)
                .map(|x| {
                    if field.is_empty_block(x, y) {
                        EMPTY
                    } else {
                        EXISTS
                    }
                })
                .collect::<String>()
        })
        .skip_while(|line| line == "__________")
        .collect::<Vec<String>>()
        .join("\n")
}
