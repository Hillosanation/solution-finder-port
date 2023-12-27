use super::{field::Field, small_field::SmallField};

// TODO: replace 6 with MAX_FIELD_HEIGHT
pub fn create_field(max_height: u8) -> Box<dyn Field> {
    match max_height {
        ..=6 => Box::new(SmallField::new()),
        7..=12 => todo!(),
        13..=24 => todo!(),
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

fn create_small_field_with_marks_and_block(marks: String, is_block: bool) -> SmallField {
    // these conditions are already checked
    debug_assert!(marks.len() <= 60, "marks is too long for SmallField");
    debug_assert_eq!(marks.len() % 10, 0, "marks length should be multiple of 10");

    let max_y = (marks.len() / 10) as u8;
    let mut field = create_small_field();
    let marks = marks.into_bytes();
    for y in 0..max_y {
        for x in 0..10 {
            // dbg!(marks[((max_y - y - 1) * 10 + x) as usize], x, y);
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

    field
}

pub fn create_middle_field() {
    todo!("MiddleField")
}

pub fn create_middle_field_with_marks(marks: String) -> SmallField {
    create_middle_field_with_marks_and_block(marks, true)
}

fn create_middle_field_with_marks_and_block(marks: String, is_block: bool) -> SmallField {
    todo!("MiddleField")
}

pub fn create_large_field() {
    todo!("LargeField")
}

fn create_large_field_with_marks(marks: String, is_block: bool) -> SmallField {
    todo!("LargeField")
}

// TODO: niche use
pub fn create_inverse_field(marks: String) -> ! {
    todo!()
}

pub fn from_colored_field() {
    todo!("ColoredField")
}
