use nohash::{BuildNoHashHasher, IntSet};

use super::candidate::Candidate;
use crate::{
    common::datastore::action::minimal_action::MinimalAction,
    sfinder_core::{
        field::{field::Field, field_constants::FIELD_WIDTH},
        mino::{
            mino_factory::MinoFactory,
            mino_shifter::{IMinoShifter, MinoShifter},
            piece::Piece,
        },
        srs::rotate::Rotate,
    },
    sfinder_lib::coordinate_walker::get_ranges,
};

pub struct HarddropCandidate<'a> {
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a MinoShifter,
}

impl<'a> HarddropCandidate<'a> {
    pub fn new(mino_factory: &'a MinoFactory, mino_shifter: &'a MinoShifter) -> Self {
        Self {
            mino_factory,
            mino_shifter,
        }
    }
}

impl Candidate for HarddropCandidate<'_> {
    fn search(
        &mut self,
        field: &dyn Field,
        piece: Piece,
        valid_height: u8,
    ) -> IntSet<MinimalAction> {
        let mut actions = IntSet::with_hasher(BuildNoHashHasher::default());

        for rotate in self.mino_shifter.get_unique_rotates(piece) {
            let mino = self.mino_factory.get(piece, rotate);

            let y = u8::try_from(valid_height as i8 - mino.get_min_y()).unwrap();
            let max_y = u8::try_from(valid_height as i8 - mino.get_max_y()).unwrap();

            let (x_range, _) = get_ranges(mino, max_y);

            for x in x_range {
                let harddrop_y = field.get_y_on_harddrop(mino, x, y);
                if harddrop_y < max_y {
                    // Porting note: since rotations are already canonical, there is no need to convert them to canonical form
                    actions.insert(MinimalAction::new(x, harddrop_y, rotate));
                }
            }
        }

        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::datastore::action::action::Action,
        sfinder_core::{field::field_factory, srs::rotate::Rotate},
        sfinder_lib::randoms,
    };

    #[test]
    fn test_search1() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mut candidate = HarddropCandidate::new(&mino_factory, &mino_shifter);

        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "____X_____",  
        );

        let actions = candidate.search(field.as_ref(), Piece::T, 4);
        assert_eq!(actions.len(), 34);
        #[rustfmt::skip]
        {
            assert_eq!(actions.iter().filter(|e| e.get_rotate() == Rotate::Spawn).count(), 8);
            assert_eq!(actions.iter().filter(|e| e.get_rotate() == Rotate::Right).count(), 9);
            assert_eq!(actions.iter().filter(|e| e.get_rotate() == Rotate::Reverse).count(), 8);
            assert_eq!(actions.iter().filter(|e| e.get_rotate() == Rotate::Left).count(), 9);
        };
    }

    #[test]
    fn test_search2() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mut candidate = HarddropCandidate::new(&mino_factory, &mino_shifter);
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "XXXX______"
                + "XXXXX_____"
                + "X___X_____"
                + "XX_XX_____",  
        );

        let actions = candidate.search(field.as_ref(), Piece::T, 4);
        assert_eq!(actions.len(), 15);
        #[rustfmt::skip]
        {
            assert_eq!(actions.iter().filter(|e| e.get_rotate() == Rotate::Spawn).count(), 3);
            assert_eq!(actions.iter().filter(|e| e.get_rotate() == Rotate::Right).count(), 4);
            assert_eq!(actions.iter().filter(|e| e.get_rotate() == Rotate::Reverse).count(), 4);
            assert_eq!(actions.iter().filter(|e| e.get_rotate() == Rotate::Left).count(), 4);
        };
    }

    #[test]
    fn test_search3() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mut candidate = HarddropCandidate::new(&mino_factory, &mino_shifter);
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "XXXX______"
                + "XX_XXXXX__"
                + "X___X_____"
                + "XX_XX_____",  
        );

        let actions = candidate.search(field.as_ref(), Piece::T, 4);
        assert_eq!(actions.len(), 3);
        assert!(actions.contains(&MinimalAction::new(9, 1, Rotate::Left)));
        assert!(actions.contains(&MinimalAction::new(8, 3, Rotate::Reverse)));
        assert!(actions.contains(&MinimalAction::new(8, 1, Rotate::Right)));
    }

    #[test]
    fn test_search4() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mut candidate = HarddropCandidate::new(&mino_factory, &mino_shifter);
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "_______XXX"
                + "_________X",  
        );

        let actions = candidate.search(field.as_ref(), Piece::T, 2);
        assert!(actions.contains(&MinimalAction::new(5, 0, Rotate::Spawn)));
        assert!(!actions.contains(&MinimalAction::new(6, 0, Rotate::Spawn)));
    }

    #[test]
    fn canonical() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mut candidate = HarddropCandidate::new(&mino_factory, &mino_shifter);
        let mut rngs = rand::thread_rng();
        let field = randoms::gen_field(&mut rngs, 8, 10);
        let piece = randoms::gen_piece(&mut rngs);

        let actions = candidate.search(field.as_ref(), piece, 4);
        for action in actions {
            assert_eq!(
                action,
                mino_shifter.create_canonical_action(
                    piece,
                    action.get_rotate(),
                    action.get_x(),
                    action.get_y()
                )
            );
        }
    }
}
