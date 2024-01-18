use super::{harddrop_reachable::HarddropReachable, reachable::Reachable, reachable_facade};
use crate::{
    common::{datastore::simple_mino_operation::SimpleMinoOperation, spin_checker::SpinChecker},
    searcher::spins::spin::TSpins,
    sfinder_core::{
        field::field::Field,
        mino::{mino::Mino, mino_factory::MinoFactory, mino_shifter::MinoShifter, piece::Piece},
        srs::{mino_rotation::MinoRotation, mino_rotation_detail::MinoRotationDetail},
    },
};

pub struct TSpinOrHarddropReachable<'a> {
    harddrop_reachable: HarddropReachable,
    min_cleared_rows: u8,
    spin_checker: SpinChecker<'a>,
    regular_only: bool,
}

impl<'a> TSpinOrHarddropReachable<'a> {
    pub fn new(
        mino_factory: &'a MinoFactory,
        mino_shifter: &'a MinoShifter,
        mino_rotation: &'a dyn MinoRotation,
        max_y: u8,
        min_cleared_rows: u8,
        regular_only: bool,
        use_180_rotation: bool,
    ) -> Self {
        Self {
            harddrop_reachable: HarddropReachable::new(max_y),
            min_cleared_rows,
            spin_checker: SpinChecker::new(
                mino_factory,
                MinoRotationDetail::new(mino_factory, mino_rotation),
                reachable_facade::create_locked(
                    mino_factory,
                    mino_shifter,
                    mino_rotation,
                    max_y,
                    use_180_rotation,
                ),
                // TODO: it seems like we should be able to just set use_180_rotation of create_locked from the constructor of SpinChecker?
                use_180_rotation,
            ),
            regular_only,
        }
    }
}

impl Reachable for TSpinOrHarddropReachable<'_> {
    fn checks(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
    ) -> bool {
        debug_assert!(field.can_put(mino, x, y));
        assert_eq!(field.get_filled_rows_key(), 0);

        if mino.get_piece() == Piece::T {
            let mut freeze = dyn_clone::clone_box(field);
            freeze.put(mino, x, y);
            let cleared_rows = freeze.clear_filled_rows() as u8;

            (cleared_rows >= self.min_cleared_rows)
                && match self.spin_checker.check(
                    field,
                    &SimpleMinoOperation::new(mino, x, y),
                    valid_height,
                    cleared_rows,
                ) {
                    Some(spin) if self.regular_only && spin.spin == TSpins::Mini => false,
                    Some(_) => true,
                    None => false,
                }
        } else {
            self.harddrop_reachable
                .checks(field, mino, x, y, valid_height)
        }
    }

    fn check(
        &mut self,
        field: &dyn Field,
        mino: &'static Mino,
        x: u8,
        y: u8,
        valid_height: u8,
    ) -> bool {
        unimplemented!(
            "SpinChecker always uses checks, so you can't search a specific action with it."
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entry::common::kicks::factory::srs_mino_rotation_factory,
        sfinder_core::field::field_factory, sfinder_core::srs::rotate::Rotate,
    };

    const MAX_Y: u8 = 8;

    #[test]
    fn visualize() {
        let mut field = field_factory::create_field(2);
        field.put(MinoFactory::new().get(Piece::T, Rotate::Spawn), 1, 1);

        #[rustfmt::skip]
        let field_2 = field_factory::create_field_with_marks(
            String::new()
                + "__________" 
                + "__________" 
                + "__________" 
                + "_XXXXXXXX_"
        );

        println!(
            "{}",
            field_2.can_put(MinoFactory::new().get(Piece::T, Rotate::Spawn), 1, 1)
        );

        // println!("{}", field);
    }

    #[test]
    fn case_harddrop() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();

        for required_rows in 1..=3 {
            let mut reachable = TSpinOrHarddropReachable::new(
                &mino_factory,
                &mino_shifter,
                mino_rotation.as_ref(),
                MAX_Y,
                required_rows,
                true,
                false,
            );

            #[rustfmt::skip]
            let field_1 = field_factory::create_field_with_marks(
                String::new()
                    + "__________"
                    + "__________"
                    + "X___XXXXXX"
                    + "XX_XXXXXXX"
            );

            assert!(!reachable.checks(
                field_1.as_ref(),
                MinoFactory::new().get(Piece::T, Rotate::Reverse),
                2,
                1,
                MAX_Y
            ));

            #[rustfmt::skip]
            let field_2 = field_factory::create_field_with_marks(
                String::new()
                    + "__________"
                    + "__________"
                    + "__XXXXXXXX"
                    + "_XXXXXXXXX"
            );

            assert!(!reachable.checks(
                field_2.as_ref(),
                MinoFactory::new().get(Piece::T, Rotate::Right),
                0,
                1,
                MAX_Y
            ));
        }
    }

    #[test]
    fn case_not_reachable() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();

        for required_rows in 1..=3 {
            let mut reachable = TSpinOrHarddropReachable::new(
                &mino_factory,
                &mino_shifter,
                mino_rotation.as_ref(),
                MAX_Y,
                required_rows,
                true,
                false,
            );

            #[rustfmt::skip]
            let field_1 = field_factory::create_field_with_marks(
                String::new()
                    + "__________"
                    + "__XX______"
                    + "X___XXXXXX"
                    + "XX_XXXXXXX"
            );

            assert!(!reachable.checks(
                field_1.as_ref(),
                MinoFactory::new().get(Piece::T, Rotate::Reverse),
                2,
                1,
                MAX_Y
            ));

            #[rustfmt::skip]
            let field_2 = field_factory::create_field_with_marks(
                String::new()
                    + "__________"
                    + "XX_XXXXXXX"
                    + "XX__XXXXXX"
                    + "XX_XXXXXXX"
            );

            assert!(!reachable.checks(
                field_2.as_ref(),
                MinoFactory::new().get(Piece::T, Rotate::Right),
                2,
                1,
                MAX_Y
            ));
        }
    }

    #[test]
    fn case_other_pieces() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();

        for required_rows in 1..=3 {
            let mut reachable = TSpinOrHarddropReachable::new(
                &mino_factory,
                &mino_shifter,
                mino_rotation.as_ref(),
                MAX_Y,
                required_rows,
                true,
                false,
            );

            #[rustfmt::skip]
            let field_1 = field_factory::create_field_with_marks(
                String::new()
                    + "__________"
                    + "XX__XXXXXX"
                    + "X__XXXXXXX"
            );

            assert!(!reachable.checks(
                field_1.as_ref(),
                // NOTE: made the MinoOperation valid and placable in field
                MinoFactory::new().get(Piece::S, Rotate::Reverse),
                2,
                1,
                MAX_Y
            ));

            #[rustfmt::skip]
            let field_2 = field_factory::create_field_with_marks(
                String::new()
                    + "__________"
                    + "XXXXXXXX__"
                    + "XXXXXXXXX_"
            );

            assert!(reachable.checks(
                field_2.as_ref(),
                MinoFactory::new().get(Piece::S, Rotate::Right),
                8,
                1,
                MAX_Y
            ));
        }
    }

    #[test]
    fn case_t_spin_without_cleared() {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();

        for required_rows in 1..=3 {
            let mut reachable = TSpinOrHarddropReachable::new(
                &mino_factory,
                &mino_shifter,
                mino_rotation.as_ref(),
                MAX_Y,
                required_rows,
                true,
                false,
            );

            #[rustfmt::skip]
            let field_1 = field_factory::create_field_with_marks(
                String::new()
                    + "__________"
                    + "___XXXXXX_"
                    + "X___XXXXX_"
                    + "XX_XXXXXX_"
            );

            assert!(!reachable.checks(
                field_1.as_ref(),
                MinoFactory::new().get(Piece::T, Rotate::Reverse),
                2,
                1,
                MAX_Y
            ));
        }
    }

    #[derive(Debug)]
    struct ExpectedThresholdResult {
        pub case_mini: [bool; 6],
        pub case_tss: [bool; 1],
        pub case_tsd: [bool; 3],
        pub case_tst: [bool; 1],
    }

    const TSPIN_ZERO_THRESHOLD: ExpectedThresholdResult = ExpectedThresholdResult {
        case_mini: [false, true, true, true, true, true],
        case_tss: [true],
        case_tsd: [true, true, true],
        case_tst: [true],
    };

    const TSPIN_MINI_THRESHOLD: ExpectedThresholdResult = ExpectedThresholdResult {
        case_mini: [false, false, false, true, true, true],
        case_tss: [true],
        case_tsd: [true, true, true],
        case_tst: [true],
    };

    const TSPIN_SINGLE_THRESHOLD: ExpectedThresholdResult = ExpectedThresholdResult {
        case_mini: [false, false, false, false, false, false],
        case_tss: [true],
        case_tsd: [true, true, true],
        case_tst: [true],
    };

    const TSPIN_DOUBLE_THRESHOLD: ExpectedThresholdResult = ExpectedThresholdResult {
        case_mini: [false, false, false, false, false, false],
        case_tss: [false],
        case_tsd: [true, true, true],
        case_tst: [true],
    };

    const TSPIN_TRIPLE_THRESHOLD: ExpectedThresholdResult = ExpectedThresholdResult {
        case_mini: [false, false, false, false, false, false],
        case_tss: [false],
        case_tsd: [false, false, false],
        case_tst: [true],
    };

    fn reachable_wrapper(
        reachable: &mut TSpinOrHarddropReachable,
        field: &dyn Field,
        piece: Piece,
        rotate: Rotate,
        x: u8,
        y: u8,
    ) -> bool {
        reachable.checks(field, MinoFactory::new().get(piece, rotate), x, y, MAX_Y)
    }

    fn test_thresholds(
        min_rows_cleared: u8,
        regular_only: bool,
        expected: ExpectedThresholdResult,
    ) {
        println!("testing {min_rows_cleared}, {regular_only}, {expected:?}");

        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();

        let mut reachable = TSpinOrHarddropReachable::new(
            &mino_factory,
            &mino_shifter,
            mino_rotation.as_ref(),
            MAX_Y,
            min_rows_cleared,
            regular_only,
            false,
        );

        // Mini tests
        #[rustfmt::skip]
        let field_1 = field_factory::create_field_with_marks(
            String::new()
                + "__________" 
                + "__________" 
                + "__________" 
                + "XXXX_XXXXX"
        );

        assert_eq!(
            reachable_wrapper(
                &mut reachable,
                field_1.as_ref(),
                Piece::T,
                Rotate::Reverse,
                4,
                1
            ),
            expected.case_mini[0]
        );

        #[rustfmt::skip]
        let field_2 = field_factory::create_field_with_marks(
            String::new()
                + "__________" 
                + "__________" 
                + "__________" 
                + "_XXXXXXXX_"
        );

        assert_eq!(
            reachable_wrapper(
                &mut reachable,
                field_2.as_ref(),
                Piece::T,
                Rotate::Right,
                0,
                1
            ),
            expected.case_mini[1]
        );
        assert_eq!(
            reachable_wrapper(
                &mut reachable,
                field_2.as_ref(),
                Piece::T,
                Rotate::Left,
                9,
                1
            ),
            expected.case_mini[2]
        );

        #[rustfmt::skip]
        let field_3 = field_factory::create_field_with_marks(
            String::new()
                + "__________" 
                + "__________" 
                + "__________" 
                + "XXXXXXXXX_"
        );

        assert_eq!(
            reachable_wrapper(
                &mut reachable,
                field_3.as_ref(),
                Piece::T,
                Rotate::Left,
                9,
                1
            ),
            expected.case_mini[3]
        );

        #[rustfmt::skip]
        let field_4 = field_factory::create_field_with_marks(
            String::new()
                + "X_________" 
                + "__________" 
                + "__________" 
                + "_XXXXXXXXX"
        );

        assert_eq!(
            reachable_wrapper(
                &mut reachable,
                field_4.as_ref(),
                Piece::T,
                Rotate::Right,
                0,
                1
            ),
            expected.case_mini[4]
        );

        // NEO
        let field_5 = field_factory::create_field_with_marks(
            String::new()
                + "XXX_______"
                + "X_________"
                + "XX________"
                + "XX__XXXXXX"
                + "XX_XXXXXXX",
        );

        assert_eq!(
            reachable_wrapper(
                &mut reachable,
                field_5.as_ref(),
                Piece::T,
                Rotate::Right,
                2,
                1
            ),
            expected.case_mini[5]
        );

        // TSS tests
        #[rustfmt::skip]
        let field_6 = field_factory::create_field_with_marks(
            String::new()
                + "__________"
                + "___XXXXXX_"
                + "X___XXXXX_"
                + "XX_XXXXXXX"
        );

        assert_eq!(
            reachable_wrapper(
                &mut reachable,
                field_6.as_ref(),
                Piece::T,
                Rotate::Reverse,
                2,
                1
            ),
            expected.case_tss[0]
        );

        // TSD tests
        #[rustfmt::skip]
        let field_7 = field_factory::create_field_with_marks(
            String::new()
                + "__________"
                + "___XXXXXXX"
                + "X___XXXXXX"
                + "XX_XXXXXXX"
        );

        assert_eq!(
            reachable_wrapper(
                &mut reachable,
                field_7.as_ref(),
                Piece::T,
                Rotate::Reverse,
                2,
                1
            ),
            expected.case_tsd[0]
        );

        // FIN
        let field_8 = field_factory::create_field_with_marks(
            String::new()
                + "XXXX______"
                + "XX________"
                + "XX________"
                + "XX__XXXXXX"
                + "XX_XXXXXXX",
        );

        assert_eq!(
            reachable_wrapper(
                &mut reachable,
                field_8.as_ref(),
                Piece::T,
                Rotate::Right,
                2,
                1
            ),
            expected.case_tsd[1]
        );

        // ISO
        let field_9 = field_factory::create_field_with_marks(
            String::new()
                + "___XXXXXXX"
                + "______XXXX"
                + "_____XXXXX"
                + "XXXX__XXXX"
                + "XXXX_XXXXX",
        );

        assert_eq!(
            reachable_wrapper(
                &mut reachable,
                field_9.as_ref(),
                Piece::T,
                Rotate::Right,
                4,
                1
            ),
            expected.case_tsd[2]
        );

        // TST tests
        let field_10 = field_factory::create_field_with_marks(
            String::new()
                + "XXXX______"
                + "XXX_______"
                + "XXX_XXXXXX"
                + "XXX__XXXXX"
                + "XXX_XXXXXX",
        );

        assert_eq!(
            reachable_wrapper(
                &mut reachable,
                field_10.as_ref(),
                Piece::T,
                Rotate::Right,
                3,
                1
            ),
            expected.case_tst[0]
        );
    }

    #[test]
    fn t_spin_tests() {
        test_thresholds(0, false, TSPIN_ZERO_THRESHOLD);
        test_thresholds(1, false, TSPIN_MINI_THRESHOLD);
        test_thresholds(1, true, TSPIN_SINGLE_THRESHOLD);
        test_thresholds(2, true, TSPIN_DOUBLE_THRESHOLD);
        test_thresholds(3, true, TSPIN_TRIPLE_THRESHOLD);
    }
}
