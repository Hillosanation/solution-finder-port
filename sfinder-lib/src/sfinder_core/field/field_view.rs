use crate::sfinder_core::field::field_constants::{BOARD_HEIGHT, FIELD_WIDTH};

use super::field::Field;

const EXISTS: char = 'X';
const EMPTY: char = '_';

pub fn to_string(field: &dyn Field) -> String {
    to_string_with_height(field, field.get_max_field_height())
}

pub fn to_string_with_height(field: &dyn Field, max_field_height: u8) -> String {
    assert!(max_field_height <= field.get_board_count() as u8 * BOARD_HEIGHT);

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
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

// TODO: this can reuse to_string_with_height if get_max_y was implemented, but it's already possible, just gated behined get_upper_y_with_4_blocks
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_core::field::field_factory;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_random() {
        let mut rngs = thread_rng();

        for _ in 0..10000 {
            let height = rngs.gen_range(1..=12);
            let empty_spots: Vec<[_; FIELD_WIDTH as usize]> = (0..height)
                .map(|_| std::array::from_fn(|_| rngs.gen_bool(0.5)))
                .collect();

            let marks = empty_spots
                .iter()
                // start from the top for the string
                .rev()
                .map(|row| {
                    row.iter()
                        .map(|&is_empty| if is_empty { EMPTY } else { EXISTS })
                        .collect::<String>()
                })
                .collect::<Vec<String>>();

            let field = field_factory::create_field_with_marks(marks.concat());

            assert_eq!(
                to_string_with_height(field.as_ref(), height),
                marks.join("\n")
            );
        }
    }

    #[test]
    fn test_random_with_height() {
        let mut rngs = thread_rng();

        for _ in 0..10000 {
            let height = rngs.gen_range(1..=12);

            // create field
            let empty_spots: Vec<[_; FIELD_WIDTH as usize]> = (0..height)
                .map(|_| std::array::from_fn(|_| rngs.gen_bool(0.5)))
                .collect();

            // parse to string for factory
            let marks = empty_spots
                .iter()
                // start from the top for the string
                .rev()
                .map(|row| {
                    row.iter()
                        .map(|&is_empty| if is_empty { EMPTY } else { EXISTS })
                        .collect::<String>()
                })
                .collect::<Vec<String>>();

            let empty_line = (12 - height) % 6;

            let expected = (0..empty_line)
                .map(|_| "__________")
                .chain(marks.iter().map(|string| string.as_str()))
                .map(|str| str.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            let field = field_factory::create_field_with_marks(marks.concat());
            assert_eq!(to_string(field.as_ref()), expected);
        }
    }
}
