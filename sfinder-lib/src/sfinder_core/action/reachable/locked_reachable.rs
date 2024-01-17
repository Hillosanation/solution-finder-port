use super::reachable::{ILockedReachable, Reachable};
use crate::{
    common::datastore::action::{action::Action, cache::locked_cache::LockedCache},
    sfinder_core::{
        action::common::{can_put_mino_in_field, FromDirection},
        field::{field::Field, field_constants::FIELD_WIDTH},
        mino::{mino::Mino, mino_factory::MinoFactory, mino_shifter::MinoShifter},
        srs::{mino_rotation::MinoRotation, rotate_direction::RotateDirection},
    },
};

pub struct LockedReachable<'a> {
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a MinoShifter,
    mino_rotation: &'a dyn MinoRotation,
    // variable during serach:
    locked_cache: LockedCache,
    appear_y: u8,
}

impl<'a> LockedReachable<'a> {
    pub fn new(
        mino_factory: &'a MinoFactory,
        mino_shifter: &'a MinoShifter,
        mino_rotation: &'a dyn MinoRotation,
        max_y: u8,
    ) -> Self {
        Self {
            mino_factory,
            mino_shifter,
            mino_rotation,
            locked_cache: LockedCache::new(max_y),
            appear_y: 0,
        }
    }

    fn check_inner(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        direction: FromDirection,
    ) -> bool {
        // 一番上までたどり着いたとき
        if self.appear_y <= y {
            return true;
        }

        let rotate = mino.get_rotate();

        // すでに訪問済みのとき
        if self.locked_cache.is_visited(x, y, rotate) {
            // 訪問済みだがまだ探索中の場合は、他の探索でカバーできるためfalseを返却
            return false;
        }

        self.locked_cache.visit(x, y, rotate);

        // harddropでたどりつけるとき
        if field.can_reach_on_harddrop(mino, x, y) {
            return true;
        }

        // 上に移動
        let up_y = y + 1;
        if up_y < self.appear_y
            && field.can_put(mino, x, up_y)
            && self.check_inner(field, mino, x, up_y, FromDirection::None)
        {
            return true;
        }

        // 左に移動
        if let Some(left_x) = x.checked_sub(1) {
            if direction != FromDirection::Left
                && -mino.get_min_x() <= left_x as i8
                && field.can_put(mino, left_x, y)
                && self.check_inner(field, mino, left_x, y, FromDirection::Right)
            {
                return true;
            }
        }

        // 右に移動
        let right_x = x + 1;
        if direction != FromDirection::Right
            && (right_x as i8) < FIELD_WIDTH as i8 - mino.get_max_x()
            && field.can_put(mino, right_x, y)
            && self.check_inner(field, mino, right_x, y, FromDirection::Left)
        {
            return true;
        }

        // 右回転でくる可能性がある場所を移動
        if self.check_rotation(field, mino, x, y, RotateDirection::Clockwise) {
            return true;
        }

        // 左回転でくる可能性がある場所を移動
        if self.check_rotation(field, mino, x, y, RotateDirection::CounterClockwise) {
            return true;
        }

        false
    }

    fn check_rotation(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        direction: RotateDirection,
    ) -> bool {
        let current_rotate = mino.get_rotate();
        let mino_before = self
            .mino_factory
            .get(mino.get_piece(), current_rotate.apply(direction));

        self.mino_rotation
            .get_patterns_from(mino_before, direction)
            .get_offsets()
            .filter_map(|pattern| {
                if let (Ok(from_x), Ok(from_y)) = (
                    u8::try_from(x as i8 - pattern.x),
                    u8::try_from(y as i8 - pattern.y),
                ) {
                    Some((pattern, from_x, from_y))
                } else {
                    None
                }
            })
            .any(|(pattern, from_x, from_y)| {
                can_put_mino_in_field(field, mino, from_x, from_y)
                    && self.mino_rotation.get_kicks(
                        field,
                        mino_before,
                        mino,
                        from_x,
                        from_y,
                        direction,
                    ) == Some(*pattern)
                    && self.check_inner(field, mino_before, from_x, from_y, FromDirection::None)
            })
    }
}

impl Reachable for LockedReachable<'_> {
    fn checks(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
    ) -> bool {
        assert!(field.can_put(mino, x, y));

        self.appear_y = valid_height;
        self.locked_cache.clear();

        let piece = mino.get_piece();
        let rotate = mino.get_rotate();

        self.mino_shifter
            .congruent_actions(piece, rotate, x, y)
            .iter()
            .any(|action| {
                self.check_inner(
                    field,
                    self.mino_factory.get(piece, action.get_rotate()),
                    action.get_x(),
                    action.get_y(),
                    FromDirection::None,
                )
            })
    }

    fn check(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
    ) -> bool {
        self.appear_y = valid_height;
        self.locked_cache.clear();

        self.check_inner(field, mino, x, y, FromDirection::None)
    }
}

impl ILockedReachable for LockedReachable<'_> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entry::common::kicks::factory::srs_mino_rotation_factory,
        sfinder_core::{
            action::reachable::reachable_facade,
            field::{field_factory, field_view},
            mino::piece::Piece::{self, I, J, L, O, S, T, Z},
            srs::rotate::Rotate::{self, Left, Reverse, Right, Spawn},
        },
    };

    #[test]
    fn debug() {
        let mut field = field_factory::create_field(4);
        field.put(MinoFactory::new().get(I, Right), 1, 2);
        println!("{}", field_view::to_string(field.as_ref()));
    }

    // Porting note: the test cases seem to actually check for the specific kick, not just if any congruent action are possible.
    // But the implementation of LockedReachable seems to also check if any congruent action is reachable.
    fn test_wrapper(marks: String, test_cases: &[(bool, Piece, Rotate, u8, u8)]) {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();

        let field = field_factory::create_field_with_marks(marks);

        let mut reachable = reachable_facade::create_90_locked(
            &mino_factory,
            &mino_shifter,
            mino_rotation.as_ref(),
            8,
        );

        for (expected, piece, rotate, x, y) in test_cases {
            let mino = mino_factory.get(*piece, *rotate);
            assert!(field.can_put(mino, *x, *y));
            let mino = mino_factory.get(*piece, *rotate);
            assert_eq!(reachable.check(field.as_ref(), mino, *x, *y, 8), *expected)
        }
    }

    mod only_90 {
        use super::*;

        mod with_i {
            use super::*;

            #[test]
            fn checks1ok1() {
                test_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_XXXXXXXX"
                        + "X_XXXXXXXX"
                        + "X_XXXXXXXX",
                    &[(false, I, Left, 1, 1), (true, I, Right, 1, 2)],
                );
            }

            #[test]
            fn checks1ng1() {
                test_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_X_XXXXXX"
                        + "X_X_XXXXXX"
                        + "X_XXXXXXXX",
                    &[(false, I, Left, 1, 1), (false, I, Right, 1, 2)],
                );
            }

            #[test]
            fn checks1ng2() {
                test_wrapper(
                    String::new()
                        + "XX________"
                        + "X_________"
                        + "X_XX_XXXXX"
                        + "X_XX_XXXXX"
                        + "X_XXXXXXXX",
                    &[(false, I, Left, 1, 1), (false, I, Right, 1, 2)],
                );
            }

            #[test]
            fn checks2ok1() {
                test_wrapper(
                    String::new()
                        + "________XX"
                        + "_________X"
                        + "XXXXXXXX_X"
                        + "XXXXXXXX_X"
                        + "XXXXXXXX_X",
                    &[(true, I, Right, 8, 2), (false, I, Left, 8, 1)],
                );
            }

            #[test]
            fn checks2ng1() {
                test_wrapper(
                    String::new()
                        + "________XX"
                        + "_________X"
                        + "XXXXXXX__X"
                        + "XXXXXXXX_X"
                        + "XXXXXXXX_X",
                    &[(false, I, Right, 8, 2), (false, I, Left, 8, 1)],
                );
            }

            #[test]
            fn checks2ng2() {
                test_wrapper(
                    String::new()
                        + "________XX"
                        + "_________X"
                        + "XXXXX_XX_X"
                        + "XXXXXXXX_X"
                        + "XXXXXXXX_X",
                    &[(false, I, Right, 8, 2), (false, I, Left, 8, 1)],
                );
            }

            #[test]
            fn checks3ok1() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "XXX_______"
                        + "XXX_______"
                        + "XXX_XXXXXX"
                        + "XXX____XXX",
                    &[(true, I, Reverse, 5, 0), (false, I, Spawn, 4, 0)],
                );
            }

            #[test]
            fn checks3ng1() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "__________"
                        + "XXX_______"
                        + "XXX_XXXXXX"
                        + "XXX____XXX",
                    &[(false, I, Reverse, 5, 0), (false, I, Spawn, 4, 0)],
                );
            }

            #[test]
            fn checks3ok2() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "__________"
                        + "XXX_______"
                        + "X____XXXXX"
                        + "XXX____XXX",
                    &[
                        (true, I, Reverse, 3, 1),
                        (false, I, Spawn, 2, 1), 
                        (false, I, Reverse, 5, 0),
                        (false, I, Spawn, 4, 0)
                    ],
                );
            }

            #[test]
            fn checks3ok3() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "X_________"
                        + "XXX___XXXX"
                        + "XXX_XXXXXX"
                        + "XXX____XXX",
                    &[(true, I, Reverse, 5, 0), (true, I, Spawn, 4, 0)],
                );
            }

            #[test]
            fn checks3ok4() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "__________"
                        + "XXX___XXXX"
                        + "XXX_XXXXXX"
                        + "XXX____XXX",
                    &[(true, I, Reverse, 5, 0), (true, I, Spawn, 4, 0)],
                );
            }

            #[test]
            fn checks4ok1() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "_______XXX"
                        + "_______XXX"
                        + "XXXXXX_XXX"
                        + "XXX____XXX",
                    &[(true, I, Reverse, 5, 0), (false, I, Spawn, 4, 0)],
                );
            }

            #[test]
            fn checks4ok2() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "__________"
                        + "_______XXX"
                        + "XXXXXX_XXX"
                        + "XXX____XXX",
                    &[(true, I, Reverse, 5, 0), (false, I, Spawn, 4, 0)],
                );
            }

            #[test]
            fn checks4ok3() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "__________"
                        + "_______XXX"
                        + "XXXXX____X"
                        + "XXX____XXX",
                    &[
                        (true, I, Reverse, 7, 1),
                        (false, I, Spawn, 6, 1),
                        (false, I, Reverse, 5, 0),
                        (false, I, Spawn, 4, 0)
                    ],
                );
            }

            #[test]
            fn checks4ok4() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "_______XXX"
                        + "XXXX___XXX"
                        + "XXXXXX_XXX"
                        + "XXX____XXX",
                    &[(true, I, Reverse, 5, 0), (true, I, Spawn, 4, 0)],
                );
            }

            #[test]
            fn checks4ok5() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "_________X"
                        + "XXXX___XXX"
                        + "XXXXXX_XXX"
                        + "XXX____XXX",
                    &[(true, I, Reverse, 5, 0), (true, I, Spawn, 4, 0)],
                );
            }
        }

        mod with_o {
            use super::*;

            #[test]
            fn checks1ok1() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "X__XXXXXXX"
                        + "X___XXXXXX"
                        + "XX__XXXXXX",
                    &[
                        (true, O, Spawn, 1, 1),
                        (false, O, Spawn, 2, 0),
                        (false, O, Right, 2, 1),
                        (false, O, Reverse, 3, 1),
                        (false, O, Left, 3, 0),
                    ],
                );
            }
        }

        mod with_s {
            use super::*;

            #[test]
            fn checks1ok1() {
                #[rustfmt::skip]
                test_wrapper(
                    String::new()
                        + "XX__XXXXXX"
                        + "X__XXXXXXX",
                    &[
                        (true, S, Reverse, 2, 1),
                        (false, S, Spawn, 2, 0),
                    ],
                );
            }
        }
    }
}

// add #[rustfmt::skip] to any fields <= 4 lines.
