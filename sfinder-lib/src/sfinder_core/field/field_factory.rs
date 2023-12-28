use super::{field::Field, middle_field::MiddleField, small_field::SmallField};

// TODO: replace 6 with MAX_FIELD_HEIGHT
pub fn create_field(max_height: u8) -> Box<dyn Field> {
    match max_height {
        ..=6 => Box::new(SmallField::new()),
        7..=12 => Box::new(MiddleField::new()),
        13..=24 => todo!("LargeField"),
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

pub fn create_large_field() {
    todo!("LargeField")
}

fn create_large_field_with_marks(marks: String, is_block: bool) -> SmallField {
    todo!("LargeField")
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

pub fn from_colored_field() {
    todo!("ColoredField")
}
