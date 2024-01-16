use super::{
    mino_rotation::MinoRotation, pattern::_Pattern, rotate_direction::RotateDirection,
    spin_result::SpinResult,
};
use crate::sfinder_core::{
    field::{field::Field, field_constants::FIELD_WIDTH},
    mino::{mino::Mino, mino_factory::MinoFactory},
};

pub struct MinoRotationDetail<'a> {
    mino_factory: &'a MinoFactory,
    mino_rotation: &'a dyn MinoRotation,
}

impl<'a> MinoRotationDetail<'a> {
    pub fn new(mino_factory: &'a MinoFactory, mino_rotation: &'a dyn MinoRotation) -> Self {
        Self {
            mino_factory,
            mino_rotation,
        }
    }

    pub fn get_kicks(
        &self,
        field: &dyn Field,
        direction: RotateDirection,
        before: &'static Mino,
        before_x: u8,
        before_y: u8,
    ) -> Option<SpinResult> {
        let offsets = self.mino_rotation.get_patterns_from(before, direction);
        let after_rotate = before.get_rotate().apply(direction);
        let after = self.mino_factory.get(before.get_piece(), after_rotate);

        // TODO: inline to reduce arguments
        self.get_kicks_inner(field, direction, before, after, before_x, before_y, offsets)
    }

    fn get_kicks_inner(
        &self,
        field: &dyn Field,
        direction: RotateDirection,
        before: &'static Mino,
        after: &'static Mino,
        before_x: u8,
        before_y: u8,
        offsets: &_Pattern,
    ) -> Option<SpinResult> {
        let min_x = -after.get_min_x();
        let max_x = FIELD_WIDTH as i8 - after.get_max_x();
        let min_y = -after.get_min_y();

        offsets
            .get_checks()
            .iter()
            .enumerate()
            .find_map(|(index, (offset, is_privilege_spins))| {
                let to_x = u8::try_from(before_x as i8 + offset.x).unwrap();
                let to_y = u8::try_from(before_y as i8 + offset.y).unwrap();

                (min_x <= (to_x as i8)
                    && (to_x as i8) < max_x
                    && min_y <= (to_y as i8)
                    && field.can_put(after, to_x, to_y))
                .then(|| {
                    SpinResult::new(
                        after,
                        to_x,
                        to_y,
                        index as u8,
                        direction,
                        *is_privilege_spins,
                    )
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entry::common::kicks::factory::srs_mino_rotation_factory,
        sfinder_core::{field::field_factory, mino::piece::Piece, srs::rotate::Rotate},
    };

    fn test_wrapper(
        marks: String,
        direction: RotateDirection,
        before: &'static Mino,
        before_x: u8,
        before_y: u8,
        opt_expected: Option<(u8, u8, RotateDirection, Rotate, u8)>,
    ) {
        let mino_factory = MinoFactory::new();
        let mino_rotation = srs_mino_rotation_factory::create();
        let detail = MinoRotationDetail::new(&mino_factory, &*mino_rotation);

        let field = field_factory::create_field_with_marks(marks);
        let actual = detail.get_kicks(field.as_ref(), direction, before, before_x, before_y);
        if let Some((x, y, direction, to_rotation, test_pattern_index)) = opt_expected {
            let actual = actual.unwrap();
            assert_eq!(actual.x, x);
            assert_eq!(actual.y, y);
            assert_eq!(actual.direction, direction);
            assert_eq!(actual.get_to_rotate(), to_rotation);
            assert_eq!(actual.test_pattern_index, test_pattern_index);
        } else {
            assert_eq!(actual, None);
        }
    }

    #[test]
    fn case_double() {
        #[rustfmt::skip]
        test_wrapper(
            String::new()
                + "X__XXXXXXX"
                + "X___XXXXXX"
                + "XX_XXXXXXX",
            RotateDirection::Clockwise,
            MinoFactory::new().get(Piece::T, Rotate::Right),
            1,
            2,
            Some((2, 1, RotateDirection::Clockwise, Rotate::Reverse, 2)),
        );
    }

    #[test]
    fn case_mini() {
        #[rustfmt::skip]
        test_wrapper(
            String::new()
                + "__________"
                + "XXXXXXXXX_"
                + "XXXXXXXXX_",
            RotateDirection::CounterClockwise,
            MinoFactory::new().get(Piece::T, Rotate::Spawn),
            8,
            2,
            Some((9, 2, RotateDirection::CounterClockwise, Rotate::Left, 1)),
        );
    }

    #[test]
    fn case_triple() {
        test_wrapper(
            String::new()
                + "XXXXX_____"
                + "XXXX______"
                + "XXXX_XXXXX"
                + "XXXX__XXXX"
                + "XXXX_XXXXX",
            RotateDirection::Clockwise,
            MinoFactory::new().get(Piece::T, Rotate::Spawn),
            5,
            3,
            Some((4, 1, RotateDirection::Clockwise, Rotate::Right, 4)),
        );
    }

    #[test]
    fn case_neo() {
        test_wrapper(
            String::new()
                + "___XXXXXXX"
                + "_____XXXXX"
                + "____XXXXXX"
                + "XX__XXXXXX"
                + "XXX_XXXXXX",
            RotateDirection::Clockwise,
            MinoFactory::new().get(Piece::T, Rotate::Reverse),
            3,
            3,
            Some((3, 1, RotateDirection::Clockwise, Rotate::Left, 3)),
        );
    }

    #[test]
    fn case_fin() {
        test_wrapper(
            String::new()
                + "XXXXXX____"
                + "XXXX______"
                + "XXXX______"
                + "XXXX__XXXX"
                + "XXXX_XXXXX",
            RotateDirection::CounterClockwise,
            MinoFactory::new().get(Piece::T, Rotate::Reverse),
            5,
            3,
            Some((4, 1, RotateDirection::CounterClockwise, Rotate::Right, 4)),
        );
    }

    #[test]
    fn case_iso() {
        test_wrapper(
            String::new()
                + "XXXXXXX___"
                + "XXXX______"
                + "XXXXX_____"
                + "XXXX__XXXX"
                + "XXXXX_XXXX",
            RotateDirection::Clockwise,
            MinoFactory::new().get(Piece::T, Rotate::Reverse),
            5,
            3,
            Some((5, 1, RotateDirection::Clockwise, Rotate::Left, 3)),
        );
    }
}
