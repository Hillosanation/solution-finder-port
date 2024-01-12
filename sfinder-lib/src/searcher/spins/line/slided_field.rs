//! Helper struct used only by LineFillRunner

use crate::sfinder_core::field::{field::Field, key_operators};

pub struct SlidedField {
    field: Box<dyn Field>,
    filled_line: u64,
    slide_down_y: i8,
}

impl SlidedField {
    pub fn new(field: Box<dyn Field>, target_y: u8, allow_deleted_line: u64) -> Self {
        assert_eq!(
            field.get_filled_rows_key() & allow_deleted_line,
            allow_deleted_line
        );

        let mut freeze = field.clone();
        freeze.delete_rows_with_key(allow_deleted_line);

        let bit_count =
            (allow_deleted_line & key_operators::get_mask_for_key_below_y(target_y)).count_ones();

        let slide_down_y = (target_y as i8 - bit_count as i8) - 3;
        if slide_down_y.is_positive() {
            freeze.slide_down(slide_down_y as u8);
        } else {
            freeze.slide_up_with_filled_row((-slide_down_y) as u8);
        }

        Self {
            field: freeze,
            filled_line: allow_deleted_line,
            slide_down_y,
        }
    }

    pub fn get_field(&self) -> &dyn Field {
        self.field.as_ref()
    }

    pub fn get_filled_line(&self) -> u64 {
        self.filled_line
    }

    pub fn get_slide_down_y(&self) -> i8 {
        self.slide_down_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_core::field::field_factory;

    #[test]
    fn case_1() {
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "XXXXX_____"
                + "XXXXX_____",
        );
        let filled_rows = field.get_filled_rows_key();
        let slided_field = SlidedField::new(field, 1, filled_rows);
        #[rustfmt::skip]
        let expected_field = field_factory::create_field_with_marks(
            String::new()
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
        );

        assert_eq!(slided_field.get_field(), expected_field.as_ref());
        assert_eq!(slided_field.get_slide_down_y(), -2);
        assert_eq!(slided_field.get_filled_line(), 0);
    }

    #[test]
    fn case_2() {
        let field = field_factory::create_field_with_marks(
            String::new()
                + "X_________"
                + "XXXXXXXXXX"
                + "XX________"
                + "XXXXXXXXXX"
                + "XXX_______"
                + "XXXX______",
        );
        let filled_rows = field.get_filled_rows_key();
        let slided_field = SlidedField::new(field, 5, filled_rows);
        #[rustfmt::skip]
        let expected_field = field_factory::create_field_with_marks(
            String::new()
                + "X_________"
                + "XX________"
                + "XXX_______"
                + "XXXX______"
        );

        assert_eq!(slided_field.get_field(), expected_field.as_ref());
        assert_eq!(slided_field.get_slide_down_y(), 0);
        assert_eq!(
            slided_field.get_filled_line(),
            key_operators::get_bit_keys(&[2, 4])
        );
    }

    #[test]
    fn case_3() {
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "X_________"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX",
        );
        let filled_rows = field.get_filled_rows_key();
        let slided_field = SlidedField::new(field, 2, filled_rows);
        #[rustfmt::skip]
        let expected_field = field_factory::create_field_with_marks(
            String::new()
                + "X_________"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
        );

        assert_eq!(slided_field.get_field(), expected_field.as_ref());
        assert_eq!(slided_field.get_slide_down_y(), -3);
        assert_eq!(
            slided_field.get_filled_line(),
            key_operators::get_bit_keys(&[0, 1])
        );
    }
}
