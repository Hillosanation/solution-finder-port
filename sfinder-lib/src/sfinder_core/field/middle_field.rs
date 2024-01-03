use super::{
    bit_operators,
    field::{Field, FieldHelper},
    field_constants::{BoardCount, BOARD_HEIGHT, FIELD_WIDTH, VALID_BOARD_RANGE},
    key_operators, long_board_map,
    small_field::SmallField,
};
use crate::sfinder_core::mino::mino::Mino;
use std::fmt::Debug;

const FIELD_ROW_BORDER_Y: u8 = BOARD_HEIGHT;
const MAX_FIELD_HEIGHT: u8 = BOARD_HEIGHT * 2;

enum Position {
    Low(u8),
    High(u8),
}

#[derive(Clone)]
pub struct MiddleField(u64, u64);

impl MiddleField {
    pub fn new() -> Self {
        Self(0, 0)
    }

    pub fn from_parts(low: u64, high: u64) -> Self {
        Self(low, high)
    }

    pub fn get_x_board_low(&self) -> u64 {
        self.0
    }

    pub fn get_x_board_high(&self) -> u64 {
        self.1
    }

    fn select(y: u8) -> Position {
        match y {
            FIELD_ROW_BORDER_Y.. => Position::High(y - FIELD_ROW_BORDER_Y),
            _ => Position::Low(y),
        }
    }

    fn combine_keys(low: u64, high: u64) -> u64 {
        low | high << 1
    }

    fn delete_row(&mut self, delete_key_low: u64, delete_key_high: u64) {
        let new_x_board_low = long_board_map::delete_row(self.0, delete_key_low);
        let new_x_board_high = long_board_map::delete_row(self.1, delete_key_high);

        let delete_row_low = delete_key_low.count_ones() as u8;

        self.0 = (new_x_board_low
            | bit_operators::board_shl(new_x_board_high, BOARD_HEIGHT - delete_row_low))
            & VALID_BOARD_RANGE;
        self.1 = bit_operators::board_shr(new_x_board_high, delete_row_low);
    }

    fn clear_all(&mut self) {
        self.0 = 0;
        self.1 = 0;
    }

    fn fill_all(&mut self) {
        self.0 = VALID_BOARD_RANGE;
        self.1 = VALID_BOARD_RANGE;
    }

    // row_fill_fn is used to factor out the two calls of this function that differ only by this argument.
    fn insert_row_with_key(&mut self, delete_key: u64, row_fill_fn: fn(u64, u64) -> u64) {
        let delete_keys = [
            <dyn Field>::extract_delete_key(delete_key, 0),
            <dyn Field>::extract_delete_key(delete_key, 1),
        ];

        let delete_rows = [
            delete_keys[0].count_ones() as u8,
            // delete_keys[1].count_ones() as u8,
        ];

        let new_x_boards = [
            <dyn Field>::create_bottom_board(self.0, delete_rows[0], delete_keys[0], row_fill_fn),
            <dyn Field>::create_upper_board(
                self.0,
                self.1,
                delete_rows[0],
                delete_keys[1],
                row_fill_fn,
            ),
        ];

        self.0 = new_x_boards[0];
        self.1 = new_x_boards[1] & VALID_BOARD_RANGE;
    }
}

impl Field for MiddleField {
    fn get_max_field_height(&self) -> u8 {
        MAX_FIELD_HEIGHT
    }

    fn get_board_count(&self) -> BoardCount {
        BoardCount::Middle
    }

    fn set_block(&mut self, x: u8, y: u8) {
        match Self::select(y) {
            Position::Low(y_off) => self.0 |= bit_operators::get_x_mask(x, y_off),
            Position::High(y_off) => self.1 |= bit_operators::get_x_mask(x, y_off),
        }
    }

    fn remove_block(&mut self, x: u8, y: u8) {
        match Self::select(y) {
            Position::Low(y_off) => self.0 &= !bit_operators::get_x_mask(x, y_off),
            Position::High(y_off) => self.1 &= !bit_operators::get_x_mask(x, y_off),
        }
    }

    fn put(&mut self, mino: &Mino, x: u8, y: u8) {
        // Lowの更新が必要
        if y as i8 + mino.get_min_y() < FIELD_ROW_BORDER_Y as i8 {
            self.0 |= mino.get_mask(x, y as i8);
        }

        // Highの更新が必要
        if y as i8 + mino.get_max_y() >= FIELD_ROW_BORDER_Y as i8 {
            self.1 |= mino.get_mask(x, y as i8 - FIELD_ROW_BORDER_Y as i8);
        }
    }

    fn can_put(&self, mino: &Mino, x: u8, y: u8) -> bool {
        match y {
            _ if y >= MAX_FIELD_HEIGHT + 2 => true,
            // Lowで完結
            _ if y as i8 + mino.get_max_y() < FIELD_ROW_BORDER_Y as i8 => {
                self.0 & mino.get_mask(x, y as i8) == 0
            }
            // Highで完結
            _ if y as i8 + mino.get_min_y() >= FIELD_ROW_BORDER_Y as i8 => {
                self.1 & mino.get_mask(x, y as i8 - FIELD_ROW_BORDER_Y as i8) == 0
            }
            _ => {
                // 分割
                self.0 & mino.get_mask(x, y as i8) == 0
                    && self.1 & mino.get_mask(x, y as i8 - FIELD_ROW_BORDER_Y as i8) == 0
            }
        }
    }

    fn remove(&mut self, mino: &Mino, x: u8, y: u8) {
        // Lowの更新が必要
        if y as i8 + mino.get_min_y() < FIELD_ROW_BORDER_Y as i8 {
            self.0 &= !mino.get_mask(x, y as i8);
        }

        // Highの更新が必要
        if y as i8 + mino.get_max_y() >= FIELD_ROW_BORDER_Y as i8 {
            self.1 &= !mino.get_mask(x, y as i8 - FIELD_ROW_BORDER_Y as i8);
        }
    }

    fn can_reach_on_harddrop(&self, mino: &Mino, x: u8, start_y: u8) -> bool {
        self._can_reach_on_harddrop(mino, x, start_y, MAX_FIELD_HEIGHT)
    }

    fn is_empty_block(&self, x: u8, y: u8) -> bool {
        match Self::select(y) {
            Position::Low(y_off) => self.0 & bit_operators::get_x_mask(x, y_off) == 0,
            Position::High(y_off) => self.1 & bit_operators::get_x_mask(x, y_off) == 0,
        }
    }

    fn exists_above_row(&self, y: u8) -> bool {
        if y >= MAX_FIELD_HEIGHT {
            return false;
        }

        match Self::select(y) {
            // すべて必要
            Position::Low(y_off) => {
                // Highのチェック
                self.1 != 0
                // Lowのチェック
                || self.0 & <dyn Field>::get_valid_mask(y_off) != 0
            }
            // Highで完結
            Position::High(y_off) => self.1 & <dyn Field>::get_valid_mask(y_off) != 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }

    fn is_filled_in_column(&self, x: u8, max_y: u8) -> bool {
        match max_y {
            0 => true,
            ..=FIELD_ROW_BORDER_Y => {
                // Lowで完結
                !self.0 & bit_operators::get_column_mask(max_y, x) == 0
            }
            _ => {
                // すべて必要
                // Lowのチェック
                !self.0 & bit_operators::get_column_mask(BOARD_HEIGHT, x) == 0
                // Highのチェック
                && !self.1 & bit_operators::get_column_mask(max_y - FIELD_ROW_BORDER_Y, x) == 0
            }
        }
    }

    fn is_wall_between_left(&self, x: u8, max_y: u8) -> bool {
        match max_y {
            0 => true,
            ..=FIELD_ROW_BORDER_Y => {
                // Lowで完結
                bit_operators::is_wall_between_left(x, max_y, self.0)
            }
            _ => {
                // すべて必要
                // Lowのチェック
                bit_operators::is_wall_between_left(x, BOARD_HEIGHT, self.0)
                // Highのチェック
                && bit_operators::is_wall_between_left(x, max_y - FIELD_ROW_BORDER_Y, self.1)
            }
        }
    }

    fn get_block_count_in_column(&self, x: u8, max_y: u8) -> u32 {
        match max_y {
            // Lowで完結
            ..=FIELD_ROW_BORDER_Y => {
                (self.0 & bit_operators::get_column_mask(max_y, x)).count_ones()
            }
            // すべて必要
            _ => {
                (self.0 & bit_operators::get_column_mask(BOARD_HEIGHT, x)).count_ones()
                    + (self.1 & bit_operators::get_column_mask(max_y - FIELD_ROW_BORDER_Y, x))
                        .count_ones()
            }
        }
    }

    fn get_block_count_in_row(&self, y: u8) -> u32 {
        match Self::select(y) {
            Position::Low(y_off) => self.0 & bit_operators::get_row_mask(y_off),
            Position::High(y_off) => self.1 & bit_operators::get_row_mask(y_off),
        }
        .count_ones()
    }

    fn exists_block_in_row(&self, y: u8) -> bool {
        (match Self::select(y) {
            Position::Low(y_off) => self.0 & bit_operators::get_row_mask(y_off),
            Position::High(y_off) => self.1 & bit_operators::get_row_mask(y_off),
        }) != 0
    }

    fn get_num_of_all_blocks(&self) -> u32 {
        self.0.count_ones() + self.1.count_ones()
    }

    fn clear_filled_rows_return_key(&mut self) -> u64 {
        let delete_key_low = key_operators::get_delete_key(self.0);
        let delete_key_high = key_operators::get_delete_key(self.1);

        self.delete_row(delete_key_low, delete_key_high);

        Self::combine_keys(delete_key_low, delete_key_high)
    }

    fn get_filled_rows_key(&self) -> u64 {
        Self::combine_keys(
            key_operators::get_delete_key(self.0),
            key_operators::get_delete_key(self.1),
        )
    }

    fn get_using_key(&self) -> u64 {
        Self::combine_keys(
            key_operators::get_using_key(self.0),
            key_operators::get_using_key(self.1),
        )
    }

    fn insert_filled_row_with_key(&mut self, delete_key: u64) {
        self.insert_row_with_key(delete_key, long_board_map::insert_filled_row);
    }

    fn insert_blank_row_with_key(&mut self, delete_key: u64) {
        self.insert_row_with_key(delete_key, long_board_map::insert_blank_row);
    }

    fn delete_rows_with_key(&mut self, delete_key: u64) {
        self.delete_row(
            <dyn Field>::extract_delete_key(delete_key, 0),
            <dyn Field>::extract_delete_key(delete_key, 1),
        );
    }

    fn fill_row(&mut self, y: u8) {
        match Self::select(y) {
            Position::Low(y_off) => self.0 |= bit_operators::get_row_mask(y_off),
            Position::High(y_off) => self.1 |= bit_operators::get_row_mask(y_off),
        }
    }

    fn get_board(&self, index: u8) -> u64 {
        match index {
            0 => self.0,
            1 => self.1,
            _ => 0,
        }
    }

    fn prune(&self, max_height: u8) -> Box<dyn Field> {
        assert!(max_height <= 12);
        match max_height {
            ..=FIELD_ROW_BORDER_Y => Box::new(SmallField::from(self.0)),
            _ => Box::new(self.clone()),
        }
    }

    fn merge(&mut self, other: &dyn Field) {
        debug_assert!(other.get_board_count() <= BoardCount::Large);

        self.0 |= other.get_board(0);

        if other.get_board_count() > BoardCount::Small {
            self.1 |= other.get_board(1);
        }
    }

    fn can_merge(&self, other: &dyn Field) -> bool {
        self.0 & other.get_board(0) == 0
            && match other.get_board_count() {
                BoardCount::Small => true,
                BoardCount::Middle => self.1 & other.get_board(1) == 0,
                BoardCount::Large => unreachable!(),
            }
    }

    fn reduce(&mut self, other: &dyn Field) {
        debug_assert!(other.get_board_count() <= BoardCount::Large);

        self.0 &= !other.get_board(0);

        if other.get_board_count() > BoardCount::Small {
            self.1 &= !other.get_board(1);
        }
    }

    fn get_upper_y_with_4_blocks(&self) -> u8 {
        assert_eq!(self.0.count_ones() + self.1.count_ones(), 4);

        if let Some(min_y) = bit_operators::try_get_highest_y(self.1) {
            min_y + FIELD_ROW_BORDER_Y
        } else {
            // すべてxBoardLowにある
            bit_operators::get_highest_y(self.0)
        }
    }

    fn get_min_x(&self) -> Option<u8> {
        bit_operators::try_get_lowest_x(self.0 | self.1)
    }

    fn get_min_y(&self) -> Option<u8> {
        if let Some(min_y) = bit_operators::try_get_lowest_y(self.0) {
            Some(min_y)
        } else if let Some(min_y) = bit_operators::try_get_lowest_y(self.1) {
            Some(min_y + FIELD_ROW_BORDER_Y)
        } else {
            None
        }
    }

    fn slide_left(&mut self, slide: u8) {
        let mask = bit_operators::get_column_mask_right_of_row(slide);

        self.0 = (self.0 & mask) >> slide;
        self.1 = (self.1 & mask) >> slide;
    }

    fn slide_right(&mut self, slide: u8) {
        let mask = bit_operators::get_column_mask_left_of_row(FIELD_WIDTH - slide);

        self.0 = (self.0 & mask) << slide;
        self.1 = (self.1 & mask) << slide;
    }

    fn slide_down_one(&mut self) {
        self.0 = (bit_operators::board_shr(self.0, 1)
            | bit_operators::board_shl(self.1, BOARD_HEIGHT - 1))
            & VALID_BOARD_RANGE;
        self.1 = bit_operators::board_shr(self.1, 1);
    }

    fn slide_down(&mut self, slide: u8) {
        match slide {
            ..=FIELD_ROW_BORDER_Y => {
                self.delete_row(key_operators::get_mask_for_key_below_y(slide), 0);
            }
            ..=MAX_FIELD_HEIGHT => {
                self.delete_row(
                    bit_operators::get_column_one_row_below_y(BOARD_HEIGHT),
                    key_operators::get_mask_for_key_below_y(slide - FIELD_ROW_BORDER_Y),
                );
            }
            _ => self.clear_all(),
        }
    }

    fn slide_up_with_empty_row(&mut self, slide: u8) {
        match slide {
            MAX_FIELD_HEIGHT.. => self.clear_all(),
            _ => self.insert_blank_row_with_key(key_operators::get_mask_for_key_below_y(slide)),
        }
    }

    fn slide_up_with_filled_row(&mut self, slide: u8) {
        match slide {
            MAX_FIELD_HEIGHT.. => self.fill_all(),
            _ => self.insert_filled_row_with_key(key_operators::get_mask_for_key_below_y(slide)),
        }
    }

    fn contains(&self, child: &dyn Field) -> bool {
        match child.get_board_count() {
            BoardCount::Small => {
                let child_board_low = child.get_board(0);
                self.0 & child_board_low == child_board_low
            }
            BoardCount::Middle => {
                let child_board_low = child.get_board(0);
                let child_board_high = child.get_board(1);

                self.0 & child_board_low == child_board_low
                    && self.1 & child_board_high == child_board_high
            }
            BoardCount::Large => {
                let child_board_low = child.get_board(0);
                let child_board_high = child.get_board(1);

                self.0 & child_board_low == child_board_low
                    && self.1 & child_board_high == child_board_high
                    && child.get_board(2) == 0
                    && child.get_board(3) == 0
            }
        }
    }

    fn invert(&mut self) {
        self.0 = !self.0 & VALID_BOARD_RANGE;
        self.1 = !self.1 & VALID_BOARD_RANGE;
    }

    fn mirror(&mut self) {
        self.0 = key_operators::mirror(self.0);
        self.1 = key_operators::mirror(self.1);
    }

    fn mask(&mut self, mask_field: &dyn Field) {
        self.0 &= mask_field.get_board(0);
        self.1 &= mask_field.get_board(1);
    }
}

impl Debug for MiddleField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MiddleField {:#062b} {:#062b}", self.0, self.1)
    }
}

impl PartialEq for MiddleField {
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
            field::{
                field_constants::FIELD_WIDTH,
                field_factory::{self, create_middle_field},
            },
            mino::{mino_factory::MinoFactory, piece::Piece},
            neighbor::original_piece::create_all_pieces,
            srs::rotate::Rotate,
        },
        sfinder_lib::randoms,
    };
    use rand::{rngs::ThreadRng, thread_rng, Rng};

    fn create_random_middle_field(rngs: &mut ThreadRng, empty_mino_count: u8) -> MiddleField {
        let field = randoms::gen_field(rngs, MAX_FIELD_HEIGHT, empty_mino_count);
        MiddleField::from_parts(field.get_board(0), field.get_board(1))
    }

    #[test]
    fn test_get_max_field_height() {
        assert_eq!(MiddleField::new().get_max_field_height(), MAX_FIELD_HEIGHT);
    }

    #[test]
    fn test_put_and_remove_block() {
        // Porting note: The original calls creates a SmallField instead
        let mut field = field_factory::create_middle_field();
        assert!(field.is_empty_block(0, 0));
        field.set_block(0, 0);
        assert!(!field.is_empty_block(0, 0));
        field.remove_block(0, 0);
        assert!(field.is_empty_block(0, 0));

        assert!(field.is_empty_block(9, 9));
        field.set_block(9, 9);
        assert!(!field.is_empty_block(9, 9));
        field.remove_block(9, 9);
        assert!(field.is_empty_block(9, 9));
    }

    #[test]
    fn test_put_and_remove_mino() {
        let mut field = field_factory::create_middle_field();

        field.put(&Mino::new(Piece::T, Rotate::Spawn), 1, 0);
        assert!(!field.is_empty_block(0, 0));
        assert!(!field.is_empty_block(1, 0));
        assert!(!field.is_empty_block(2, 0));
        assert!(!field.is_empty_block(1, 1));

        field.put(&Mino::new(Piece::I, Rotate::Left), 4, 6);
        assert!(!field.is_empty_block(4, 5));
        assert!(!field.is_empty_block(4, 6));
        assert!(!field.is_empty_block(4, 7));
        assert!(!field.is_empty_block(4, 8));

        field.put(&Mino::new(Piece::O, Rotate::Spawn), 8, 8);
        assert!(!field.is_empty_block(8, 8));
        assert!(!field.is_empty_block(8, 9));
        assert!(!field.is_empty_block(9, 8));
        assert!(!field.is_empty_block(9, 9));

        field.remove(&Mino::new(Piece::T, Rotate::Spawn), 1, 0);
        field.remove(&Mino::new(Piece::I, Rotate::Left), 4, 6);
        field.remove(&Mino::new(Piece::O, Rotate::Spawn), 8, 8);

        assert!(field.is_empty());
    }

    #[test]
    fn test_put_and_remove_piece() {
        let mut field = field_factory::create_middle_field();
        let max_field_height = field.get_max_field_height();

        for piece in create_all_pieces(&MinoFactory::new(), max_field_height) {
            // Initialize
            let mino = piece.get_mino();
            let x = piece.get_x();
            let y = piece.get_y();

            // Expect
            let mut expected = field_factory::create_middle_field();
            expected.put(&mino, x, y);

            // Test
            field.put_piece(&piece);

            assert_eq!(field, expected);

            field.remove_piece(&piece);

            assert!(field.is_empty());
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_get_y_on_harddrop() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "X_________"
                + "__________"
                + "__________"
                + "__________"
                + "_________X"
                + "____X_____",
        );

        assert_eq!(field.get_y_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 1, 10), 6);
        assert_eq!(field.get_y_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 2, 10), 0);
        assert_eq!(field.get_y_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 3, 10), 1);
        assert_eq!(field.get_y_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 8, 10), 2);
    }

    #[test]
    fn test_can_reach_on_harddrop() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "X_________"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "_________X"
                + "____X_____",
        );

        assert!(!field.can_reach_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 1, 4));
        assert!(field.can_reach_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 2, 4));
        assert!(field.can_reach_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 2, 3));
        assert!(!field.can_reach_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 1, 1));
    }

    #[test]
    fn test_can_reach_on_harddrop_2_random() {
        let mut rngs = thread_rng();
        let field = create_random_middle_field(&mut rngs, 25);

        for piece in create_all_pieces(&MinoFactory::new(), MAX_FIELD_HEIGHT) {
            let mino = piece.get_mino();
            let x = piece.get_x();
            let y = piece.get_y();

            assert_eq!(
                field.can_reach_on_harddrop_piece(&piece),
                field.can_put(&mino, x, y) && field.can_reach_on_harddrop(&mino, x, y)
            );
        }
    }

    #[test]
    fn test_exist_above_row() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "X_________"
                + "__________"
                + "__________"
                + "__________"
                + "___X______"
                + "__________"
                + "_______X__"
                + "__________",
        );

        assert!(field.exists_above_row(0));
        assert!(field.exists_above_row(6));
        assert!(field.exists_above_row(7));
        assert!(!field.exists_above_row(8));
        assert!(!field.exists_above_row(9));
    }

    #[test]
    fn test_is_perfect() {
        let mut field = field_factory::create_middle_field();

        assert!(field.is_empty_block(0, 0));
        assert!(field.is_empty());

        field.set_block(7, 8);

        assert!(!field.is_empty_block(7, 8));
        assert!(!field.is_empty());
    }
    #[test]
    fn test_is_filled_in_column() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "____X_____"
                + "____X_____"
                + "____X_____"
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
        assert!(field.is_filled_in_column(4, 9));
        assert!(!field.is_filled_in_column(4, 10));
        assert!(!field.is_filled_in_column(5, 7));
    }

    #[test]
    fn test_is_wall_between_left() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "____X_____"
                + "____X_____"
                + "____X_____"
                + "____X_____"
                + "____X_____"
                + "_X_XX_____"
                + "_XXXX_____"
                + "X_XXX_____"
                + "_XXXXX____",
        );

        assert!(field.is_wall_between_left(1, 4));
        assert!(!field.is_wall_between_left(1, 5));
        assert!(field.is_wall_between_left(2, 4));
        assert!(!field.is_wall_between_left(2, 5));
        assert!(field.is_wall_between_left(3, 4));
        assert!(!field.is_wall_between_left(3, 5));
        assert!(field.is_wall_between_left(4, 9));
        assert!(!field.is_wall_between_left(4, 10));
        assert!(field.is_wall_between_left(5, 9));
        assert!(!field.is_wall_between_left(5, 10));
        assert!(!field.is_wall_between_left(6, 6));
    }

    #[test]
    fn test_can_put_mino() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "______X___"
                + "___X______"
                + "___XX_____"
                + "___XX_____"
                + "___XX_____"
                + "__X_X_____"
                + "X___X_____"
                + "__X_XX____",
        );

        assert!(field.can_put(&Mino::new(Piece::T, Rotate::Spawn), 4, 7));
        assert!(field.can_put(&Mino::new(Piece::T, Rotate::Spawn), 5, 6));
        assert!(field.can_put(&Mino::new(Piece::T, Rotate::Right), 1, 1));
        assert!(field.can_put(&Mino::new(Piece::T, Rotate::Reverse), 1, 3));
        assert!(field.can_put(&Mino::new(Piece::T, Rotate::Left), 3, 1));

        assert!(!field.can_put(&Mino::new(Piece::T, Rotate::Spawn), 5, 7));
        assert!(!field.can_put(&Mino::new(Piece::T, Rotate::Spawn), 4, 6));
        assert!(!field.can_put(&Mino::new(Piece::T, Rotate::Right), 0, 1));
        assert!(!field.can_put(&Mino::new(Piece::T, Rotate::Reverse), 1, 1));
        assert!(!field.can_put(&Mino::new(Piece::T, Rotate::Left), 1, 1));
    }

    #[test]
    fn test_can_put_mino_2() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X"
                + "XXXXXXXX_X",
        );

        assert!(field.can_put(&Mino::new(Piece::I, Rotate::Left), 8, 1));
        assert!(field.can_put(&Mino::new(Piece::I, Rotate::Left), 8, 11));
        assert!(field.can_put(&Mino::new(Piece::I, Rotate::Left), 8, 12));
        assert!(field.can_put(&Mino::new(Piece::I, Rotate::Left), 8, 13));
        assert!(field.can_put(&Mino::new(Piece::I, Rotate::Left), 8, 14));
    }

    #[test]
    fn test_can_put_piece() {
        let mut rngs = thread_rng();
        let field = create_random_middle_field(&mut rngs, 25);
        let max_field_height = field.get_max_field_height();

        for piece in create_all_pieces(&MinoFactory::new(), max_field_height) {
            let mino = piece.get_mino();
            let x = piece.get_x();
            let y = piece.get_y();

            assert_eq!(field.can_put_piece(&piece), field.can_put(&mino, x, y));
        }
    }

    #[test]
    fn test_is_on_ground() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "X_________"
                + "___X______"
                + "___XX_____"
                + "___XX_____"
                + "__X_X_____"
                + "X___X_____"
                + "__X_XX____",
        );

        assert!(field.is_on_ground(&Mino::new(Piece::T, Rotate::Spawn), 1, 7));
        assert!(field.is_on_ground(&Mino::new(Piece::T, Rotate::Spawn), 5, 5));
        assert!(field.is_on_ground(&Mino::new(Piece::T, Rotate::Right), 8, 1));
        assert!(field.is_on_ground(&Mino::new(Piece::T, Rotate::Reverse), 1, 3));
        assert!(field.is_on_ground(&Mino::new(Piece::T, Rotate::Left), 1, 2));

        assert!(!field.is_on_ground(&Mino::new(Piece::T, Rotate::Spawn), 1, 6));
        assert!(!field.is_on_ground(&Mino::new(Piece::T, Rotate::Spawn), 6, 5));
        assert!(!field.is_on_ground(&Mino::new(Piece::T, Rotate::Spawn), 8, 1));
        assert!(!field.is_on_ground(&Mino::new(Piece::T, Rotate::Right), 8, 2));
        assert!(!field.is_on_ground(&Mino::new(Piece::T, Rotate::Reverse), 7, 3));
        assert!(!field.is_on_ground(&Mino::new(Piece::T, Rotate::Left), 9, 2));
    }

    #[test]
    fn test_get_block_count_in_column() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "___XX_____"
                + "___XX_____"
                + "___XX_____"
                + "____X_____"
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
        assert_eq!(field.get_block_count_in_column(3, 9), 5);
        assert_eq!(field.get_block_count_in_column(3, 10), 6);
        assert_eq!(field.get_block_count_in_column(4, 6), 6);
        assert_eq!(field.get_block_count_in_column(4, 9), 9);
        assert_eq!(field.get_block_count_in_column(4, 10), 10);
    }

    #[test]
    fn test_get_all_block_count() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "___XX_____"
                + "___XX_____"
                + "___XX_____"
                + "___XX_____"
                + "___XX_____"
                + "__X_X_____"
                + "X___X_____"
                + "__X_XX____",
        );

        assert_eq!(field.get_num_of_all_blocks(), 17);
    }

    #[test]
    fn test_clear_filled_rows_1() {
        let mut field = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXXXXX_X"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "XXX_XXXXXX"
                + "XXXXXXXXXX"
                + "X_XXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXX_XXXXX"
                + "XXXXXXXXXX",
        );

        let delete_rows = field.clear_filled_rows();

        assert_eq!(delete_rows, 6);

        assert!(field.exists_above_row(3));
        assert!(!field.exists_above_row(4));

        assert!(!field.is_empty_block(0, 0));
        assert!(field.is_empty_block(4, 0));
        assert!(field.is_empty_block(1, 1));
        assert!(field.is_empty_block(3, 2));
        assert!(field.is_empty_block(8, 3));
    }

    #[test]
    fn test_clear_filled_rows_2() {
        let mut field = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXXXXXXX"
                + "XXXXXXXX_X"
                + "XXXXXXX_XX"
                + "XXXXXX_XXX"
                + "XXXXXXXXXX"
                + "XXXXX_XXXX"
                + "XXXX_XXXXX"
                + "XXX_XXXXXX"
                + "XX_XXXXXXX"
                + "XXXXXXXXXX"
                + "X_XXXXXXXX"
                + "_XXXXXXXXX",
        );

        let delete_rows = field.clear_filled_rows();

        assert_eq!(delete_rows, 3);

        assert!(field.exists_above_row(8));
        assert!(!field.exists_above_row(9));

        for index in 0..9 {
            assert!(field.is_empty_block(index, index));
        }
    }

    #[test]
    fn test_clear_filled_rows_3() {
        let mut field = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXXXXXXX"
                + "XXXXX_XXXX"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXX_XXXXX"
                + "XXX_XXXXXX"
                + "XX_XXXXXXX"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "X_XXXXXXXX"
                + "_XXXXXXXXX",
        );

        let delete_rows = field.clear_filled_rows();

        assert_eq!(delete_rows, 6);

        assert!(field.exists_above_row(5));
        assert!(!field.exists_above_row(6));

        for index in 0..6 {
            assert!(field.is_empty_block(index, index));
        }
    }

    #[test]
    fn test_clear_filled_rows_and_insert_filled_rows() {
        let mut field = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXXXXX_X"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "XXX_XXXXXX"
                + "XXXXXXXXXX"
                + "X_XXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXX_XXXXX"
                + "XXXXXXXXXX",
        );

        let freeze = field.prune(field.get_max_field_height());

        let delete_key = field.clear_filled_rows_return_key();
        assert_eq!(delete_key.count_ones(), 6);
        field.insert_filled_row_with_key(delete_key);

        for index in 0..freeze.get_board_count() as u8 {
            assert_eq!(field.get_board(index), freeze.get_board(index));
        }
    }

    #[test]
    fn test_clear_filled_rows_and_insert_blank_rows() {
        let mut field = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXXXXX_X"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXXXXXXXX"
                + "XXX_XXXXXX"
                + "XXXXXXXXXX"
                + "X_XXXXXXXX"
                + "XXXXXXXXXX"
                + "XXXX_XXXXX"
                + "XXXXXXXXXX",
        );

        let expected = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXXXXX_X"
                + "__________"
                + "__________"
                + "__________"
                + "XXX_XXXXXX"
                + "__________"
                + "X_XXXXXXXX"
                + "__________"
                + "XXXX_XXXXX"
                + "__________",
        );

        let delete_key = field.clear_filled_rows_return_key();
        assert_eq!(delete_key.count_ones(), 6);
        field.insert_blank_row_with_key(delete_key);

        for index in 0..field.get_board_count() as u8 {
            assert_eq!(field.get_board(index), expected.get_board(index));
        }
    }

    #[test]
    fn test_get_board() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "_________X"
                + "_________X"
                + "_________X"
                + "X_________"
                + "X_________"
                + "X_________"
                + "X_________"
                + "X_________"
                + "X_________",
        );

        assert_eq!(field.get_board_count(), BoardCount::Middle);
        assert_eq!(field.get_board(0), 0x4010040100401);
        assert_eq!(field.get_board(0), field.get_x_board_low());
        assert_eq!(field.get_board(1), 0x20080200);
        assert_eq!(field.get_board(1), field.get_x_board_high());

        for index in 2..100 {
            assert_eq!(field.get_board(index), 0);
        }
    }

    #[test]
    fn test_freeze() {
        #[rustfmt::skip]
        let mut field = field_factory::create_middle_field_with_marks(
            String::new()
                + "X_________"
                + "X_________"
                + "X_________"
                + "X_________"
                + "",
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
        let field1 = field_factory::create_middle_field_with_marks(marks.to_string());
        let field2 = field_factory::create_middle_field_with_marks(marks.to_string());
        assert_eq!(field1, field2);

        let field3 =
            field_factory::create_middle_field_with_marks(marks.to_string() + "XXXXXX____");
        assert_ne!(field1, field3);

        let field4 = field_factory::create_small_field_with_marks(marks.to_string());
        assert_eq!(&field1 as &dyn Field, &field4 as &dyn Field);
    }

    #[test]
    fn test_get_block_count_in_row() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "X__X__X___"
                + "XXXXXXXXXX"
                + "XXX_XXX__X"
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
        assert_eq!(field.get_block_count_in_row(5), 7);
        assert_eq!(field.get_block_count_in_row(6), 10);
        assert_eq!(field.get_block_count_in_row(7), 3);
    }

    #[test]
    fn test_can_merge_1() {
        let field1 = field_factory::create_middle_field_with_marks(
            String::new()
                + "X_X_X_X__X"
                + "X__X____XX"
                + "__________"
                + "__________"
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "__________"
                + "__________",
        );

        let field2 = field_factory::create_middle_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "X_XX_X_X_X"
                + "XXXXXXXXXX"
                + "__________"
                + "__________"
                + "X__X_X_X__"
                + "XXX_XX___X",
        );

        assert!(field1.can_merge(&field2));
    }

    #[test]
    fn test_can_merge_2() {
        let field1 = field_factory::create_middle_field_with_marks(
            String::new()
                + "__XX_X_X__"
                + "__________"
                + "__________"
                + "__XX_X_X__"
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "XXXXX_____"
                + "XXXXX_____",
        );

        let field2 = field_factory::create_middle_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "X__X_X_X__"
                + "XXXXXXXXXX"
                + "__________"
                + "__________"
                + "__________"
                + "__________",
        );

        assert!(!field1.can_merge(&field2));
    }

    #[test]
    fn test_merge_1() {
        let mut field1 = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "__________"
                + "__________"
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "__________"
                + "__________",
        );

        let field2 = field_factory::create_middle_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "X__X_X_X__"
                + "XXX_XX___X"
                + "__________"
                + "__________"
                + "X__X_X_X__"
                + "XXX_XX___X",
        );

        let expected = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "X__X_X_X__"
                + "XXX_XX___X"
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "X__X_X_X__"
                + "XXX_XX___X",
        );

        field1.merge(&field2);
        assert_eq!(field1, expected);
        assert_ne!(field2, expected);
    }

    #[test]
    fn test_merge_2() {
        let mut field1 = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "XXXXX_____"
                + "XXXXX_____",
        );

        let field2 = field_factory::create_middle_field_with_marks(
            String::new()
                + "__________"
                + "__________"
                + "X__X_X_X__"
                + "XXX_XX___X"
                + "__________"
                + "__________"
                + "X__X_X_X__"
                + "XXX_XX___X",
        );

        let expected = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "XXXXXX_X__"
                + "XXXXXX___X"
                + "XXX_XXX__X"
                + "X__X___XX_"
                + "XXXXXX_X__"
                + "XXXXXX___X",
        );

        field1.merge(&field2);
        assert_eq!(field1, expected);
        assert_ne!(field2, expected);
    }

    #[test]
    fn test_reduce() {
        let mut field1 = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXXXXXX_"
                + "__________"
                + "__________"
                + "XXXXXXXXX_"
                + "XXXXXXXXX_"
                + "__________"
                + "__________"
                + "XXXXXXXXX_",
        );

        let field2 = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXX_____"
                + "_X___X____"
                + "X__X_X_X__"
                + "XXX_XX___X"
                + "XXXXX_____"
                + "_X___X____"
                + "X__X_X_X__"
                + "XXX_XX___X",
        );

        let expected = field_factory::create_middle_field_with_marks(
            String::new()
                + "_____XXXX_"
                + "__________"
                + "__________"
                + "___X__XXX_"
                + "_____XXXX_"
                + "__________"
                + "__________"
                + "___X__XXX_",
        );

        field1.reduce(&field2);
        assert_eq!(field1, expected);
        assert_ne!(field2, expected);
    }

    #[test]
    fn test_get_upper_y_with_4_blocks() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "__________"
                + "_____X____"
                + "____XXX___"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________",
        );

        assert_eq!(field.get_upper_y_with_4_blocks(), 7);
    }

    #[test]
    fn test_get_upper_y_with_4_blocks_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_middle_field();
            let mut max_y = None;
            while field.get_num_of_all_blocks() != 4 {
                let x = rngs.gen_range(0..FIELD_WIDTH);
                let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);
                field.set_block(x, y);

                max_y = max_y.max(Some(y));
            }

            assert_eq!(
                field.get_upper_y_with_4_blocks(),
                max_y.unwrap(),
                "{field:?}"
            );
        }
    }

    #[test]
    fn test_get_min_y() {
        let field = field_factory::create_middle_field_with_marks(
            String::new()
                + "__________"
                + "_____X____"
                + "____XXX___"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________",
        );

        assert_eq!(field.get_min_y(), Some(8));
    }

    #[test]
    fn test_get_min_y_with_empty() {
        let field = field_factory::create_middle_field();
        assert_eq!(field.get_min_y(), None);
    }

    #[test]
    fn test_get_min_y_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_middle_field();
            let mut min_y: Option<u8> = None;
            while field.get_num_of_all_blocks() != 0 {
                let x = rngs.gen_range(0..FIELD_WIDTH);
                let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);
                field.set_block(x, y);

                if let Some(min_y_value) = min_y {
                    min_y = Some(min_y_value.min(y));
                } else {
                    min_y = Some(y);
                }
            }

            assert_eq!(field.get_min_y(), min_y);
        }
    }

    #[test]
    fn test_slide_left() {
        let mut field = field_factory::create_middle_field_with_marks(
            String::new()
                + "__________"
                + "_____X____"
                + "____XXX___"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________",
        );

        let expected = field_factory::create_middle_field_with_marks(
            String::new()
                + "__________"
                + "__X_______"
                + "_XXX______"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________",
        );

        field.slide_left(3);
        assert_eq!(field, expected);
    }

    #[test]
    fn test_slide_left_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let slide = rngs.gen_range(0..10);

            let mut field = field_factory::create_middle_field();
            let mut expected = field_factory::create_middle_field();

            let num_of_blocks = rngs.gen_range(1..FIELD_WIDTH * MAX_FIELD_HEIGHT);
            for _ in 0..num_of_blocks {
                let x = rngs.gen_range(0..FIELD_WIDTH);
                let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);

                field.set_block(x, y);
                if let Some(new_x) = x.checked_sub(slide) {
                    expected.set_block(new_x, y);
                }
            }

            field.slide_left(slide);

            assert_eq!(field, expected);
        }
    }

    #[test]
    fn test_slide_left_random_2() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_middle_field();
            let mut expected = field_factory::create_middle_field();

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
    fn fill_row() {
        for y in 0..MAX_FIELD_HEIGHT {
            let mut field = field_factory::create_middle_field();
            field.fill_row(y);

            for x in 0..FIELD_WIDTH {
                assert!(!field.is_empty_block(x, y));
            }

            field.clear_filled_rows();
            assert!(field.is_empty());
        }
    }

    #[test]
    fn contains() {
        let parent = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____",
        );

        let child1 = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____",
        );
        let child2 = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXX_______"
                + "XXX_______"
                + "XXX_______"
                + "XXX_______"
                + "XXX_______"
                + "XXX_______",
        );
        let child3 = field_factory::create_middle_field_with_marks(
            String::new()
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX_____"
                + "XXXXX__X__",
        );
        let child4 = field_factory::create_middle_field_with_marks(
            String::new() + "__________" + "__________" + "__________" + "__________",
        );
        let child5 = field_factory::create_middle_field_with_marks(
            String::new() + "XXXXXXXXXX" + "XXXXXXXXXX" + "XXXXXXXXXX" + "XXXXXXXXXX",
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
            let empty_minos = rngs.gen_range(4..15);
            let init_field = create_random_middle_field(&mut rngs, empty_minos);

            {
                let mut field = init_field.prune(init_field.get_max_field_height());
                for _ in 0..100 {
                    let x = rngs.gen_range(0..FIELD_WIDTH);
                    let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);
                    field.remove_block(x, y);

                    assert!(init_field.contains(field.as_ref()));
                }
            }

            {
                let mut field = init_field.prune(init_field.get_max_field_height());
                for _ in 0..100 {
                    let x = rngs.gen_range(0..FIELD_WIDTH);
                    let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);

                    if field.exists_block(x, y) {
                        continue;
                    }

                    field.set_block(x, y);

                    assert!(
                        !init_field.contains(field.as_ref()),
                        "{init_field:?}\n{field:?} {x} {y}"
                    );
                }
            }
        }
    }

    #[test]
    fn slide_down_one_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_middle_field();
            let mut expected = field_factory::create_middle_field();

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
    fn slide_down_n_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = create_random_middle_field(&mut rngs, 20);
            let slide = rngs.gen_range(0..MAX_FIELD_HEIGHT + 1);

            let mut freeze = field.prune(field.get_max_field_height());
            for _ in 0..slide {
                freeze.slide_down_one();
            }

            field.slide_down(slide);

            assert_eq!(&field as &dyn Field, freeze.as_ref());
        }
    }

    #[test]
    fn slide_up_with_empty_row_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = create_random_middle_field(&mut rngs, 20);

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
            let mut field = create_random_middle_field(&mut rngs, 20);

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
            let mut field = create_random_middle_field(&mut rngs, 20);
            let slide = rngs.gen_range(0..MAX_FIELD_HEIGHT + 1);

            let mut freeze = field.clone();
            for _ in 0..slide {
                freeze.slide_up_with_empty_row(1);
            }

            field.slide_up_with_empty_row(slide);
            assert_eq!(field, freeze);
        }
    }

    #[test]
    fn slide_up_with_filled_line_n_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = create_random_middle_field(&mut rngs, 20);
            let slide = rngs.gen_range(0..MAX_FIELD_HEIGHT + 1);

            let mut freeze = field.clone();
            for _ in 0..slide {
                freeze.slide_up_with_filled_row(1);
            }

            field.slide_up_with_filled_row(slide);
            assert_eq!(field, freeze);
        }
    }

    #[test]
    fn test_slide_right_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = create_middle_field();
            let mut expected = create_middle_field();

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
            let empty_minos = rngs.gen_range(4..15);
            let init_field = create_random_middle_field(&mut rngs, empty_minos);
            let mut field = init_field.prune(MAX_FIELD_HEIGHT);

            field.invert();

            for x in 0..FIELD_WIDTH {
                for y in 0..MAX_FIELD_HEIGHT {
                    assert_ne!(
                        init_field.is_empty_block(x, y),
                        field.is_empty_block(x, y),
                        "{init_field:?}\n{field:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn mirror_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(3..10);
            let init_field = create_random_middle_field(&mut rngs, empty_minos);
            let mut field = init_field.prune(MAX_FIELD_HEIGHT);

            field.mirror();

            for x in 0..FIELD_WIDTH {
                for y in 0..MAX_FIELD_HEIGHT {
                    assert_eq!(
                        init_field.is_empty_block(x, y),
                        field.is_empty_block(9 - x, y)
                    );
                }
            }
        }
    }

    #[test]
    fn get_min_x_random() {
        assert_eq!(field_factory::create_middle_field().get_min_x(), None);

        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(3..10);
            let init_field = create_random_middle_field(&mut rngs, empty_minos);
            let field = init_field.prune(MAX_FIELD_HEIGHT);

            let mut expected_min_x: Option<u8> = None;
            for x in 0..FIELD_WIDTH {
                for y in 0..MAX_FIELD_HEIGHT {
                    if field.exists_block(x, y) {
                        if let Some(min_x) = expected_min_x {
                            expected_min_x = Some(min_x.min(x));
                        } else {
                            expected_min_x = Some(x);
                        }
                        break;
                    }
                }
            }

            assert_eq!(field.get_min_x(), expected_min_x);
        }
    }

    #[test]
    fn exists_block_in_row_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(3..10);
            let init_field = create_random_middle_field(&mut rngs, empty_minos);

            for y in 0..MAX_FIELD_HEIGHT {
                assert_eq!(
                    init_field.exists_block_in_row(y),
                    (0..FIELD_WIDTH).any(|x| init_field.exists_block(x, y))
                );
            }
        }
    }

    #[test]
    fn delete_filled_row_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            // 適度にフィールドのラインが揃うようにランダムに地形を作る
            let empty_minos = rngs.gen_range(3..10);
            let mut field = create_random_middle_field(&mut rngs, empty_minos);

            let max_count = rngs.gen_range(0..MAX_FIELD_HEIGHT * 2);
            for _ in 0..max_count {
                field.fill_row(rngs.gen_range(0..MAX_FIELD_HEIGHT));
            }

            let mut expected = field.clone();
            let delete_key = expected.clear_filled_rows_return_key();

            field.delete_rows_with_key(delete_key);

            assert_eq!(field, expected);
        }
    }

    #[test]
    fn mask_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            // 適度にフィールドのラインが揃うようにランダムに地形を作る
            let empty_minos1 = rngs.gen_range(3..10);
            let field1 = create_random_middle_field(&mut rngs, empty_minos1);
            let empty_minos2 = rngs.gen_range(3..10);
            let field2 = create_random_middle_field(&mut rngs, empty_minos2);

            // 期待値
            let mut expected = field_factory::create_middle_field();
            for x in 0..FIELD_WIDTH {
                for y in 0..MAX_FIELD_HEIGHT {
                    if !field1.is_empty_block(x, y) && !field2.is_empty_block(x, y) {
                        expected.set_block(x, y);
                    }
                }
            }

            {
                let mut freeze = field1.prune(field1.get_max_field_height());
                freeze.mask(&field2);
                assert_eq!(freeze.as_ref(), &expected as &_);
            }

            {
                let mut freeze = field2.prune(field2.get_max_field_height());
                freeze.mask(&field1);
                assert_eq!(freeze.as_ref(), &expected as &_);
            }
        }
    }

    #[test]
    fn get_using_key_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(1..10);
            let field = create_random_middle_field(&mut rngs, empty_minos);

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
