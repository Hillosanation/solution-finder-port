use super::{pattern::Pattern, rotate::Rotate, rotate_direction::RotateDirection};
use crate::{
    common::datastore::coordinate::Coordinate,
    sfinder_core::{
        field::{field::Field, field_constants::FIELD_WIDTH},
        mino::mino::Mino,
    },
};

pub trait MinoRotation {
    // Porting note: refactors retrieval of map
    fn get_map(&self, direction: RotateDirection) -> &[Pattern];

    fn get_kicks(
        &self,
        field: &dyn Field,
        before: &'static Mino,
        after: &'static Mino,
        x: u8,
        y: u8,
        direction: RotateDirection,
    ) -> Option<Coordinate> {
        // guard for 180 rotation
        // TODO: so do we need to check for supported rotation directions before calling this function?
        if direction == RotateDirection::Rotate180 && self.no_supports_180() {
            return None;
        }
        _get_kicks(
            field,
            x,
            y,
            after,
            self.get_patterns_from(before, direction),
        )
    }

    // Porting note: replaces getOffsetsFrom and isPrivilegeSpins, call the methods directly in Pattern instead
    fn get_patterns_from(&self, current: &'static Mino, direction: RotateDirection) -> &Pattern {
        &self.get_map(direction)[into_val(current)]
    }

    fn supports_180(&self) -> bool;

    fn no_supports_180(&self) -> bool {
        !self.supports_180()
    }
}

// TODO: merge with instance in MinoFactory
fn into_val(mino: &'static Mino) -> usize {
    mino.get_piece() as usize * Rotate::get_size() + mino.get_rotate() as usize
}

fn _get_kicks(
    field: &dyn Field,
    x: u8,
    y: u8,
    after: &'static Mino,
    pattern: &Pattern,
) -> Option<Coordinate> {
    let min_x = -after.get_min_x();
    let max_x = FIELD_WIDTH as i8 - after.get_max_x();
    let min_y = -after.get_min_y();

    pattern
        .get_offsets()
        .find_map(|offset| {
            let to_x = x as i8 + offset.x;
            let to_y = y as i8 + offset.y;

            (min_x <= to_x
                && to_x < max_x
                && min_y <= to_y
                && field.can_put(after, to_x as u8, to_y as u8))
            .then_some(offset)
        })
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entry::common::kicks::factory::srs_mino_rotation_factory,
        sfinder_core::{
            field::field_factory,
            mino::{mino_factory::MinoFactory, piece::Piece},
        },
    };

    fn create_srs_rotation() -> Box<dyn MinoRotation> {
        srs_mino_rotation_factory::create()
    }

    mod kicks {
        use super::*;

        fn check_kick(
            rotation: &dyn MinoRotation,
            marks: String,
            mino: &'static Mino,
            x: u8,
            y: u8,
            direction: RotateDirection,
            expected: Option<Coordinate>,
        ) {
            let field = field_factory::create_field_with_marks(marks);
            // checks test case validity, mino should not collide with the field
            assert!(field.can_put(mino, x, y));

            let after =
                MinoFactory::new().get(mino.get_piece(), mino.get_rotate().apply(direction));

            assert_eq!(
                rotation.get_kicks(field.as_ref(), mino, &after, x, y, direction),
                expected
            );
        }

        fn assert_wrapper(
            marks: String,
            test_cases: &[(Piece, Rotate, u8, u8, RotateDirection, Option<Coordinate>)],
        ) {
            let mino_rotation = create_srs_rotation();
            for (piece, rotate, x, y, direction, expected) in test_cases {
                let mino = MinoFactory::new().get(*piece, *rotate);
                check_kick(
                    mino_rotation.as_ref(),
                    marks.clone(),
                    mino,
                    *x,
                    *y,
                    *direction,
                    *expected,
                );
            }
        }

        mod with_i {
            use super::*;

            #[test]
            fn checks1ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_XXXXXXXX"
                        + "X_XXXXXXXX"
                        + "X_XXXXXXXX",
                    &[(Piece::I, Rotate::Spawn, 2, 3, RotateDirection::Clockwise, Some(Coordinate::new(-1, -1)))],
                );
            }

            #[test]
            fn checks1ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_X_XXXXXX"
                        + "X_X_XXXXXX"
                        + "X_XXXXXXXX",
                    &[(Piece::I, Rotate::Spawn, 2, 3, RotateDirection::Clockwise, Some(Coordinate::new(1, 0)))],
                );
            }

            #[test]
            fn checks1ng2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_XX_XXXXX"
                        + "X_XX_XXXXX"
                        + "X_XXXXXXXX",
                    &[(Piece::I, Rotate::Spawn, 2, 3, RotateDirection::Clockwise, Some(Coordinate::new(2, 0)))],
                );
            }

            #[test]
            fn checks2ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "________XX"
                        + "_________X"
                        + "XXXXXXXX_X"
                        + "XXXXXXXX_X"
                        + "XXXXXXXX_X",
                    &[(Piece::I, Rotate::Reverse, 7, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(1, -1)))],
                );
            }

            #[test]
            fn checks2ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "________XX"
                        + "_________X"
                        + "XXXXXXX__X"
                        + "XXXXXXXX_X"
                        + "XXXXXXXX_X",
                    &[(Piece::I, Rotate::Reverse, 7, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 1)))],
                );
            }

            #[test]
            fn checks2ng2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "________XX"
                        + "_________X"
                        + "XXXXX_XX_X"
                        + "XXXXXXXX_X"
                        + "XXXXXXXX_X",
                    &[(Piece::I, Rotate::Reverse, 7, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(-2, 1)))],
                );
            }

            #[test]
            fn checks3ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX_______"
                        + "XXX_______"
                        + "XXX_XXXXXX"
                        + "XXX____XXX",
                    &[
                        (Piece::I, Rotate::Right, 3, 2, RotateDirection::Clockwise, Some(Coordinate::new(2, -2))),
                        (Piece::I, Rotate::Left, 3, 1, RotateDirection::Clockwise, Some(Coordinate::new(1, 1))),
                    ],
                );
            }

            #[test]
            fn checks3ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "__________"
                        + "XXX_______"
                        + "X____XXXXX"
                        + "XXX____XXX",
                    &[
                        (Piece::I, Rotate::Right, 3, 2, RotateDirection::Clockwise, Some(Coordinate::new(0, -1))),
                        (Piece::I, Rotate::Left, 3, 1, RotateDirection::Clockwise, Some(Coordinate::new(1, 1))),
                    ],
                );
            }

            #[test]
            fn checks3ok3() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                            + "X_________"
                            + "XXX___XXXX"
                            + "XXX_XXXXXX"
                            + "XXX____XXX",
                    &[
                        (Piece::I, Rotate::Right, 3, 2, RotateDirection::Clockwise, Some(Coordinate::new(2, -2))),
                        (Piece::I, Rotate::Left, 3, 1, RotateDirection::Clockwise, Some(Coordinate::new(1, -1))),
                    ],
                );
            }

            #[test]
            fn checks3ok4() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "__________"
                        + "XXX___XXXX"
                        + "XXX_XXXXXX"
                        + "XXX____XXX",
                    &[
                        (Piece::I, Rotate::Right, 3, 2, RotateDirection::Clockwise, Some(Coordinate::new(-1, 1))),
                        (Piece::I, Rotate::Left, 3, 1, RotateDirection::Clockwise, Some(Coordinate::new(1, -1))),
                    ],
                );
            }

            #[test]
            fn checks4ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_______XXX"
                        + "_______XXX"
                        + "XXXXXX_XXX"
                        + "XXX____XXX",
                    &[
                        (Piece::I, Rotate::Left, 6, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, -1))),
                        (Piece::I, Rotate::Right, 6, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-2, 0))),
                    ],
                );
            }

            #[test]
            fn checks4ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "__________"
                        + "_______XXX"
                        + "XXXXXX_XXX"
                        + "XXX____XXX",
                    &[
                        (Piece::I, Rotate::Left, 6, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, -1))),
                        (Piece::I, Rotate::Right, 6, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-2, 0))),
                    ],
                );
            }

            #[test]
            fn checks4ok3() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "__________"
                        + "_______XXX"
                        + "XXXXX____X"
                        + "XXX____XXX",
                    &[
                        (Piece::I, Rotate::Left, 6, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(1, 0))),
                        (Piece::I, Rotate::Right, 6, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-2, 0))),
                    ],
                );
            }

            #[test]
            fn checks4ok4() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_______XXX"
                        + "XXXX___XXX"
                        + "XXXXXX_XXX"
                        + "XXX____XXX",
                    &[
                        (Piece::I, Rotate::Left, 6, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, -1))),
                        (Piece::I, Rotate::Right, 6, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-2, -2))),
                    ],
                );
            }

            #[test]
            fn checks4ok5() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_________X"
                        + "XXXX___XXX"
                        + "XXXXXX_XXX"
                        + "XXX____XXX",
                    &[
                        (Piece::I, Rotate::Left, 6, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, -1))),
                        (Piece::I, Rotate::Right, 6, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-2, -2))),
                    ],
                );
            }

            #[test]
            fn checks5() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "__________"
                        + "XXXXX_____"
                        + "XX________"
                        + "X_________",
                    &[
                        (Piece::I, Rotate::Spawn, 2, 0, RotateDirection::Clockwise, None),
                        (Piece::I, Rotate::Spawn, 2, 0, RotateDirection::CounterClockwise, None),
                    ],
                );
            }
        }

        mod with_o {
            use super::*;

            #[test]
            fn checks_right() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "X__XXXXXXX"
                        + "X___XXXXXX"
                        + "XX__XXXXXX",
                    &[
                        (Piece::O, Rotate::Spawn, 1, 1, RotateDirection::Clockwise, Some(Coordinate::new(0, 1))),
                        (Piece::O, Rotate::Right, 1, 2, RotateDirection::Clockwise, Some(Coordinate::new(1, 0))),
                        (Piece::O, Rotate::Reverse, 2, 2, RotateDirection::Clockwise, Some(Coordinate::new(0, -1))),
                        (Piece::O, Rotate::Left, 2, 1, RotateDirection::Clockwise, Some(Coordinate::new(-1, 0))),
                    ],
                );
            }

            #[test]
            fn checks_left() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "X__XXXXXXX"
                        + "X___XXXXXX"
                        + "XX__XXXXXX",
                    &[
                        (Piece::O, Rotate::Spawn, 1, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(1, 0))),
                        (Piece::O, Rotate::Left, 2, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 1))),
                        (Piece::O, Rotate::Reverse, 2, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, 0))),
                        (Piece::O, Rotate::Right, 1, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(0, -1))),
                    ],
                );
            }
        }

        mod with_s {
            use super::*;

            #[test]
            fn checks1ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX__XXXXXX"
                        + "X__XXXXXXX",
                    &[(Piece::S, Rotate::Right, 1, 2, RotateDirection::Clockwise, Some(Coordinate::new(1, -1)))],
                );
            }

            #[test]
            fn checks2ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX__XXXXXX"
                        + "XX__XXXXXX"
                        + "X__XXXXXXX",
                    &[(Piece::S, Rotate::Left, 3, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, -1)))],
                );
            }

            #[test]
            fn checks2ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "XX__XXXXXX"
                        + "X__XXXXXXX",
                    &[(Piece::S, Rotate::Left, 3, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks3ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "___X______"
                        + "X_XXXXXXXX"
                        + "X__XXXXXXX"
                        + "XX_XXXXXXX",
                    &[(Piece::S, Rotate::Spawn, 2, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(0, -2)))],
                );
            }

            #[test]
            fn checks3ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "__________"
                        + "X_XXXXXXXX"
                        + "X__XXXXXXX"
                        + "XX_XXXXXXX",
                    &[(Piece::S, Rotate::Spawn, 2, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(1, 1)))],
                );
            }

            #[test]
            fn checks4ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_XXXXXXXX"
                        + "X__XXXXXXX"
                        + "XX_XXXXXXX",
                    &[(Piece::S, Rotate::Spawn, 2, 3, RotateDirection::Clockwise, Some(Coordinate::new(-1, -2)))],
                );
            }

            #[test]
            fn checks4ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_________"
                        + "X_XXXXXXXX"
                        + "X__XXXXXXX"
                        + "XX_XXXXXXX",
                    &[(Piece::S, Rotate::Spawn, 2, 3, RotateDirection::Clockwise, Some(Coordinate::new(-1, -2)))],
                );
            }

            #[test]
            fn checks4ok3() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "X_________"
                        + "__________"
                        + "X_XXXXXXXX"
                        + "X__XXXXXXX"
                        + "XX_XXXXXXX",
                    &[(Piece::S, Rotate::Spawn, 1, 3, RotateDirection::Clockwise, Some(Coordinate::new(0, -2)))],
                );
            }

            #[test]
            fn checks4ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "X_________"
                        + "__________"
                        + "__________"
                        + "X_XXXXXXXX"
                        + "X__XXXXXXX"
                        + "XX_XXXXXXX",
                    &[(Piece::S, Rotate::Spawn, 1, 3, RotateDirection::Clockwise, Some(Coordinate::new(-1, 0)))],
                );
            }

            #[test]
            fn checks5ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_________",
                    &[
                        (Piece::S, Rotate::Spawn, 2, 0, RotateDirection::Clockwise, None),
                        (Piece::S, Rotate::Spawn, 2, 0, RotateDirection::CounterClockwise, Some(Coordinate::new(1, 1))),
                    ],
                );
            }
        }

        mod with_z {
            use super::*;

            #[test]
            fn checks1ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "X__XXXXXXX"
                        + "XX__XXXXXX",
                    &[(Piece::Z, Rotate::Left, 3, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, -1)))],
                );
            }

            #[test]
            fn checks2ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "X__XXXXXXX"
                        + "X__XXXXXXX"
                        + "XX__XXXXXX",
                    &[(Piece::Z, Rotate::Right, 1, 2, RotateDirection::Clockwise, Some(Coordinate::new(1, -1)))],
                );
            }

            #[test]
            fn checks2ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "___XXXXXXX"
                        + "X__XXXXXXX"
                        + "XX__XXXXXX",
                    &[(Piece::Z, Rotate::Right, 1, 2, RotateDirection::Clockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks3ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "__X_______"
                        + "XXXX_XXXXX"
                        + "XXX__XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::Z, Rotate::Spawn, 3, 3, RotateDirection::Clockwise, Some(Coordinate::new(0, -2)))],
                );
            }

            #[test]
            fn checks3ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "__________"
                        + "XXXX_XXXXX"
                        + "XXX__XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::Z, Rotate::Spawn, 3, 3, RotateDirection::Clockwise, Some(Coordinate::new(-1, 1)))],
                );
            }

            #[test]
            fn checks4ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "____XX____"
                        + "_____X____"
                        + "XXXX_XXXXX"
                        + "XXX__XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::Z, Rotate::Spawn, 3, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(1, -2)))],
                );
            }

            #[test]
            fn checks4ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "____XX____"
                        + "_____X____"
                        + "_____X____"
                        + "XXXX_XXXXX"
                        + "XXX__XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::Z, Rotate::Spawn, 3, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(1, -2)))],
                );
            }

            #[test]
            fn checks4ok3() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_____XX___"
                        + "______X___"
                        + "XXXX_XXXXX"
                        + "XXX__XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::Z, Rotate::Spawn, 4, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(0, -2)))],
                );
            }

            #[test]
            fn checks4ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_____XX___"
                        + "______X___"
                        + "______X___"
                        + "XXXX_XXXXX"
                        + "XXX__XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::Z, Rotate::Spawn, 4, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(1, 0)))],
                );
            }

            #[test]
            fn checks5ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "________XX"
                        + "_________X"
                        + "_________X",
                    &[
                        (Piece::Z, Rotate::Spawn, 7, 0, RotateDirection::CounterClockwise, None),
                        (Piece::Z, Rotate::Spawn, 7, 0, RotateDirection::Clockwise, Some(Coordinate::new(-1, 1))),
                    ],
                );
            }
        }

        mod with_l {
            use super::*;

            #[test]
            fn checks1ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX_______"
                        + "XX________"
                        + "XX_X______",
                    &[(Piece::L, Rotate::Left, 4, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, 0)))],
                );
            }

            #[test]
            fn checks2ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX__XXXXX"
                        + "XX___XXXXX"
                        + "XX_XXXXXXX",
                    &[(Piece::L, Rotate::Left, 4, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, -1)))],
                );
            }

            #[test]
            fn checks2ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX___XXXX"
                        + "XX___XXXXX"
                        + "XX_XXXXXXX",
                    &[(Piece::L, Rotate::Left, 4, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks3ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXX______"
                        + "XX________"
                        + "XX_XXXXXXX",
                    &[(Piece::L, Rotate::Left, 4, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, -1)))],
                );
            }

            #[test]
            fn checks3ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX_______"
                        + "XX________"
                        + "XX_XXXXXXX",
                    &[
                        (Piece::L, Rotate::Left, 4, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 0))),
                        (Piece::L, Rotate::Left, 3, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 2))),
                    ],
                );
            }

            #[test]
            fn checks4ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "____XXXXXX"
                        + "XX___XXXXX"
                        + "XX_XXXXXXX",
                    &[
                        (Piece::L, Rotate::Right, 2, 2, RotateDirection::Clockwise, Some(Coordinate::new(1, -1))),
                        (Piece::L, Rotate::Right, 3, 2, RotateDirection::Clockwise, Some(Coordinate::new(0, 2))),
                    ],
                );
            }

            #[test]
            fn checks4ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "____XXXXXX"
                        + "X____XXXXX"
                        + "XX_XXXXXXX",
                    &[(Piece::L, Rotate::Right, 2, 2, RotateDirection::Clockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks5ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "___XXXXXXX"
                        + "_____XXXXX"
                        + "XX_XXXXXXX",
                    &[(Piece::L, Rotate::Right, 2, 2, RotateDirection::Clockwise, Some(Coordinate::new(1, -1)))],
                );
            }

            #[test]
            fn checks5ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "____XXXXXX"
                        + "_____XXXXX"
                        + "XX_XXXXXXX",
                    &[(Piece::L, Rotate::Right, 2, 2, RotateDirection::Clockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks6ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXX_XXXXX"
                        + "XX___XXXXX",
                    &[(Piece::L, Rotate::Left, 4, 1, RotateDirection::Clockwise, Some(Coordinate::new(-1, -1)))],
                );
            }

            #[test]
            fn checks6ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX__XXXXX"
                        + "XX___XXXXX",
                    &[(Piece::L, Rotate::Left, 4, 1, RotateDirection::Clockwise, Some(Coordinate::new(-1, -1)))],
                );
            }

            #[test]
            fn checks6ok3() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXX__XXXX"
                        + "XX___XXXXX",
                    &[(Piece::L, Rotate::Left, 4, 1, RotateDirection::Clockwise, Some(Coordinate::new(-1, -1)))],
                );
            }

            #[test]
            fn checks6ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX___XXXX"
                        + "XX___XXXXX",
                    &[(Piece::L, Rotate::Left, 4, 1, RotateDirection::Clockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks7ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "____XXXXXX"
                        + "XX___XXXXX"
                        + "XX___XXXXX",
                    &[
                        (Piece::L, Rotate::Right, 2, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(1, -1))),
                        (Piece::L, Rotate::Right, 3, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 2))),
                    ],
                );
            }

            #[test]
            fn checks7ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "____XXXXXX"
                        + "X____XXXXX"
                        + "XX___XXXXX",
                    &[(Piece::L, Rotate::Right, 2, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks8ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX_______"
                        + "XX________"
                        + "XX_XXXXXXX"
                        + "XX_XXXXXXX"
                        + "XX__XXXXXX",
                    &[(Piece::L, Rotate::Spawn, 3, 3, RotateDirection::Clockwise, Some(Coordinate::new(-1, -2)))],
                );
            }

            #[test]
            fn checks8ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX_______"
                        + "XX________"
                        + "XX________"
                        + "XX_XXXXXXX"
                        + "XX_XXXXXXX"
                        + "XX__XXXXXX",
                    &[(Piece::L, Rotate::Spawn, 3, 3, RotateDirection::Clockwise, Some(Coordinate::new(-1, -2)))],
                );
            }

            #[test]
            fn checks8ok3() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "XX_XXXXXXX"
                        + "XX_XXXXXXX"
                        + "XX__XXXXXX",
                    &[(Piece::L, Rotate::Spawn, 2, 3, RotateDirection::Clockwise, Some(Coordinate::new(0, -2)))],
                );
            }

            #[test]
            fn checks8ok4() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_________"
                        + "XX_XXXXXXX"
                        + "XX_XXXXXXX"
                        + "XX__XXXXXX",
                    &[(Piece::L, Rotate::Spawn, 2, 3, RotateDirection::Clockwise, Some(Coordinate::new(0, -2)))],
                );
            }

            #[test]
            fn checks8ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX_XXXXXXX"
                        + "XX_XXXXXXX"
                        + "XX__XXXXXX",
                    &[(Piece::L, Rotate::Spawn, 2, 3, RotateDirection::Clockwise, Some(Coordinate::new(-1, 1)))],
                );
            }

            #[test]
            fn checks9ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "________XX"
                        + "_________X"
                        + "_________X",
                    &[
                        (Piece::L, Rotate::Spawn, 7, 0, RotateDirection::CounterClockwise, None),
                        (Piece::L, Rotate::Spawn, 7, 0, RotateDirection::Clockwise, Some(Coordinate::new(-1, 1))),
                    ],
                );
            }
        }

        mod with_j {
            use super::*;

            #[test]
            fn checks1ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_______XXX"
                        + "________XX"
                        + "______X_XX",
                    &[(Piece::J, Rotate::Right, 5, 1, RotateDirection::Clockwise, Some(Coordinate::new(1, 0)))],
                );
            }

            #[test]
            fn checks2ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXX__XXX"
                        + "XXXXX___XX"
                        + "XXXXXXX_XX",
                    &[(Piece::J, Rotate::Right, 5, 2, RotateDirection::Clockwise, Some(Coordinate::new(1, -1)))],
                );
            }

            #[test]
            fn checks2ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXX___XXX"
                        + "XXXXX___XX"
                        + "XXXXXXX_XX",
                    &[(Piece::J, Rotate::Right, 5, 2, RotateDirection::Clockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks3ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "______XXXX"
                        + "________XX"
                        + "XXXXXXX_XX",
                    &[(Piece::J, Rotate::Right, 5, 2, RotateDirection::Clockwise, Some(Coordinate::new(1, -1)))],
                );
            }

            #[test]
            fn checks3ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_______XXX"
                        + "________XX"
                        + "XXXXXXX_XX",
                    &[
                        (Piece::J, Rotate::Right, 5, 2, RotateDirection::Clockwise, Some(Coordinate::new(0, 0))),
                        (Piece::J, Rotate::Right, 6, 2, RotateDirection::Clockwise, Some(Coordinate::new(0, 2))),
                    ],
                );
            }

            #[test]
            fn checks4ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXXX____"
                        + "XXXXX___XX"
                        + "XXXXXXX_XX",
                    &[
                        (Piece::J, Rotate::Left, 7, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, -1))),
                        (Piece::J, Rotate::Left, 6, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 2))),
                    ],
                );
            }

            #[test]
            fn checks4ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXXX____"
                        + "XXXXX____X"
                        + "XXXXXXX_XX",
                    &[(Piece::J, Rotate::Left, 7, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks5ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXXXX___"
                        + "XXXXX_____"
                        + "XXXXXXX_XX",
                    &[(Piece::J, Rotate::Left, 7, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, -1)))],
                );
            }

            #[test]
            fn checks5ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXXX____"
                        + "XXXXX_____"
                        + "XXXXXXX_XX",
                    &[(Piece::J, Rotate::Left, 7, 2, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks6ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXX_XXXX"
                        + "XXXXX___XX",
                    &[(Piece::J, Rotate::Right, 5, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(1, -1)))],
                );
            }

            #[test]
            fn checks6ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXX__XXX"
                        + "XXXXX___XX",
                    &[(Piece::J, Rotate::Right, 5, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(1, -1)))],
                );
            }

            #[test]
            fn checks6ok3() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXX__XXXX"
                        + "XXXXX___XX",
                    &[(Piece::J, Rotate::Right, 5, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(1, -1)))],
                );
            }

            #[test]
            fn checks6ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXX___XXX"
                        + "XXXXX___XX",
                    &[(Piece::J, Rotate::Right, 5, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks7ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXXX____"
                        + "XXXXX___XX"
                        + "XXXXX___XX",
                    &[
                        (Piece::J, Rotate::Left, 7, 1, RotateDirection::Clockwise, Some(Coordinate::new(-1, -1))),
                        (Piece::J, Rotate::Left, 6, 1, RotateDirection::Clockwise, Some(Coordinate::new(0, 2))),
                    ],
                );
            }

            #[test]
            fn checks7ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXXX____"
                        + "XXXXX____X"
                        + "XXXXX___XX",
                    &[(Piece::J, Rotate::Left, 7, 1, RotateDirection::Clockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks8ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_______XXX"
                        + "________XX"
                        + "XXXXXXX_XX"
                        + "XXXXXXX_XX"
                        + "XXXXXX__XX",
                    &[(Piece::J, Rotate::Spawn, 6, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(1, -2)))],
                );
            }

            #[test]
            fn checks8ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_______XXX"
                        + "________XX"
                        + "________XX"
                        + "XXXXXXX_XX"
                        + "XXXXXXX_XX"
                        + "XXXXXX__XX",
                    &[(Piece::J, Rotate::Spawn, 6, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(1, -2)))],
                );
            }

            #[test]
            fn checks8ok3() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "________XX"
                        + "_________X"
                        + "_________X"
                        + "XXXXXXX_XX"
                        + "XXXXXXX_XX"
                        + "XXXXXX__XX",
                    &[(Piece::J, Rotate::Spawn, 7, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(0, -2)))],
                );
            }

            #[test]
            fn checks8ok4() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "________XX"
                        + "_________X"
                        + "_________X"
                        + "XXXXXXX_XX"
                        + "XXXXXXX_XX"
                        + "XXXXXX__XX",
                    &[(Piece::J, Rotate::Spawn, 7, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(0, -2)))],
                );
            }

            #[test]
            fn checks8ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXXXX_XX"
                        + "XXXXXXX_XX"
                        + "XXXXXX__XX",
                    &[(Piece::J, Rotate::Spawn, 7, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(1, 1)))],
                );
            }

            #[test]
            fn checks9ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_________",
                    &[
                        (Piece::J, Rotate::Spawn, 2, 0, RotateDirection::Clockwise, None),
                        (Piece::J, Rotate::Spawn, 2, 0, RotateDirection::CounterClockwise, Some(Coordinate::new(1, 1))),
                    ],
                );
            }
        }

        mod with_t {
            use super::*;

            #[test]
            fn checks1ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_________"
                        + "X_XXXXXXXX",
                    &[(Piece::T, Rotate::Spawn, 2, 1, RotateDirection::Clockwise, Some(Coordinate::new(-1, 0)))],
                );
            }

            #[test]
            fn checks1ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "________XX"
                        + "_________X"
                        + "_________X"
                        + "XXXXXXXX_X",
                    &[(Piece::T, Rotate::Spawn, 7, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(1, 0)))],
                );
            }

            #[test]
            fn checks2ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_____XXXXX"
                        + "XXX__XXXXX"
                        + "XX___XXXXX",
                    &[(Piece::T, Rotate::Left, 4, 1, RotateDirection::Clockwise, Some(Coordinate::new(-1, -1)))],
                );
            }

            #[test]
            fn checks2ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "XX__XXXXXX"
                        + "XX___XXXXX",
                    &[(Piece::T, Rotate::Right, 2, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(1, -1)))],
                );
            }

            #[test]
            fn checks2ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_____XXXXX"
                        + "XXX___XXXX"
                        + "XX___XXXXX",
                    &[(Piece::T, Rotate::Left, 4, 1, RotateDirection::Clockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks2ng2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XX________"
                        + "X___XXXXXX"
                        + "XX___XXXXX",
                    &[(Piece::T, Rotate::Right, 2, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks3ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "____XXXXXX"
                        + "XX___XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::T, Rotate::Left, 3, 1, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks3ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX_______"
                        + "XX___XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::T, Rotate::Right, 3, 1, RotateDirection::Clockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks4ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXX______"
                        + "XXX_______"
                        + "XXX_XXXXXX"
                        + "XXX__XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::T, Rotate::Spawn, 4, 3, RotateDirection::Clockwise, Some(Coordinate::new(-1, -2)))],
                );
            }

            #[test]
            fn checks4ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "___XX_____"
                        + "____X_____"
                        + "XXX_XXXXXX"
                        + "XX__XXXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::T, Rotate::Spawn, 2, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(1, -2)))],
                );
            }

            #[test]
            fn checks4ng1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXX______"
                        + "XXX_______"
                        + "XXX_______"
                        + "XXX_XXXXXX"
                        + "XXX__XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::T, Rotate::Spawn, 4, 3, RotateDirection::Clockwise, Some(Coordinate::new(-1, 0)))],
                );
            }

            #[test]
            fn checks4ng2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "___XX____X"
                        + "____X____X"
                        + "____X____X"
                        + "XXX_XXXXXX"
                        + "XX__XXXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::T, Rotate::Spawn, 2, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(1, 0)))],
                );
            }

            #[test]
            fn checks4ng3() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX_XXXXXX"
                        + "XXX__XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::T, Rotate::Spawn, 3, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks4ng4() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXX_XXXXXX"
                        + "XX__XXXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::T, Rotate::Spawn, 3, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(0, 0)))],
                );
            }

            #[test]
            fn checks5ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXX______"
                        + "XX________"
                        + "XXX_______"
                        + "XXX__XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::T, Rotate::Reverse, 3, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(0, -2)))],
                );
            }

            #[test]
            fn checks5ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "______XXXX"
                        + "________XX"
                        + "_______XXX"
                        + "XXXXX__XXX"
                        + "XXXXXX_XXX",
                    &[(Piece::T, Rotate::Reverse, 6, 3, RotateDirection::Clockwise, Some(Coordinate::new(0, -2)))],
                );
            }

            #[test]
            fn checks6ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXX_____"
                        + "XXX_______"
                        + "XXX_______"
                        + "XXX__XXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::T, Rotate::Reverse, 4, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(-1, -2)))],
                );
            }

            #[test]
            fn checks6ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_____XXXXX"
                        + "_______XXX"
                        + "_______XXX"
                        + "XXXXX__XXX"
                        + "XXXXXX_XXX",
                    &[(Piece::T, Rotate::Reverse, 5, 3, RotateDirection::Clockwise, Some(Coordinate::new(1, -2)))],
                );
            }

            #[test]
            fn checks7ok1() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "XXXXX_____"
                        + "XX________"
                        + "XXX_______"
                        + "XX__XXXXXX"
                        + "XXX_XXXXXX",
                    &[(Piece::T, Rotate::Reverse, 3, 3, RotateDirection::Clockwise, Some(Coordinate::new(0, -2)))],
                );
            }

            #[test]
            fn checks7ok2() {
                #[rustfmt::skip]
                assert_wrapper(
                    String::new()
                        + "_____XXXXX"
                        + "________XX"
                        + "_______XXX"
                        + "XXXXXX__XX"
                        + "XXXXXX_XXX",
                    &[(Piece::T, Rotate::Reverse, 6, 3, RotateDirection::CounterClockwise, Some(Coordinate::new(0, -2)))],
                );
            }
        }

        mod rotate_180 {
            use super::*;
            use crate::entry::common::kicks::factory::file_mino_rotation_factory;
            use std::path::PathBuf;

            fn create_nullpomino180() -> Box<dyn MinoRotation> {
                file_mino_rotation_factory::create(PathBuf::from(
                    std::env::var("CARGO_MANIFEST_DIR").unwrap()
                        + "/kicks/nullpomino180.properties",
                ))
                .unwrap()
            }

            fn assert_wrapper_180(
                marks: String,
                test_cases: &[(Piece, Rotate, u8, u8, Option<Coordinate>)],
            ) {
                let mino_rotation = create_nullpomino180();
                for (piece, rotate, x, y, expected) in test_cases {
                    let mino = MinoFactory::new().get(*piece, *rotate);
                    check_kick(
                        mino_rotation.as_ref(),
                        marks.clone(),
                        mino,
                        *x,
                        *y,
                        RotateDirection::Rotate180,
                        *expected,
                    );
                }
            }

            mod with_i {
                use super::*;

                #[test]
                fn case1() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "XXXXXXXX__"
                            + "XXXXXXX___"
                            + "XXXXXXX___"
                            + "XXXXXXX___"
                            + "XXXXXXX_X_"
                            + "XXXXXXXXX_"
                            + "XXXXXXXXX_",
                        &[(Piece::I, Rotate::Right, 8, 5, Some(Coordinate::new(-1, -2)))],
                    );
                }

                #[test]
                fn case2() {
                    assert_wrapper_180(
                        String::new()
                            + "__XX______"
                            + "___XXXXXXX"
                            + "___XXXXXXX"
                            + "_X_XXXXXXX"
                            + "_X_XXXXXXX"
                            + "_XXXXXXXXX",
                        &[(Piece::I, Rotate::Left, 1, 4, Some(Coordinate::new(1, -1)))],
                    );
                }

                #[test]
                fn case3() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "_XXXXXXXXX"
                            + "_X_XXXXXXX"
                            + "_X_XXXXXXX"
                            + "_X_XXXXXXX"
                            + "_X_XXXXXXX",
                        &[(Piece::I, Rotate::Left, 0, 1, Some(Coordinate::new(2, 1)))],
                    );
                }

                #[test]
                fn case4() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "__________"
                            + "XXX____X__"
                            + "_XXXXXXXXX"
                            + "XXX____XXX",
                        &[(Piece::I, Rotate::Spawn, 4, 2, Some(Coordinate::new(1, -2)))],
                    );
                }
            }

            mod with_o {
                use super::*;

                #[test]
                fn case1() {
                    assert_wrapper_180(
                        String::new() + "__________" + "__________" + "__________" + "__________",
                        &[
                            (Piece::O, Rotate::Spawn, 4, 1, Some(Coordinate::new(1, 1))),
                            (
                                Piece::O,
                                Rotate::Reverse,
                                5,
                                2,
                                Some(Coordinate::new(-1, -1)),
                            ),
                            (Piece::O, Rotate::Right, 5, 2, Some(Coordinate::new(1, -1))),
                            (Piece::O, Rotate::Left, 6, 1, Some(Coordinate::new(-1, 1))),
                        ],
                    );
                }
            }

            mod with_l {
                use super::*;

                #[test]
                fn case1() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "XX________"
                            + "XXX___XXXX"
                            + "XXX___XXXX"
                            + "XX____XXXX"
                            + "XX_XXXXXXX",
                        &[(Piece::L, Rotate::Spawn, 3, 1, Some(Coordinate::new(0, 0)))],
                    );
                }

                #[test]
                fn case2() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "XX________"
                            + "XX__XXXXXX"
                            + "XXX_XXXXXX"
                            + "XXX_XXXXXX"
                            + "XXX__XXXXX",
                        &[(Piece::L, Rotate::Left, 3, 2, Some(Coordinate::new(0, -1)))],
                    );
                }
            }

            mod with_j {
                use super::*;

                #[test]
                fn case1() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "XXXXX__XXX"
                            + "XXXXX__XXX"
                            + "XXXXX_XXXX"
                            + "XXXX__XXXX",
                        &[(Piece::J, Rotate::Right, 5, 1, Some(Coordinate::new(0, 0)))],
                    );
                }
            }

            mod with_s {
                use super::*;

                #[test]
                fn case1() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "XXXXXX____"
                            + "XXXXXX____"
                            + "XXXXXX__X_"
                            + "XXXXX__XXX",
                        &[(Piece::S, Rotate::Spawn, 7, 1, Some(Coordinate::new(-1, 0)))],
                    );
                }

                #[test]
                fn case2() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "_X____X___"
                            + "XX__XXX___"
                            + "XXX__XXXXX"
                            + "XXXX_XXXXX",
                        &[(Piece::S, Rotate::Left, 3, 2, Some(Coordinate::new(0, -1)))],
                    );
                }

                #[test]
                fn case3() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "____XXXXXX"
                            + "X__XXXXXXX"
                            + "XXXX__XXXX"
                            + "XXX__XXXXX",
                        &[(Piece::S, Rotate::Spawn, 2, 2, Some(Coordinate::new(2, -1)))],
                    );
                }

                #[test]
                fn case4() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "____XXX___"
                            + "XXXXX_X___"
                            + "XXXXX__X__"
                            + "XXXXXX_XXX",
                        &[(Piece::S, Rotate::Right, 7, 2, Some(Coordinate::new(-1, -1)))],
                    );
                }

                #[test]
                fn case5() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "XXXXXXX___"
                            + "XXXXX_XX__"
                            + "XXXXX__XX_"
                            + "XXXXXX_XXX",
                        &[(Piece::S, Rotate::Right, 7, 3, Some(Coordinate::new(-1, -2)))],
                    );
                }
            }

            mod with_z {
                use super::*;

                #[test]
                fn case1() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "__________"
                            + "X____XXXXX"
                            + "X____XXXXX"
                            + "XXXX__XXXX",
                        &[(Piece::Z, Rotate::Spawn, 3, 1, Some(Coordinate::new(1, 0)))],
                    );
                }

                #[test]
                fn case2() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "__________"
                            + "XXXXXX_XXX"
                            + "XXXXX__XXX"
                            + "XXXXX_XXXX",
                        &[(Piece::Z, Rotate::Right, 6, 3, Some(Coordinate::new(0, -2)))],
                    );
                }

                #[test]
                fn case3() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "X_________"
                            + "X__XXXXXXX"
                            + "X__X_XXXXX"
                            + "X_X__XXXXX"
                            + "XXX_XXXXXX",
                        &[(Piece::Z, Rotate::Left, 2, 2, Some(Coordinate::new(1, -1)))],
                    );
                }

                #[test]
                fn case4() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "___XXXXXXX"
                            + "__XX_XXXXX"
                            + "_XX__XXXXX"
                            + "XXX_XXXXXX",
                        &[(Piece::Z, Rotate::Left, 2, 3, Some(Coordinate::new(1, -2)))],
                    );
                }
            }

            mod with_t {
                use super::*;

                #[test]
                fn case1() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "__________"
                            + "XX___XXXXX"
                            + "XXXX__XXXX"
                            + "XXXX_XXXXX",
                        &[(Piece::T, Rotate::Left, 4, 2, Some(Coordinate::new(0, -1)))],
                    );
                }

                #[test]
                fn case2() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "_____XXXXX"
                            + "_____XXXXX"
                            + "XXXX_XXXXX"
                            + "XXXX__XXXX"
                            + "XXXX_XXXXX",
                        &[(Piece::T, Rotate::Left, 4, 3, Some(Coordinate::new(0, -2)))],
                    );
                }

                #[test]
                fn case3() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "XX___XX___"
                            + "XXX__XX___"
                            + "X_____XXXX"
                            + "XX_XXXXXXX",
                        &[(Piece::T, Rotate::Spawn, 4, 1, Some(Coordinate::new(-2, 0)))],
                    );
                }

                #[test]
                fn case4() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "___X______"
                            + "XXXX___XX_"
                            + "XXXX__XXXX"
                            + "X______XXX"
                            + "XX_XXXXXXX",
                        &[(Piece::T, Rotate::Spawn, 5, 1, Some(Coordinate::new(-3, 0)))],
                    );
                }

                #[test]
                fn case5() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "XX___XXXXX"
                            + "XXX___XXXX"
                            + "X___XXXXXX"
                            + "XX_XXXXXXX",
                        &[(Piece::T, Rotate::Spawn, 4, 2, Some(Coordinate::new(-2, -1)))],
                    );
                }

                #[test]
                fn case6() {
                    assert_wrapper_180(
                        String::new()
                            + "__________"
                            + "__________"
                            + "XX________"
                            + "XXX___XXXX"
                            + "X___XXXXXX"
                            + "XX_XXXXXXX",
                        &[(Piece::T, Rotate::Spawn, 4, 2, Some(Coordinate::new(-2, -1)))],
                    );
                }
            }
        }
    }

    mod offset {
        use super::*;

        fn test_wrapper(piece: Piece, direction: RotateDirection, expected: Vec<&[Coordinate]>) {
            assert_eq!(expected.len(), Rotate::get_size());

            let rotation = create_srs_rotation();
            let actual = Rotate::value_list()
                .iter()
                .copied()
                .map(|rotate| {
                    let mino = MinoFactory::new().get(piece, rotate);
                    rotation
                        .get_patterns_from(mino, direction)
                        .get_offsets()
                        .copied()
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            assert_eq!(actual, expected);
        }

        mod other {
            use super::*;

            const OTHER_PIECES: [Piece; 5] = [Piece::J, Piece::L, Piece::S, Piece::T, Piece::Z];

            #[test]
            fn ccw() {
                for piece in OTHER_PIECES {
                    #[rustfmt::skip]
                    test_wrapper(
                        piece,
                        RotateDirection::CounterClockwise,
                        vec![
                            &[Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(1, 1), Coordinate::new(0, -2), Coordinate::new(1, -2)],
                            &[Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(1, -1), Coordinate::new(0, 2), Coordinate::new(1, 2)],
                            &[Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(-1, 1), Coordinate::new(0, -2), Coordinate::new(-1, -2)],
                            &[Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(-1, -1), Coordinate::new(0, 2), Coordinate::new(-1, 2)],
                        ],
                    );
                }
            }

            #[test]
            fn cw() {
                for piece in OTHER_PIECES {
                    #[rustfmt::skip]
                    test_wrapper(
                        piece,
                        RotateDirection::Clockwise,
                        vec![
                            &[Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(-1, 1), Coordinate::new(0, -2), Coordinate::new(-1, -2)],
                            &[Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(1, -1), Coordinate::new(0, 2), Coordinate::new(1, 2)],
                            &[Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(1, 1), Coordinate::new(0, -2), Coordinate::new(1, -2)],
                            &[Coordinate::new(0, 0), Coordinate::new(-1, 0), Coordinate::new(-1, -1), Coordinate::new(0, 2), Coordinate::new(-1, 2)],
                        ],
                    );
                }
            }
        }

        mod i {
            use super::*;

            #[test]
            fn ccw() {
                #[rustfmt::skip]
                test_wrapper(
                    Piece::I,
                    RotateDirection::CounterClockwise,
                    vec![
                        &[Coordinate::new(0, -1), Coordinate::new(-1, -1), Coordinate::new(2, -1), Coordinate::new(-1, 1), Coordinate::new(2, -2)],
                        &[Coordinate::new(-1, 0), Coordinate::new(1, 0), Coordinate::new(-2, 0), Coordinate::new(1, 1), Coordinate::new(-2, -2)],
                        &[Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(-2, 1), Coordinate::new(1, -1), Coordinate::new(-2, 2)],
                        &[Coordinate::new(1, 0), Coordinate::new(-1, 0), Coordinate::new(2, 0), Coordinate::new(-1, -1), Coordinate::new(2, 2)],
                    ],
                );
            }

            #[test]
            fn cw() {
                #[rustfmt::skip]
                test_wrapper(
                    Piece::I,
                    RotateDirection::Clockwise,
                    vec![
                        &[Coordinate::new(1, 0), Coordinate::new(-1, 0), Coordinate::new(2, 0), Coordinate::new(-1, -1), Coordinate::new(2, 2)],
                        &[Coordinate::new(0, -1), Coordinate::new(-1, -1), Coordinate::new(2, -1), Coordinate::new(-1, 1), Coordinate::new(2, -2)],
                        &[Coordinate::new(-1, 0), Coordinate::new(1, 0), Coordinate::new(-2, 0), Coordinate::new(1, 1), Coordinate::new(-2, -2)],
                        &[Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(-2, 1), Coordinate::new(1, -1), Coordinate::new(-2, 2)],
                    ],
                );
            }
        }

        mod o {
            use super::*;

            #[test]
            fn ccw() {
                #[rustfmt::skip]
                test_wrapper(
                    Piece::O,
                    RotateDirection::CounterClockwise,
                    vec![&[Coordinate::new(1, 0)], &[Coordinate::new(0, -1)], &[Coordinate::new(-1, 0)], &[Coordinate::new(0, 1)]],
                );
            }

            #[test]
            fn cw() {
                #[rustfmt::skip]
                test_wrapper(
                    Piece::O,
                    RotateDirection::Clockwise,
                    vec![&[Coordinate::new(0, 1)], &[Coordinate::new(1, 0)], &[Coordinate::new(0, -1)], &[Coordinate::new(-1, 0)]],
                );
            }
        }
    }
}
