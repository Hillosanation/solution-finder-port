use super::colored_field::ColoredField;
use crate::sfinder_core::field::field_constants::FIELD_WIDTH;

pub fn to_string(field: &dyn ColoredField) -> String {
    to_string_with_height(field, field.get_max_height())
}

pub fn to_string_with_height(field: &dyn ColoredField, get_max_height: usize) -> String {
    (0..get_max_height as u8)
        .rev()
        .map(|y| {
            (0..FIELD_WIDTH)
                .map(|x| (field.get_color(x, y) as usize).to_string())
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn to_string_with_type(field: &dyn ColoredField) -> String {
    to_string_with_type_with_height(field, field.get_max_height())
}

pub fn to_string_with_type_with_height(field: &dyn ColoredField, get_max_height: usize) -> String {
    (0..get_max_height as u8)
        .rev()
        .map(|y| {
            (0..FIELD_WIDTH)
                .map(|x| field.get_color(x, y).to_string())
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
