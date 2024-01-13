use std::fmt::Debug;

pub trait ColumnField: Debug {
    // 指定した位置にブロックをおく
    fn set_block(&mut self, x: u8, y: u8, height: u8);

    // 指定した位置からブロックを取り除く
    fn remove_block(&mut self, x: u8, y: u8, height: u8);

    // 指定した位置にブロックがないとき true を返却
    fn is_empty_block(&self, x: u8, y: u8, height: u8) -> bool;

    // 指定した番号の6列分のフィールドを表現するボードを返却（0が最下層）
    fn get_board(&self, index: u8) -> u64;

    // 6列分のフィールドを表現するボードの個数を返却
    fn get_board_count(&self) -> u8;

    // すべてのブロックの個数を返却
    fn get_num_of_all_blocks(&self) -> u32;

    // 指定したフィールドのブロックを重ね合せる
    fn merge(&mut self, field: &dyn ColumnField);

    // 指定したフィールドのブロックを取り除く
    fn reduce(&mut self, field: &dyn ColumnField);

    // 指定したフィールドのブロックが重ならないときfalseを返却
    fn can_merge(&self, field: &dyn ColumnField) -> bool;

    // 現在のフィールドのコピーを返却
    fn prune(&self) -> Box<dyn ColumnField>;
}

// same implementations as Field

impl PartialEq for dyn ColumnField + '_ {
    fn eq(&self, other: &Self) -> bool {
        let largest_board_count = self.get_board_count().max(other.get_board_count());
        (0..largest_board_count).all(|index| self.get_board(index) == other.get_board(index))
    }
}

impl PartialOrd for dyn ColumnField + '_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let largest_board_count = self.get_board_count().max(other.get_board_count());

        // there isn't a way to chain a variable number of then_with together
        for index in 0..largest_board_count {
            // takes advantage of the fact that indexing out of bounds of the boards for a Field returns 0
            let cmp = self.get_board(index).cmp(&other.get_board(index));
            if cmp != std::cmp::Ordering::Equal {
                return Some(cmp);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        extras::test_functions::assert_partialord_symmetric,
        sfinder_core::{column_field::column_field_factory, field::field_constants::FIELD_WIDTH},
    };

    use super::*;
    use rand::{thread_rng, Rng};

    // test retrieved from ColumnFieldComparatorTest.java

    #[test]
    fn cmp() {
        let mut rngs = thread_rng();

        // Porting note: this test relies of the fact that set_block is able to overflow?
        for _ in 0..10000 {
            // same field
            let height = rngs.gen_range(1..FIELD_WIDTH);
            let mut field1 = column_field_factory::create_small_field();
            let mut field2 = column_field_factory::create_small_field();

            for _ in 0..rngs.gen_range(1..15) {
                let x = rngs.gen_range(0..FIELD_WIDTH);
                let y = rngs.gen_range(0..height);
                field1.set_block(x, y, height);
                field2.set_block(x, y, height);
            }

            assert_eq!(field1, field2);
            assert_eq!(field2, field1);

            // 1block different field
            let x = rngs.gen_range(0..FIELD_WIDTH);
            let y = rngs.gen_range(0..height);
            if field1.is_empty_block(x, y, height) {
                field1.set_block(x, y, height);
            } else {
                field1.remove_block(x, y, height);
            }

            assert_partialord_symmetric(field1, field2);
        }
    }
}
