use crate::sfinder_core::mino::mino::Mino;

use super::{
    bit_operators,
    field::{BoardCount, Field, FieldHelper, FIELD_WIDTH, VALID_BOARD_RANGE},
    key_operators, long_board_map,
    middle_field::MiddleField,
    small_field::SmallField,
};

const BOARD_HEIGHT: u8 = 6;

const FIELD_ROW_MID_LOW_BORDER_Y: u8 = BOARD_HEIGHT;
const FIELD_ROW_MID_HIGH_BORDER_Y: u8 = BOARD_HEIGHT * 2;
const FIELD_ROW_HIGH_BORDER_Y: u8 = BOARD_HEIGHT * 3;
const MAX_FIELD_HEIGHT: u8 = BOARD_HEIGHT * 4;

/// attached u8 is the adjusted y position such that it is less than 6
enum Position {
    Low(u8),
    MidLow(u8),
    MidHigh(u8),
    High(u8),
}

#[derive(Debug, Clone)]
pub struct LargeField(u64, u64, u64, u64);

impl LargeField {
    pub fn new() -> Self {
        Self(0, 0, 0, 0)
    }

    pub fn from_parts(low: u64, mid_low: u64, mid_high: u64, high: u64) -> Self {
        Self(low, mid_low, mid_high, high)
    }

    pub fn get_x_board_low(&self) -> u64 {
        self.0
    }

    pub fn get_x_board_mid_low(&self) -> u64 {
        self.1
    }

    pub fn get_x_board_mid_high(&self) -> u64 {
        self.2
    }

    pub fn get_x_board_high(&self) -> u64 {
        self.3
    }

    // TODO: do this for MiddleField as well
    fn select(y: u8) -> Position {
        match y {
            FIELD_ROW_HIGH_BORDER_Y.. => Position::High(y - FIELD_ROW_HIGH_BORDER_Y),
            FIELD_ROW_MID_HIGH_BORDER_Y.. => Position::MidHigh(y - FIELD_ROW_MID_HIGH_BORDER_Y),
            FIELD_ROW_MID_LOW_BORDER_Y.. => Position::MidLow(y - FIELD_ROW_MID_LOW_BORDER_Y),
            _ => Position::Low(y),
        }
    }

    fn combine_keys(low: u64, mid_low: u64, mid_high: u64, high: u64) -> u64 {
        low | mid_low << 1 | mid_high << 2 | high << 3
    }

    fn delete_row(&mut self, key_low: u64, key_mid_low: u64, key_mid_high: u64, key_high: u64) {
        // TODO: refactor, make non-dependant stuff be run first
        let new_x_boards = [
            long_board_map::delete_row(self.0, key_low),
            long_board_map::delete_row(self.1, key_mid_low),
            long_board_map::delete_row(self.2, key_mid_high),
            long_board_map::delete_row(self.3, key_high),
        ];

        let delete_rows = [
            key_low.count_ones() as u8,
            key_mid_low.count_ones() as u8,
            key_mid_high.count_ones() as u8,
            // key_high.count_ones() as u8, // not needed
        ];

        let boards = [
            // 下半分
            (new_x_boards[0]
                | <dyn Field>::board_shl(new_x_boards[1], BOARD_HEIGHT - delete_rows[0]))
                & VALID_BOARD_RANGE,
            <dyn Field>::board_shr(new_x_boards[1], delete_rows[0]),
            // 上半分
            (new_x_boards[2]
                | <dyn Field>::board_shl(new_x_boards[3], BOARD_HEIGHT - delete_rows[2]))
                & VALID_BOARD_RANGE,
            <dyn Field>::board_shr(new_x_boards[3], delete_rows[2]),
        ];

        let delete_row_bottom = delete_rows[0] + delete_rows[1];
        // 上半分と下半分をマージ
        if delete_row_bottom >= BOARD_HEIGHT {
            let slide = delete_row_bottom - BOARD_HEIGHT;
            self.0 = (boards[0] | (<dyn Field>::board_shl(boards[2], BOARD_HEIGHT - slide)))
                & VALID_BOARD_RANGE;
            self.1 = <dyn Field>::board_shr(boards[2], slide)
                | <dyn Field>::board_shl(boards[3], BOARD_HEIGHT - slide) & VALID_BOARD_RANGE;
            self.2 = <dyn Field>::board_shr(boards[3], slide);
            self.3 = 0;
        } else {
            self.0 = boards[0];
            self.1 = (boards[1]
                | <dyn Field>::board_shl(boards[2], BOARD_HEIGHT - delete_row_bottom))
                & VALID_BOARD_RANGE;
            self.2 = <dyn Field>::board_shr(boards[2], delete_row_bottom)
                | <dyn Field>::board_shl(boards[3], BOARD_HEIGHT - delete_row_bottom)
                    & VALID_BOARD_RANGE;
            self.3 = <dyn Field>::board_shr(boards[3], delete_row_bottom);
        }
    }

    fn clear_all(&mut self) {
        self.0 = 0;
        self.1 = 0;
        self.2 = 0;
        self.3 = 0;
    }

    fn fill_all(&mut self) {
        self.0 = VALID_BOARD_RANGE;
        self.1 = VALID_BOARD_RANGE;
        self.2 = VALID_BOARD_RANGE;
        self.3 = VALID_BOARD_RANGE;
    }
}

impl Field for LargeField {
    fn get_max_field_height(&self) -> u8 {
        MAX_FIELD_HEIGHT
    }

    fn get_board_count(&self) -> BoardCount {
        BoardCount::Large
    }

    fn set_block(&mut self, x: u8, y: u8) {
        match Self::select(y) {
            Position::Low(y_off) => self.0 |= <dyn Field>::get_x_mask(x, y_off),
            Position::MidLow(y_off) => self.1 |= <dyn Field>::get_x_mask(x, y_off),
            Position::MidHigh(y_off) => self.2 |= <dyn Field>::get_x_mask(x, y_off),
            Position::High(y_off) => self.3 |= <dyn Field>::get_x_mask(x, y_off),
        }
    }

    fn remove_block(&mut self, x: u8, y: u8) {
        match Self::select(y) {
            Position::Low(y_off) => self.0 &= !<dyn Field>::get_x_mask(x, y_off),
            Position::MidLow(y_off) => self.1 &= !<dyn Field>::get_x_mask(x, y_off),
            Position::MidHigh(y_off) => self.2 &= !<dyn Field>::get_x_mask(x, y_off),
            Position::High(y_off) => self.3 &= !<dyn Field>::get_x_mask(x, y_off),
        }
    }

    fn put(&mut self, mino: &Mino, x: u8, y: u8) {
        match Self::select(y) {
            Position::Low(y_off) => {
                // no lower board

                self.0 |= mino.get_mask(x, y_off as i8);

                // MidLowの更新が必要
                if y_off as i8 + mino.get_max_y() >= 6 {
                    self.1 |= mino.get_mask(x, y_off as i8 - BOARD_HEIGHT as i8)
                }
            }
            Position::MidLow(y_off) => {
                // Lowの更新が必要
                if y_off as i8 + mino.get_min_y() < 0 {
                    self.0 |= mino.get_mask(x, y_off as i8 + BOARD_HEIGHT as i8)
                }

                self.1 |= mino.get_mask(x, y_off as i8);

                // MidHighの更新が必要
                if y_off as i8 + mino.get_max_y() >= BOARD_HEIGHT as i8 {
                    self.2 |= mino.get_mask(x, y_off as i8 - BOARD_HEIGHT as i8)
                }
            }
            Position::MidHigh(y_off) => {
                // MidLowの更新が必要
                if y_off as i8 + mino.get_min_y() < 0 {
                    self.1 |= mino.get_mask(x, y_off as i8 + BOARD_HEIGHT as i8);
                }

                self.2 |= mino.get_mask(x, y_off as i8);

                // Highの更新が必要
                if y_off as i8 + mino.get_max_y() >= BOARD_HEIGHT as i8 {
                    self.3 |= mino.get_mask(x, y_off as i8 - BOARD_HEIGHT as i8);
                }
            }
            Position::High(y_off) => {
                // MidHighの更新が必要
                if y_off as i8 + mino.get_min_y() < 0 {
                    self.2 |= mino.get_mask(x, y_off as i8 + BOARD_HEIGHT as i8);
                }

                self.3 |= mino.get_mask(x, y_off as i8);

                // no higher field
            }
        }
    }

    fn can_put(&self, mino: &Mino, x: u8, y: u8) -> bool {
        match Self::select(y) {
            Position::Low(y_off) => {
                // Low
                self.0 & mino.get_mask(x, y_off as i8) == 0
                    && if y_off as i8 + mino.get_max_y() >= BOARD_HEIGHT as i8 {
                        // MidLow
                        self.1 & mino.get_mask(x, y_off as i8 - BOARD_HEIGHT as i8) == 0
                    } else {
                        true
                    }
            }
            Position::MidLow(y_off) => {
                // MidLow
                self.1 & mino.get_mask(x, y_off as i8) == 0
                    && if y_off as i8 + mino.get_min_y() < 0 {
                        // Low
                        self.0 & mino.get_mask(x, y_off as i8 + BOARD_HEIGHT as i8) == 0
                    } else if y_off as i8 + mino.get_max_y() >= BOARD_HEIGHT as i8 {
                        // MidHigh
                        self.2 & mino.get_mask(x, y_off as i8 - BOARD_HEIGHT as i8) == 0
                    } else {
                        true
                    }
            }
            Position::MidHigh(y_off) => {
                self.2 & mino.get_mask(x, y_off as i8) == 0
                    && if y_off as i8 + mino.get_min_y() < 0 {
                        // MidLow
                        self.1 & mino.get_mask(x, y_off as i8 + BOARD_HEIGHT as i8) == 0
                    } else if y_off as i8 + mino.get_max_y() >= BOARD_HEIGHT as i8 {
                        // High
                        self.3 & mino.get_mask(x, y_off as i8 - BOARD_HEIGHT as i8) == 0
                    } else {
                        true
                    }
            }
            Position::High(y_off) => {
                self.3 & mino.get_mask(x, y_off as i8) == 0
                    && if y_off as i8 + mino.get_min_y() < 0 {
                        // MidHigh
                        self.2 & mino.get_mask(x, y_off as i8 + BOARD_HEIGHT as i8) == 0
                    } else {
                        true
                    }
            }
        }
    }

    fn remove(&mut self, mino: &Mino, x: u8, y: u8) {
        match Self::select(y) {
            Position::Low(y_off) => {
                // no lower board

                self.0 &= !mino.get_mask(x, y_off as i8);

                // MidLowの更新が必要
                if y_off as i8 + mino.get_max_y() >= BOARD_HEIGHT as i8 {
                    self.1 &= !mino.get_mask(x, y_off as i8 - BOARD_HEIGHT as i8);
                }
            }
            Position::MidLow(y_off) => {
                // Lowの更新が必要
                if y_off as i8 + mino.get_min_y() < 0 {
                    self.0 &= !mino.get_mask(x, y_off as i8 + BOARD_HEIGHT as i8);
                }

                self.1 &= !mino.get_mask(x, y_off as i8);

                // MidHighの更新が必要
                if y_off as i8 + mino.get_max_y() >= BOARD_HEIGHT as i8 {
                    self.2 &= !mino.get_mask(x, y_off as i8 - BOARD_HEIGHT as i8);
                }
            }
            Position::MidHigh(y_off) => {
                // MidLowの更新が必要
                if y_off as i8 + mino.get_min_y() < 0 {
                    self.1 &= !mino.get_mask(x, y_off as i8 + BOARD_HEIGHT as i8);
                }

                self.2 |= !mino.get_mask(x, y_off as i8);

                // Highの更新が必要
                if y_off as i8 + mino.get_max_y() >= BOARD_HEIGHT as i8 {
                    self.3 &= !mino.get_mask(x, y_off as i8 - BOARD_HEIGHT as i8);
                }
            }
            Position::High(y_off) => {
                // MidHighの更新が必要
                if y_off as i8 + mino.get_min_y() < 0 {
                    self.2 &= !mino.get_mask(x, y_off as i8 + BOARD_HEIGHT as i8);
                }

                self.3 &= !mino.get_mask(x, y_off as i8);

                // no higher board
            }
        }
    }

    fn can_reach_on_harddrop(&self, mino: &Mino, x: u8, start_y: u8) -> bool {
        self._can_reach_on_harddrop(mino, x, start_y, MAX_FIELD_HEIGHT)
    }

    fn is_empty_block(&self, x: u8, y: u8) -> bool {
        match Self::select(y) {
            Position::Low(y_off) => self.0 & <dyn Field>::get_x_mask(x, y_off) == 0,
            Position::MidLow(y_off) => self.1 & <dyn Field>::get_x_mask(x, y_off) == 0,
            Position::MidHigh(y_off) => self.2 & <dyn Field>::get_x_mask(x, y_off) == 0,
            Position::High(y_off) => self.3 & <dyn Field>::get_x_mask(x, y_off) == 0,
        }
    }

    fn exists_above_row(&self, y: u8) -> bool {
        if y >= MAX_FIELD_HEIGHT {
            return false;
        }

        match Self::select(y) {
            Position::Low(y_off) => {
                // すべて必要
                // High & MidHigh & MidLowのチェック
                self.1 != 0 || self.2 != 0 || self.3 != 0
                // Lowのチェック
                || self.0 & <dyn Field>::get_valid_mask(y_off) != 0
            }
            Position::MidLow(y_off) => {
                // High & MidHighのチェック
                self.2 != 0 || self.3 != 0
                // MidLowのチェック
                || self.1 & <dyn Field>::get_valid_mask(y_off) != 0
            }
            Position::MidHigh(y_off) => {
                // Highのチェック
                self.3 != 0
                // MidHighのチェック
                || self.2 & <dyn Field>::get_valid_mask(y_off) != 0
            }
            Position::High(y_off) => {
                // Highで完結
                self.3 & <dyn Field>::get_valid_mask(y_off) != 0
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.0 == 0 && self.1 == 0 && self.2 == 0 && self.3 == 0
    }

    fn is_filled_in_column(&self, x: u8, max_y: u8) -> bool {
        if max_y == 0 {
            return true;
        }

        match Self::select(max_y) {
            Position::Low(y_off) => {
                // Lowで完結
                self.0 | !bit_operators::get_column_mask(y_off, x) == !0
            }
            Position::MidLow(y_off) => {
                // Lowのチェック
                self.0 | !bit_operators::get_column_mask(BOARD_HEIGHT, x) == !0
                // MidLowのチェック
                && self.1 | !bit_operators::get_column_mask(y_off, x) == !0
            }
            Position::MidHigh(y_off) => {
                // Low & MidLowのチェック
                (self.0 & self.1) | !bit_operators::get_column_mask(BOARD_HEIGHT, x) == !0
                // MidHighのチェック
                && self.2 | !bit_operators::get_column_mask(y_off, x) == !0
            }
            Position::High(y_off) => {
                // Low & MidLow & MidHighのチェック
                (self.0 & self.1 & self.2) | !bit_operators::get_column_mask(BOARD_HEIGHT, x)
                    == !0
                // Highのチェック
                && self.3 | !bit_operators::get_column_mask(y_off, x) == !0
            }
        }
    }

    fn is_wall_between_left(&self, x: u8, max_y: u8) -> bool {
        if max_y == 0 {
            return true;
        }

        match Self::select(max_y) {
            Position::Low(y_off) => {
                // Lowで完結
                bit_operators::is_wall_between_left(x, y_off, self.0)
            }
            Position::MidLow(y_off) => {
                // Low & MidLowのチェック
                bit_operators::is_wall_between_left(x, BOARD_HEIGHT, self.0)
                    && bit_operators::is_wall_between_left(x, y_off, self.1)
            }
            Position::MidHigh(y_off) => {
                // Low & MidLow & MidHighのチェック
                bit_operators::is_wall_between_left(x, BOARD_HEIGHT, self.0)
                    && bit_operators::is_wall_between_left(x, BOARD_HEIGHT, self.1)
                    && bit_operators::is_wall_between_left(x, y_off, self.2)
            }
            Position::High(y_off) => {
                // Low & MidLow & MidHigh & Highのチェック
                bit_operators::is_wall_between_left(x, BOARD_HEIGHT, self.0)
                    && bit_operators::is_wall_between_left(x, BOARD_HEIGHT, self.1)
                    && bit_operators::is_wall_between_left(x, BOARD_HEIGHT, self.2)
                    && bit_operators::is_wall_between_left(x, y_off, self.3)
            }
        }
    }

    fn get_block_count_in_column(&self, x: u8, max_y: u8) -> u32 {
        match Self::select(max_y) {
            Position::Low(y_off) => {
                // Low
                (self.0 & bit_operators::get_column_mask(y_off, x)).count_ones()
            }
            Position::MidLow(y_off) => {
                // Low + MidLow
                (self.0 & bit_operators::get_column_mask(BOARD_HEIGHT, x)).count_ones()
                    + (self.1 & bit_operators::get_column_mask(y_off, x)).count_ones()
            }
            Position::MidHigh(y_off) => {
                let full_mask = bit_operators::get_column_mask(BOARD_HEIGHT, x);
                // Low + MidLow + MidHigh
                (self.0 & full_mask).count_ones()
                    + (self.1 & full_mask).count_ones()
                    + (self.2 & bit_operators::get_column_mask(y_off, x)).count_ones()
            }
            Position::High(y_off) => {
                let full_mask = bit_operators::get_column_mask(BOARD_HEIGHT, x);
                // Low + MidLow + MidHigh + High
                (self.0 & full_mask).count_ones()
                    + (self.1 & full_mask).count_ones()
                    + (self.2 & full_mask).count_ones()
                    + (self.3 & bit_operators::get_column_mask(y_off, x)).count_ones()
            }
        }
    }

    fn get_block_count_in_row(&self, y: u8) -> u32 {
        match Self::select(y) {
            Position::Low(y_off) => self.0 & <dyn Field>::get_row_mask(y_off),
            Position::MidLow(y_off) => self.1 & <dyn Field>::get_row_mask(y_off),
            Position::MidHigh(y_off) => self.2 & <dyn Field>::get_row_mask(y_off),
            Position::High(y_off) => self.3 & <dyn Field>::get_row_mask(y_off),
        }
        .count_ones()
    }

    fn exists_block_in_row(&self, y: u8) -> bool {
        (match Self::select(y) {
            Position::Low(y_off) => self.0 & <dyn Field>::get_row_mask(y_off),
            Position::MidLow(y_off) => self.1 & <dyn Field>::get_row_mask(y_off),
            Position::MidHigh(y_off) => self.2 & <dyn Field>::get_row_mask(y_off),
            Position::High(y_off) => self.3 & <dyn Field>::get_row_mask(y_off),
        }) != 0
    }

    fn get_num_of_all_blocks(&self) -> u32 {
        self.0.count_ones() + self.1.count_ones() + self.2.count_ones() + self.3.count_ones()
    }

    fn clear_filled_rows_return_key(&mut self) -> u64 {
        let delete_key_low = key_operators::get_delete_key(self.0);
        let delete_key_mid_low = key_operators::get_delete_key(self.1);
        let delete_key_mid_high = key_operators::get_delete_key(self.2);
        let delete_key_high = key_operators::get_delete_key(self.3);

        self.delete_row(
            delete_key_low,
            delete_key_mid_low,
            delete_key_mid_high,
            delete_key_high,
        );

        Self::combine_keys(
            delete_key_low,
            delete_key_mid_low,
            delete_key_mid_high,
            delete_key_high,
        )
    }

    fn get_filled_rows_key(&self) -> u64 {
        Self::combine_keys(
            key_operators::get_delete_key(self.0),
            key_operators::get_delete_key(self.1),
            key_operators::get_delete_key(self.2),
            key_operators::get_delete_key(self.3),
        )
    }

    fn get_using_key(&self) -> u64 {
        Self::combine_keys(
            key_operators::get_using_key(self.0),
            key_operators::get_using_key(self.1),
            key_operators::get_using_key(self.2),
            key_operators::get_using_key(self.3),
        )
    }

    fn insert_filled_row_with_key(&mut self, delete_key: u64) {
        let delete_keys: [_; 4] =
            std::array::from_fn(|index| <dyn Field>::extract_delete_key(delete_key, index as u8));

        let delete_rows = delete_keys[0..3]
            .iter()
            .scan(0, |sum, delete_key| {
                *sum += delete_key.count_ones() as u8;
                Some(*sum)
            })
            .collect::<Vec<_>>();

        // used for boards that are not the bottommost
        fn create_upper_board(
            board_low: u64,
            board_high: u64,
            delete_row: u8,
            delete_key: u64,
        ) -> u64 {
            let left_row = BOARD_HEIGHT - delete_row;
            long_board_map::insert_filled_row(
                <dyn Field>::board_shl(board_high, delete_row)
                    // why mask and shift? aren't those bits shifted out?
                    | <dyn Field>::board_shr(board_low & bit_operators::get_row_mask_above_y(left_row), left_row),
                delete_key,
            )
        }

        fn create_bottom_board(board_bottom: u64, delete_row: u8, delete_key: u64) -> u64 {
            let left_row = BOARD_HEIGHT - delete_row;
            long_board_map::insert_filled_row(
                board_bottom & bit_operators::get_row_mask_below_y(left_row),
                delete_key,
            )
        }

        #[rustfmt::skip]
        let new_x_boards = if delete_rows[2] >= FIELD_ROW_MID_HIGH_BORDER_Y {
            // Low & MidLow
            [
                create_bottom_board(self.0, delete_rows[0], delete_keys[0]),
                create_upper_board(self.0, self.1, delete_rows[0], delete_keys[1]),
                create_upper_board(self.0, self.1, delete_rows[1] - BOARD_HEIGHT, delete_keys[2]),
                create_upper_board(self.0, self.1, delete_rows[2] - FIELD_ROW_MID_HIGH_BORDER_Y, delete_keys[3]),
            ]
        } else if delete_rows[2] >= BOARD_HEIGHT {
            // Low & MidLow & MidHigh
            if delete_rows[1] >= BOARD_HEIGHT {
                // Low & MidLow
                [
                    create_bottom_board(self.0, delete_rows[0], delete_keys[0]),
                    create_upper_board(self.0, self.1, delete_rows[0], delete_keys[1]),
                    create_upper_board(self.0, self.1, delete_rows[1] - BOARD_HEIGHT, delete_keys[2]),
                    create_upper_board(self.1, self.2, delete_rows[2] - BOARD_HEIGHT, delete_keys[3]),
                ]
            } else {
                // Low & MidLow & MidHigh
                [
                    create_bottom_board(self.0, delete_rows[0], delete_keys[0]),
                    create_upper_board(self.0, self.1, delete_rows[0], delete_keys[1]),
                    create_upper_board(self.1, self.2, delete_rows[1], delete_keys[2]),
                    create_upper_board(self.1, self.2, delete_rows[2] - BOARD_HEIGHT, delete_keys[3]),
                ]
            }
        } else {
            // Low & MidLow & MidHigh & High
            [
                create_bottom_board(self.0, delete_rows[0], delete_keys[0]),
                create_upper_board(self.0, self.1, delete_rows[0], delete_keys[1]),
                create_upper_board(self.1, self.2, delete_rows[1], delete_keys[2]),
                create_upper_board(self.2, self.3, delete_rows[2], delete_keys[3]),
            ]
        };

        self.0 = new_x_boards[0];
        self.1 = new_x_boards[1] & VALID_BOARD_RANGE;
        self.2 = new_x_boards[2] & VALID_BOARD_RANGE;
        self.3 = new_x_boards[3] & VALID_BOARD_RANGE;
    }

    fn insert_blank_row_with_key(&mut self, delete_key: u64) {
        let delete_keys: [_; 4] =
            std::array::from_fn(|index| <dyn Field>::extract_delete_key(delete_key, index as u8));

        let delete_rows = delete_keys[0..3]
            .iter()
            .scan(0, |sum, delete_key| {
                *sum += delete_key.count_ones() as u8;
                Some(*sum)
            })
            .collect::<Vec<_>>();

        // used for boards that are not the bottommost
        fn create_upper_board(
            board_low: u64,
            board_high: u64,
            delete_row: u8,
            delete_key: u64,
        ) -> u64 {
            let left_row = BOARD_HEIGHT - delete_row;
            long_board_map::insert_blank_row(
                <dyn Field>::board_shl(board_high, delete_row)
                    // why mask and shift? aren't those bits shifted out?
                    | <dyn Field>::board_shr(board_low & bit_operators::get_row_mask_above_y(left_row), left_row),
                delete_key,
            )
        }

        fn create_bottom_board(board_bottom: u64, delete_row: u8, delete_key: u64) -> u64 {
            let left_row = BOARD_HEIGHT - delete_row;
            long_board_map::insert_blank_row(
                board_bottom & bit_operators::get_row_mask_below_y(left_row),
                delete_key,
            )
        }

        #[rustfmt::skip]
        let new_x_boards = if delete_rows[2] >= FIELD_ROW_MID_HIGH_BORDER_Y {
            // Low & MidLow
            [
                create_bottom_board(self.0, delete_rows[0], delete_keys[0]),
                create_upper_board(self.0, self.1, delete_rows[0], delete_keys[1]),
                create_upper_board(self.0, self.1, delete_rows[1] - BOARD_HEIGHT, delete_keys[2]),
                create_upper_board(self.0, self.1, delete_rows[2] - FIELD_ROW_MID_HIGH_BORDER_Y, delete_keys[3]),
            ]
        } else if delete_rows[2] >= BOARD_HEIGHT {
            // Low & MidLow & MidHigh
            if delete_rows[1] >= BOARD_HEIGHT {
                // Low & MidLow
                [
                    create_bottom_board(self.0, delete_rows[0], delete_keys[0]),
                    create_upper_board(self.0, self.1, delete_rows[0], delete_keys[1]),
                    create_upper_board(self.0, self.1, delete_rows[1] - BOARD_HEIGHT, delete_keys[2]),
                    create_upper_board(self.1, self.2, delete_rows[2] - BOARD_HEIGHT, delete_keys[3]),
                ]
            } else {
                // Low & MidLow & MidHigh
                [
                    create_bottom_board(self.0, delete_rows[0], delete_keys[0]),
                    create_upper_board(self.0, self.1, delete_rows[0], delete_keys[1]),
                    create_upper_board(self.1, self.2, delete_rows[1], delete_keys[2]),
                    create_upper_board(self.1, self.2, delete_rows[2] - BOARD_HEIGHT, delete_keys[3]),
                ]
            }
        } else {
            // Low & MidLow & MidHigh & High
            [
                create_bottom_board(self.0, delete_rows[0], delete_keys[0]),
                create_upper_board(self.0, self.1, delete_rows[0], delete_keys[1]),
                create_upper_board(self.1, self.2, delete_rows[1], delete_keys[2]),
                create_upper_board(self.2, self.3, delete_rows[2], delete_keys[3]),
            ]
        };

        self.0 = new_x_boards[0];
        self.1 = new_x_boards[1] & VALID_BOARD_RANGE;
        self.2 = new_x_boards[2] & VALID_BOARD_RANGE;
        self.3 = new_x_boards[3] & VALID_BOARD_RANGE;
    }

    fn delete_rows_with_key(&mut self, delete_key: u64) {
        self.delete_row(
            <dyn Field>::extract_delete_key(delete_key, 0),
            <dyn Field>::extract_delete_key(delete_key, 1),
            <dyn Field>::extract_delete_key(delete_key, 2),
            <dyn Field>::extract_delete_key(delete_key, 3),
        )
    }

    fn fill_row(&mut self, y: u8) {
        match Self::select(y) {
            Position::Low(y_off) => self.0 |= <dyn Field>::get_row_mask(y_off),
            Position::MidLow(y_off) => self.1 |= <dyn Field>::get_row_mask(y_off),
            Position::MidHigh(y_off) => self.2 |= <dyn Field>::get_row_mask(y_off),
            Position::High(y_off) => self.3 |= <dyn Field>::get_row_mask(y_off),
        }
    }

    fn get_board(&self, index: u8) -> u64 {
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            3 => self.3,
            _ => 0,
        }
    }

    fn prune(&self, max_height: u8) -> Box<dyn Field> {
        match max_height {
            ..=6 => Box::new(SmallField::from(self.0)),
            ..=12 => Box::new(MiddleField::from_parts(self.0, self.1)),
            _ => Box::new(self.clone()),
        }
    }

    fn merge(&mut self, other: &dyn Field) {
        match other.get_board_count() {
            BoardCount::Small => self.0 |= other.get_board(0),
            BoardCount::Middle => {
                self.0 |= other.get_board(0);
                self.1 |= other.get_board(1);
            }
            BoardCount::Large => {
                self.0 |= other.get_board(0);
                self.1 |= other.get_board(1);
                self.2 |= other.get_board(2);
                self.3 |= other.get_board(3);
            }
        }
    }

    fn can_merge(&self, other: &dyn Field) -> bool {
        match other.get_board_count() {
            BoardCount::Small => self.0 & other.get_board(0) == 0,
            BoardCount::Middle => {
                self.0 & other.get_board(0) == 0 && self.1 & other.get_board(1) == 0
            }
            BoardCount::Large => {
                self.0 & other.get_board(0) == 0
                    && self.1 & other.get_board(1) == 0
                    && self.2 & other.get_board(2) == 0
                    && self.3 & other.get_board(3) == 0
            }
        }
    }

    fn reduce(&mut self, other: &dyn Field) {
        match other.get_board_count() {
            BoardCount::Small => self.0 &= !other.get_board(0),
            BoardCount::Middle => {
                self.0 &= !other.get_board(0);
                self.1 &= !other.get_board(1);
            }
            BoardCount::Large => {
                self.0 &= !other.get_board(0);
                self.1 &= !other.get_board(1);
                self.2 &= !other.get_board(2);
                self.3 &= !other.get_board(3);
            }
        }
    }

    fn get_upper_y_with_4_blocks(&self) -> u8 {
        assert_eq!(
            self.0.count_ones() + self.1.count_ones() + self.2.count_ones() + self.3.count_ones(),
            4
        );

        if let Some(min_y) = bit_operators::try_get_highest_y(self.3) {
            min_y + FIELD_ROW_HIGH_BORDER_Y
        } else if let Some(min_y) = bit_operators::try_get_highest_y(self.2) {
            min_y + FIELD_ROW_MID_HIGH_BORDER_Y
        } else if let Some(min_y) = bit_operators::try_get_highest_y(self.1) {
            min_y + FIELD_ROW_MID_LOW_BORDER_Y
        } else {
            bit_operators::get_highest_y(self.0)
        }
    }

    fn get_min_x(&self) -> Option<u8> {
        bit_operators::try_get_lowest_x(self.0 | self.1 | self.2 | self.3)
    }

    fn get_min_y(&self) -> Option<u8> {
        if let Some(min_y) = bit_operators::try_get_lowest_y(self.0) {
            Some(min_y)
        } else if let Some(min_y) = bit_operators::try_get_lowest_y(self.1) {
            Some(min_y + FIELD_ROW_MID_LOW_BORDER_Y)
        } else if let Some(min_y) = bit_operators::try_get_lowest_y(self.2) {
            Some(min_y + FIELD_ROW_MID_HIGH_BORDER_Y)
        } else if let Some(min_y) = bit_operators::try_get_lowest_y(self.3) {
            Some(min_y + FIELD_ROW_HIGH_BORDER_Y)
        } else {
            None
        }
    }

    fn slide_left(&mut self, slide: u8) {
        let mask = bit_operators::get_column_mask_right_of_row(slide);
        self.0 = (self.0 & mask) >> slide;
        self.1 = (self.1 & mask) >> slide;
        self.2 = (self.2 & mask) >> slide;
        self.3 = (self.3 & mask) >> slide;
    }

    fn slide_right(&mut self, slide: u8) {
        let mask = bit_operators::get_column_mask_left_of_row(FIELD_WIDTH - slide);
        self.0 = (self.0 & mask) << slide;
        self.1 = (self.1 & mask) << slide;
        self.2 = (self.2 & mask) << slide;
        self.3 = (self.3 & mask) << slide;
    }

    fn slide_down_one(&mut self) {
        self.0 = (self.0 >> FIELD_WIDTH | <dyn Field>::board_shl(self.1, BOARD_HEIGHT - 1))
            & VALID_BOARD_RANGE;
        self.1 = (self.1 >> FIELD_WIDTH | <dyn Field>::board_shl(self.2, BOARD_HEIGHT - 1))
            & VALID_BOARD_RANGE;
        self.2 = (self.2 >> FIELD_WIDTH | <dyn Field>::board_shl(self.3, BOARD_HEIGHT - 1))
            & VALID_BOARD_RANGE;
        self.3 >>= FIELD_WIDTH;
    }

    fn slide_down(&mut self, slide: u8) {
        // cannot reuse select because the ranges are different
        match slide {
            ..=FIELD_ROW_MID_LOW_BORDER_Y => {
                self.delete_row(key_operators::get_mask_for_key_below_y(slide), 0, 0, 0)
            }
            ..=FIELD_ROW_MID_HIGH_BORDER_Y => self.delete_row(
                key_operators::get_mask_for_key_below_y(BOARD_HEIGHT),
                key_operators::get_mask_for_key_below_y(slide - FIELD_ROW_MID_LOW_BORDER_Y),
                0,
                0,
            ),
            ..=FIELD_ROW_HIGH_BORDER_Y => self.delete_row(
                key_operators::get_mask_for_key_below_y(BOARD_HEIGHT),
                key_operators::get_mask_for_key_below_y(BOARD_HEIGHT),
                key_operators::get_mask_for_key_below_y(slide - FIELD_ROW_MID_HIGH_BORDER_Y),
                0,
            ),
            ..=MAX_FIELD_HEIGHT => self.delete_row(
                key_operators::get_mask_for_key_below_y(BOARD_HEIGHT),
                key_operators::get_mask_for_key_below_y(BOARD_HEIGHT),
                key_operators::get_mask_for_key_below_y(BOARD_HEIGHT),
                key_operators::get_mask_for_key_below_y(slide - FIELD_ROW_HIGH_BORDER_Y),
            ),
            _ => self.clear_all(),
        }
    }

    fn slide_up_with_empty_row(&mut self, slide: u8) {
        if slide < MAX_FIELD_HEIGHT {
            self.insert_blank_row_with_key(key_operators::get_mask_for_key_below_y(slide));
        } else {
            self.clear_all();
        }
    }

    fn slide_up_with_filled_row(&mut self, slide: u8) {
        if slide < MAX_FIELD_HEIGHT {
            self.insert_filled_row_with_key(key_operators::get_mask_for_key_below_y(slide));
        } else {
            self.fill_all();
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
                let child_board_mid_low = child.get_board(1);
                self.0 & child_board_low == child_board_low
                    && self.1 & child_board_mid_low == child_board_mid_low
            }
            BoardCount::Large => {
                let child_board_low = child.get_board(0);
                let child_board_mid_low = child.get_board(1);
                let child_board_mid_high = child.get_board(2);
                let child_board_high = child.get_board(3);
                self.0 & child_board_low == child_board_low
                    && self.1 & child_board_mid_low == child_board_mid_low
                    && self.2 & child_board_mid_high == child_board_mid_high
                    && self.3 & child_board_high == child_board_high
            }
        }
    }

    fn invert(&mut self) {
        self.0 = !self.0 & VALID_BOARD_RANGE;
        self.1 = !self.1 & VALID_BOARD_RANGE;
        self.2 = !self.2 & VALID_BOARD_RANGE;
        self.3 = !self.3 & VALID_BOARD_RANGE;
    }

    fn mirror(&mut self) {
        self.0 = key_operators::mirror(self.0);
        self.1 = key_operators::mirror(self.1);
        self.2 = key_operators::mirror(self.2);
        self.3 = key_operators::mirror(self.3);
    }

    fn mask(&mut self, mask_field: &dyn Field) {
        self.0 &= mask_field.get_board(0);
        self.1 &= mask_field.get_board(1);
        self.2 &= mask_field.get_board(2);
        self.3 &= mask_field.get_board(3);
    }
}
