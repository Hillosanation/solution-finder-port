// Porting note: iirc these are transforming between ordered/unordered operations
// TODO: is consuming the lists in the functions better? It usually isn't used afterwards, and it takes up some memory

use crate::{
    common::datastore::{
        full_operation_with_key::FullOperationWithKey,
        mino_operation_with_key::MinoOperationWithKey, operation::Operation,
        operations::Operations, simple_operation::SimpleOperation,
    },
    sfinder_core::{
        field::{field::Field, field_factory, key_operators},
        mino::{mino::Mino, mino_factory::MinoFactory},
    },
};

// List<OperationWithKey>に変換する。正しく組み立てられるかはチェックしない
pub fn parse_to_operation_with_keys<'a, O: Operation<u8>>(
    field_origin: &'a dyn Field,
    operations: &'a Operations<O>,
    mino_factory: &'a MinoFactory,
    height: u8,
) -> Vec<impl MinoOperationWithKey + 'a> {
    let mut keys = Vec::with_capacity(operations.get_operations().len());

    let mut field = field_origin.prune(height);
    for operation in operations.get_operations() {
        let mino = mino_factory.get(operation.get_piece(), operation.get_rotate());
        let delete_key = field.clear_filled_rows_return_key();
        let x = operation.get_x();
        let y = operation.get_y();
        keys.push(to_full_operation_with_key(mino, x, y, delete_key, height));

        // 次のフィールドを作成
        field.put(mino, x, y);
        field.insert_filled_row_with_key(delete_key);
    }

    keys
}

pub fn to_full_operation_with_key(
    mino: &Mino,
    x: u8,
    y: u8,
    delete_key: u64,
    height: u8,
) -> FullOperationWithKey {
    // 一番上と一番下のy座標を抽出
    let mut vanilla = field_factory::create_field(height);
    vanilla.put(mino, x, y);
    vanilla.insert_blank_row_with_key(delete_key);
    assert!(
        vanilla.get_num_of_all_blocks() == 4,
        "The blocks don't fit in the field"
    );
    let lower_y = vanilla.get_min_y().unwrap();
    let upper_y = vanilla.get_upper_y_with_4_blocks();

    // 接着に必ず消去されている必要がある行を抽出
    // TODO: clean up the bit twiddling here
    let above_lower_y = key_operators::get_mask_for_key_above_y(lower_y);
    let below_upper_y = key_operators::get_mask_for_key_below_y(upper_y + 1);
    let key_line = above_lower_y & below_upper_y;
    let need_deleted_key = delete_key & key_line;
    let using_key = key_line & !need_deleted_key;

    // 操作・消去されている必要がある行をセットで記録
    FullOperationWithKey::new_with_lower_y(mino, x, need_deleted_key, using_key, lower_y)
}

// List<Operation>に変換する。正しく組み立てられるかはチェックしない
// operationWithKeysは組み立てられる順番に並んでいること
// 初めにライン消去を行う
pub fn parse_to_operations(
    field_origin: &dyn Field,
    operation_with_keys: &[impl MinoOperationWithKey],
    height: u8,
) -> Operations<impl Operation<u8>> {
    let mut operations = Vec::with_capacity(operation_with_keys.len());

    let mut field = field_origin.prune(height);
    for operation_with_key in operation_with_keys {
        let delete_key = field.clear_filled_rows_return_key();

        // すでに下のラインが消えているときは、その分スライドさせる
        let original_y = operation_with_key.get_y();
        let deleted_lines =
            (key_operators::get_mask_for_key_below_y(original_y) & delete_key).count_ones() as u8;

        let mino = operation_with_key.get_mino();
        let x = operation_with_key.get_x();
        let y = original_y - deleted_lines;

        operations.push(SimpleOperation::new(
            mino.get_piece(),
            mino.get_rotate(),
            x,
            y,
        ));

        field.put(mino, x, y);
        field.insert_filled_row_with_key(delete_key);
    }

    Operations::from_vec(operations)
}

pub fn parse_to_block_field() -> ! {
    todo!("BlockFIeld");
}

pub fn parse_to_field(
    operation_with_keys: &[impl MinoOperationWithKey],
    height: u8,
) -> Box<dyn Field> {
    let mut field = field_factory::create_field(height);
    for operation in operation_with_keys {
        let mut piece_field = field_factory::create_field(height);
        piece_field.put(operation.get_mino(), operation.get_x(), operation.get_y());
        piece_field.insert_blank_row_with_key(operation.get_need_deleted_key());

        field.merge(piece_field.as_ref());
    }
    field
}

// 最も低いブロックのy座標を取得
pub fn get_min_y(mino_factory: &MinoFactory, operations_list: &[impl Operation<u8>]) -> Option<u8> {
    operations_list
        .iter()
        .map(|operation| {
            let mino = mino_factory.get(operation.get_piece(), operation.get_rotate());
            u8::try_from(operation.get_y() as i8 + mino.get_min_y()).unwrap()
        })
        .min()
}

// 最も高いブロックのy座標を取得
pub fn get_max_y(mino_factory: &MinoFactory, operations_list: &[impl Operation<u8>]) -> Option<u8> {
    operations_list
        .iter()
        .map(|operation| {
            let mino = mino_factory.get(operation.get_piece(), operation.get_rotate());
            u8::try_from(operation.get_y() as i8 + mino.get_max_y()).unwrap()
        })
        .max()
}
