use crate::{
    common::datastore::action::{action::Action, cache::minimal_locked_cache::MinimalLockedCache},
    sfinder_core::{
        action::common::{can_put_mino_in_field, FromDirection},
        field::{field::Field, field_constants::FIELD_WIDTH},
        mino::{mino::Mino, mino_factory::MinoFactory, mino_shifter::IMinoShifter},
        srs::{mino_rotation::MinoRotation, rotate_direction::RotateDirection},
    },
};

use super::reachable::{ILockedReachable, Reachable};

pub struct Locked180Reachable<'a> {
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a dyn IMinoShifter,
    mino_rotation: &'a dyn MinoRotation,
    // variable during search:
    locked_cache: MinimalLockedCache,
    appear_y: u8,
}

impl<'a> Locked180Reachable<'a> {
    pub fn new(
        mino_factory: &'a MinoFactory,
        mino_shifter: &'a dyn IMinoShifter,
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
            locked_cache: MinimalLockedCache::new(max_y),
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
        if self.appear_y <= y {
            return true;
        }

        let rotate = mino.get_rotate();

        if self.locked_cache.is_visited(x, y, rotate) {
            return false;
        }

        self.locked_cache.visit(x, y, rotate);

        if field.can_reach_on_harddrop(mino, x, y) {
            return true;
        }

        let up_y = y + 1;
        if up_y < self.appear_y
            && field.can_put(mino, x, up_y)
            && self.check_inner(field, mino, x, up_y, FromDirection::None)
        {
            return true;
        }

        if let Some(left_x) = x.checked_sub(1) {
            if direction != FromDirection::Left
                && -mino.get_min_x() <= left_x as i8
                && field.can_put(mino, left_x, y)
                && self.check_inner(field, mino, left_x, y, FromDirection::Right)
            {
                return true;
            }
        }

        let right_x = x + 1;
        if direction != FromDirection::Right
            && (right_x as i8) < FIELD_WIDTH as i8 - mino.get_max_x()
            && field.can_put(mino, right_x, y)
            && self.check_inner(field, mino, right_x, y, FromDirection::Left)
        {
            return true;
        }

        if self.check_rotation(field, mino, x, y, RotateDirection::Clockwise) {
            return true;
        }

        if self.check_rotation(field, mino, x, y, RotateDirection::CounterClockwise) {
            return true;
        }

        // same as LockedCandidate, but with 180 rotation
        if self.check_rotation(field, mino, x, y, RotateDirection::Rotate180) {
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
            .get(mino.get_piece(), current_rotate.apply(direction.reverse()));

        self.mino_rotation
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
                    && self.check_inner(field, mino_before, from_x, from_y, FromDirection::None)
            })
    }
}

impl Reachable for Locked180Reachable<'_> {
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
        debug_assert!(field.can_put(mino, x, y));

        self.appear_y = valid_height;
        self.locked_cache.clear();

        self.check_inner(field, mino, x, y, FromDirection::None)
    }
}
