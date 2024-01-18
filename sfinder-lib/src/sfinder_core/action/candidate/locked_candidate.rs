use super::candidate::{Candidate, ILockedCandidate};
use crate::{
    common::datastore::action::{cache::locked_cache::LockedCache, minimal_action::MinimalAction},
    sfinder_core::{
        action::common::{can_put_mino_in_field, FromDirection},
        field::{field::Field, field_constants::FIELD_WIDTH},
        mino::{
            mino::Mino,
            mino_factory::MinoFactory,
            mino_shifter::{IMinoShifter, MinoShifter},
            piece::Piece,
        },
        srs::{mino_rotation::MinoRotation, rotate::Rotate, rotate_direction::RotateDirection},
    },
    sfinder_lib::coordinate_walker::get_ranges,
};
use nohash::{BuildNoHashHasher, IntSet};

pub struct LockedCandidate<'a> {
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a MinoShifter,
    mino_rotation: &'a dyn MinoRotation,
    // variable during search:
    locked_cache: LockedCache,
    appear_y: u8,
}

impl<'a> LockedCandidate<'a> {
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

    fn check(
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
            // その時の結果を返却。訪問済みだが結果が出てないときは他の探索でカバーできるためfalseを返却
            return self.locked_cache.is_found(x, y, rotate);
        }

        self.locked_cache.visit(x, y, rotate);

        let result = {
            // harddropでたどりつけるとき
            field.can_reach_on_harddrop(mino, x, y)
                || {
                    // 上に移動
                    let up_y = y + 1;

                    up_y < self.appear_y
                        && field.can_put(mino, x, up_y)
                        && self.check(field, mino, x, up_y, FromDirection::None)
                }
                // 左に移動
                || x.checked_sub(1).map_or(false, |left_x| {
                    direction != FromDirection::Left
                        && -mino.get_min_x() <= left_x as i8
                        && field.can_put(mino, left_x, y)
                        && self.check(field, mino, left_x, y, FromDirection::Right)
                    })
                || {
                    // 右に移動
                    let right_x = x + 1;

                    direction != FromDirection::Right
                        && (right_x as i8) < FIELD_WIDTH as i8 - mino.get_max_x()
                        && field.can_put(mino, right_x, y)
                        && self.check(field, mino, right_x, y, FromDirection::Left)
                }
                // 右回転でくる可能性がある場所を移動
                || self.check_rotation(field, mino, x, y, RotateDirection::Clockwise)
                // 左回転でくる可能性がある場所を移動
                || self.check_rotation(field, mino, x, y, RotateDirection::CounterClockwise)
        };

        if result {
            self.locked_cache.found(x, y, rotate);
        }

        result
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
            .get(mino.get_piece(), current_rotate.apply(direction.reverse()));

        let result = self
            .mino_rotation
            .get_patterns_from(mino_before, direction)
            .get_offsets()
            .filter_map(|pattern| {
                Some((
                    pattern,
                    // TODO: this filtering should be done in can_put_mino_in_field instead to avoid comparing twice
                    u8::try_from(x as i8 - pattern.x).ok()?,
                    u8::try_from(y as i8 - pattern.y).ok()?,
                ))
            })
            .any(|(pattern, from_x, from_y)| {
                can_put_mino_in_field(field, mino_before, from_x, from_y)
                    && self.mino_rotation.get_kicks(
                        field,
                        mino_before,
                        mino,
                        from_x,
                        from_y,
                        direction,
                    ) == Some(*pattern)
                    && self.check(field, mino_before, from_x, from_y, FromDirection::None)
            });

        if result {
            self.locked_cache.found(x, y, current_rotate);
        }

        result
    }
}

impl Candidate for LockedCandidate<'_> {
    fn search(
        &mut self,
        field: &dyn Field,
        piece: Piece,
        valid_height: u8,
    ) -> IntSet<MinimalAction> {
        self.appear_y = valid_height;
        self.locked_cache.clear();

        let mut actions = IntSet::with_hasher(BuildNoHashHasher::default());

        for &rotate in Rotate::value_list() {
            let mino = self.mino_factory.get(piece, rotate);

            let (x_range, y_range) = get_ranges(mino, valid_height);

            for x in x_range {
                for y in y_range.clone().rev() {
                    if field.can_put(mino, x, y) && field.is_on_ground(mino, x, y) {
                        if self.check(field, mino, x, y, FromDirection::None) {
                            // println!("passed, {:?}", (piece, rotate, x, y));

                            let action = self
                                .mino_shifter
                                .create_canonical_action(piece, rotate, x, y);
                            actions.insert(action);
                        }
                        self.locked_cache.reset_trail();
                    }
                }
            }
        }

        actions
    }
}

impl ILockedCandidate for LockedCandidate<'_> {}

#[cfg(test)]
mod tests {
    use crate::{
        common::datastore::action::action::Action,
        entry::common::kicks::factory::srs_mino_rotation_factory,
        sfinder_core::{
            action::{candidate::candidate_facade, reachable::reachable_facade},
            field::{field_factory, field_view},
        },
        sfinder_lib::{coordinate_walker, randoms},
    };
    use rand::{thread_rng, Rng};
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_search1() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();
        let mut candidates = candidate_facade::create_90_locked(
            &mino_factory,
            &mino_shifter,
            mino_rotation.as_ref(),
            4,
        );

        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "____X_____"
        );

        let actions = candidates.search(field.as_ref(), Piece::T, 4);

        #[rustfmt::skip]
        {
            assert_eq!(actions.iter().filter(|action| action.get_rotate() == Rotate::Spawn).count(), 8);
            assert_eq!(actions.iter().filter(|action| action.get_rotate() == Rotate::Right).count(), 9);
            assert_eq!(actions.iter().filter(|action| action.get_rotate() == Rotate::Reverse).count(), 8);
            assert_eq!(actions.iter().filter(|action| action.get_rotate() == Rotate::Left).count(), 9);
        };
    }

    #[test]
    fn test_search2() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();
        let mut candidates = candidate_facade::create_90_locked(
            &mino_factory,
            &mino_shifter,
            mino_rotation.as_ref(),
            4,
        );

        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "XXXX______"
                + "XXXXX_____"
                + "X___X_____"
                + "XX_XX_____"
        );

        let actions = candidates.search(field.as_ref(), Piece::T, 4);

        #[rustfmt::skip]
        {
            assert_eq!(actions.iter().filter(|action| action.get_rotate() == Rotate::Spawn).count(), 3);
            assert_eq!(actions.iter().filter(|action| action.get_rotate() == Rotate::Right).count(), 4);
            assert_eq!(actions.iter().filter(|action| action.get_rotate() == Rotate::Reverse).count(), 4);
            assert_eq!(actions.iter().filter(|action| action.get_rotate() == Rotate::Left).count(), 4);
        };
    }

    #[test]
    fn test_search3() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();
        let mut candidates = candidate_facade::create_90_locked(
            &mino_factory,
            &mino_shifter,
            mino_rotation.as_ref(),
            4,
        );

        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "XXXX______"
                + "XX_XXXXX__"
                + "X___X_____"
                + "XX_X______"
        );

        assert_eq!(
            candidates.search(field.as_ref(), Piece::T, 4),
            [
                MinimalAction::new(8, 0, Rotate::Spawn),
                MinimalAction::new(7, 0, Rotate::Spawn),
                MinimalAction::new(6, 0, Rotate::Spawn),
                MinimalAction::new(5, 0, Rotate::Spawn),
                MinimalAction::new(8, 1, Rotate::Reverse),
                MinimalAction::new(7, 1, Rotate::Reverse),
                MinimalAction::new(6, 1, Rotate::Reverse),
                MinimalAction::new(8, 3, Rotate::Reverse),
                MinimalAction::new(9, 1, Rotate::Left),
                MinimalAction::new(8, 1, Rotate::Left),
                MinimalAction::new(8, 1, Rotate::Right),
            ]
            .into_iter()
            .collect::<HashSet<_, _>>()
        );
    }

    #[test]
    fn test_search4() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();
        let mut candidates = candidate_facade::create_90_locked(
            &mino_factory,
            &mino_shifter,
            mino_rotation.as_ref(),
            4,
        );

        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "X_________"
                + "XX__XXXXXX"
                + "X__XXXXXXX"
                + "X_XXXXXXXX"
        );

        assert_eq!(
            mino_shifter.create_canonical_action(Piece::Z, Rotate::Reverse, 2, 3),
            MinimalAction::new(2, 2, Rotate::Spawn)
        );
        assert_eq!(
            mino_shifter.create_canonical_action(Piece::Z, Rotate::Left, 3, 2),
            MinimalAction::new(2, 2, Rotate::Right)
        );

        assert_eq!(
            candidates.search(field.as_ref(), Piece::Z, 4),
            [
                MinimalAction::new(2, 2, Rotate::Spawn),
                // MinimalAction::new(2, 3, Rotate::Reverse),
                // MinimalAction::new(3, 2, Rotate::Left),
                MinimalAction::new(2, 2, Rotate::Right),
                MinimalAction::new(1, 1, Rotate::Right),
            ]
            .into_iter()
            .collect::<HashSet<_, _>>()
        );
    }

    #[test]
    fn test_random() {
        let mut rngs = thread_rng();

        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();

        for _ in 0..10000 {
            let random_height = rngs.gen_range(2..=12);
            let num_of_minos = rngs.gen_range(4..random_height * 10 / 4);
            let mut field = randoms::gen_field(&mut rngs, random_height, num_of_minos);
            let height = random_height - field.clear_filled_rows() as u8;
            let piece = randoms::gen_piece(&mut rngs);

            let mut candidates = candidate_facade::create_90_locked(
                &mino_factory,
                &mino_shifter,
                mino_rotation.as_ref(),
                height,
            );
            let actions = candidates.search(field.as_ref(), piece, height);

            // println!("{actions:?}");

            let mut reachable = reachable_facade::create_90_locked(
                &mino_factory,
                &mino_shifter,
                mino_rotation.as_ref(),
                height,
            );

            for &rotate in Rotate::value_list() {
                // dbg!((rotate, height));
                let mino = mino_factory.get(piece, rotate);
                for (x, y) in coordinate_walker::walk(mino, height) {
                    // {
                    //     println!("{:?}", (piece, rotate, x, y));
                    //     println!("{}", field_view::to_string(field.as_ref()));
                    //     println!("%%%%%%%%%%");
                    //     let mut mino_field = field_factory::create_field(height);
                    //     mino_field.put(mino, x, y);
                    //     println!("{field:?}");
                    //     println!("{}", field_view::to_reduced_string(mino_field.as_ref()));
                    // }

                    let can_put = field.can_put(mino, x, y);
                    let on_ground = field.is_on_ground(mino, x, y);
                    let reachable = can_put && reachable.checks(field.as_ref(), mino, x, y, height);

                    let canonical_action =
                        mino_shifter.create_canonical_action(piece, rotate, x, y);
                    // Porting note: because actions only contains canonical actions, we need to normalize our action before checking
                    let is_candidate = actions.contains(&canonical_action);
                    let expected = can_put && on_ground && reachable;

                    assert_eq!(
                        is_candidate,
                        expected,
                        "{actions:?}\n{can_put} {on_ground} {reachable}\n{}\n%%%%%%%%%%\n{}",
                        field_view::to_string(field.as_ref()),
                        {
                            let mut mino_field = field_factory::create_field(height);
                            mino_field.put(mino, x, y);
                            field_view::to_reduced_string(mino_field.as_ref())
                        }
                    );
                }
            }
        }
    }
}
