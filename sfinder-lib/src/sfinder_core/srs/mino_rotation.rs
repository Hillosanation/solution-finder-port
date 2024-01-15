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
        _get_kicks(
            field,
            x,
            y,
            after,
            self.get_patterns_from(before, direction),
        )
    }

    // You way want to use get_offsets_from instead, if you don't need information about the privilege spins
    fn get_patterns_from(&self, current: &'static Mino, direction: RotateDirection) -> &Pattern {
        &self.get_map(direction)[into_val(current)]
    }

    // Porting note: replaces getPatternsFrom
    fn get_offsets_from(
        &self,
        current: &'static Mino,
        direction: RotateDirection,
    ) -> &[Coordinate] {
        self.get_patterns_from(current, direction).get_offsets()
    }

    fn is_privilege_spins(
        &self,
        before: &'static Mino,
        direction: RotateDirection,
        test_pattern_index: u8,
    ) -> bool {
        self.get_patterns_from(before, direction)
            .is_privilege_spins_at(test_pattern_index)
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
    let offsets = pattern.get_offsets();
    let min_x = -after.get_min_x();
    let max_x = FIELD_WIDTH as i8 - after.get_max_x();
    let min_y = -after.get_min_y();

    offsets
        .iter()
        .find(|offset| {
            let to_x = x as i8 + offset.x;
            let to_y = y as i8 + offset.y;

            min_x <= to_x
                && to_x < max_x
                && min_y <= to_y
                && field.can_put(after, to_x as u8, to_y as u8)
        })
        .copied()
}
