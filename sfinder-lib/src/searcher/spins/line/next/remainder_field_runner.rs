//! Helper struct used only by LineFillRunner

use super::remainder_field::RemainderField;
use crate::sfinder_core::field::{
    bit_operators, field::Field, field_constants::BoardCount, field_factory, key_operators,
    large_field::LargeField, middle_field::MiddleField, small_field::SmallField,
};

pub fn extract(init_field: &dyn Field, target_y: u8) -> Vec<RemainderField> {
    let max_field_height = init_field.get_max_field_height();

    let mut remainder_block = field_factory::create_field(max_field_height);
    let bit_key = key_operators::get_bit_key(target_y);

    remainder_block.insert_filled_row_with_key(bit_key);
    remainder_block.reduce(init_field);

    extract_inner(remainder_block)
}

fn extract_inner(mut remainder_block: Box<dyn Field>) -> Vec<RemainderField> {
    let max_board_count = remainder_block.get_board_count();

    let mut pairs = Vec::new();

    assert!(!remainder_block.is_empty());

    while !remainder_block.is_empty() {
        let (remainder_field, ret_remainder_block) =
            calc_remainder_field_pair(remainder_block.as_ref(), &max_board_count);

        pairs.push(remainder_field);

        remainder_block = ret_remainder_block;
    }

    assert!(!pairs.is_empty());

    pairs
}

fn calc_remainder_field_pair(
    rest_block: &dyn Field,
    max_board_count: &BoardCount,
) -> (RemainderField, Box<dyn Field>) {
    debug_assert!(!rest_block.is_empty());

    match max_board_count {
        BoardCount::Small => {
            let low = rest_block.get_board(0);

            {
                let next_board = get_next_board(low);
                let next_rest_block = SmallField::from(next_board);
                to_remainder_field_pair(low, next_board, Box::new(next_rest_block)).unwrap()
            }
        }
        BoardCount::Middle => {
            let low = rest_block.get_board(0);
            let high = rest_block.get_board(1);

            if low != 0 {
                let next_board = get_next_board(low);
                let next_rest_block = MiddleField::from_parts(next_board, high);
                to_remainder_field_pair(low, next_board, Box::new(next_rest_block)).unwrap()
            } else {
                let next_board = get_next_board(high);
                let next_rest_block = MiddleField::from_parts(low, next_board);
                to_remainder_field_pair(high, next_board, Box::new(next_rest_block)).unwrap()
            }
        }
        BoardCount::Large => {
            let low = rest_block.get_board(0);
            let mid_low = rest_block.get_board(1);
            let mid_high = rest_block.get_board(2);
            let high = rest_block.get_board(3);

            if low != 0 {
                let next_board = get_next_board(low);
                let next_rest_block = LargeField::from_parts(next_board, mid_low, mid_high, high);
                to_remainder_field_pair(low, next_board, Box::new(next_rest_block)).unwrap()
            } else if mid_low != 0 {
                let next_board = get_next_board(mid_low);
                let next_rest_block = LargeField::from_parts(0, next_board, mid_high, high);
                to_remainder_field_pair(mid_low, next_board, Box::new(next_rest_block)).unwrap()
            } else if mid_high != 0 {
                let next_board = get_next_board(mid_high);
                let next_rest_block = LargeField::from_parts(0, 0, next_board, high);
                to_remainder_field_pair(mid_high, next_board, Box::new(next_rest_block)).unwrap()
            } else {
                let next_board = get_next_board(high);
                let next_rest_block = LargeField::from_parts(0, 0, 0, next_board);
                to_remainder_field_pair(high, next_board, Box::new(next_rest_block)).unwrap()
            }
        }
    }
}

fn get_next_board(board: u64) -> u64 {
    ((board | (board - 1)) + 1) & board
}

fn to_remainder_field_pair(
    current_board: u64,
    next_board: u64,
    next_rest_block: Box<dyn Field>,
) -> Option<(RemainderField, Box<dyn Field>)> {
    // this filters the lowest section of set bits
    let target_board = current_board ^ next_board;
    let min_x = bit_operators::try_get_lowest_x(target_board)?;

    Some((
        RemainderField::new(min_x, target_board.count_ones() as _),
        next_rest_block,
    ))
}
