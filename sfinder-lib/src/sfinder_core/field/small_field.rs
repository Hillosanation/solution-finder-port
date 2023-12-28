use super::{
    field::{BoardCount, Field, FieldHelper, FIELD_WIDTH, VALID_BOARD_RANGE},
    key_operators, long_board_map,
};
use crate::{
    extras::hash_code::HashCode,
    sfinder_core::{
        field::bit_operators, mino::mino::Mino, neighbor::original_piece::OriginalPiece,
    },
};
use std::fmt::Debug;

const MAX_FIELD_HEIGHT: u8 = 6;

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

impl Field for SmallField {
    fn get_max_field_height(&self) -> u8 {
        MAX_FIELD_HEIGHT
    }

    fn get_board_count(&self) -> BoardCount {
        BoardCount::Small
    }

    fn set_block(&mut self, x: u8, y: u8) {
        self.0 |= <dyn Field>::get_x_mask(x, y);
    }

    fn remove_block(&mut self, x: u8, y: u8) {
        self.0 &= !<dyn Field>::get_x_mask(x, y);
    }

    fn put(&mut self, mino: &Mino, x: u8, y: u8) {
        self.0 |= mino.get_mask(x, y as i8);
    }

    fn put_piece(&mut self, piece: OriginalPiece) {
        self.merge(piece.get_mino_field())
    }

    fn can_put(&self, mino: &Mino, x: u8, y: u8) -> bool {
        MAX_FIELD_HEIGHT + 2 <= y || self.0 & mino.get_mask(x, y as i8) == 0
    }

    fn can_put_piece(&self, piece: OriginalPiece) -> bool {
        self.can_merge(piece.get_mino_field())
    }

    fn remove(&mut self, mino: &Mino, x: u8, y: u8) {
        self.0 &= !mino.get_mask(x, y as i8);
    }

    fn remove_piece(&mut self, piece: OriginalPiece) {
        self.reduce(piece.get_mino_field())
    }

    fn get_y_on_harddrop(&self, mino: &Mino, x: u8, start_y: u8) -> u8 {
        let min = -mino.get_min_y() as u8;
        (min..start_y)
            .rev()
            .find(|&y| !self.can_put(mino, x, y))
            .map(|y| y + 1)
            .unwrap_or(min)
    }

    fn can_reach_on_harddrop(&self, mino: &Mino, x: u8, start_y: u8) -> bool {
        (start_y + 1..MAX_FIELD_HEIGHT - mino.get_min_y() as u8).all(|y| self.can_put(mino, x, y))
    }

    fn can_reach_on_harddrop_piece(&self, piece: OriginalPiece) -> bool {
        self.can_merge(piece.get_harddrop_collider())
    }

    fn is_empty_block(&self, x: u8, y: u8) -> bool {
        self.0 & <dyn Field>::get_x_mask(x, y) == 0
    }

    fn exists_block(&self, x: u8, y: u8) -> bool {
        !self.is_empty_block(x, y)
    }

    fn exists_above_row(&self, y: u8) -> bool {
        let mask = VALID_BOARD_RANGE << (y * FIELD_WIDTH);
        y < MAX_FIELD_HEIGHT && (self.0 & mask) != 0
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn is_filled_in_column(&self, x: u8, max_y: u8) -> bool {
        if max_y == 0 {
            return true;
        }

        let column = bit_operators::get_column_one_row_below_y(max_y) << x;
        (!self.0 & column) == !0
    }

    fn is_wall_between_left(&self, x: u8, max_y: u8) -> bool {
        bit_operators::is_wall_between_left(x, max_y, self.0)
    }

    fn is_on_ground(&self, mino: &Mino, x: u8, y: u8) -> bool {
        y <= -mino.get_min_y() as u8 || !self.can_put(mino, x, y - 1)
    }

    fn get_block_count_in_column(&self, x: u8, max_y: u8) -> u32 {
        let column = bit_operators::get_column_one_row_below_y(max_y) << x;
        (self.0 & column).count_ones()
    }

    fn get_block_count_in_row(&self, y: u8) -> u32 {
        (self.0 & 0x3ff << (y * FIELD_WIDTH)).count_ones()
    }

    fn exists_block_in_row(&self, y: u8) -> bool {
        (self.0 & 0x3ff << (y * FIELD_WIDTH)) != 0
    }

    fn get_num_of_all_blocks(&self) -> u32 {
        self.0.count_ones()
    }

    fn clear_filled_rows(&mut self) -> u32 {
        self.clear_filled_rows_return_key().count_ones()
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
        self.0 |= <dyn Field>::get_row_mask(y);
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

        // 下から順に3bit分、オフする
        let mut board = self.0;
        board = board & (board - 1);
        board = board & (board - 1);
        board = board & (board - 1);

        // find the y coordinate of the most significant bit
        bit_operators::bit_to_y(board)
    }

    fn get_min_x(&self) -> Option<u8> {
        (!self.is_empty()).then(|| {
            // Porting note: refactor this since it's used multiple times
            let mut board = self.0;
            board = board | (board >> 20);
            board = board | (board >> 20);
            board = board | (board >> 10);

            bit_operators::bit_to_x(key_operators::extract_lower_bit(board))
        })
    }

    fn get_min_y(&self) -> Option<u8> {
        (!self.is_empty()).then(|| bit_operators::bit_to_y(key_operators::get_lowest_bit(self.0)))
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
        self.0 <<= slide * FIELD_WIDTH;
    }

    fn slide_up_with_empty_row(&mut self, slide: u8) {
        let count = slide * FIELD_WIDTH;
        self.0 = (self.0 << count) | ((1 << count) - 1);
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

impl HashCode for SmallField {
    type Output = u64;

    fn hash_code(&self) -> Self::Output {
        self.0 ^ self.0 >> 32
    }
}

impl Debug for SmallField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#060x}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_all_pieces<'a>(field_height: u8) -> Vec<OriginalPiece<'a>> {
        todo!("OriginalPiece");
    }
}
