use super::{
    field::Field, large_field::LargeField, middle_field::MiddleField, small_field::SmallField,
};
use crate::common::tetfu::{common::color_type::ColorType, field::colored_field::ColoredField};

// TODO: replace 6 with MAX_FIELD_HEIGHT
pub fn create_field(max_height: u8) -> Box<dyn Field> {
    match max_height {
        ..=6 => Box::new(SmallField::new()),
        7..=12 => Box::new(MiddleField::new()),
        13..=24 => Box::new(LargeField::new()),
        _ => panic!("Field height should be equal or less than 24: height={max_height}"),
    }
}

pub fn create_field_with_marks_and_height(marks: String, max_height: u8) -> Box<dyn Field> {
    let mut field = create_field(max_height);
    field.merge(create_field_with_marks(marks).as_ref());
    field
}

pub fn create_field_with_marks(marks: String) -> Box<dyn Field> {
    assert_eq!(marks.len() % 10, 0, "marks length should be multiple of 10");
    match marks.len() / 10 {
        ..=6 => Box::new(create_small_field_with_marks(marks)),
        7..=12 => Box::new(create_middle_field_with_marks(marks)),
        max_y => panic!("Field height is too large {max_y}"),
    }
}

pub fn create_small_field() -> SmallField {
    SmallField::new()
}

pub fn create_small_field_with_marks(marks: String) -> SmallField {
    create_small_field_with_marks_and_block(marks, true)
}

/// This function does not guarentee that the marks can be set to the field and needs to be checked.
fn set_marks_to_field(marks: String, field: &mut dyn Field, is_block: bool) {
    assert!(marks.is_ascii());
    let max_y = (marks.len() / 10) as u8;
    let marks = marks.into_bytes();

    for y in 0..max_y {
        for x in 0..10 {
            match marks[((max_y - y - 1) * 10 + x) as usize] {
                b' ' | b'_' => {
                    if !is_block {
                        field.set_block(x, y)
                    }
                }
                _ => {
                    if is_block {
                        field.set_block(x, y)
                    }
                }
            }
            // dbg!(field.get_x_board());
        }
    }
}

pub fn create_small_field_with_marks_and_block(marks: String, is_block: bool) -> SmallField {
    assert!(marks.len() <= 60, "marks is too long for SmallField");
    assert_eq!(
        marks.len() % 10,
        0,
        "length of marks should be multiple of 10"
    );

    let mut field = create_small_field();

    set_marks_to_field(marks, &mut field, is_block);

    field
}

pub fn create_middle_field() -> MiddleField {
    MiddleField::new()
}

pub fn create_middle_field_with_marks(marks: String) -> MiddleField {
    create_middle_field_with_marks_and_block(marks, true)
}

pub fn create_middle_field_with_marks_and_block(marks: String, is_block: bool) -> MiddleField {
    assert!(marks.len() <= 120, "marks is too long for MiddleField");
    assert_eq!(
        marks.len() % 10,
        0,
        "length of marks should be multiple of 10"
    );

    let mut field = create_middle_field();

    set_marks_to_field(marks, &mut field, is_block);

    field
}

pub fn create_large_field() -> LargeField {
    LargeField::new()
}

fn create_large_field_with_marks_and_block(marks: String, is_block: bool) -> LargeField {
    assert!(marks.len() <= 240, "marks is too long for LargeField");
    assert_eq!(
        marks.len() % 10,
        0,
        "length of marks should be multiple of 10"
    );

    let mut field = create_large_field();

    set_marks_to_field(marks, &mut field, is_block);

    field
}

pub fn create_large_field_with_marks(marks: String) -> LargeField {
    create_large_field_with_marks_and_block(marks, true)
}

// TODO: niche use
pub fn create_inverse_field(marks: String) -> Box<dyn Field> {
    assert_eq!(
        marks.len() % 10,
        0,
        "length of marks should be multiple of 10"
    );

    match marks.len() / 10 {
        ..=6 => Box::new(create_small_field_with_marks_and_block(marks, false)),
        ..=12 => Box::new(create_middle_field_with_marks_and_block(marks, false)),
        max_y => panic!("Field height is too large, {max_y}"),
    }
}

// TODO: move this to From<ColoredField>?
pub fn from_colored_field(colored_field: &dyn ColoredField, height: u8) -> Box<dyn Field> {
    let mut field = create_field(height);

    for y in 0..height {
        for x in 0..10 {
            if colored_field.get_color(x, y) != ColorType::Empty {
                field.set_block(x, y);
            }
        }
    }

    field
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use crate::sfinder_core::field::field::BoardCount;

    use super::*;

    #[test]
    fn test_small() {
        let field = create_field_with_marks(
            String::new()
                + "XXXXX_XXXX"
                + "XXXX_XXXXX"
                + "XXX_XXXXXX"
                + "XX_XXXXXXX"
                + "X_XXXXXXXX"
                + "_XXXXXXXXX",
        );

        assert_eq!(field.get_board_count(), BoardCount::Small);

        for y in 0..6 {
            for x in 0..10 {
                assert_eq!(field.is_empty_block(x, y), x == y);
            }
        }
    }

    #[test]
    fn test_middle() {
        let field = create_field_with_marks(
            String::new()
                + "X_XXXXXXXX"
                + "_XXXXXXXXX"
                + "XXXXXXXXX_"
                + "XXXXXXXX_X"
                + "XXXXXXX_XX"
                + "XXXXXX_XXX"
                + "XXXXX_XXXX"
                + "XXXX_XXXXX"
                + "XXX_XXXXXX"
                + "XX_XXXXXXX"
                + "X_XXXXXXXX"
                + "_XXXXXXXXX",
        );

        assert_eq!(field.get_board_count(), BoardCount::Middle);

        for y in 0..12 {
            for x in 0..10 {
                assert_eq!(field.is_empty_block(x, y), x == y % 10);
            }
        }
    }

    #[test]
    fn test_random() {
        let mut rngs = thread_rng();

        for _ in 0..10000 {
            let height = rngs.gen_range(1..=12);
            let empty_spots: Vec<[_; 10]> = (0..height)
                .map(|_| std::array::from_fn(|_| rngs.gen_bool(0.5)))
                .collect();

            let marks: String = empty_spots
                .iter()
                // start from the top for the string
                .rev()
                .flat_map(|row_spots| {
                    row_spots
                        .iter()
                        .map(|is_empty| if *is_empty { ' ' } else { 'X' })
                })
                .collect();

            // println!("marks: {marks}");
            // println!("empty_spots: {empty_spots:?}");

            let field = create_field_with_marks(marks);

            for y in 0..height {
                for x in 0..10 {
                    assert_eq!(
                        field.is_empty_block(x, y),
                        empty_spots[y as usize][x as usize],
                        "x: {x}, y: {y}"
                    );
                }
            }
        }
    }
}
