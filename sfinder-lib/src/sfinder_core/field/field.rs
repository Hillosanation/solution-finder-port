use crate::sfinder_core::mino::mino::Mino;

// TODO: add translated documentation
// Porting note: Altered the naming convention to: no suffix for Mino, -block for xy coordinates, -piece for OriginalPiece
pub trait Field /* : PartialOrd */ {
    // フィールドの最大高さを返却
    fn get_max_field_height(&self) -> u8;

    // Porting note: This function is used in conjunction with getBoard to be able to access
    // the other Field's data even when they may be different concrete types
    // 6列分のフィールドを表現するボードの個数を返却
    fn get_board_count(&self) -> u8;

    // TODO: Bundle coordinates with Coordinate struct
    // 指定した位置にブロックをおく
    fn set_block(&mut self, x: u8, y: u8);

    // 指定した位置からブロックを取り除く
    fn remove_block(&mut self, x: u8, y: u8);

    // 指定した位置にミノの形にブロックをおく
    fn put(&mut self, mino: Mino, x: u8, y: u8);

    // 指定した位置にピースの形にブロックをおく
    fn put_piece(&mut self, piece: OriginalPiece);

    // 指定した位置にミノを置くことができるとき true を返却
    fn can_put(&self, mino: Mino, x: u8, y: u8) -> bool;

    // 指定した位置にピースをおくことができるか（足場は確認しない）
    fn can_put_piece(&self, piece: OriginalPiece) -> bool;

    // 指定した位置のミノの形でブロックを消す
    fn remove(&mut self, mino: Mino, x: u8, y: u8);

    // 指定した位置のピースの形でブロックを消す
    fn remove_piece(&mut self, piece: OriginalPiece);

    // 指定した位置からミノをharddropしたとき、接着するyを返却
    fn get_y_on_harddrop(&self, mino: Mino, x: u8, y: u8) -> u8;

    // 一番上からharddropで指定した位置を通過するとき true を返却
    fn can_reach_on_harddrop(&self, mino: Mino, x: u8, y: u8) -> bool;

    // 一番上からharddropで指定した位置を通過するとき true を返却
    fn can_reach_on_harddrop_piece(&self, piece: OriginalPiece) -> bool;

    // 指定した位置にブロックがないとき true を返却
    fn is_empty_block(&self, x: u8, y: u8) -> bool;

    // 指定した位置にブロックがあるとき true を返却
    fn exists_block(&self, x: u8, y: u8) -> bool;

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
    fn is_on_ground(&self, mino: Mino, x: u8, y: u8) -> bool;

    // Porting note: replaces getBlockCountBelowOnX, altered name to match is_filled_in_column
    // x列上で、maxY行より下にあるブロックの個数を返却 （maxY行上のブロックは対象に含まない）
    fn get_block_count_in_column(&self, x: u8, max_y: u8) -> u8;

    // Porting note: replaces getBlockCountBelowOnY, altered name to match is_filled_in_column
    // y行上にあるブロックの個数を返却
    fn get_block_count_in_row(&self, y: u8) -> u8;

    // y行上にブロックがあるとき true を返却
    fn exists_block_in_row(&self, y: u8) -> bool;

    // すべてのブロックの個数を返却
    fn get_num_of_all_blocks(&self) -> u8;

    // Porting note: replaces clearLine
    // ブロックがそろった行を削除し、削除した行数を返却
    fn clear_filled_rows(&mut self) -> u8;

    // TODO: wrap in newtype for functions that return a Key representing the cleared rows

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
    fn insert_blank_row_with_key(&mut self, delete_key: u64);

    // ブロックがそろった行を空白の状態で復元する
    fn insert_filled_row_with_key(&mut self, delete_key: u64);

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

    // 最も低い位置にあるブロックのY座標を取得
    fn get_lower_y(&self) -> u8;

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
    fn slide_up_with_filled_row(&mut self, slide: u8);

    // フィールドを上に指定したブロック分スライドさせる。ブロックで埋まったラインを追加する
    fn slide_up_with_empty_row(&mut self, slide: u8);

    // Porting note: use Option instead
    // 最も小さいx座標を取得。ブロックが存在しないとき -1 を返却
    fn get_min_x(&self) -> Option<u8>;

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

pub trait FieldHelper {
    fn is_in(mino: Mino, x: i8, y: i8) -> bool {
        let min_x = x + mino.get_min_x();
        let max_x = x + mino.get_max_x();
        let min_y = y + mino.get_min_y();

        0 <= min_x && max_x < 10 && 0 <= min_y
    }
}

impl FieldHelper for dyn Field {}

// temp struct
pub struct OriginalPiece {}
