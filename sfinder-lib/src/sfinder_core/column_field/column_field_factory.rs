use super::column_small_field::ColumnSmallField;
use crate::sfinder_core::{
    column_field::column_field::ColumnField, field::field_constants::BOARD_HEIGHT,
};

pub fn create_small_field() -> ColumnSmallField {
    ColumnSmallField::new()
}

// Porting note: merged the two constructors that took in boards
// TODO: change API to only accept one board? create_small_field already provides a way to a new ColumnSmallField
pub fn create_small_field_from_inner(boards: &[u64]) -> ColumnSmallField {
    match boards {
        [] => ColumnSmallField::new(),
        [board] => ColumnSmallField::from(*board),
        _ => panic!("Too many boards"),
    }
}

pub fn create_small_field_with_marks(marks: String, height: u8) -> ColumnSmallField {
    assert!(marks.is_ascii());

    let max = BOARD_HEIGHT * height;
    assert!(
        max >= marks.len() as u8,
        "length of marks is too long for height={height}"
    );

    assert_eq!(
        marks.len() as u8 % height,
        0,
        "length of marks must be a multiple of height={height}"
    );

    let mut field = ColumnSmallField::new();
    let width = marks.len() as u8 / height;
    let bytes = marks.into_bytes();
    for x in 0..width {
        for y in 0..height {
            if !matches!(bytes[((height - y - 1) * width + x) as usize], b' ' | b'_') {
                field.set_block(x, y, height);
            }
        }
    }

    field
}

#[cfg(test)]
mod tests {
    use crate::sfinder_core::column_field::column_field_factory;

    use super::*;

    #[test]
    fn create_field() {
        assert_eq!(
            column_field_factory::create_small_field().get_num_of_all_blocks(),
            0
        );
    }

    #[test]
    fn create_field1() {
        let field = column_field_factory::create_small_field_from_inner(&[0b1110]);
        assert_eq!(field.get_num_of_all_blocks(), 3);
        assert!(field.is_empty(0, 0, 4));
        assert!(!field.is_empty(0, 1, 4));
        assert!(!field.is_empty(0, 2, 4));
        assert!(!field.is_empty(0, 3, 4));
        assert!(field.is_empty(1, 0, 4));
    }

    #[test]
    fn create_field_2() {
        assert_eq!(
            column_field_factory::create_small_field_from_inner(&[]).get_num_of_all_blocks(),
            0
        );
    }

    #[test]
    fn create_field_3() {
        let field = column_field_factory::create_small_field_from_inner(&[0b1010101]);
        assert_eq!(field.get_num_of_all_blocks(), 4);
        assert!(!field.is_empty(0, 0, 4));
        assert!(field.is_empty(0, 1, 4));
        assert!(!field.is_empty(0, 2, 4));
        assert!(field.is_empty(0, 3, 4));
        assert!(!field.is_empty(1, 0, 4));
    }

    #[test]
    fn create_field_4() {
        #[rustfmt::skip]
        let field = column_field_factory::create_small_field_with_marks(
            String::new()
                + "_X_"
                + "__X"
                + "_X_"
                + "X__",
            4
        );

        assert_eq!(field.get_num_of_all_blocks(), 4);
        assert!(!field.is_empty(0, 0, 4));
        assert!(field.is_empty(0, 1, 4));
        assert!(field.is_empty(0, 2, 4));
        assert!(field.is_empty(0, 3, 4));
    }
}
