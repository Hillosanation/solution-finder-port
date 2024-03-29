//! Helper struct used only by LineFillRunner

use super::remainder_field::RemainderField;
use crate::sfinder_core::field::{bit_operators, field::Field};

pub fn extract(init_field: &dyn Field, target_y: u8) -> Vec<RemainderField> {
    let mut field = dyn_clone::clone_box(init_field);
    field.slide_down(target_y);

    let board = field.get_board(0);
    const BOTTOMMOST_ROW: u64 = bit_operators::get_row_mask(0);

    // only the lowermost row will have filled blocks
    extract_inner(!board & BOTTOMMOST_ROW)
}

fn extract_inner(mut remainder_board: u64) -> Vec<RemainderField> {
    assert!(remainder_board != 0);
    let mut pairs = Vec::new();

    while remainder_board != 0 {
        // removes the lowest continuous section of set bits
        // never overflows since remainder_board != 0
        let next_board = ((remainder_board | (remainder_board - 1)) + 1) & remainder_board;

        // parse the lowest continuous section of set bits
        pairs.push(to_remainder_field(remainder_board ^ next_board));

        remainder_board = next_board;
    }

    pairs
}

fn to_remainder_field(target_board: u64) -> RemainderField {
    assert!(target_board != 0);
    // since only the lowermost row can have filled blocks, trailing_zeros directly corresponds to the x coordinate
    let min_x = target_board.trailing_zeros() as _;

    RemainderField::new(min_x, target_board.count_ones() as _)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        sfinder_core::field::{field_constants::FIELD_WIDTH, field_factory},
        sfinder_lib::randoms,
    };
    use rand::{thread_rng, Rng};

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

    #[test]
    fn random() {
        // this test doesn't balance the other branches, since it's likely that the lower boards are not completely empty
        let mut rngs = thread_rng();

        for _ in 0..10000 {
            let height = rngs.gen_range(1..24);
            let num_of_empty_minos = rngs.gen_range(0..(height * FIELD_WIDTH) / 4);
            let field = randoms::gen_field(&mut rngs, height, num_of_empty_minos);

            for y in 0..height {
                let mut row = field_factory::create_field(height);
                row.fill_row(y);

                // println!("{height} {y}");
                // println!("{row:?}");
                // println!("{field:?}");

                row.reduce(field.as_ref());

                if row.is_empty() {
                    continue;
                }

                let remainder_fields = extract(field.as_ref(), y);
                // println!("{remainder_fields:?}");

                let mut expected_runs = Vec::new();
                {
                    // manually find run length of row
                    let mut current_start = 0;
                    let mut current_run = 0;
                    for x in 0..FIELD_WIDTH {
                        if field.is_empty_block(x, y) {
                            if current_run == 0 {
                                current_start = x;
                            }
                            current_run += 1;
                        } else {
                            if current_run > 0 {
                                expected_runs.push(RemainderField::new(current_start, current_run));
                                current_run = 0;
                            }
                        }
                    }
                    if current_run > 0 {
                        expected_runs.push(RemainderField::new(current_start, current_run));
                    }
                }

                assert_eq!(remainder_fields, expected_runs);
            }
        }
    }
}
