use super::{
    bit_operators,
    field_constants::{BoardCount, BOARD_HEIGHT, FIELD_WIDTH, VALID_BOARD_RANGE},
};
use crate::{
    extras::hash_code::HashCode,
    sfinder_core::{mino::mino::Mino, neighbor::original_piece::OriginalPiece},
};
use dyn_clone::DynClone;
use std::fmt::Debug;

// TODO(#8): add translated documentation
// Porting note: Altered the naming convention to: no suffix for Mino, -block for xy coordinates, -piece for OriginalPiece
// Each field is split into multiple bitboards in its internal representation.
// Rather than keeping the unused bits in a board unset, it is at an unknown state and must be masked off before being shifted into VALID_BOARD_RANGE.
// THe only function in the Field interface that could alter the unused bits is put.
pub trait Field: Debug + DynClone /* + PartialOrd */ {
    fn new() -> Self
    where
        Self: Sized;

    // フィールドの最大高さを返却
    fn get_max_field_height(&self) -> u8;

    // Porting note: This function is used in conjunction with getBoard to be able to access
    // the other Field's data even when they may be different concrete types
    // 6列分のフィールドを表現するボードの個数を返却
    fn get_board_count(&self) -> BoardCount;

    // TODO(#9): Bundle coordinates with Coordinate struct
    // 指定した位置にブロックをおく
    fn set_block(&mut self, x: u8, y: u8);

    // 指定した位置からブロックを取り除く
    fn remove_block(&mut self, x: u8, y: u8);

    // Porting note: reused by FieldMemory
    fn clear_all(&mut self);

    // 指定した位置にミノの形にブロックをおく
    fn put(&mut self, mino: &Mino, x: u8, y: u8);

    // 指定した位置にピースの形にブロックをおく
    fn put_piece(&mut self, piece: &OriginalPiece) {
        self.merge(piece.get_mino_field())
    }

    // 指定した位置にミノを置くことができるとき true を返却
    fn can_put(&self, mino: &Mino, x: u8, y: u8) -> bool;

    // 指定した位置にピースをおくことができるか（足場は確認しない）
    fn can_put_piece(&self, piece: &OriginalPiece) -> bool {
        self.can_merge(piece.get_mino_field())
    }

    // 指定した位置のミノの形でブロックを消す
    fn remove(&mut self, mino: &Mino, x: u8, y: u8);

    // 指定した位置のピースの形でブロックを消す
    fn remove_piece(&mut self, piece: &OriginalPiece) {
        self.reduce(piece.get_mino_field())
    }

    // 指定した位置からミノをharddropしたとき、接着するyを返却
    fn get_y_on_harddrop(&self, mino: &Mino, x: u8, start_y: u8) -> u8 {
        let min = -mino.get_min_y() as u8;
        (min..start_y)
            .rev()
            .find(|&y| !self.can_put(mino, x, y))
            .map_or(min, |y| y + 1)
    }

    // 一番上からharddropで指定した位置を通過するとき true を返却
    fn can_reach_on_harddrop(&self, mino: &Mino, x: u8, start_y: u8) -> bool;
    /// Internal function for implementing can_reach_on_harddrop
    /// TODO(#11): find a way to make this private
    fn _can_reach_on_harddrop(
        &self,
        mino: &Mino,
        x: u8,
        start_y: u8,
        max_field_height: u8,
    ) -> bool {
        // TODO: check if masking off columns is faster than checking each y placement
        (start_y + 1..max_field_height + (-mino.get_min_y()) as u8)
            .all(|y| self.can_put(mino, x, y))
    }

    // 一番上からharddropで指定した位置を通過するとき true を返却
    fn can_reach_on_harddrop_piece(&self, piece: &OriginalPiece) -> bool {
        self.can_merge(piece.get_harddrop_collider())
    }

    // 指定した位置にブロックがないとき true を返却
    fn is_empty_block(&self, x: u8, y: u8) -> bool;

    // 指定した位置にブロックがあるとき true を返却
    fn exists_block(&self, x: u8, y: u8) -> bool {
        !self.is_empty_block(x, y)
    }

    // Porting note: replaces existsAbove
    // y行以上にブロックがあるとき true を返却（y行上のブロックも対象に含む）
    fn exists_above_row(&self, y: u8) -> bool;

    // フィールド内に1つもブロックがないとき true を返却
    fn is_empty(&self) -> bool;

    // x列上で、maxY行より下がすべてブロックで埋まっているとき true を返却
    fn is_filled_in_column(&self, x: u8, max_y: u8) -> bool;

    // x列とその左の列の間が壁（隙間がない）とき true を返却。1 <= xであること
    fn is_wall_between_left(&self, x: u8, max_y: u8) -> bool;

    // 指定した位置のミノが接着できるとき true を返却
    fn is_on_ground(&self, mino: &Mino, x: u8, y: u8) -> bool {
        y <= -mino.get_min_y() as u8 || !self.can_put(mino, x, y - 1)
    }

    // Porting note: replaces getBlockCountBelowOnX, altered name to match is_filled_in_column
    // x列上で、maxY行より下にあるブロックの個数を返却 （maxY行上のブロックは対象に含まない）
    fn get_block_count_in_column(&self, x: u8, max_y: u8) -> u32;

    // Porting note: replaces getBlockCountBelowOnY, altered name to match is_filled_in_column
    // y行上にあるブロックの個数を返却
    fn get_block_count_in_row(&self, y: u8) -> u32;

    // y行上にブロックがあるとき true を返却
    fn exists_block_in_row(&self, y: u8) -> bool;

    // すべてのブロックの個数を返却
    fn get_num_of_all_blocks(&self) -> u32;

    // Porting note: replaces clearLine
    // ブロックがそろった行を削除し、削除した行数を返却
    fn clear_filled_rows(&mut self) -> u32 {
        self.clear_filled_rows_return_key().count_ones()
    }

    // TODO(#7): wrap in newtype for functions that return a Key representing the cleared rows

    // Porting note: replaces clearLineReturnKey
    // ブロックがそろった行を削除し、削除した行を表すマスクを返却
    fn clear_filled_rows_return_key(&mut self) -> u64;

    // ブロックが揃っている行を表すマスクを返却
    fn get_filled_rows_key(&self) -> u64;

    // ブロックがある行を表すマスクを返却
    fn get_using_key(&self) -> u64;

    // ブロックがそろった行を埋めた状態で復元する
    // deleteKeyは以下のビット位置に、対応する行が揃っているときフラグをたてる
    //       5.******** 最上位
    //       4.********
    //       39********
    //       28********
    //       17********
    // 最下位 06********
    fn insert_filled_row_with_key(&mut self, delete_key: u64);

    // ブロックがそろった行を空白の状態で復元する
    fn insert_blank_row_with_key(&mut self, delete_key: u64);

    // 指定された行を削除する
    fn delete_rows_with_key(&mut self, delete_key: u64);

    // y行上をブロックで塗りつぶす
    fn fill_row(&mut self, y: u8);

    // 指定した番号の6列分のフィールドを表現するボードを返却（0が最下層）
    fn get_board(&self, index: u8) -> u64;

    // Porting note: replaces freeze, the other freeze function is replaced by clone
    // Prunes the field if a smaller field can contain the specified height
    // 現在のフィールドのコピーを返却  // 拡張はしない
    fn prune(&self, max_height: u8) -> Box<dyn Field>;

    // 現在のフィールドのコピーを返却  // 現在の地形と同じ高さのフィールドをコピー

    // 指定したフィールドのブロックを重ね合せる
    fn merge(&mut self, other: &dyn Field);

    // 指定したフィールドのブロックが重ならないときfalseを返却
    fn can_merge(&self, other: &dyn Field) -> bool;

    // 指定したフィールドのブロックを取り除く
    fn reduce(&mut self, other: &dyn Field);

    // フィールド内には必ず4ブロックだけ存在している前提のもと、最も高い位置にあるブロックのY座標を取得
    fn get_upper_y_with_4_blocks(&self) -> u8;

    // Porting note: use Option instead
    // 最も小さいx座標を取得。ブロックが存在しないとき -1 を返却
    fn get_min_x(&self) -> Option<u8>;

    // Porting note: replaces getLowerY
    // 最も低い位置にあるブロックのY座標を取得
    fn get_min_y(&self) -> Option<u8>;

    // フィールドを左に指定したブロック分スライドさせる
    fn slide_left(&mut self, slide: u8);

    // フィールドを右に指定したブロック分スライドさせる
    fn slide_right(&mut self, slide: u8);

    // Porting note: replaces slideDown to remove polymorphism
    // フィールドを下に1段スライドさせる
    fn slide_down_one(&mut self);

    // フィールドを下に指定したブロック分スライドさせる
    fn slide_down(&mut self, slide: u8);

    // フィールドを上に指定したブロック分スライドさせる。空のラインを追加する
    fn slide_up_with_empty_row(&mut self, slide: u8);

    // フィールドを上に指定したブロック分スライドさせる。ブロックで埋まったラインを追加する
    fn slide_up_with_filled_row(&mut self, slide: u8);

    // childの全てのブロックが、フィールド内の同じ位置にブロックがあればtrue
    fn contains(&self, child: &dyn Field) -> bool;

    // Porting note: replaces inverse
    // ブロックと空白を反転させる
    fn invert(&mut self);

    // ブロックが左右に反転させる
    fn mirror(&mut self);

    // `maskField` のブロックだけマスクする
    fn mask(&mut self, mask_field: &dyn Field);
}

// Porting note: This collection of helper functions is distinct from the FieldHelper class, which is dropped.
pub trait FieldHelper {
    /// TODO(#11): is_in should be the only function that should be exposed
    fn is_in(mino: &Mino, x: i8, y: i8) -> bool {
        let min_x = x + mino.get_min_x();
        let max_x = x + mino.get_max_x();
        let min_y = y + mino.get_min_y();

        0 <= min_x && max_x < FIELD_WIDTH as i8 && 0 <= min_y
    }

    // returns a mask of the rows above y
    #[inline]
    fn get_valid_mask(y: u8) -> u64 {
        bit_operators::board_shl(VALID_BOARD_RANGE, y)
    }

    #[inline]
    fn extract_delete_key(delete_key: u64, index: u8) -> u64 {
        assert!(index <= 4);
        (delete_key >> index) & bit_operators::get_column_one_row_below_y(BOARD_HEIGHT)
    }

    #[inline]
    // used for boards that are not the bottommost
    fn create_upper_board(
        board_low: u64,
        board_high: u64,
        delete_row: u8,
        delete_key: u64,
        row_fill_fn: fn(u64, u64) -> u64,
    ) -> u64 {
        let left_row = BOARD_HEIGHT - delete_row;
        row_fill_fn(
            bit_operators::board_shl(board_high, delete_row)
                | bit_operators::board_shr(board_low & VALID_BOARD_RANGE, left_row),
            delete_key,
        )
    }

    #[inline]
    fn create_bottom_board(
        board_bottom: u64,
        delete_row: u8,
        delete_key: u64,
        row_fill_fn: fn(u64, u64) -> u64,
    ) -> u64 {
        let left_row = BOARD_HEIGHT - delete_row;
        row_fill_fn(
            board_bottom & bit_operators::get_row_mask_below_y(left_row),
            delete_key,
        )
    }
}

impl FieldHelper for dyn Field {}

impl std::cmp::PartialEq for dyn Field + '_ {
    fn eq(&self, other: &Self) -> bool {
        let largest_board_count = self.get_board_count().max(other.get_board_count());
        (0..largest_board_count as u8).all(|index| self.get_board(index) == other.get_board(index))
    }
}

impl HashCode for dyn Field + '_ {
    type Output = u32;

    fn hash_code(&self) -> Self::Output {
        (0..self.get_board_count() as u8)
            .map(|index| {
                let board = self.get_board(index);
                (board ^ (board >> 32)) as u32
            })
            .fold(0, |acc, partial| 31 * acc + partial)
    }
}

impl std::cmp::PartialOrd for dyn Field + '_ {
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

dyn_clone::clone_trait_object!(Field);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_core::field::key_operators;
    use rand::{thread_rng, Rng};

    fn legacy_extract_delete_key(delete_key: u64, index: u8) -> u64 {
        assert!(index <= 4);
        (delete_key & (0x4010040100401 << index)) >> index
    }

    #[test]
    fn extract_delete_key_agrees() {
        let mut rngs = thread_rng();

        for _ in 0..1000 {
            let column_key = rngs.gen_range(0..1 << 24);
            let delete_key = key_operators::to_bit_key(column_key);

            // println!("testing {delete_key:060b}");

            for index in 0..4 {
                assert_eq!(
                    legacy_extract_delete_key(delete_key, index),
                    <dyn Field>::extract_delete_key(delete_key, index),
                    "delete_key: {delete_key}",
                )
            }
        }
    }
}
