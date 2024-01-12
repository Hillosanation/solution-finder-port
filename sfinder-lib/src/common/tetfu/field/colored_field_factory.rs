use super::{array_colored_field::ArrayColoredField, colored_field::ColoredField};
use crate::{
    common::tetfu::common::color_type::ColorType,
    sfinder_core::field::{field::Field, field_constants::FIELD_WIDTH},
};

const MAX_HEIGHT: u8 = 24;

pub fn create_field(max_height: u8) -> ArrayColoredField {
    ArrayColoredField::new(max_height)
}

pub fn create_colored_field(marks: String) -> ArrayColoredField {
    assert_eq!(
        marks.len() % FIELD_WIDTH as usize,
        0,
        "marks length must be multiple of 10"
    );
    assert!(marks.is_ascii());

    let max_y = (marks.len() / FIELD_WIDTH as usize) as u8;
    assert!(max_y <= MAX_HEIGHT);

    let mut field = ArrayColoredField::new(MAX_HEIGHT);
    let marks = marks.into_bytes();
    for y in 0..max_y {
        for x in 0..FIELD_WIDTH {
            let mark = marks[((max_y - y - 1) * FIELD_WIDTH + x) as usize];
            let color_type = get(mark);
            if color_type != ColorType::Empty {
                field.set_color(x, y, color_type)
            }
        }
    }

    field
}

fn get(s: u8) -> ColorType {
    match s {
        b'I' => ColorType::I,
        b'L' => ColorType::L,
        b'O' => ColorType::O,
        b'Z' => ColorType::Z,
        b'T' => ColorType::T,
        b'J' => ColorType::J,
        b'S' => ColorType::S,
        b' ' | b'_' => ColorType::Empty,
        b'x' | b'*' | b'.' | _ => ColorType::Gray,
    }
}

// TODO: move to impl From<&dyn Field> for ArrayColoredField?
pub fn create_gray_field(field: &dyn Field) -> ArrayColoredField {
    let mut colored_field = ArrayColoredField::new(MAX_HEIGHT);

    for y in 0..field.get_max_field_height() {
        for x in 0..FIELD_WIDTH {
            if field.exists_block(x, y) {
                colored_field.set_color(x, y, ColorType::Gray);
            }
        }
    }

    colored_field
}
