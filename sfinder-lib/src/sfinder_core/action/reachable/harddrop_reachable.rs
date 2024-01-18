use crate::{
    common::datastore::action::action::Action,
    sfinder_core::{
        field::field::Field,
        mino::{mino::Mino, mino_factory::MinoFactory, mino_shifter::MinoShifter},
    },
};

use super::reachable::Reachable;

pub struct HarddropReachable<'a> {
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a MinoShifter,
    // variable during search:
    appear_y: u8,
}

impl<'a> HarddropReachable<'a> {
    // TODO: max_y is not used
    pub fn new(mino_factory: &'a MinoFactory, mino_shifter: &'a MinoShifter, _max_y: u8) -> Self {
        Self {
            mino_factory,
            mino_shifter,
            appear_y: 0,
        }
    }

    fn check_inner(&self, field: &dyn Field, mino: &Mino, x: u8, y: u8) -> bool {
        let max_y = u8::try_from(self.appear_y as i8 - mino.get_min_y()).unwrap();
        let harddrop_y = field.get_y_on_harddrop(mino, x, max_y);
        harddrop_y == y
    }
}

impl Reachable for HarddropReachable<'_> {
    fn checks(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
    ) -> bool {
        debug_assert!(field.can_put(mino, x, y));

        self.appear_y = valid_height;

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
        self.check_inner(field, mino, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_core::{field::field_factory, mino::piece::Piece, srs::rotate::Rotate};

    #[test]
    fn checks() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mut reachable = HarddropReachable::new(&mino_factory, &mino_shifter, 4);
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "_XX_____XX"
                + "__XX____XX"
                + "X_XX____XX"
                + "XXXXX___XX"
        );

        // assertThat(reachable.checks(field, minoFactory.create(Piece.I, Rotate.Spawn), 5, 1, 4)).isTrue();
        // assertThat(reachable.checks(field, minoFactory.create(Piece.I, Rotate.Spawn), 5, 2, 4)).isFalse();

        // assertThat(reachable.checks(field, minoFactory.create(Piece.S, Rotate.Left), 1, 2, 4)).isFalse();

        #[rustfmt::skip]
        {
            assert!(reachable.checks(field.as_ref(), mino_factory.get(Piece::I, Rotate::Spawn), 5, 1, 4));
            assert!(!reachable.checks(field.as_ref(), mino_factory.get(Piece::I, Rotate::Spawn), 5, 2, 4));
            assert!(!reachable.checks(field.as_ref(), mino_factory.get(Piece::S, Rotate::Left), 1, 2, 4));
        };
    }
}
