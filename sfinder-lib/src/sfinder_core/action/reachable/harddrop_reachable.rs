use crate::sfinder_core::{
    field::field::Field,
    mino::{mino::Mino, mino_factory::MinoFactory, mino_shifter::MinoShifter},
};

use super::reachable::Reachable;

pub struct HarddropReachable {
    // variable during search:
    appear_y: u8,
}

impl HarddropReachable {
    // TODO: max_y is not used
    pub fn new(max_y: u8) -> Self {
        Self { appear_y: 0 }
    }

    fn check_inner(&self, field: &dyn Field, mino: &Mino, x: u8, y: u8) -> bool {
        let max_y = u8::try_from(self.appear_y as i8 - mino.get_min_y()).unwrap();
        let harddrop_y = field.get_y_on_harddrop(mino, x, max_y);
        harddrop_y == y
    }
}

impl Reachable for HarddropReachable {
    fn checks(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
    ) -> bool {
        // Porting note: congruent actions do not matter, since we are only harddropping
        self.check(field, mino, x, y, valid_height)
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
    use crate::{
        common::datastore::action::action::Action,
        sfinder_core::{field::field_factory, mino::piece::Piece, srs::rotate::Rotate},
        sfinder_lib::{coordinate_walker::get_ranges, randoms},
    };
    use rand::thread_rng;

    #[test]
    fn checks() {
        let mino_factory = MinoFactory::new();
        let mut reachable = HarddropReachable::new(4);
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "_XX_____XX"
                + "__XX____XX"
                + "X_XX____XX"
                + "XXXXX___XX"
        );

        #[rustfmt::skip]
        {
            assert!(reachable.checks(field.as_ref(), mino_factory.get(Piece::I, Rotate::Spawn), 5, 1, 4));
            assert!(!reachable.checks(field.as_ref(), mino_factory.get(Piece::I, Rotate::Spawn), 5, 2, 4));
            assert!(!reachable.checks(field.as_ref(), mino_factory.get(Piece::S, Rotate::Left), 1, 2, 4));
        };
    }

    #[test]
    fn congruents() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mut reachable = HarddropReachable::new(4);

        let mut rngs = thread_rng();
        let field = randoms::gen_field(&mut rngs, 4, 5);

        let piece = randoms::gen_piece(&mut rngs);
        let rotate = randoms::gen_rotate(&mut rngs);
        let mino = mino_factory.get(piece, rotate);

        let (x_range, y_range) = get_ranges(mino, 4);

        for x in x_range {
            for y in y_range.clone() {
                if field.can_put(mino, x, y) && reachable.checks(field.as_ref(), mino, x, y, 4) {
                    for congruent in mino_shifter.congruent_actions(piece, rotate, x, y) {
                        assert!(
                            reachable.checks(
                                field.as_ref(),
                                mino_factory.get(piece, congruent.get_rotate()),
                                congruent.get_x(),
                                congruent.get_y(),
                                4
                            ),
                            "{piece} {rotate} {x} {y} -> {} {} {}",
                            congruent.get_rotate(),
                            congruent.get_x(),
                            congruent.get_y()
                        );
                    }
                }
            }
        }
    }
}
