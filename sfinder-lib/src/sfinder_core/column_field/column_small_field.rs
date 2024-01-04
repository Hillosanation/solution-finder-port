use super::column_field::ColumnField;
use crate::extras::hash_code::HashCode;

/**
 * heightの値はメモリ節約のためインスタンス内で保持しない (ただしheight <= 10を想定)
 * 必要な場合はインスタンス外で記録しておくこと
 * 幅は最大6
 */
// Porting note: to save space, the actual height of the ColumnField is not stored, thus indexing requires you to pass in the height.
pub struct ColumnSmallField(u64);

impl ColumnSmallField {
    pub fn new() -> Self {
        Self(0)
    }
}

impl From<u64> for ColumnSmallField {
    fn from(board: u64) -> Self {
        Self(board)
    }
}

const fn get_y_mask(x: u8, y: u8, height: u8) -> u64 {
    1 << (y + height * x)
}

impl ColumnField for ColumnSmallField {
    fn set_block(&mut self, x: u8, y: u8, height: u8) {
        self.0 |= get_y_mask(x, y, height);
    }

    fn remove_block(&mut self, x: u8, y: u8, height: u8) {
        self.0 &= !get_y_mask(x, y, height);
    }

    fn is_empty(&self, x: u8, y: u8, height: u8) -> bool {
        self.0 & get_y_mask(x, y, height) == 0
    }

    fn get_board(&self, index: u8) -> u64 {
        match index {
            0 => self.0,
            _ => 0,
        }
    }

    fn get_board_count(&self) -> u8 {
        1
    }

    fn get_num_of_all_blocks(&self) -> u32 {
        self.0.count_ones()
    }

    fn merge(&mut self, other: &dyn ColumnField) {
        self.0 |= other.get_board(0);
    }

    fn reduce(&mut self, other: &dyn ColumnField) {
        self.0 &= !other.get_board(0);
    }

    fn can_merge(&self, other: &dyn ColumnField) -> bool {
        self.0 & other.get_board(0) == 0
    }

    fn prune(&self) -> Box<dyn ColumnField> {
        Box::new(Self(self.0))
    }
}

impl PartialEq for ColumnSmallField {
    fn eq(&self, other: &Self) -> bool {
        self as &dyn ColumnField == other as &_
    }
}

impl HashCode for ColumnSmallField {
    type Output = u64;

    fn hash_code(&self) -> u64 {
        self.0 ^ (self.0 >> 32)
    }
}

impl std::fmt::Debug for ColumnSmallField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ColumnSmallField")
            .field("board", &self.0)
            .finish()
    }
}
