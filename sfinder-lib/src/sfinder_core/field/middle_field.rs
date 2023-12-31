use super::{
    bit_operators,
    field::{BoardCount, Field, FieldHelper, FIELD_WIDTH, VALID_BOARD_RANGE},
    key_operators, long_board_map,
    small_field::SmallField,
};
use crate::sfinder_core::mino::mino::Mino;
use std::fmt::Debug;

const BOARD_HEIGHT: u8 = 6;
const FIELD_ROW_BORDER_Y: u8 = BOARD_HEIGHT;
const MAX_FIELD_HEIGHT: u8 = 12;

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

    fn combine_keys(low: u64, high: u64) -> u64 {
        low | high << 1
    }

    fn delete_row(&mut self, delete_key_low: u64, delete_key_high: u64) {
        let new_x_board_low = long_board_map::delete_row(self.0, delete_key_low);
        let new_x_board_high = long_board_map::delete_row(self.1, delete_key_high);

        let delete_row_low = delete_key_low.count_ones() as u8;

        self.0 = (new_x_board_low
            | <dyn Field>::board_shl(new_x_board_high, BOARD_HEIGHT - delete_row_low))
            & VALID_BOARD_RANGE;
        self.1 = <dyn Field>::board_shr(new_x_board_high, delete_row_low);
    }

    fn clear_all(&mut self) {
        self.0 = 0;
        self.1 = 0;
    }

    fn fill_all(&mut self) {
        self.0 = VALID_BOARD_RANGE;
        self.1 = VALID_BOARD_RANGE;
    }
}

impl Field for MiddleField {
    fn get_max_field_height(&self) -> u8 {
        MAX_FIELD_HEIGHT
    }

    fn get_board_count(&self) -> BoardCount {
        BoardCount::Middle
    }

    // Porting note: flipped the conditional to avoid using exclusive ranges

    fn set_block(&mut self, x: u8, y: u8) {
        match y {
            FIELD_ROW_BORDER_Y.. => {
                self.1 |= <dyn Field>::get_x_mask(x, y - FIELD_ROW_BORDER_Y);
            }
            _ => {
                self.0 |= <dyn Field>::get_x_mask(x, y);
            }
        }
    }

    fn remove_block(&mut self, x: u8, y: u8) {
        match y {
            FIELD_ROW_BORDER_Y.. => {
                self.1 &= !<dyn Field>::get_x_mask(x, y - FIELD_ROW_BORDER_Y);
            }
            _ => {
                self.0 &= !<dyn Field>::get_x_mask(x, y);
            }
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
        match y {
            FIELD_ROW_BORDER_Y.. => {
                self.1 & <dyn Field>::get_x_mask(x, y - FIELD_ROW_BORDER_Y) == 0
            }
            _ => self.0 & <dyn Field>::get_x_mask(x, y) == 0,
        }
    }

    fn exists_above_row(&self, y: u8) -> bool {
        match y {
            MAX_FIELD_HEIGHT.. => false,
            FIELD_ROW_BORDER_Y.. => {
                // Highで完結
                self.1 & <dyn Field>::get_valid_mask(y - FIELD_ROW_BORDER_Y) != 0
            }
            _ => {
                // すべて必要
                // Highのチェック
                self.1 != 0
                // Lowのチェック
                || self.0 & <dyn Field>::get_valid_mask(y) != 0
            }
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
        match y {
            FIELD_ROW_BORDER_Y.. => {
                (self.1 & <dyn Field>::get_row_mask(y - FIELD_ROW_BORDER_Y)).count_ones()
            }
            _ => (self.0 & <dyn Field>::get_row_mask(y)).count_ones(),
        }
    }

    fn exists_block_in_row(&self, y: u8) -> bool {
        match y {
            FIELD_ROW_BORDER_Y.. => self.1 & <dyn Field>::get_row_mask(y - FIELD_ROW_BORDER_Y) != 0,
            _ => self.0 & <dyn Field>::get_row_mask(y) != 0,
        }
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
        let delete_key_low = <dyn Field>::extract_delete_key(delete_key, 0);
        let delete_row_low = delete_key_low.count_ones() as u8;
        let left_row_low_y = FIELD_ROW_BORDER_Y - delete_row_low;
        let new_x_board_low = long_board_map::insert_filled_row(
            self.0 & bit_operators::get_row_mask_below_y(left_row_low_y),
            delete_key_low,
        );

        let delete_key_high = <dyn Field>::extract_delete_key(delete_key, 1);
        let new_x_board_high = long_board_map::insert_filled_row(
            self.1 << (FIELD_WIDTH * delete_row_low)
                | ((self.0 & bit_operators::get_row_mask_above_y(left_row_low_y))
                    >> (FIELD_WIDTH * left_row_low_y)),
            delete_key_high,
        );

        self.0 = new_x_board_low;
        self.1 = new_x_board_high & VALID_BOARD_RANGE;
    }

    fn insert_blank_row_with_key(&mut self, delete_key: u64) {
        let delete_key_low = <dyn Field>::extract_delete_key(delete_key, 0);
        let delete_row_low = delete_key_low.count_ones() as u8;
        let left_row_low_y = FIELD_ROW_BORDER_Y - delete_row_low;
        let new_x_board_low = long_board_map::insert_blank_row(
            self.0 & bit_operators::get_row_mask_below_y(left_row_low_y),
            delete_key_low,
        );

        let delete_key_high = <dyn Field>::extract_delete_key(delete_key, 1);
        let new_x_board_high = long_board_map::insert_blank_row(
            self.1 << (FIELD_WIDTH * delete_row_low)
                | ((self.0 & bit_operators::get_row_mask_above_y(left_row_low_y))
                    >> (FIELD_WIDTH * left_row_low_y)),
            delete_key_high,
        );

        self.0 = new_x_board_low;
        self.1 = new_x_board_high & VALID_BOARD_RANGE;
    }

    fn delete_rows_with_key(&mut self, delete_key: u64) {
        self.delete_row(
            <dyn Field>::extract_delete_key(delete_key, 0),
            <dyn Field>::extract_delete_key(delete_key, 1),
        );
    }

    fn fill_row(&mut self, y: u8) {
        self.0 |= <dyn Field>::get_row_mask(match y {
            FIELD_ROW_BORDER_Y.. => y - FIELD_ROW_BORDER_Y,
            _ => y,
        })
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
            ..=6 => Box::new(SmallField::from(self.0)),
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

        if let Some(min_y) = bit_operators::try_get_lowest_y(self.1) {
            min_y + FIELD_ROW_BORDER_Y
        } else {
            // すべてxBoardLowにある
            bit_operators::get_lowest_y(self.0)
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
        self.0 = (self.0 >> FIELD_WIDTH | <dyn Field>::board_shl(self.1, BOARD_HEIGHT - 1))
            & VALID_BOARD_RANGE;
        self.1 = self.1 >> FIELD_WIDTH;
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

                self.0 & child_board_low == child_board_low && self.1 & child_board_high == 0
            }
            BoardCount::Large => {
                let child_board_low = child.get_board(0);
                let child_board_high = child.get_board(1);

                self.0 & child_board_low == child_board_low
                    && self.1 & child_board_high == child_board_high
                    && self.1 & child.get_board(2) == 0
                    && self.1 & child.get_board(3) == 0
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
        write!(f, "MiddleField {:#060b} {:#060b}", self.0, self.1)
    }
}
