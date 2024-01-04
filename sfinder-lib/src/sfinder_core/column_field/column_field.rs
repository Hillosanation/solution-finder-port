pub trait ColumnField {
    // 指定した位置にブロックをおく
    fn set_block(&mut self, x: u8, y: u8, height: u8);

    // 指定した位置からブロックを取り除く
    fn remove_block(&mut self, x: u8, y: u8, height: u8);

    // 指定した位置にブロックがないとき true を返却
    fn is_empty(&self, x: u8, y: u8, height: u8) -> bool;

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
        (0..largest_board_count as u8).all(|index| self.get_board(index) == other.get_board(index))
    }
}

impl PartialOrd for dyn ColumnField + '_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let largest_board_count = self.get_board_count().max(other.get_board_count());

        // there isn't a way to chain a variable number of then_with together
        for index in 0..largest_board_count as u8 {
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
    use super::*;
    use rand::{thread_rng, Rng};

    // test retrieved from ColumnFieldComparatorTest.java

    #[test]
    fn cmp() {
        let mut rngs = thread_rng();

        for _ in 0..10000 {
            // same field
            let height = rngs.gen_range(1..10);
            let field1 = todo!("ColumnFieldFactory");
        }
    }
}
