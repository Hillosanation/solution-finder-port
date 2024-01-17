use crate::{
    common::datastore::action::{action::Action, cache::locked_cache::LockedCache},
    sfinder_core::{
        action::common::{can_put_mino_in_field, FromDirection},
        field::{field::Field, field_constants::FIELD_WIDTH},
        mino::{mino::Mino, mino_factory::MinoFactory, mino_shifter::MinoShifter, piece::Piece},
        srs::{mino_rotation::MinoRotation, rotate::Rotate, rotate_direction::RotateDirection},
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

    pub fn checks(
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
                self.check(
                    field,
                    piece,
                    action.get_x(),
                    action.get_y(),
                    action.get_rotate(),
                )
            })
    }

    fn check(&mut self, field: &dyn Field, piece: Piece, x: u8, y: u8, rotate: Rotate) -> bool {
        self.check_inner(
            field,
            self.mino_factory.get(piece, rotate),
            x,
            y,
            FromDirection::None,
        )
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
