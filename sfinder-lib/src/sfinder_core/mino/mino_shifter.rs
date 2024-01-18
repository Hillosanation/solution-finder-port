//! Porting note: I introduce congruence here, where two Minos occupy the same cells/coordinates.
//! We arbitrarily pick one of them as the "canonical" one, and call the others "congruent".

use super::{mino_transform::MinoTransform, piece::Piece};
use crate::{
    common::datastore::action::minimal_action::MinimalAction, sfinder_core::srs::rotate::Rotate,
};

pub trait IMinoShifter {
    // Porting note: replaces createTransformedRotate
    fn create_canonical_rotate(&self, piece: Piece, rotate: Rotate) -> Rotate;

    // The other version accepting an Action is dropped since it's not used
    // Used by Candidate
    // Porting note: replaces createTransformedAction
    fn create_canonical_action(&self, piece: Piece, rotate: Rotate, x: u8, y: u8) -> MinimalAction;

    // Used by Reachable
    // Porting note: replaces enumerateSameOtherActions
    /// Porting note: Note that this also adds the current action into the result, see #17.
    fn congruent_actions(&self, piece: Piece, rotate: Rotate, x: u8, y: u8) -> Vec<MinimalAction>;

    // Porting note: this is usually iterated over immediately anyways, so just return a Vec instead of HashSet.
    // Note that this also gives all canonical rotations of the piece.
    fn get_unique_rotates(&self, piece: Piece) -> Vec<Rotate>;
}

#[derive(Debug)]
pub struct MinoShifter {
    transformers: [MinoTransform; Piece::get_size()],
}

impl MinoShifter {
    pub fn new() -> Self {
        Self {
            transformers: [
                MinoTransform::new(),
                // I
                MinoTransform::set_with(&[
                    (Rotate::Right, 0, -1, Rotate::Left),
                    (Rotate::Reverse, -1, 0, Rotate::Spawn),
                ]),
                MinoTransform::new(),
                MinoTransform::new(),
                // S
                MinoTransform::set_with(&[
                    (Rotate::Right, 1, 0, Rotate::Left),
                    (Rotate::Reverse, 0, -1, Rotate::Spawn),
                ]),
                // Z
                MinoTransform::set_with(&[
                    (Rotate::Left, -1, 0, Rotate::Right),
                    (Rotate::Reverse, 0, -1, Rotate::Spawn),
                ]),
                // O
                MinoTransform::set_with(&[
                    (Rotate::Right, 0, -1, Rotate::Spawn),
                    (Rotate::Reverse, -1, -1, Rotate::Spawn),
                    (Rotate::Left, -1, 0, Rotate::Spawn),
                ]),
            ],
        }
    }
}

impl IMinoShifter for MinoShifter {
    fn create_canonical_rotate(&self, piece: Piece, rotate: Rotate) -> Rotate {
        self.transformers[piece as usize].transform_rotate(rotate)
    }

    fn create_canonical_action(&self, piece: Piece, rotate: Rotate, x: u8, y: u8) -> MinimalAction {
        self.transformers[piece as usize].transform(x, y, rotate)
    }

    fn congruent_actions(&self, piece: Piece, rotate: Rotate, x: u8, y: u8) -> Vec<MinimalAction> {
        self.transformers[piece as usize].congruent_actions(x, y, rotate)
    }

    fn get_unique_rotates(&self, piece: Piece) -> Vec<Rotate> {
        self.transformers[piece as usize].get_unique_rotates()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_rotates_are_canonical() {
        let shifter = MinoShifter::new();

        for &piece in Piece::value_list() {
            let unique = shifter.get_unique_rotates(piece);
            let canonical = Rotate::value_list()
                .iter()
                .map(|rotate| shifter.create_canonical_rotate(piece, *rotate))
                .collect::<Vec<_>>();

            assert!(
                unique.iter().all(|rotate| canonical.contains(rotate)),
                "{unique:?} does not contain all of {canonical:?}"
            );
        }
    }

    fn check_transformed_action(input: Vec<((Piece, Rotate, u8, u8), MinimalAction)>) {
        let shifter = MinoShifter::new();

        for ((piece, rotate, x, y), expected) in input {
            assert_eq!(
                shifter.create_canonical_action(piece, rotate, x, y),
                expected
            );
        }
    }

    fn check_transformed_rotate(input: Vec<((Piece, Rotate), Rotate)>) {
        let shifter = MinoShifter::new();

        for ((piece, rotate), expected) in input {
            assert_eq!(shifter.create_canonical_rotate(piece, rotate), expected);
        }
    }

    fn check_same_other_actions(input: Vec<((Piece, Rotate, u8, u8), Vec<MinimalAction>)>) {
        let shifter = MinoShifter::new();

        for ((piece, rotate, x, y), expected_others) in input {
            let result = shifter.congruent_actions(piece, rotate, x, y);

            // dbg!(piece, rotate, x, y, &result, &expected_others);
            assert_eq!(
                expected_others.len() + 1,
                result.len(),
                "expected: {expected_others:?}, result: {result:?}",
            );
            assert!(
                expected_others.iter().all(|action| result.contains(action)),
                "{result:?} does not contain all of {expected_others:?}",
            );
            assert!(result.contains(&MinimalAction::new(x, y, rotate)));
        }
    }

    mod i {
        use super::*;
        #[test]
        fn create_transformed_action() {
            check_transformed_action(vec![
                (
                    (Piece::I, Rotate::Spawn, 1, 0),
                    MinimalAction::new(1, 0, Rotate::Spawn),
                ),
                (
                    (Piece::I, Rotate::Reverse, 2, 0),
                    MinimalAction::new(1, 0, Rotate::Spawn),
                ),
                (
                    (Piece::I, Rotate::Left, 0, 1),
                    MinimalAction::new(0, 1, Rotate::Left),
                ),
                (
                    (Piece::I, Rotate::Right, 0, 2),
                    MinimalAction::new(0, 1, Rotate::Left),
                ),
            ]);
        }

        #[test]
        fn create_transformed_rotate() {
            check_transformed_rotate(vec![
                ((Piece::I, Rotate::Spawn), Rotate::Spawn),
                ((Piece::I, Rotate::Reverse), Rotate::Spawn),
                ((Piece::I, Rotate::Left), Rotate::Left),
                ((Piece::I, Rotate::Right), Rotate::Left),
            ]);
        }

        #[test]
        fn enumerate_same_other_actions() {
            check_same_other_actions(vec![
                (
                    (Piece::I, Rotate::Spawn, 1, 0),
                    vec![MinimalAction::new(2, 0, Rotate::Reverse)],
                ),
                (
                    (Piece::I, Rotate::Reverse, 2, 0),
                    vec![MinimalAction::new(1, 0, Rotate::Spawn)],
                ),
                (
                    (Piece::I, Rotate::Left, 0, 1),
                    vec![MinimalAction::new(0, 2, Rotate::Right)],
                ),
                (
                    (Piece::I, Rotate::Right, 0, 2),
                    vec![MinimalAction::new(0, 1, Rotate::Left)],
                ),
            ]);
        }
    }

    mod s {
        use super::*;

        #[test]
        fn create_transformed_action() {
            check_transformed_action(vec![
                (
                    (Piece::S, Rotate::Spawn, 1, 0),
                    MinimalAction::new(1, 0, Rotate::Spawn),
                ),
                (
                    (Piece::S, Rotate::Reverse, 1, 1),
                    MinimalAction::new(1, 0, Rotate::Spawn),
                ),
                (
                    (Piece::S, Rotate::Left, 1, 1),
                    MinimalAction::new(1, 1, Rotate::Left),
                ),
                (
                    (Piece::S, Rotate::Right, 0, 1),
                    MinimalAction::new(1, 1, Rotate::Left),
                ),
            ]);
        }

        #[test]
        fn create_transformed_rotate() {
            check_transformed_rotate(vec![
                ((Piece::S, Rotate::Spawn), Rotate::Spawn),
                ((Piece::S, Rotate::Reverse), Rotate::Spawn),
                ((Piece::S, Rotate::Left), Rotate::Left),
                ((Piece::S, Rotate::Right), Rotate::Left),
            ]);
        }

        #[test]
        fn enumerate_same_other_actions() {
            check_same_other_actions(vec![
                (
                    (Piece::S, Rotate::Spawn, 1, 0),
                    vec![MinimalAction::new(1, 1, Rotate::Reverse)],
                ),
                (
                    (Piece::S, Rotate::Reverse, 1, 1),
                    vec![MinimalAction::new(1, 0, Rotate::Spawn)],
                ),
                (
                    (Piece::S, Rotate::Left, 1, 1),
                    vec![MinimalAction::new(0, 1, Rotate::Right)],
                ),
                (
                    (Piece::S, Rotate::Right, 0, 1),
                    vec![MinimalAction::new(1, 1, Rotate::Left)],
                ),
            ]);
        }
    }

    mod z {
        use super::*;

        #[test]
        fn create_transformed_action() {
            check_transformed_action(vec![
                (
                    (Piece::Z, Rotate::Spawn, 1, 0),
                    MinimalAction::new(1, 0, Rotate::Spawn),
                ),
                (
                    (Piece::Z, Rotate::Reverse, 1, 1),
                    MinimalAction::new(1, 0, Rotate::Spawn),
                ),
                (
                    (Piece::Z, Rotate::Left, 1, 1),
                    MinimalAction::new(0, 1, Rotate::Right),
                ),
                (
                    (Piece::Z, Rotate::Right, 0, 1),
                    MinimalAction::new(0, 1, Rotate::Right),
                ),
            ]);
        }

        #[test]
        fn create_transformed_rotate() {
            check_transformed_rotate(vec![
                ((Piece::Z, Rotate::Spawn), Rotate::Spawn),
                ((Piece::Z, Rotate::Reverse), Rotate::Spawn),
                ((Piece::Z, Rotate::Left), Rotate::Right),
                ((Piece::Z, Rotate::Right), Rotate::Right),
            ]);
        }

        #[test]
        fn enumerate_same_other_actions() {
            check_same_other_actions(vec![
                (
                    (Piece::Z, Rotate::Spawn, 1, 0),
                    vec![MinimalAction::new(1, 1, Rotate::Reverse)],
                ),
                (
                    (Piece::Z, Rotate::Reverse, 1, 1),
                    vec![MinimalAction::new(1, 0, Rotate::Spawn)],
                ),
                (
                    (Piece::Z, Rotate::Left, 1, 1),
                    vec![MinimalAction::new(0, 1, Rotate::Right)],
                ),
                (
                    (Piece::Z, Rotate::Right, 0, 1),
                    vec![MinimalAction::new(1, 1, Rotate::Left)],
                ),
            ]);
        }
    }

    mod o {
        use super::*;

        #[test]
        fn create_transformed_action() {
            check_transformed_action(vec![
                (
                    (Piece::O, Rotate::Spawn, 0, 0),
                    MinimalAction::new(0, 0, Rotate::Spawn),
                ),
                (
                    (Piece::O, Rotate::Reverse, 1, 1),
                    MinimalAction::new(0, 0, Rotate::Spawn),
                ),
                (
                    (Piece::O, Rotate::Left, 1, 0),
                    MinimalAction::new(0, 0, Rotate::Spawn),
                ),
                (
                    (Piece::O, Rotate::Right, 0, 1),
                    MinimalAction::new(0, 0, Rotate::Spawn),
                ),
            ]);
        }

        #[test]
        fn create_transformed_rotate() {
            check_transformed_rotate(vec![
                ((Piece::O, Rotate::Spawn), Rotate::Spawn),
                ((Piece::O, Rotate::Reverse), Rotate::Spawn),
                ((Piece::O, Rotate::Left), Rotate::Spawn),
                ((Piece::O, Rotate::Right), Rotate::Spawn),
            ]);
        }

        #[test]
        fn enumerate_same_other_actions() {
            check_same_other_actions(vec![
                (
                    (Piece::O, Rotate::Spawn, 0, 0),
                    vec![
                        MinimalAction::new(1, 1, Rotate::Reverse),
                        MinimalAction::new(1, 0, Rotate::Left),
                        MinimalAction::new(0, 1, Rotate::Right),
                    ],
                ),
                (
                    (Piece::O, Rotate::Reverse, 1, 1),
                    vec![
                        MinimalAction::new(0, 0, Rotate::Spawn),
                        MinimalAction::new(1, 0, Rotate::Left),
                        MinimalAction::new(0, 1, Rotate::Right),
                    ],
                ),
                (
                    (Piece::O, Rotate::Left, 1, 0),
                    vec![
                        MinimalAction::new(0, 0, Rotate::Spawn),
                        MinimalAction::new(1, 1, Rotate::Reverse),
                        MinimalAction::new(0, 1, Rotate::Right),
                    ],
                ),
                (
                    (Piece::O, Rotate::Right, 0, 1),
                    vec![
                        MinimalAction::new(0, 0, Rotate::Spawn),
                        MinimalAction::new(1, 1, Rotate::Reverse),
                        MinimalAction::new(1, 0, Rotate::Left),
                    ],
                ),
            ]);
        }
    }

    mod t {
        use super::*;

        #[test]
        fn create_transformed_action() {
            check_transformed_action(vec![
                (
                    (Piece::T, Rotate::Spawn, 1, 0),
                    MinimalAction::new(1, 0, Rotate::Spawn),
                ),
                (
                    (Piece::T, Rotate::Reverse, 1, 1),
                    MinimalAction::new(1, 1, Rotate::Reverse),
                ),
                (
                    (Piece::T, Rotate::Left, 1, 1),
                    MinimalAction::new(1, 1, Rotate::Left),
                ),
                (
                    (Piece::T, Rotate::Right, 0, 1),
                    MinimalAction::new(0, 1, Rotate::Right),
                ),
            ]);
        }

        #[test]
        fn create_transformed_rotate() {
            check_transformed_rotate(vec![
                ((Piece::T, Rotate::Spawn), Rotate::Spawn),
                ((Piece::T, Rotate::Reverse), Rotate::Reverse),
                ((Piece::T, Rotate::Left), Rotate::Left),
                ((Piece::T, Rotate::Right), Rotate::Right),
            ]);
        }

        #[test]
        fn enumerate_same_other_actions() {
            check_same_other_actions(vec![
                ((Piece::T, Rotate::Spawn, 1, 0), vec![]),
                ((Piece::T, Rotate::Reverse, 1, 1), vec![]),
                ((Piece::T, Rotate::Left, 1, 1), vec![]),
                ((Piece::T, Rotate::Right, 0, 1), vec![]),
            ]);
        }
    }

    mod l {
        use super::*;

        #[test]
        fn create_transformed_action() {
            check_transformed_action(vec![
                (
                    (Piece::L, Rotate::Spawn, 1, 0),
                    MinimalAction::new(1, 0, Rotate::Spawn),
                ),
                (
                    (Piece::L, Rotate::Reverse, 1, 1),
                    MinimalAction::new(1, 1, Rotate::Reverse),
                ),
                (
                    (Piece::L, Rotate::Left, 1, 1),
                    MinimalAction::new(1, 1, Rotate::Left),
                ),
                (
                    (Piece::L, Rotate::Right, 0, 1),
                    MinimalAction::new(0, 1, Rotate::Right),
                ),
            ]);
        }

        #[test]
        fn create_transformed_rotate() {
            check_transformed_rotate(vec![
                ((Piece::L, Rotate::Spawn), Rotate::Spawn),
                ((Piece::L, Rotate::Reverse), Rotate::Reverse),
                ((Piece::L, Rotate::Left), Rotate::Left),
                ((Piece::L, Rotate::Right), Rotate::Right),
            ]);
        }

        #[test]
        fn enumerate_same_other_actions() {
            check_same_other_actions(vec![
                ((Piece::L, Rotate::Spawn, 1, 0), vec![]),
                ((Piece::L, Rotate::Reverse, 1, 1), vec![]),
                ((Piece::L, Rotate::Left, 1, 1), vec![]),
                ((Piece::L, Rotate::Right, 0, 1), vec![]),
            ]);
        }
    }

    mod j {
        use super::*;

        #[test]
        fn create_transformed_action() {
            check_transformed_action(vec![
                (
                    (Piece::J, Rotate::Spawn, 1, 0),
                    MinimalAction::new(1, 0, Rotate::Spawn),
                ),
                (
                    (Piece::J, Rotate::Reverse, 1, 1),
                    MinimalAction::new(1, 1, Rotate::Reverse),
                ),
                (
                    (Piece::J, Rotate::Left, 1, 1),
                    MinimalAction::new(1, 1, Rotate::Left),
                ),
                (
                    (Piece::J, Rotate::Right, 0, 1),
                    MinimalAction::new(0, 1, Rotate::Right),
                ),
            ]);
        }

        #[test]
        fn create_transformed_rotate() {
            check_transformed_rotate(vec![
                ((Piece::J, Rotate::Spawn), Rotate::Spawn),
                ((Piece::J, Rotate::Reverse), Rotate::Reverse),
                ((Piece::J, Rotate::Left), Rotate::Left),
                ((Piece::J, Rotate::Right), Rotate::Right),
            ]);
        }

        #[test]
        fn enumerate_same_other_actions() {
            check_same_other_actions(vec![
                ((Piece::J, Rotate::Spawn, 1, 0), vec![]),
                ((Piece::J, Rotate::Reverse, 1, 1), vec![]),
                ((Piece::J, Rotate::Left, 1, 1), vec![]),
                ((Piece::J, Rotate::Right, 0, 1), vec![]),
            ]);
        }
    }
}
