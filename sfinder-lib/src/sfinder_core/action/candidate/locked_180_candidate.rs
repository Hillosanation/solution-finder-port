use super::candidate::{Candidate, ILockedCandidate};
use crate::{
    common::datastore::action::{cache::locked_cache::LockedCache, minimal_action::MinimalAction},
    sfinder_core::{
        action::common::{can_put_mino_in_field, FromDirection},
        field::{field::Field, field_constants::FIELD_WIDTH},
        mino::{mino::Mino, mino_factory::MinoFactory, mino_shifter::MinoShifter, piece::Piece},
        srs::{mino_rotation::MinoRotation, rotate::Rotate, rotate_direction::RotateDirection},
    },
    sfinder_lib::coordinate_walker::get_ranges,
};
use nohash::{BuildNoHashHasher, IntSet};

pub struct Locked180Candidate<'a> {
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a MinoShifter,
    mino_rotation: &'a dyn MinoRotation,
    // variable during search:
    locked_cache: LockedCache,
    appear_y: u8,
}

impl<'a> Locked180Candidate<'a> {
    pub fn new(
        mino_factory: &'a MinoFactory,
        mino_shifter: &'a MinoShifter,
        mino_rotation: &'a dyn MinoRotation,
        max_y: u8,
    ) -> Self {
        assert!(
            mino_rotation.supports_180(),
            "180 rotation should be supported by mino rotation"
        );

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
        if self.appear_y <= y {
            return true;
        }

        let rotate = mino.get_rotate();

        if self.locked_cache.is_visited(x, y, rotate) {
            return self.locked_cache.is_found(x, y, rotate);
        }

        self.locked_cache.visit(x, y, rotate);

        if field.can_reach_on_harddrop(mino, x, y) {
            self.locked_cache.found(x, y, rotate);
            return true;
        }

        let up_y = y + 1;
        if up_y < self.appear_y
            && field.can_put(mino, x, up_y)
            && self.check(field, mino, x, up_y, FromDirection::None)
        {
            self.locked_cache.found(x, y, rotate);
            return true;
        }

        if let Some(left_x) = x.checked_sub(1) {
            if direction != FromDirection::Left
                && -mino.get_min_x() <= left_x as i8
                && field.can_put(mino, left_x, y)
                && self.check(field, mino, left_x, y, FromDirection::Right)
            {
                self.locked_cache.found(x, y, rotate);
                return true;
            }
        }

        let right_x = x + 1;
        if direction != FromDirection::Right
            && (right_x as i8) < FIELD_WIDTH as i8 - mino.get_max_x()
            && field.can_put(mino, right_x, y)
            && self.check(field, mino, right_x, y, FromDirection::Left)
        {
            self.locked_cache.found(x, y, rotate);
            return true;
        }

        if self.check_rotation(field, mino, x, y, RotateDirection::Clockwise) {
            self.locked_cache.found(x, y, rotate);
            return true;
        }

        if self.check_rotation(field, mino, x, y, RotateDirection::CounterClockwise) {
            self.locked_cache.found(x, y, rotate);
            return true;
        }

        // same as LockedCandidate, but with 180 rotation
        if self.check_rotation(field, mino, x, y, RotateDirection::Rotate180) {
            self.locked_cache.found(x, y, rotate);
            return true;
        }

        return false;
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

impl Candidate for Locked180Candidate<'_> {
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
                            actions.insert(
                                self.mino_shifter
                                    .create_canonical_action(piece, rotate, x, y),
                            );
                        }
                        self.locked_cache.reset_trail();
                    }
                }
            }
        }

        actions
    }
}

impl ILockedCandidate for Locked180Candidate<'_> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entry::common::kicks::factory::{file_mino_rotation_factory, srs_mino_rotation_factory},
        sfinder_core::{action::candidate::candidate_facade, field::field_factory},
    };
    use std::path::PathBuf;

    #[test]
    fn test_search1() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = file_mino_rotation_factory::create(PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/kicks/nullpomino180.properties",
        ))
        .unwrap();
        let mut candidate = candidate_facade::create_180_locked(
            &mino_factory,
            &mino_shifter,
            mino_rotation.as_ref(),
            6,
        );

        let field = field_factory::create_field_with_marks(
            String::new()
                + "_XXXXXXXXX"
                + "_X_XXXXXX_"
                + "_X_XXXXXX_"
                + "_X_XXXXXX_"
                + "_X_XXXXXX_",
        );

        let actions = candidate.search(field.as_ref(), Piece::I, 6);
        assert_eq!(actions.len(), 9);
        for action in [
            MinimalAction::new(1, 5, Rotate::Spawn),
            MinimalAction::new(2, 5, Rotate::Spawn),
            MinimalAction::new(3, 5, Rotate::Spawn),
            MinimalAction::new(4, 5, Rotate::Spawn),
            MinimalAction::new(5, 5, Rotate::Spawn),
            MinimalAction::new(6, 5, Rotate::Spawn),
            MinimalAction::new(7, 5, Rotate::Spawn),
            MinimalAction::new(0, 1, Rotate::Left),
            MinimalAction::new(2, 1, Rotate::Left),
        ] {
            assert!(actions.contains(&action));
        }
    }

    #[test]
    fn test_search2() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "XXXXXX____"
                + "XXXXXX__X_"
                + "XXXXXXX__X",
        );

        let mut actions;

        {
            let mino_rotation = srs_mino_rotation_factory::create();
            let mut candidate = candidate_facade::create_90_locked(
                &mino_factory,
                &mino_shifter,
                mino_rotation.as_ref(),
                24,
            );
            actions = candidate.search(field.as_ref(), Piece::L, 24);
            assert_eq!(actions.len(), 34);
        }
        {
            let mino_rotation = file_mino_rotation_factory::create(PathBuf::from(
                std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/kicks/nokicks.properties",
            ))
            .unwrap();
            let mut candidate = candidate_facade::create_180_locked(
                &mino_factory,
                &mino_shifter,
                mino_rotation.as_ref(),
                24,
            );
            let actions_nokick = candidate.search(field.as_ref(), Piece::L, 24);
            assert_eq!(actions_nokick.len(), 35);

            // SRSでは到達できないActionを加えると同じになる
            actions.insert(MinimalAction::new(7, 1, Rotate::Right));
            assert_eq!(actions, actions_nokick);
        }
    }
}
