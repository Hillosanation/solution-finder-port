use super::{mino_shifter::IMinoShifter, piece::Piece};
use crate::{
    common::datastore::action::minimal_action::MinimalAction, sfinder_core::srs::rotate::Rotate,
};

pub struct PassedMinoShifter {}

impl PassedMinoShifter {
    pub fn new() -> Self {
        Self {}
    }
}

impl IMinoShifter for PassedMinoShifter {
    fn create_canonical_rotate(&self, _piece: Piece, rotate: Rotate) -> Rotate {
        rotate
    }

    fn create_canonical_action(
        &self,
        _piece: Piece,
        rotate: Rotate,
        x: u8,
        y: u8,
    ) -> MinimalAction {
        MinimalAction::new(x, y, rotate)
    }

    fn congruent_actions(&self, _piece: Piece, rotate: Rotate, x: u8, y: u8) -> Vec<MinimalAction> {
        vec![MinimalAction::new(x, y, rotate)]
    }

    fn get_unique_rotates(&self, piece: Piece) -> Vec<Rotate> {
        Rotate::value_list().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{sfinder_core::field::field_constants::FIELD_WIDTH, sfinder_lib::randoms};
    use rand::{thread_rng, Rng};

    #[test]
    fn create_canonical_action() {
        let mut rngs = thread_rng();
        let mino_shifter = PassedMinoShifter::new();

        for _ in 0..10000 {
            let piece = randoms::gen_piece(&mut rngs);
            let rotate = randoms::gen_rotate(&mut rngs);
            let x = rngs.gen_range(0..FIELD_WIDTH);
            let y = rngs.gen_range(0..12);
            let action = MinimalAction::new(x, y, rotate);

            let actual = mino_shifter.create_canonical_action(piece, rotate, x, y);

            assert_eq!(actual, action);
        }
    }

    #[test]
    fn create_canonical_rotate() {
        let mut rngs = thread_rng();
        let mino_shifter = PassedMinoShifter::new();

        for _ in 0..10000 {
            let piece = randoms::gen_piece(&mut rngs);
            let rotate = randoms::gen_rotate(&mut rngs);

            let actual = mino_shifter.create_canonical_rotate(piece, rotate);

            assert_eq!(actual, rotate);
        }
    }

    #[test]
    fn congruent_actions() {
        let mut rngs = thread_rng();
        let mino_shifter = PassedMinoShifter::new();

        for _ in 0..10000 {
            let piece = randoms::gen_piece(&mut rngs);
            let rotate = randoms::gen_rotate(&mut rngs);
            let x = rngs.gen_range(0..FIELD_WIDTH);
            let y = rngs.gen_range(0..12);

            let actual = mino_shifter.congruent_actions(piece, rotate, x, y);
            assert_eq!(actual, vec![MinimalAction::new(x, y, rotate)]);
        }
    }
}
