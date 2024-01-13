use crate::sfinder_core::column_field::column_field::ColumnField;

const EMPTY: char = '_';
const EXISTS: char = 'X';

pub fn to_string(field: &dyn ColumnField, max_width: u8, max_height: u8) -> String {
    (0..max_height)
        .rev()
        .map(|y| {
            (0..max_width)
                .map(|x| {
                    if field.is_empty_block(x, y, max_height) {
                        EMPTY
                    } else {
                        EXISTS
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_core::{
        column_field::column_field_factory,
        field::field_constants::{BOARD_HEIGHT, FIELD_WIDTH},
    };
    use rand::{thread_rng, Rng};

    #[test]
    fn test() {
        let height = 4;
        let width = 3;

        let mut field = column_field_factory::create_small_field();
        field.set_block(0, 0, height);
        field.set_block(1, 1, height);
        field.set_block(2, 2, height);
        field.set_block(1, 3, height);

        #[rustfmt::skip]
        let expect = [
            "_X_",
            "__X",
            "_X_",
            "X__"
        ].join("\n");

        assert_eq!(to_string(&field, width, height), expect);
    }

    #[test]
    fn test_random() {
        let mut rngs = thread_rng();

        for _ in 0..10000 {
            let width = rngs.gen_range(1..=BOARD_HEIGHT);
            let height = rngs.gen_range(1..=FIELD_WIDTH);

            // create fields
            let fields = (0..height)
                .map(|_| (0..width).map(|_| rngs.gen_bool(0.5)).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            // parse to long
            let mut board = 0;
            for x in 0..width {
                for y in 0..height {
                    board += if fields[y as usize][x as usize] {
                        0
                    } else {
                        1 << (x * height + y)
                    };
                }
            }
            let field = column_field_factory::create_small_field_from_inner(board);

            // parse to strings
            let expect = fields
                .iter()
                .rev()
                .map(|row| {
                    row.iter()
                        .map(|&is_empty| if is_empty { EMPTY } else { EXISTS })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n");

            assert_eq!(to_string(&field, width, height), expect);
        }
    }
}
