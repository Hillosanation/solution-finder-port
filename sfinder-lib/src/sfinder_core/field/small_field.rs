use super::{
    bit_operators,
    field::{Field, FieldHelper},
    field_constants::{BOARD_HEIGHT, FIELD_WIDTH, VALID_BOARD_RANGE},
    key_operators, long_board_map,
};
use crate::sfinder_core::{field::field_constants::BoardCount, mino::mino::Mino};
use std::fmt::Debug;

pub const MAX_FIELD_HEIGHT: u8 = BOARD_HEIGHT;

/// Porting note: clone replaces copy constructor
#[derive(Clone)]
pub struct SmallField(u64);

impl SmallField {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get_x_board(&self) -> u64 {
        self.0
    }
}

impl From<u64> for SmallField {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<SmallField> for u64 {
    fn from(value: SmallField) -> Self {
        value.0
    }
}

impl Field for SmallField {
    fn get_max_field_height(&self) -> u8 {
        MAX_FIELD_HEIGHT
    }

    fn get_board_count(&self) -> BoardCount {
        BoardCount::Small
    }

    fn set_block(&mut self, x: u8, y: u8) {
        self.0 |= bit_operators::get_x_mask(x, y);
    }

    fn remove_block(&mut self, x: u8, y: u8) {
        self.0 &= !bit_operators::get_x_mask(x, y);
    }

    fn put(&mut self, mino: &Mino, x: u8, y: u8) {
        self.0 |= mino.get_mask(x, y as i8);
    }

    fn can_put(&self, mino: &Mino, x: u8, y: u8) -> bool {
        MAX_FIELD_HEIGHT + 2 <= y || self.0 & mino.get_mask(x, y as i8) == 0
    }

    fn remove(&mut self, mino: &Mino, x: u8, y: u8) {
        self.0 &= !mino.get_mask(x, y as i8);
    }

    fn can_reach_on_harddrop(&self, mino: &Mino, x: u8, start_y: u8) -> bool {
        self._can_reach_on_harddrop(mino, x, start_y, MAX_FIELD_HEIGHT)
    }

    fn is_empty_block(&self, x: u8, y: u8) -> bool {
        self.0 & bit_operators::get_x_mask(x, y) == 0
    }

    fn exists_above_row(&self, y: u8) -> bool {
        let mask = <dyn Field>::get_valid_mask(y);
        y < MAX_FIELD_HEIGHT && (self.0 & mask) != 0
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn is_filled_in_column(&self, x: u8, max_y: u8) -> bool {
        if max_y == 0 {
            return true;
        }

        self.0 | !bit_operators::get_column_mask(max_y, x) == !0
    }

    fn is_wall_between_left(&self, x: u8, max_y: u8) -> bool {
        bit_operators::is_wall_between_left(x, max_y, self.0)
    }

    fn get_block_count_in_column(&self, x: u8, max_y: u8) -> u32 {
        (self.0 & bit_operators::get_column_mask(max_y, x)).count_ones()
    }

    fn get_block_count_in_row(&self, y: u8) -> u32 {
        (self.0 & bit_operators::get_row_mask(y)).count_ones()
    }

    fn exists_block_in_row(&self, y: u8) -> bool {
        (self.0 & bit_operators::get_row_mask(y)) != 0
    }

    fn get_num_of_all_blocks(&self) -> u32 {
        self.0.count_ones()
    }

    fn clear_filled_rows_return_key(&mut self) -> u64 {
        let delete_key = self.get_filled_rows_key();

        self.delete_rows_with_key(delete_key);

        delete_key
    }

    fn get_filled_rows_key(&self) -> u64 {
        key_operators::get_delete_key(self.0)
    }

    fn get_using_key(&self) -> u64 {
        key_operators::get_using_key(self.0)
    }

    fn insert_filled_row_with_key(&mut self, delete_key: u64) {
        self.0 = long_board_map::insert_filled_row(self.0, delete_key);
    }

    fn insert_blank_row_with_key(&mut self, delete_key: u64) {
        self.0 = long_board_map::insert_blank_row(self.0, delete_key);
    }

    fn delete_rows_with_key(&mut self, delete_key: u64) {
        self.0 = long_board_map::delete_row(self.0, delete_key);
    }

    fn fill_row(&mut self, y: u8) {
        self.0 |= bit_operators::get_row_mask(y);
    }

    fn get_board(&self, index: u8) -> u64 {
        if index == 0 {
            self.0
        } else {
            0
        }
    }

    fn prune(&self, _max_height: u8) -> Box<dyn Field> {
        Box::new(self.clone())
    }

    fn merge(&mut self, other: &dyn Field) {
        self.0 |= other.get_board(0);
    }

    fn can_merge(&self, other: &dyn Field) -> bool {
        self.0 & other.get_board(0) == 0
    }

    fn reduce(&mut self, other: &dyn Field) {
        self.0 &= !other.get_board(0);
    }

    fn get_upper_y_with_4_blocks(&self) -> u8 {
        assert_eq!(self.get_num_of_all_blocks(), 4);
        bit_operators::get_highest_y(self.0)
    }

    fn get_min_x(&self) -> Option<u8> {
        bit_operators::try_get_lowest_x(self.0)
    }

    fn get_min_y(&self) -> Option<u8> {
        bit_operators::try_get_lowest_y(self.0)
    }

    // The bitshifts are moving the values to lower/higer significance, which is why they are opposite of the semantic direction

    fn slide_left(&mut self, slide: u8) {
        let mask = bit_operators::get_column_mask_right_of_row(slide);
        self.0 = (self.0 & mask) >> slide;
    }

    fn slide_right(&mut self, slide: u8) {
        let mask = bit_operators::get_column_mask_left_of_row(FIELD_WIDTH - slide);
        self.0 = (self.0 & mask) << slide;
    }

    fn slide_down_one(&mut self) {
        self.0 >>= FIELD_WIDTH;
    }

    fn slide_down(&mut self, slide: u8) {
        self.0 >>= slide * FIELD_WIDTH;
    }

    fn slide_up_with_filled_row(&mut self, slide: u8) {
        let count = slide * FIELD_WIDTH;
        self.0 = (self.0 << count) | ((1 << count) - 1);
    }

    fn slide_up_with_empty_row(&mut self, slide: u8) {
        self.0 <<= slide * FIELD_WIDTH;
    }

    fn contains(&self, child: &dyn Field) -> bool {
        // prevents Large from ever running
        assert!(child.get_board_count() <= BoardCount::Middle);

        let child_board_low = child.get_board(0);

        match child.get_board_count() {
            BoardCount::Small => self.0 & child_board_low == child_board_low,
            BoardCount::Middle => {
                self.0 & child_board_low == child_board_low && child.get_board(1) == 0
            }
            BoardCount::Large => {
                self.0 & child_board_low == child_board_low
                    && child.get_board(1) == 0
                    && child.get_board(2) == 0
                    && child.get_board(3) == 0
            }
        }
    }

    fn invert(&mut self) {
        self.0 = !self.0 & VALID_BOARD_RANGE;
    }

    fn mirror(&mut self) {
        self.0 = key_operators::mirror(self.0);
    }

    fn mask(&mut self, mask_field: &dyn Field) {
        self.0 &= mask_field.get_board(0);
    }
}

impl Debug for SmallField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SmallField {:#062b}", self.0)
    }
}

impl PartialEq for SmallField {
    fn eq(&self, other: &Self) -> bool {
        self as &dyn Field == other as &_
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::datastore::{action::action::Action, mino_operation::MinoOperation},
        sfinder_core::{
            field::{field_constants::BoardCount, field_factory},
            mino::{mino_factory::MinoFactory, piece::Piece},
            neighbor::original_piece::create_all_pieces,
            srs::rotate::Rotate,
        },
        sfinder_lib::{boolean_walker, randoms},
    };
    use rand::{rngs::ThreadRng, thread_rng, Rng};

    fn create_random_small_field(rngs: &mut ThreadRng, empty_minos: u8) -> SmallField {
        // although gen_field should always return a SmallField here, it is erased by the return type
        let field = randoms::gen_field(rngs, MAX_FIELD_HEIGHT, empty_minos);
        SmallField::from(field.get_board(0))
    }

    #[test]
    fn test_get_max_field_height() {
        assert_eq!(
            field_factory::create_small_field().get_max_field_height(),
            MAX_FIELD_HEIGHT
        );
    }

    #[test]
    fn test_put_and_remove_block() {
        let mut field = field_factory::create_small_field();
        assert!(field.is_empty_block(0, 0));
        field.set_block(0, 0);
        assert!(!field.is_empty_block(0, 0));
        field.remove_block(0, 0);
        assert!(field.is_empty_block(0, 0));
    }

    #[test]
    fn test_put_and_remove_mino() {
        let mut field = field_factory::create_small_field();
        field.put(&Mino::new(Piece::T, Rotate::Spawn), 1, 0);
        assert!(!field.is_empty_block(0, 0));
        assert!(!field.is_empty_block(1, 0));
        assert!(!field.is_empty_block(2, 0));
        assert!(!field.is_empty_block(1, 1));

        field.remove(&Mino::new(Piece::T, Rotate::Spawn), 1, 0);
        assert!(field.is_empty());
    }

    #[test]
    fn test_put_and_remove_piece() {
        let mut field = field_factory::create_small_field();
        let field_height = field.get_max_field_height();

        for piece in create_all_pieces(&MinoFactory::new(), field_height) {
            // Initialize
            let mino = piece.get_mino();
            let x = piece.get_x();
            let y = piece.get_y();

            // Expect
            let mut expected = field_factory::create_small_field();
            expected.put(mino, x, y);

            // Test
            field.put_piece(&piece);

            assert_eq!(field, expected);

            field.remove_piece(&piece);

            assert!(field.is_empty());
        }
    }

    #[test]
    fn test_get_y_on_harddrop() {
        #[rustfmt::skip]
        let field = field_factory::create_small_field_with_marks(String::new() +
            "__________" +
            "_________X" +
            "____X_____"
        );

        let mino = Mino::new(Piece::T, Rotate::Spawn);
        assert_eq!(field.get_y_on_harddrop(&mino, 1, 4), 0);
        assert_eq!(field.get_y_on_harddrop(&mino, 3, 4), 1);
        assert_eq!(field.get_y_on_harddrop(&mino, 8, 4), 2);
    }

    #[test]
    fn test_can_reach_on_harddrop() {
        #[rustfmt::skip]
        let field = field_factory::create_small_field_with_marks(String::new() +
            "x_________" +
            "_________X" +
            "____X_____"
        );

        let mino = Mino::new(Piece::T, Rotate::Spawn);
        assert!(field.can_reach_on_harddrop(&mino, 1, 4));
        assert!(field.can_reach_on_harddrop(&mino, 1, 3));
        assert!(!field.can_reach_on_harddrop(&mino, 1, 1));
    }

    #[test]
    fn test_can_reach_on_harddrop_2() {
        let mut rngs = thread_rng();
        let field = create_random_small_field(&mut rngs, 8);

        for piece in create_all_pieces(&MinoFactory::new(), field.get_max_field_height()) {
            let mino = piece.get_mino();
            let x = piece.get_x();
            let y = piece.get_y();

            assert_eq!(
                field.can_reach_on_harddrop_piece(&piece),
                field.can_put(mino, x, y) && field.can_reach_on_harddrop(mino, x, y)
            );
        }
    }

    #[test]
    fn test_exist_above() {
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "___X______"
                + "__________"
                + "_______X__"
                + "__________",
        );

        assert!(field.exists_above_row(0));
        assert!(field.exists_above_row(1));
        assert!(field.exists_above_row(2));
        assert!(field.exists_above_row(3));
        assert!(!field.exists_above_row(4));
        assert!(!field.exists_above_row(5));
    }

    #[test]
    fn test_is_perfect() {
        let mut field = field_factory::create_small_field();

        assert!(field.is_empty_block(0, 0));
        assert!(field.is_empty());

        field.set_block(0, 0);

        assert!(!field.is_empty_block(0, 0));
        assert!(!field.is_empty());
    }

    #[test]
    fn test_is_filled_in_column() {
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "____X_____"
                + "____X_____"
                + "___XX_____"
                + "__XXX_____"
                + "X__XX_____"
                + "__XXXX____",
        );

        assert!(!field.is_filled_in_column(0, 4));
        assert!(!field.is_filled_in_column(1, 4));
        assert!(!field.is_filled_in_column(2, 4));
        assert!(field.is_filled_in_column(3, 4));
        assert!(!field.is_filled_in_column(3, 6));
        assert!(field.is_filled_in_column(4, 4));
        assert!(field.is_filled_in_column(4, 6));
        assert!(field.is_filled_in_column(5, 0));
    }

    #[test]
    fn test_is_wall_between_left() {
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "____X_____"
                + "____X_____"
                + "X__XX_____"
                + "_XXXX_____"
                + "XX_XX_____"
                + "_XXXXX____",
        );

        assert!(field.is_wall_between_left(1, 4));
        assert!(!field.is_wall_between_left(1, 5));
        assert!(field.is_wall_between_left(2, 3));
        assert!(!field.is_wall_between_left(2, 4));
        assert!(field.is_wall_between_left(2, 3));
        assert!(!field.is_wall_between_left(2, 4));
        assert!(field.is_wall_between_left(4, 6));
        assert!(field.is_wall_between_left(5, 6));
        assert!(!field.is_wall_between_left(6, 6));
    }

    #[test]
    fn test_can_put_mino() {
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "___X______"
                + "___XX_____"
                + "__X_X_____"
                + "X___X_____"
                + "__X_XX____",
        );

        assert!(field.can_put(&Mino::new(Piece::T, Rotate::Spawn), 5, 4));
        assert!(field.can_put(&Mino::new(Piece::T, Rotate::Right), 1, 1));
        assert!(field.can_put(&Mino::new(Piece::T, Rotate::Reverse), 1, 3));
        assert!(field.can_put(&Mino::new(Piece::T, Rotate::Left), 3, 1));

        assert!(!field.can_put(&Mino::new(Piece::T, Rotate::Spawn), 3, 0));
        assert!(!field.can_put(&Mino::new(Piece::T, Rotate::Right), 0, 1));
        assert!(!field.can_put(&Mino::new(Piece::T, Rotate::Reverse), 1, 1));
        assert!(!field.can_put(&Mino::new(Piece::T, Rotate::Left), 1, 1));
    }

    #[test]
    fn test_can_put_mino_2() {
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X",
        );

        assert!(field.can_put(&Mino::new(Piece::I, Rotate::Left), 8, 1));
        assert!(field.can_put(&Mino::new(Piece::I, Rotate::Left), 8, 6));
        assert!(field.can_put(&Mino::new(Piece::I, Rotate::Left), 8, 7));
        assert!(field.can_put(&Mino::new(Piece::I, Rotate::Left), 8, 8));
        assert!(field.can_put(&Mino::new(Piece::I, Rotate::Left), 8, 9));
    }

    #[test]
    fn test_can_put_piece() {
        let mut rngs = thread_rng();
        let field = create_random_small_field(&mut rngs, 8);

        for piece in create_all_pieces(&MinoFactory::new(), field.get_max_field_height()) {
            let mino = piece.get_mino();
            let x = piece.get_x();
            let y = piece.get_y();

            assert_eq!(field.can_put_piece(&piece), field.can_put(mino, x, y));
        }
    }

    #[test]
    fn test_is_on_ground() {
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "___X______"
                + "___XX_____"
                + "___XX_____"
                + "__X_X_____"
                + "X___X_____"
                + "__X_XX____",
        );

        assert!(field.is_on_ground(&Mino::new(Piece::T, Rotate::Spawn), 5, 5));
        assert!(field.is_on_ground(&Mino::new(Piece::T, Rotate::Right), 8, 1));
        assert!(field.is_on_ground(&Mino::new(Piece::T, Rotate::Reverse), 1, 3));
        assert!(field.is_on_ground(&Mino::new(Piece::T, Rotate::Left), 1, 2));

        assert!(!field.is_on_ground(&Mino::new(Piece::T, Rotate::Spawn), 6, 5));
        assert!(!field.is_on_ground(&Mino::new(Piece::T, Rotate::Spawn), 8, 1));
        assert!(!field.is_on_ground(&Mino::new(Piece::T, Rotate::Right), 8, 2));
        assert!(!field.is_on_ground(&Mino::new(Piece::T, Rotate::Reverse), 7, 3));
        assert!(!field.is_on_ground(&Mino::new(Piece::T, Rotate::Left), 9, 2));
    }

    #[test]
    fn test_get_block_count_below_on_x() {
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "___XX_____"
                + "___XX_____"
                + "___XX_____"
                + "__X_X_____"
                + "X___X_____"
                + "__X_XX____",
        );

        assert_eq!(field.get_block_count_in_column(0, 1), 0);
        assert_eq!(field.get_block_count_in_column(0, 2), 1);
        assert_eq!(field.get_block_count_in_column(2, 4), 2);
        assert_eq!(field.get_block_count_in_column(3, 4), 1);
        assert_eq!(field.get_block_count_in_column(3, 6), 3);
        assert_eq!(field.get_block_count_in_column(4, 4), 4);
        assert_eq!(field.get_block_count_in_column(4, 6), 6);
    }

    #[test]
    fn test_get_all_block_count() {
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "___XX_____"
                + "___XX_____"
                + "___XX_____"
                + "__X_X_____"
                + "X___X_____"
                + "__X_XX____",
        );

        assert_eq!(field.get_num_of_all_blocks(), 13);
    }

    #[test]
    fn test_clear_line() {
        let mut field = field_factory::create_small_field_with_marks(
            String::new()
                + "XXX_XXXXXX"
                + "XXXXXXXXXX"
                + "X_XXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXX_XXXXX"
                + "XXXXXXXXXX",
        );

        assert_eq!(field.clear_filled_rows(), 3);

        assert!(field.exists_above_row(2));
        assert!(!field.exists_above_row(3));

        assert!(!field.is_empty_block(0, 0));
        assert!(field.is_empty_block(4, 0));
        assert!(field.is_empty_block(1, 1));
        assert!(field.is_empty_block(3, 2));
    }

    #[test]
    fn test_clear_line_2() {
        for remove_flags in boolean_walker::walk(BOARD_HEIGHT) {
            let delete_rows = remove_flags.iter().filter(|&&flag| flag).count() as u8;

            let mut field = field_factory::create_small_field();
            for (index, &flag) in remove_flags.iter().enumerate() {
                if flag {
                    for x in 0..FIELD_WIDTH {
                        field.set_block(x, index as u8);
                    }
                } else {
                    for x in 0..FIELD_WIDTH - 1 {
                        field.set_block(x, index as u8);
                    }
                }
            }

            assert_eq!(field.clear_filled_rows(), delete_rows as u32);

            if delete_rows < MAX_FIELD_HEIGHT {
                assert!(field.exists_above_row(MAX_FIELD_HEIGHT - delete_rows - 1));
            }
            assert!(!field.exists_above_row(MAX_FIELD_HEIGHT - delete_rows));
        }
    }

    #[test]
    fn test_clear_line_and_insert_black_line() {
        let mut field = field_factory::create_small_field_with_marks(
            String::new()
                + "XXX_XXXXXX"
                + "XXXXXXXXXX"
                + "X_XXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXX_XXXXX"
                + "XXXXXXXXXX",
        );
        let freeze = field.prune(field.get_max_field_height());

        let delete_key = field.clear_filled_rows_return_key();
        assert_eq!(delete_key.count_ones(), 3);
        field.insert_filled_row_with_key(delete_key);

        // Porting note: A testing error, the original uses getNumOfAllBlocks instead
        for index in 0..freeze.get_board_count() as u8 {
            assert_eq!(field.get_board(index), freeze.get_board(index))
        }
    }

    #[test]
    fn test_clear_line_and_insert_white_line() {
        let mut field = field_factory::create_small_field_with_marks(
            String::new()
                + "XXX_XXXXXX"
                + "XXXXXXXXXX"
                + "X_XXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXX_XXXXX"
                + "XXXXXXXXXX",
        );

        let expected = field_factory::create_small_field_with_marks(
            String::new()
                + "XXX_XXXXXX"
                + "__________"
                + "X_XXXXXXXX"
                + "__________"
                + "XXXX_XXXXX"
                + "__________",
        );

        let delete_key = field.clear_filled_rows_return_key();
        assert_eq!(delete_key.count_ones(), 3);
        field.insert_blank_row_with_key(delete_key);

        // Porting note: A testing error, the original uses getNumOfAllBlocks instead
        for index in 0..expected.get_board_count() as u8 {
            assert_eq!(field.get_board(index), expected.get_board(index))
        }
    }

    #[test]
    fn fill_line() {
        for y in 0..MAX_FIELD_HEIGHT {
            let mut field = field_factory::create_small_field();
            field.fill_row(y);

            for x in 0..FIELD_WIDTH {
                assert!(!field.is_empty_block(x, y));
            }

            field.clear_filled_rows();
            assert!(field.is_empty());
        }
    }

    #[test]
    fn test_get_board() {
        #[rustfmt::skip]
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "X_________"
                + "X_________"
                + "X_________"
                + "X_________",
        );

        assert_eq!(field.get_board_count(), BoardCount::Small);
        assert_eq!(field.get_board(0), field.get_x_board());
        assert_eq!(field.get_board(0), 0x40100401);

        for index in 1..100 {
            assert_eq!(field.get_board(index), 0);
        }
    }

    #[test]
    fn test_freeze() {
        #[rustfmt::skip]
        let mut field = field_factory::create_small_field_with_marks(
            String::new()
                + "X_________"
                + "X_________"
                + "X_________"
                + "X_________",
        );

        assert_eq!(field.get_num_of_all_blocks(), 4);
        let freeze = field.prune(field.get_max_field_height());
        field.set_block(9, 0);

        assert_eq!(field.get_num_of_all_blocks(), 5);
        assert_eq!(freeze.get_num_of_all_blocks(), 4);
    }

    #[test]
    fn test_equal() {
        let marks = "XXXXXX____";
        let field1 = field_factory::create_small_field_with_marks(marks.to_string());
        let field2 = field_factory::create_small_field_with_marks(marks.to_string());
        assert_eq!(field1, field2);

        let field3 =
            field_factory::create_small_field_with_marks(String::new() + marks + "XXXXXX____");
        assert_ne!(field1, field3);

        let field4 = field_factory::create_middle_field_with_marks(marks.to_string());
        assert_eq!(&field1 as &dyn Field, &field4 as &_);
    }

    #[test]
    fn test_get_block_count_on_y() {
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "__________"
                + "X___XXX__X"
                + "X__X___XX_"
                + "X____X____"
                + "X_________",
        );
        assert_eq!(field.get_block_count_in_row(0), 1);
        assert_eq!(field.get_block_count_in_row(1), 2);
        assert_eq!(field.get_block_count_in_row(2), 4);
        assert_eq!(field.get_block_count_in_row(3), 5);
        assert_eq!(field.get_block_count_in_row(4), 0);
    }

    #[test]
    fn test_can_merge_1() {
        #[rustfmt::skip]
        let field1 = field_factory::create_small_field_with_marks(
            String::new()
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "__________"
                + "__________"
        );
        #[rustfmt::skip]
        let field2 = field_factory::create_small_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "X__X_X_X__"
                + "XXX_XX___X"
        );

        assert!(field1.can_merge(&field2));
    }

    #[test]
    fn test_can_merge_2() {
        #[rustfmt::skip]
        let field1 = field_factory::create_small_field_with_marks(
            String::new()
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "XXXXX_____"
                + "XXXXX_____"
        );
        #[rustfmt::skip]
        let field2 = field_factory::create_small_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "X__X_X_X__"
                + "XXX_XX___X"
        );

        assert!(!field1.can_merge(&field2));
    }

    #[test]
    fn test_merge_1() {
        #[rustfmt::skip]
        let mut field1 = field_factory::create_small_field_with_marks(
            String::new()
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "__________"
                + "__________"
        );

        #[rustfmt::skip]
        let field2 = field_factory::create_small_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "X__X_X_X__"
                + "XXX_XX___X"
        );

        #[rustfmt::skip]
        let field_expected = field_factory::create_small_field_with_marks(
            String::new()
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "X__X_X_X__"
                + "XXX_XX___X"
        );

        field1.merge(&field2);
        assert_eq!(field1, field_expected);
        assert_ne!(field2, field_expected);
    }

    #[test]
    fn test_merge_2() {
        #[rustfmt::skip]
        let mut field1 = field_factory::create_small_field_with_marks(
            String::new()
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "XXXXX_____"
                + "XXXXX_____"
        );

        #[rustfmt::skip]
        let field2 = field_factory::create_small_field_with_marks(
            String::new()
                + "__________" 
                + "__________" 
                + "X__X_X_X__" 
                + "XXX_XX___X" 

        );

        #[rustfmt::skip]
        let field_expected = field_factory::create_small_field_with_marks(
            String::new()
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "XXXXXX_X__"
                + "XXXXXX___X"
        );

        field1.merge(&field2);
        assert_eq!(field1, field_expected);
        assert_ne!(field2, field_expected);
    }

    #[test]
    fn test_reduce() {
        #[rustfmt::skip]
        let mut field1 = field_factory::create_small_field_with_marks(
            String::new()
                + "XXXXXXXXX_"
                + "__________"
                + "__________"
                + "XXXXXXXXX_"
        );

        #[rustfmt::skip]
        let field2 = field_factory::create_small_field_with_marks(
            String::new()
                + "XXXXX_____"
                + "_X___X____"
                + "X__X_X_X__"
                + "XXX_XX___X"
        );

        #[rustfmt::skip]
        let field_expected = field_factory::create_small_field_with_marks(
            String::new()
                + "_____XXXX_"
                + "__________"
                + "__________"
                + "___X__XXX_"
        );

        field1.reduce(&field2);
        assert_eq!(field1, field_expected);
        assert_ne!(field2, field_expected);
    }

    #[test]
    fn test_get_upper_ywith4_blocks() {
        #[rustfmt::skip]
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "__________" 
                + "_____X____" 
                + "____XXX___" 
                + "__________" 
        );

        assert_eq!(field.get_upper_y_with_4_blocks(), 2);
    }

    #[test]
    fn test_get_upper_y_with_4_blocks_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_small_field();
            let mut max_y = None;
            while field.get_num_of_all_blocks() < 4 {
                let x = rngs.gen_range(0..FIELD_WIDTH);
                let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);
                field.set_block(x, y);

                max_y = max_y.max(Some(y));
            }

            assert_eq!(field.get_upper_y_with_4_blocks(), max_y.unwrap());
        }
    }

    #[test]
    fn test_get_lower_y() {
        #[rustfmt::skip]
        let field = field_factory::create_small_field_with_marks(
            String::new()
                + "__________"
                + "_____X____"
                + "____XXX___"
                + "__________"
        );
        assert_eq!(field.get_min_y(), Some(1));
    }

    #[test]
    fn test_get_lower_y_with_empty() {
        assert_eq!(field_factory::create_small_field().get_min_y(), None);
    }

    #[test]
    fn test_get_lower_y_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_small_field();
            let mut min_y: Option<u8> = None;

            for _ in 0..rngs.gen_range(1..FIELD_WIDTH * MAX_FIELD_HEIGHT) {
                let x = rngs.gen_range(0..FIELD_WIDTH);
                let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);

                field.set_block(x, y);
                // cannot just use std::cmp::min because Option implements None as smaller than Some
                if let Some(min) = min_y {
                    min_y = Some(min.min(y))
                } else {
                    min_y = Some(y)
                }
            }

            assert_eq!(field.get_min_y(), min_y);
        }
    }

    #[test]
    fn test_slide_left() {
        #[rustfmt::skip]
        let mut field = field_factory::create_small_field_with_marks(
            String::new()
                + "__________"
                + "_____X____"
                + "____XXX___"
                + "__________"
        );

        field.slide_left(3);

        #[rustfmt::skip]
        let field_expected = field_factory::create_small_field_with_marks(
            String::new()
                + "__________"
                + "__X_______"
                + "_XXX______"
                + "__________"
        );

        assert_eq!(field, field_expected);
    }

    #[test]
    fn test_slide_left_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let slide = rngs.gen_range(0..FIELD_WIDTH);

            let mut field = field_factory::create_small_field();
            let mut expect = field_factory::create_small_field();

            for _ in 0..rngs.gen_range(1..FIELD_WIDTH * MAX_FIELD_HEIGHT) {
                let x = rngs.gen_range(0..FIELD_WIDTH);
                let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);

                field.set_block(x, y);
                if let Some(new_x) = x.checked_sub(slide) {
                    expect.set_block(new_x, y);
                }
            }

            field.slide_left(slide);

            assert_eq!(field, expect);
        }
    }

    #[test]
    fn contains() {
        #[rustfmt::skip]
        let parent = field_factory::create_small_field_with_marks(
            String::new()
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
        );

        #[rustfmt::skip]
        let child1 = field_factory::create_small_field_with_marks(
            String::new()
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
        );
        #[rustfmt::skip]
        let child2 = field_factory::create_small_field_with_marks(
            String::new()
                + "XXX_______"
                + "XXX_______"
                + "XXX_______"
        );
        #[rustfmt::skip]
        let child3 = field_factory::create_small_field_with_marks(
            String::new()
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX__X__"
        );
        #[rustfmt::skip]
        let child4 = field_factory::create_small_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "__________"
                + "__________"
        );
        #[rustfmt::skip]
        let child5 = field_factory::create_small_field_with_marks(
            String::new()
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
        );

        assert!(parent.contains(&child1));
        assert!(parent.contains(&child2));
        assert!(!parent.contains(&child3));
        assert!(parent.contains(&child4));
        assert!(!parent.contains(&child5));
    }

    #[test]
    fn contains_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(3..10);
            let init_field = create_random_small_field(&mut rngs, empty_minos);

            {
                let mut field = init_field.prune(MAX_FIELD_HEIGHT);
                for _ in 0..100 {
                    field.remove_block(
                        rngs.gen_range(0..FIELD_WIDTH),
                        rngs.gen_range(0..MAX_FIELD_HEIGHT),
                    );

                    assert!(init_field.contains(field.as_ref()));
                }
            }

            {
                let mut field = init_field.prune(MAX_FIELD_HEIGHT);
                for _ in 0..100 {
                    let x = rngs.gen_range(0..FIELD_WIDTH);
                    let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);

                    if field.exists_block(x, y) {
                        continue;
                    }
                    field.set_block(x, y);

                    assert!(!init_field.contains(field.as_ref()));
                }
            }
        }
    }

    #[test]
    fn slide_down_one_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_small_field();
            let mut expected = field_factory::create_small_field();

            for x in 0..FIELD_WIDTH {
                if rngs.gen_bool(0.5) {
                    field.set_block(x, 0);
                }
            }

            for y in 1..MAX_FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    if rngs.gen_bool(0.5) {
                        field.set_block(x, y);
                        expected.set_block(x, y - 1);
                    }
                }
            }

            field.slide_down_one();
            assert_eq!(field, expected);
        }
    }

    #[test]
    fn slide_down_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = create_random_small_field(&mut rngs, 10);
            let slide = rngs.gen_range(0..=MAX_FIELD_HEIGHT);

            let mut freeze = field.clone();
            for _ in 0..slide {
                freeze.slide_down_one();
            }

            field.slide_down(slide);
            assert_eq!(field, freeze);
        }
    }

    #[test]
    fn slide_up_with_empty_row_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = create_random_small_field(&mut rngs, 10);

            let mut freeze = field.clone();
            freeze.slide_down_one();

            freeze.slide_up_with_empty_row(1);

            for x in 0..FIELD_WIDTH {
                field.remove_block(x, 0);
            }

            assert_eq!(field, freeze);
        }
    }

    #[test]
    fn slide_up_with_filled_row_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = create_random_small_field(&mut rngs, 10);

            let mut freeze = field.clone();
            freeze.slide_down_one();

            freeze.slide_up_with_filled_row(1);

            for x in 0..FIELD_WIDTH {
                field.set_block(x, 0);
            }

            assert_eq!(field, freeze);
        }
    }

    #[test]
    fn slide_up_with_empty_row_n_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = create_random_small_field(&mut rngs, 10);
            let slide = rngs.gen_range(0..=MAX_FIELD_HEIGHT);

            let mut freeze = field.clone();

            for _ in 0..slide {
                freeze.slide_up_with_empty_row(1);
            }

            field.slide_up_with_empty_row(slide);

            assert_eq!(field, freeze);
        }
    }

    #[test]
    fn slide_up_with_filled_row_n_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = create_random_small_field(&mut rngs, 10);
            let slide = rngs.gen_range(0..=MAX_FIELD_HEIGHT);

            let mut freeze = field.clone();

            for _ in 0..slide {
                freeze.slide_up_with_filled_row(1);
            }

            field.slide_up_with_filled_row(slide);

            assert_eq!(field, freeze);
        }
    }

    #[test]
    fn slide_left_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_small_field();
            let mut expected = field_factory::create_small_field();

            let slide = rngs.gen_range(0..FIELD_WIDTH);

            for x in 0..slide {
                for y in 0..MAX_FIELD_HEIGHT {
                    if rngs.gen_bool(0.5) {
                        field.set_block(x, y);
                    }
                }
            }

            for x in slide..FIELD_WIDTH {
                for y in 0..MAX_FIELD_HEIGHT {
                    if rngs.gen_bool(0.5) {
                        field.set_block(x, y);
                        expected.set_block(x - slide, y);
                    }
                }
            }

            field.slide_left(slide);

            assert_eq!(field, expected);
        }
    }

    #[test]
    fn slide_right_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_small_field();
            let mut expected = field_factory::create_small_field();

            let slide = rngs.gen_range(0..FIELD_WIDTH);

            for x in FIELD_WIDTH - slide..FIELD_WIDTH {
                for y in 0..MAX_FIELD_HEIGHT {
                    if rngs.gen_bool(0.5) {
                        field.set_block(x, y);
                    }
                }
            }

            for x in 0..FIELD_WIDTH - slide {
                for y in 0..MAX_FIELD_HEIGHT {
                    if rngs.gen_bool(0.5) {
                        field.set_block(x, y);
                        expected.set_block(x + slide, y);
                    }
                }
            }

            field.slide_right(slide);

            assert_eq!(field, expected);
        }
    }

    #[test]
    fn invert_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(3..10);
            let init_field = create_random_small_field(&mut rngs, empty_minos);

            let mut field = init_field.prune(MAX_FIELD_HEIGHT);
            field.invert();

            for y in 0..MAX_FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    assert_ne!(field.is_empty_block(x, y), init_field.is_empty_block(x, y));
                }
            }
        }
    }

    #[test]
    fn mirror_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(3..10);
            let init_field = create_random_small_field(&mut rngs, empty_minos);

            let mut field = init_field.prune(MAX_FIELD_HEIGHT);
            field.mirror();

            for y in 0..MAX_FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    assert_eq!(
                        field.is_empty_block(x, y),
                        init_field.is_empty_block(FIELD_WIDTH - 1 - x, y)
                    );
                }
            }
        }
    }

    #[test]
    fn get_min_x_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(3..10);
            let init_field = create_random_small_field(&mut rngs, empty_minos);

            let field = init_field.prune(MAX_FIELD_HEIGHT);
            let min_x = field.get_min_x();

            let mut expected_min_x = None;

            for x in 0..FIELD_WIDTH {
                if (0..MAX_FIELD_HEIGHT).any(|y| !field.is_empty_block(x, y)) {
                    expected_min_x = Some(x);
                    break;
                }
            }

            assert_eq!(min_x, expected_min_x);
        }
    }

    #[test]
    fn exists_block_in_row_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(3..10);
            let init_field = create_random_small_field(&mut rngs, empty_minos);

            for y in 0..MAX_FIELD_HEIGHT {
                assert_eq!(
                    init_field.exists_block_in_row(y),
                    (0..FIELD_WIDTH).any(|x| !init_field.is_empty_block(x, y))
                );
            }
        }
    }

    #[test]
    fn delete_filled_rows_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            // 適度にフィールドのラインが揃うようにランダムに地形を作る
            let empty_minos = rngs.gen_range(3..10);
            let mut field = create_random_small_field(&mut rngs, empty_minos);

            for _ in 0..rngs.gen_range(0..MAX_FIELD_HEIGHT * 2) {
                field.fill_row(rngs.gen_range(0..=MAX_FIELD_HEIGHT));
            }

            let mut expected = field.clone();
            let deleted_key = expected.clear_filled_rows_return_key();

            field.delete_rows_with_key(deleted_key);

            assert_eq!(field, expected);
        }
    }

    #[test]
    fn mask_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            // 適度にフィールドのラインが揃うようにランダムに地形を作る
            let empty_minos = rngs.gen_range(3..10);
            let field_1 = create_random_small_field(&mut rngs, empty_minos);
            let empty_minos = rngs.gen_range(3..10);
            let field_2 = create_random_small_field(&mut rngs, empty_minos);

            // 期待値
            let mut expected = field_factory::create_small_field();
            for y in 0..MAX_FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    if !field_1.is_empty_block(x, y) && !field_2.is_empty_block(x, y) {
                        expected.set_block(x, y);
                    }
                }
            }

            {
                let mut field = field_1.clone();
                field.mask(&field_2);
                assert_eq!(field, expected);
            }

            {
                let mut field = field_2.clone();
                field.mask(&field_1);
                assert_eq!(field, expected);
            }
        }
    }

    #[test]
    fn get_using_key_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(1..10);
            let field = create_random_small_field(&mut rngs, empty_minos);

            // 期待値
            let mut expected = 0;
            for y in 0..MAX_FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    if field.exists_block(x, y) {
                        expected |= key_operators::get_delete_bit_key(y);
                        break;
                    }
                }
            }

            assert_eq!(field.get_using_key(), expected);
        }
    }
}
