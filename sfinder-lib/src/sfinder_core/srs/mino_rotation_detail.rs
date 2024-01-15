use super::{
    mino_rotation::MinoRotation, pattern::Pattern, rotate_direction::RotateDirection,
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
        offsets: &Pattern,
    ) -> Option<SpinResult> {
        let min_x = -after.get_min_x();
        let max_x = FIELD_WIDTH as i8 - after.get_max_x();
        let min_y = -after.get_min_y();

        // TODO: we could just check for priviledge spins here
        offsets
            .get_offsets()
            .iter()
            .enumerate()
            .find_map(|(index, offset)| {
                let to_x = u8::try_from(before_x as i8 + offset.x).unwrap();
                let to_y = u8::try_from(before_y as i8 + offset.y).unwrap();

                (min_x <= (to_x as i8)
                    && (to_x as i8) < max_x
                    && min_y <= (to_y as i8)
                    && field.can_put(after, to_x, to_y))
                .then(|| (to_x, to_y, index))
            })
            .map(|(to_x, to_y, index)| {
                let mut freeze = dyn_clone::clone_box(field);
                freeze.put(after, to_x, to_y);
                let is_privilege_spins =
                    self.mino_rotation
                        .is_privilege_spins(before, direction, index as u8);
                SpinResult::new(
                    after,
                    to_x,
                    to_y,
                    index as u8,
                    direction,
                    is_privilege_spins,
                )
            })
    }
}
