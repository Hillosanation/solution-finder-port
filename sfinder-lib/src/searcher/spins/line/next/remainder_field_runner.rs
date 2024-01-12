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
                let next_board = get_next_board(low).unwrap();
                let next_rest_block = SmallField::from(next_board);
                to_remainder_field_pair(low, next_board, Box::new(next_rest_block))
            }
        }
        BoardCount::Middle => {
            let low = rest_block.get_board(0);
            let high = rest_block.get_board(1);

            if let Some(next_board) = get_next_board(low) {
                let next_rest_block = MiddleField::from_parts(next_board, high);
                to_remainder_field_pair(low, next_board, Box::new(next_rest_block))
            } else {
                let next_board = get_next_board(high).unwrap();
                let next_rest_block = MiddleField::from_parts(0, next_board);
                to_remainder_field_pair(high, next_board, Box::new(next_rest_block))
            }
        }
        BoardCount::Large => {
            let low = rest_block.get_board(0);
            let mid_low = rest_block.get_board(1);
            let mid_high = rest_block.get_board(2);
            let high = rest_block.get_board(3);

            if let Some(next_board) = get_next_board(low) {
                let next_rest_block = LargeField::from_parts(next_board, mid_low, mid_high, high);
                to_remainder_field_pair(low, next_board, Box::new(next_rest_block))
            } else if let Some(next_board) = get_next_board(mid_low) {
                let next_rest_block = LargeField::from_parts(0, next_board, mid_high, high);
                to_remainder_field_pair(mid_low, next_board, Box::new(next_rest_block))
            } else if let Some(next_board) = get_next_board(mid_high) {
                let next_rest_block = LargeField::from_parts(0, 0, next_board, high);
                to_remainder_field_pair(mid_high, next_board, Box::new(next_rest_block))
            } else {
                let next_board = get_next_board(high).unwrap();
                let next_rest_block = LargeField::from_parts(0, 0, 0, next_board);
                to_remainder_field_pair(high, next_board, Box::new(next_rest_block))
            }
        }
    }
}

fn get_next_board(board: u64) -> Option<u64> {
    (board != 0).then_some(((board | (board - 1)) + 1) & board)
}

fn to_remainder_field_pair(
    current_board: u64,
    next_board: u64,
    next_rest_block: Box<dyn Field>,
) -> (RemainderField, Box<dyn Field>) {
    // this filters the lowest section of set bits
    let target_board = current_board ^ next_board;
    let min_x = bit_operators::try_get_lowest_x(target_board).unwrap();

    (
        RemainderField::new(min_x, target_board.count_ones() as _),
        next_rest_block,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "__________"
                + "__________"
        );

        let remainder_fields = extract(field.as_ref(), 2);

        assert_eq!(remainder_fields.len(), 1);

        assert_eq!(remainder_fields[0].min_x, 0);
        assert_eq!(remainder_fields[0].target_block_count, 10);
    }

    #[test]
    fn case_2() {
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "__XX__XX__"
        );

        let remainder_fields = extract(field.as_ref(), 0);

        assert_eq!(remainder_fields.len(), 3);

        assert_eq!(remainder_fields[0].min_x, 0);
        assert_eq!(remainder_fields[0].target_block_count, 2);

        assert_eq!(remainder_fields[1].min_x, 4);
        assert_eq!(remainder_fields[1].target_block_count, 2);

        assert_eq!(remainder_fields[2].min_x, 8);
        assert_eq!(remainder_fields[2].target_block_count, 2);
    }
}
