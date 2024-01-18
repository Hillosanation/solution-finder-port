use crate::{
    searcher::spins::{spin::Spin, spin_commons},
    sfinder_core::{
        action::{common::can_put_mino_in_field, reachable::reachable::ILockedReachable},
        field::field::Field,
        mino::{mino::Mino, mino_factory::MinoFactory, piece::Piece},
        srs::{mino_rotation_detail::MinoRotationDetail, rotate_direction::RotateDirection},
    },
};

use super::datastore::operation::Operation;

pub struct SpinChecker<'a> {
    mino_factory: &'a MinoFactory,
    mino_rotation_detail: MinoRotationDetail<'a>,
    locked_reachable: Box<dyn ILockedReachable + 'a>,
    search_rotations: Vec<RotateDirection>,
}

impl<'a> SpinChecker<'a> {
    pub fn new(
        mino_factory: &'a MinoFactory,
        mino_rotation_detail: MinoRotationDetail<'a>,
        locked_reachable: Box<dyn ILockedReachable + 'a>,
        use_180_rotation: bool,
    ) -> Self {
        Self {
            mino_factory,
            mino_rotation_detail,
            locked_reachable,
            search_rotations: (if use_180_rotation {
                RotateDirection::values_with_180()
            } else {
                RotateDirection::values_no_180()
            })
            .to_vec(),
        }
    }
}

impl SpinChecker<'_> {
    // TODO: this is only mutable becuase of locked_reachable
    pub fn check(
        &mut self,
        field: &dyn Field,
        operation: &dyn Operation, // TODO: seems like this is always constructed right before calling this function. Is replacing the bound with Action sufficient?
        field_height: u8,
        cleared_rows: u8,
    ) -> Option<Spin> {
        let rotate = operation.get_rotate();
        let x = operation.get_x();
        let y = operation.get_y();

        if !spin_commons::can_t_spin(field, x, y) {
            return None;
        }

        self.search_rotations
            // TODO: cloning is only necessary because self is mutable
            .clone()
            .into_iter()
            .flat_map(|direction| {
                let before_direction = direction.reverse();

                let before = self
                    .mino_factory
                    .get(Piece::T, rotate.apply(before_direction));

                self.get_spins(
                    field,
                    operation,
                    before,
                    direction,
                    field_height,
                    cleared_rows,
                )
            })
            .max()
    }

    fn get_spins(
        &mut self,
        field_without_t: &dyn Field,
        operation: &dyn Operation,
        before: &'static Mino,
        direction: RotateDirection,
        max_height: u8,
        cleared_rows: u8,
    ) -> Vec<Spin> {
        self.mino_rotation_detail
            .get_patterns_from(before, direction)
            .filter_map(|coord| {
                // TODO: this filtering should be done in can_put_mino_in_field instead to avoid comparing twice
                Some((
                    u8::try_from(operation.get_x() as i8 - coord.x).ok()?,
                    u8::try_from(operation.get_y() as i8 - coord.y).ok()?,
                ))
            })
            .filter(|(before_x, before_y)| {
                can_put_mino_in_field(field_without_t, before, *before_x, *before_y)
                    && (*before_y < max_height - before.get_max_y() as u8)
            })
            .filter_map(|(before_x, before_y)| {
                self.mino_rotation_detail
                    .get_kicks(field_without_t, direction, before, before_x, before_y)
                    .map(|spin_result| (before_x, before_y, spin_result))
            })
            .filter(|(before_x, before_y, spin_result)| {
                // 回転後に元の場所に戻る
                spin_result.x == operation.get_x()
                    && spin_result.y == operation.get_y()
                    // 回転前の位置に移動できる
                    && self.locked_reachable.checks(
                        field_without_t,
                        before,
                        *before_x,
                        *before_y,
                        max_height,
                    )
            })
            .map(|(_, _, spin_result)| {
                spin_commons::get_spins(field_without_t, &spin_result, cleared_rows)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::datastore::simple_operation::SimpleOperation,
        entry::common::kicks::factory::{file_mino_rotation_factory, srs_mino_rotation_factory},
        searcher::spins::spin::{ClearedRows, TSpins},
        sfinder_core::{
            action::reachable::reachable_facade, field::field_factory,
            mino::mino_shifter::MinoShifter, srs::rotate::Rotate,
        },
    };
    use std::path::PathBuf;

    #[test]
    fn spin() {
        let max_y = 24;
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();
        let mino_rotation_detail = MinoRotationDetail::new(&mino_factory, mino_rotation.as_ref());
        let locked_reachable = reachable_facade::create_90_locked(
            &mino_factory,
            &mino_shifter,
            mino_rotation.as_ref(),
            max_y,
        );
        let mut spin_checker =
            SpinChecker::new(&mino_factory, mino_rotation_detail, locked_reachable, false);

        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "X__XX_X___"
                + "X___XXXXXX"
                + "XX_XXXXXXX"
        );

        let operation = SimpleOperation::new(Piece::T, Rotate::Reverse, 2, 1);
        let spin = spin_checker
            .check(field.as_ref(), &operation, max_y, 2)
            .unwrap();

        assert_eq!(spin.spin, TSpins::Regular);
        assert_eq!(spin.cleared_rows, ClearedRows::Double);
    }

    #[test]
    fn spin_180() {
        let max_y = 24;
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = file_mino_rotation_factory::create(PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/kicks/jstris180.properties",
        ))
        .unwrap();
        let mino_rotation_detail = MinoRotationDetail::new(&mino_factory, mino_rotation.as_ref());
        let locked_reachable = reachable_facade::create_180_locked(
            &mino_factory,
            &mino_shifter,
            mino_rotation.as_ref(),
            max_y,
        );
        let mut spin_checker =
            SpinChecker::new(&mino_factory, mino_rotation_detail, locked_reachable, false);

        let field = field_factory::create_field_with_marks(
            String::new()
                + "____X_____"
                + "__XXXXX__X"
                + "_XXXX____X"
                + "XXXXX___XX"
                + "XXXXX_XXXX",
        );

        let operation = SimpleOperation::new(Piece::T, Rotate::Right, 5, 1);
        let spin = spin_checker
            .check(field.as_ref(), &operation, max_y, 1)
            .unwrap();

        assert_eq!(spin.spin, TSpins::Mini);
        assert_eq!(spin.cleared_rows, ClearedRows::Single);
    }
}
