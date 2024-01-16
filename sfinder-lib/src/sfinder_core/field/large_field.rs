use super::{
    bit_operators,
    field::{Field, FieldHelper},
    field_constants::{BoardCount, BOARD_HEIGHT, FIELD_WIDTH, VALID_BOARD_RANGE},
    key_operators, long_board_map,
    middle_field::MiddleField,
    small_field::SmallField,
};
use crate::sfinder_core::mino::mino::Mino;
use std::fmt::Debug;

const FIELD_ROW_MID_LOW_BORDER_Y: u8 = BOARD_HEIGHT;
const FIELD_ROW_MID_HIGH_BORDER_Y: u8 = BOARD_HEIGHT * 2;
const FIELD_ROW_HIGH_BORDER_Y: u8 = BOARD_HEIGHT * 3;
pub const MAX_FIELD_HEIGHT: u8 = BOARD_HEIGHT * 4;

/// attached u8 is the adjusted y position such that it is less than BOARD_HEIGHT
enum Position {
    Low(u8),
    MidLow(u8),
    MidHigh(u8),
    High(u8),
}

#[derive(Clone)]
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
                | bit_operators::board_shl(new_x_boards[1], BOARD_HEIGHT - delete_rows[0]))
                & VALID_BOARD_RANGE,
            bit_operators::board_shr(new_x_boards[1], delete_rows[0]),
            // 上半分
            (new_x_boards[2]
                | bit_operators::board_shl(new_x_boards[3], BOARD_HEIGHT - delete_rows[2]))
                & VALID_BOARD_RANGE,
            bit_operators::board_shr(new_x_boards[3], delete_rows[2]),
        ];

        let delete_row_bottom = delete_rows[0] + delete_rows[1];
        // 上半分と下半分をマージ
        if delete_row_bottom >= BOARD_HEIGHT {
            let slide = delete_row_bottom - BOARD_HEIGHT;
            self.0 = (boards[0] | (bit_operators::board_shl(boards[2], BOARD_HEIGHT - slide)))
                & VALID_BOARD_RANGE;
            self.1 = bit_operators::board_shr(boards[2], slide)
                | bit_operators::board_shl(boards[3], BOARD_HEIGHT - slide) & VALID_BOARD_RANGE;
            self.2 = bit_operators::board_shr(boards[3], slide);
            self.3 = 0;
        } else {
            self.0 = boards[0];
            self.1 = (boards[1]
                | bit_operators::board_shl(boards[2], BOARD_HEIGHT - delete_row_bottom))
                & VALID_BOARD_RANGE;
            self.2 = bit_operators::board_shr(boards[2], delete_row_bottom)
                | bit_operators::board_shl(boards[3], BOARD_HEIGHT - delete_row_bottom)
                    & VALID_BOARD_RANGE;
            self.3 = bit_operators::board_shr(boards[3], delete_row_bottom);
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

    // row_fill_fn is used to factor out the two calls of this function that differ only by this argument.
    fn insert_row_with_key(&mut self, delete_key: u64, row_fill_fn: fn(u64, u64) -> u64) {
        let delete_keys: [_; 4] =
            std::array::from_fn(|index| <dyn Field>::extract_delete_key(delete_key, index as u8));

        let delete_rows = delete_keys[0..3]
            .iter()
            .scan(0, |sum, delete_key| {
                *sum += delete_key.count_ones() as u8;
                Some(*sum)
            })
            .collect::<Vec<_>>();

        let create_new_x_boards = |[mid_high, high]: [(u64, u64, u8); 2]| {
            [
                <dyn Field>::create_bottom_board(
                    self.0,
                    delete_rows[0],
                    delete_keys[0],
                    row_fill_fn,
                ),
                <dyn Field>::create_upper_board(
                    self.0,
                    self.1,
                    delete_rows[0],
                    delete_keys[1],
                    row_fill_fn,
                ),
                <dyn Field>::create_upper_board(
                    mid_high.0,
                    mid_high.1,
                    delete_rows[1] - mid_high.2,
                    delete_keys[2],
                    row_fill_fn,
                ),
                <dyn Field>::create_upper_board(
                    high.0,
                    high.1,
                    delete_rows[2] - high.2,
                    delete_keys[3],
                    row_fill_fn,
                ),
            ]
        };

        let new_x_boards = create_new_x_boards(match delete_rows[2] {
            FIELD_ROW_MID_HIGH_BORDER_Y.. => [
                (self.0, self.1, BOARD_HEIGHT),
                (self.0, self.1, FIELD_ROW_MID_HIGH_BORDER_Y),
            ],
            BOARD_HEIGHT.. if delete_rows[1] >= BOARD_HEIGHT => [
                (self.0, self.1, BOARD_HEIGHT),
                (self.1, self.2, BOARD_HEIGHT),
            ],
            BOARD_HEIGHT.. => [(self.1, self.2, 0), (self.1, self.2, BOARD_HEIGHT)],
            _ => [(self.1, self.2, 0), (self.2, self.3, 0)],
        });

        self.0 = new_x_boards[0];
        self.1 = new_x_boards[1] & VALID_BOARD_RANGE;
        self.2 = new_x_boards[2] & VALID_BOARD_RANGE;
        self.3 = new_x_boards[3] & VALID_BOARD_RANGE;
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
            Position::Low(y_off) => self.0 |= bit_operators::get_x_mask(x, y_off),
            Position::MidLow(y_off) => self.1 |= bit_operators::get_x_mask(x, y_off),
            Position::MidHigh(y_off) => self.2 |= bit_operators::get_x_mask(x, y_off),
            Position::High(y_off) => self.3 |= bit_operators::get_x_mask(x, y_off),
        }
    }

    fn remove_block(&mut self, x: u8, y: u8) {
        match Self::select(y) {
            Position::Low(y_off) => self.0 &= !bit_operators::get_x_mask(x, y_off),
            Position::MidLow(y_off) => self.1 &= !bit_operators::get_x_mask(x, y_off),
            Position::MidHigh(y_off) => self.2 &= !bit_operators::get_x_mask(x, y_off),
            Position::High(y_off) => self.3 &= !bit_operators::get_x_mask(x, y_off),
        }
    }

    fn put(&mut self, mino: &Mino, x: u8, y: u8) {
        match Self::select(y) {
            Position::Low(y_off) => {
                // no lower board

                self.0 |= mino.get_mask(x, y_off as i8);

                // MidLowの更新が必要
                if y_off as i8 + mino.get_max_y() >= BOARD_HEIGHT as i8 {
                    self.1 |= mino.get_mask(x, y_off as i8 - BOARD_HEIGHT as i8);
                }
            }
            Position::MidLow(y_off) => {
                // Lowの更新が必要
                if y_off as i8 + mino.get_min_y() < 0 {
                    self.0 |= mino.get_mask(x, y_off as i8 + BOARD_HEIGHT as i8);
                }

                self.1 |= mino.get_mask(x, y_off as i8);

                // MidHighの更新が必要
                if y_off as i8 + mino.get_max_y() >= BOARD_HEIGHT as i8 {
                    self.2 |= mino.get_mask(x, y_off as i8 - BOARD_HEIGHT as i8);
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

                self.2 &= !mino.get_mask(x, y_off as i8);

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
            Position::Low(y_off) => self.0 & bit_operators::get_x_mask(x, y_off) == 0,
            Position::MidLow(y_off) => self.1 & bit_operators::get_x_mask(x, y_off) == 0,
            Position::MidHigh(y_off) => self.2 & bit_operators::get_x_mask(x, y_off) == 0,
            Position::High(y_off) => self.3 & bit_operators::get_x_mask(x, y_off) == 0,
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
            Position::Low(y_off) => self.0 & bit_operators::get_row_mask(y_off),
            Position::MidLow(y_off) => self.1 & bit_operators::get_row_mask(y_off),
            Position::MidHigh(y_off) => self.2 & bit_operators::get_row_mask(y_off),
            Position::High(y_off) => self.3 & bit_operators::get_row_mask(y_off),
        }
        .count_ones()
    }

    fn exists_block_in_row(&self, y: u8) -> bool {
        (match Self::select(y) {
            Position::Low(y_off) => self.0 & bit_operators::get_row_mask(y_off),
            Position::MidLow(y_off) => self.1 & bit_operators::get_row_mask(y_off),
            Position::MidHigh(y_off) => self.2 & bit_operators::get_row_mask(y_off),
            Position::High(y_off) => self.3 & bit_operators::get_row_mask(y_off),
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
        self.insert_row_with_key(delete_key, long_board_map::insert_filled_row);
    }

    fn insert_blank_row_with_key(&mut self, delete_key: u64) {
        self.insert_row_with_key(delete_key, long_board_map::insert_blank_row);
    }

    fn delete_rows_with_key(&mut self, delete_key: u64) {
        self.delete_row(
            <dyn Field>::extract_delete_key(delete_key, 0),
            <dyn Field>::extract_delete_key(delete_key, 1),
            <dyn Field>::extract_delete_key(delete_key, 2),
            <dyn Field>::extract_delete_key(delete_key, 3),
        );
    }

    fn fill_row(&mut self, y: u8) {
        match Self::select(y) {
            Position::Low(y_off) => self.0 |= bit_operators::get_row_mask(y_off),
            Position::MidLow(y_off) => self.1 |= bit_operators::get_row_mask(y_off),
            Position::MidHigh(y_off) => self.2 |= bit_operators::get_row_mask(y_off),
            Position::High(y_off) => self.3 |= bit_operators::get_row_mask(y_off),
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
            ..=BOARD_HEIGHT => Box::new(SmallField::from(self.0)),
            ..=FIELD_ROW_MID_HIGH_BORDER_Y => Box::new(MiddleField::from_parts(self.0, self.1)),
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
        #[allow(clippy::manual_map)]
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
        self.0 = (bit_operators::board_shr(self.0, 1)
            | bit_operators::board_shl(self.1, BOARD_HEIGHT - 1))
            & VALID_BOARD_RANGE;
        self.1 = (bit_operators::board_shr(self.1, 1)
            | bit_operators::board_shl(self.2, BOARD_HEIGHT - 1))
            & VALID_BOARD_RANGE;
        self.2 = (bit_operators::board_shr(self.2, 1)
            | bit_operators::board_shl(self.3, BOARD_HEIGHT - 1))
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

impl Debug for LargeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LargeField {:#062b} {:#062b} {:#062b} {:#062b}",
            self.0, self.1, self.2, self.3
        )
    }
}

impl PartialEq for LargeField {
    fn eq(&self, other: &Self) -> bool {
        <dyn Field>::eq(self, other)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        common::{
            datastore::{action::action::Action, mino_operation::MinoOperation},
            tetfu::{
                common::color_type::ColorType,
                field::{array_colored_field::ArrayColoredField, colored_field::ColoredField},
            },
        },
        sfinder_core::{
            field::{field_constants::FIELD_WIDTH, field_factory},
            mino::{mino_factory::MinoFactory, piece::Piece},
            neighbor::original_piece::create_all_pieces,
            srs::rotate::Rotate,
        },
        sfinder_lib::randoms,
    };
    use rand::{rngs::ThreadRng, thread_rng, Rng};

    fn create_random_large_field(rngs: &mut ThreadRng, empty_minos: u8) -> LargeField {
        let random_field = randoms::gen_field(rngs, MAX_FIELD_HEIGHT, empty_minos);
        LargeField::from_parts(
            random_field.get_board(0),
            random_field.get_board(1),
            random_field.get_board(2),
            random_field.get_board(3),
        )
    }

    #[test]
    fn get_max_field_height() {
        assert_eq!(LargeField::new().get_max_field_height(), MAX_FIELD_HEIGHT);
    }

    #[test]
    fn block() {
        let mut field = field_factory::create_large_field();

        for index in 0..MAX_FIELD_HEIGHT {
            field.set_block(index % FIELD_WIDTH, index);
        }

        assert_eq!(field.get_num_of_all_blocks(), MAX_FIELD_HEIGHT as u32);

        for y in 0..MAX_FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                assert_eq!(field.is_empty_block(x, y), x != y % FIELD_WIDTH);
            }
        }

        for index in 0..MAX_FIELD_HEIGHT {
            field.remove_block(index % FIELD_WIDTH, index);
        }

        for y in 0..MAX_FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                assert!(field.is_empty_block(x, y));
            }
        }
    }

    #[test]
    fn put() {
        for y in 1..MAX_FIELD_HEIGHT - 2 {
            for x in 0..FIELD_WIDTH - 2 {
                let mut field = field_factory::create_large_field();

                field.put(&Mino::new(Piece::T, Rotate::Right), x, y);
                assert!(!field.is_empty_block(x, y));
                assert!(!field.is_empty_block(x, y - 1));
                assert!(!field.is_empty_block(x, y + 1));
                assert!(!field.is_empty_block(x + 1, y));
            }
        }
    }

    #[test]
    fn put_2() {
        for piece in create_all_pieces(&MinoFactory::new(), MAX_FIELD_HEIGHT) {
            let mut field1 = field_factory::create_large_field();

            assert!(field1.can_put(piece.get_mino(), piece.get_x(), piece.get_y()));
            field1.put(piece.get_mino(), piece.get_x(), piece.get_y());
            assert!(!field1.can_put(piece.get_mino(), piece.get_x(), piece.get_y()));

            let mut field2 = field_factory::create_large_field();

            assert!(field2.can_put(piece.get_mino(), piece.get_x(), piece.get_y()));
            field2.put(piece.get_mino(), piece.get_x(), piece.get_y());
            assert!(!field2.can_put(piece.get_mino(), piece.get_x(), piece.get_y()));

            assert_eq!(field1.get_board(0), field2.get_board(0));
            assert_eq!(field1.get_board(1), field2.get_board(1));
            assert_eq!(field1.get_board(2), field2.get_board(2));
            assert_eq!(field1.get_board(3), field2.get_board(3));

            assert!(!field1.is_empty());
        }
    }

    #[test]
    fn remove() {
        for y in 1..MAX_FIELD_HEIGHT - 2 {
            for x in 0..FIELD_WIDTH - 2 {
                let mut field = field_factory::create_large_field();
                field.invert();

                field.remove(&Mino::new(Piece::T, Rotate::Right), x, y);
                // println!("{x} {y} {field:?}");

                assert!(field.is_empty_block(x, y));
                assert!(field.is_empty_block(x, y - 1));
                assert!(field.is_empty_block(x, y + 1));
                assert!(field.is_empty_block(x + 1, y));
            }
        }
    }

    #[test]
    fn remove_2() {
        for piece in create_all_pieces(&MinoFactory::new(), MAX_FIELD_HEIGHT) {
            let mut field1 = field_factory::create_large_field();
            field1.invert();
            field1.remove(piece.get_mino(), piece.get_x(), piece.get_y());

            let mut field2 = field_factory::create_large_field();
            field2.invert();
            field2.remove_piece(&piece);

            assert_eq!(field1.get_board(0), field2.get_board(0));
            assert_eq!(field1.get_board(1), field2.get_board(1));
            assert_eq!(field1.get_board(2), field2.get_board(2));
            assert_eq!(field1.get_board(3), field2.get_board(3));
        }
    }

    #[test]
    fn get_y_on_harddrop() {
        let field = field_factory::create_large_field_with_marks(
            String::new()
                + "X_________"
                + "__________"
                + "__________"
                + "__________"
                + "_________X"
                + "____X_____"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________",
        );

        assert_eq!(
            field.get_y_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 1, MAX_FIELD_HEIGHT),
            12
        );
        assert_eq!(
            field.get_y_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 2, MAX_FIELD_HEIGHT),
            0
        );
        assert_eq!(
            field.get_y_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 3, MAX_FIELD_HEIGHT),
            7
        );
        assert_eq!(
            field.get_y_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 8, MAX_FIELD_HEIGHT),
            8
        );
    }

    #[test]
    fn can_reach_on_harddrop() {
        let field = field_factory::create_large_field_with_marks(
            String::new()
                + "X_________"
                + "__________"
                + "__________"
                + "__________"
                + "_________X"
                + "____X_____"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________"
                + "__________",
        );

        assert!(!field.can_reach_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 1, 4));
        assert!(field.can_reach_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 2, 4));
        assert!(field.can_reach_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 2, 3));
        assert!(!field.can_reach_on_harddrop(&Mino::new(Piece::T, Rotate::Spawn), 1, 1));
    }

    #[test]
    fn can_reach_on_harddrop_2() {
        let mut rngs = thread_rng();
        let field = create_random_large_field(&mut rngs, 50);

        for pieces in create_all_pieces(&MinoFactory::new(), MAX_FIELD_HEIGHT) {
            let mino = pieces.get_mino();
            let x = pieces.get_x();
            let y = pieces.get_y();

            assert_eq!(
                field.can_reach_on_harddrop_piece(&pieces),
                field.can_put(mino, x, y) && field.can_reach_on_harddrop(mino, x, y)
            );
        }
    }

    #[test]
    fn exist_above_row() {
        for y in 0..MAX_FIELD_HEIGHT {
            let mut field = field_factory::create_large_field();
            field.set_block(0, y);

            for y2 in 0..MAX_FIELD_HEIGHT {
                assert_eq!(field.exists_above_row(y2), y2 <= y);
            }
        }
    }

    #[test]
    fn is_empty() {
        assert!(field_factory::create_large_field().is_empty());
    }

    #[test]
    fn is_filled_in_column() {
        for y in 0..MAX_FIELD_HEIGHT {
            for x in 1..FIELD_WIDTH {
                let mut field = field_factory::create_large_field();
                for i in 0..y {
                    field.set_block(x, i);
                }

                for i in 0..MAX_FIELD_HEIGHT {
                    assert_eq!(field.is_filled_in_column(x, i), i <= y);
                }
            }
        }
    }

    #[test]
    fn is_wall_between_left() {
        let mut rngs = thread_rng();
        for y in 0..MAX_FIELD_HEIGHT {
            for x in 1..FIELD_WIDTH {
                // println!("testing {x}");

                let mut field = field_factory::create_large_field();
                for i in 0..y {
                    if rngs.gen_bool(0.5) {
                        field.set_block(x, i);
                    } else {
                        field.set_block(x - 1, i);
                    }
                }

                for i in 0..MAX_FIELD_HEIGHT {
                    assert_eq!(field.is_wall_between_left(x, i), i <= y, "{field:?} {i}");
                }
            }
        }
    }

    #[test]
    fn is_on_ground() {
        let field = field_factory::create_large_field();
        assert!(field.is_on_ground(&Mino::new(Piece::I, Rotate::Spawn), 3, 0));
        assert!(!field.is_on_ground(&Mino::new(Piece::I, Rotate::Spawn), 3, 1));

        for y in 2..MAX_FIELD_HEIGHT {
            let mut field = field_factory::create_large_field();
            field.set_block(4, y - 2);

            assert!(!field.is_on_ground(&Mino::new(Piece::I, Rotate::Spawn), 4, y));
            assert!(field.is_on_ground(&Mino::new(Piece::I, Rotate::Spawn), 4, y - 1));
        }
    }

    #[test]
    fn get_block_count_in_column() {
        let mut rngs = thread_rng();
        let field = create_random_large_field(&mut rngs, 25);

        for y in 0..MAX_FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                assert_eq!(
                    field.get_block_count_in_column(x, y),
                    (0..y).filter(|&y2| field.exists_block(x, y2)).count() as u32
                );
            }
        }
    }

    #[test]
    fn clear_filled_rows() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(5..20);
            let mut field = create_random_large_field(&mut rngs, empty_minos);

            // 配列ベースのフィールドに変換
            let mut colored_field = ArrayColoredField::new(MAX_FIELD_HEIGHT);
            for y in 0..MAX_FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    colored_field.set_color(
                        x,
                        y,
                        if field.is_empty_block(x, y) {
                            ColorType::Empty
                        } else {
                            ColorType::Gray
                        },
                    );
                }
            }

            // ライン消去
            field.clear_filled_rows();
            colored_field.clear_filled_rows();

            // 確認
            for y in 0..MAX_FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    assert_eq!(
                        field.is_empty_block(x, y),
                        colored_field.get_color(x, y) == ColorType::Empty
                    );
                }
            }
        }
    }

    #[test]
    fn insert_filled_row_with_key() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(5..20);
            let mut field = create_random_large_field(&mut rngs, empty_minos);

            let freeze = field.prune(MAX_FIELD_HEIGHT);

            let key = field.clear_filled_rows_return_key();
            field.insert_filled_row_with_key(key);

            assert_eq!(&field as &dyn Field, freeze.as_ref(), "{key:#b}");
        }
    }

    #[test]
    fn insert_blank_row_with_key() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(5..20);
            let mut field = create_random_large_field(&mut rngs, empty_minos);

            for y in 0..MAX_FIELD_HEIGHT {
                if (0..FIELD_WIDTH).all(|x| field.exists_block(x, y)) {
                    for x in 0..FIELD_WIDTH {
                        field.remove_block(x, y);
                    }
                }
            }

            let freeze = field.prune(MAX_FIELD_HEIGHT);

            let key = field.clear_filled_rows_return_key();
            field.insert_blank_row_with_key(key);

            assert_eq!(&field as &dyn Field, freeze.as_ref());
        }
    }

    #[test]
    fn fill_row() {
        for y in 0..MAX_FIELD_HEIGHT {
            let mut field = field_factory::create_large_field();
            field.fill_row(y);

            for x in 0..FIELD_WIDTH {
                assert!(field.exists_block(x, y));
            }

            field.clear_filled_rows();
            assert!(field.is_empty());
        }
    }

    #[test]
    fn get_upper_y_with_4_blocks_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_large_field();
            let mut max_y = None;

            while field.get_num_of_all_blocks() != 4 {
                let x = rngs.gen_range(0..FIELD_WIDTH);
                let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);
                field.set_block(x, y);

                max_y = max_y.max(Some(y))
            }

            assert_eq!(field.get_upper_y_with_4_blocks(), max_y.unwrap());
        }
    }

    #[test]
    fn get_min_y_random() {
        // empty
        assert_eq!(field_factory::create_large_field().get_min_y(), None);

        // 10 blocks
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_large_field();
            let mut min_y: Option<u8> = None;
            for i in 0..FIELD_WIDTH {
                let x = rngs.gen_range(0..FIELD_WIDTH);
                let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);
                field.set_block(x, y);

                if let Some(min) = min_y {
                    min_y = Some(min.min(y));
                } else {
                    min_y = Some(y);
                }
            }

            assert_eq!(field.get_min_y(), min_y);
        }
    }

    #[test]
    fn contains_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(5..20);
            let init_field = create_random_large_field(&mut rngs, empty_minos);

            let mut field = init_field.prune(MAX_FIELD_HEIGHT);
            for _ in 0..100 {
                let x = rngs.gen_range(0..FIELD_WIDTH);
                let y = rngs.gen_range(0..MAX_FIELD_HEIGHT);
                field.remove_block(x, y);

                assert!(init_field.contains(field.as_ref()));
            }

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

    #[test]
    fn slide_down_one_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_large_field();
            let mut expected = field_factory::create_large_field();

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
            let mut field = create_random_large_field(&mut rngs, 30);
            let slide = rngs.gen_range(0..MAX_FIELD_HEIGHT + 1);

            let mut expected = field.clone();
            for _ in 0..slide {
                expected.slide_down_one();
            }

            field.slide_down(slide);

            assert_eq!(field, expected);
        }
    }

    #[test]
    fn slide_up_with_empty_row_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = create_random_large_field(&mut rngs, 30);

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
            let mut field = create_random_large_field(&mut rngs, 30);

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
            let mut field = create_random_large_field(&mut rngs, 30);
            let slide = rngs.gen_range(0..MAX_FIELD_HEIGHT + 1);

            let mut freeze = field.clone();
            for _ in 0..slide {
                freeze.slide_up_with_empty_row(1);
            }

            field.slide_up_with_empty_row(slide);
            assert_eq!(field, freeze, "{slide}");
        }
    }

    #[test]
    fn slide_up_with_filled_row_n_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = create_random_large_field(&mut rngs, 30);
            let slide = rngs.gen_range(0..MAX_FIELD_HEIGHT + 1);

            let mut freeze = field.clone();
            for _ in 0..slide {
                freeze.slide_up_with_filled_row(1);
            }

            field.slide_up_with_filled_row(slide);
            assert_eq!(field, freeze, "{slide}");
        }
    }

    #[test]
    fn slide_left_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let mut field = field_factory::create_large_field();
            let mut expected = field_factory::create_large_field();

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
            let mut field = field_factory::create_large_field();
            let mut expected = field_factory::create_large_field();

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
    fn invert() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(5..20);
            let init_field = create_random_large_field(&mut rngs, empty_minos);
            let mut field = init_field.clone();

            field.invert();

            for y in 0..MAX_FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    assert_ne!(field.is_empty_block(x, y), init_field.is_empty_block(x, y));
                }
            }
        }
    }

    #[test]
    fn mirror() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(3..10);
            let init_field = create_random_large_field(&mut rngs, empty_minos);

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
    fn get_min_x() {
        assert_eq!(field_factory::create_large_field().get_min_x(), None);

        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(3..10);
            let init_field = create_random_large_field(&mut rngs, empty_minos);

            let field = init_field.prune(MAX_FIELD_HEIGHT);
            let expected_min_x =
                (0..FIELD_WIDTH).find(|&x| (0..MAX_FIELD_HEIGHT).any(|y| field.exists_block(x, y)));

            assert_eq!(field.get_min_x(), expected_min_x);
        }
    }

    #[test]
    fn exists_block_in_row() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(3..10);
            let init_field = create_random_large_field(&mut rngs, empty_minos);

            for y in 0..MAX_FIELD_HEIGHT {
                let expected = (0..FIELD_WIDTH).any(|x| init_field.exists_block(x, y));

                assert_eq!(init_field.exists_block_in_row(y), expected);
            }
        }
    }

    #[test]
    fn delete_row() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            // 適度にフィールドのラインが揃うようにランダムに地形を作る
            let empty_minos = rngs.gen_range(3..10);
            let mut field = create_random_large_field(&mut rngs, empty_minos);

            let max_count = rngs.gen_range(0..MAX_FIELD_HEIGHT * 2);
            for _ in 0..max_count {
                field.fill_row(rngs.gen_range(0..MAX_FIELD_HEIGHT));
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
            let empty_minos1 = rngs.gen_range(3..10);
            let field1 = create_random_large_field(&mut rngs, empty_minos1);
            let empty_minos2 = rngs.gen_range(3..10);
            let field2 = create_random_large_field(&mut rngs, empty_minos2);

            // 期待値
            let mut expected = field_factory::create_large_field();
            for y in 0..MAX_FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    if !field1.is_empty_block(x, y) && !field2.is_empty_block(x, y) {
                        expected.set_block(x, y);
                    }
                }
            }

            let mut freeze = field1.clone();
            freeze.mask(&field2);
            assert_eq!(freeze, expected);

            let mut freeze = field2.clone();
            freeze.mask(&field1);
            assert_eq!(freeze, expected);
        }
    }

    #[test]
    fn get_using_key_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let empty_minos = rngs.gen_range(1..10);
            let field = create_random_large_field(&mut rngs, empty_minos);

            // 期待値
            let mut expected = 0;
            for y in 0..MAX_FIELD_HEIGHT {
                if (0..FIELD_WIDTH).any(|x| field.exists_block(x, y)) {
                    expected |= key_operators::get_delete_bit_key(y);
                }
            }

            assert_eq!(field.get_using_key(), expected);
        }
    }
}
