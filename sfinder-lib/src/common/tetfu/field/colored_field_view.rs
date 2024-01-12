use super::colored_field::ColoredField;
use crate::sfinder_core::field::field_constants::FIELD_WIDTH;

pub fn to_string(field: &dyn ColoredField) -> String {
    to_string_with_height(field, field.get_max_height())
}

pub fn to_string_with_height(field: &dyn ColoredField, get_max_height: usize) -> String {
    let mut result = String::new();
    for y in (0..get_max_height as u8).rev() {
        for x in 0..FIELD_WIDTH {
            let color = field.get_color(x, y);
            result += &(color as usize).to_string();
        }
        result.push('\n');
    }
    result
}

pub fn to_string_with_type(field: &dyn ColoredField) -> String {
    to_string_with_type_with_height(field, field.get_max_height())
}

pub fn to_string_with_type_with_height(field: &dyn ColoredField, get_max_height: usize) -> String {
    let mut result = String::new();
    for y in (0..get_max_height as u8).rev() {
        for x in 0..FIELD_WIDTH {
            let color = field.get_color(x, y);
            result += &color.to_string();
        }
        result.push('\n');
    }
    result
}
