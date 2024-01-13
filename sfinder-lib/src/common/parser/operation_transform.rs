//! Porting note: iirc these are transforming between ordered/unordered operations
//! TODO: is consuming the lists in the functions better? It usually isn't used afterwards, and it takes up some memory

use crate::{
    common::datastore::{
        block_field::BlockField, full_operation_with_key::FullOperationWithKey,
        mino_operation_with_key::MinoOperationWithKey, operation::Operation,
        operations::Operations, simple_operation::SimpleOperation,
    },
    sfinder_core::{
        field::{field::Field, field_factory, key_operators},
        mino::{mino::Mino, mino_factory::MinoFactory},
    },
};

// List<OperationWithKey>に変換する。正しく組み立てられるかはチェックしない
// Porting note: replaces parseToOperationWithKeys
pub fn parse_to_operations_with_key<'a, O: Operation>(
    field_origin: &'a dyn Field,
    operations: &'a Operations<O>,
    mino_factory: &'a MinoFactory,
    height: u8,
) -> Vec<FullOperationWithKey> {
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
    mino: &'static Mino,
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
    let above_lower_y = key_operators::get_mask_for_key_above_y(lower_y);
    let below_upper_y = key_operators::get_mask_for_key_below_y(upper_y + 1);
    let key_line = above_lower_y & below_upper_y;
    let need_deleted_key = delete_key & key_line;
    let using_key = !delete_key & key_line;

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
) -> Operations<SimpleOperation> {
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

pub fn parse_to_block_field(
    operation_with_keys: &[impl MinoOperationWithKey],
    height: u8,
) -> BlockField {
    let mut block_field = BlockField::new(height);
    for operation in operation_with_keys {
        block_field.merge(
            operation.create_mino_field(height).as_ref(),
            operation.get_piece(),
        );
    }
    block_field
}

pub fn parse_to_field(
    operation_with_keys: &[impl MinoOperationWithKey],
    height: u8,
) -> Box<dyn Field> {
    let mut field = field_factory::create_field(height);
    for operation in operation_with_keys {
        field.merge(operation.create_mino_field(height).as_ref());
    }
    field
}

// 最も低いブロックのy座標を取得
pub fn get_min_y(mino_factory: &MinoFactory, operations_list: &[impl Operation]) -> Option<u8> {
    operations_list
        .iter()
        .map(|operation| {
            let mino = mino_factory.get(operation.get_piece(), operation.get_rotate());
            u8::try_from(operation.get_y() as i8 + mino.get_min_y()).unwrap()
        })
        .min()
}

// 最も高いブロックのy座標を取得
pub fn get_max_y(mino_factory: &MinoFactory, operations_list: &[impl Operation]) -> Option<u8> {
    operations_list
        .iter()
        .map(|operation| {
            let mino = mino_factory.get(operation.get_piece(), operation.get_rotate());
            u8::try_from(operation.get_y() as i8 + mino.get_max_y()).unwrap()
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::parser::operation_with_key_interpreter,
        sfinder_core::{mino::piece::Piece, srs::rotate::Rotate},
    };

    fn assert_operations_with_put<O: Operation>(
        mino_factory: &MinoFactory,
        init_field: &dyn Field,
        operations: &Operations<O>,
        expected: &dyn Field,
        height: u8,
    ) {
        let mut f1 = init_field.prune(height);
        for operation in operations.get_operations() {
            let mino = mino_factory.get(operation.get_piece(), operation.get_rotate());
            f1.put(mino, operation.get_x(), operation.get_y());
            f1.clear_filled_rows();
        }

        let mut f2 = dyn_clone::clone_box(expected);
        f2.clear_filled_rows();
        assert_eq!(&f1, &f2);
    }

    fn assert_operations_with_mino_field(
        init_field: &dyn Field,
        operations: &[impl MinoOperationWithKey],
        // the type is &Box to get around lifetimes
        expected: &dyn Field,
        height: u8,
    ) {
        let mut f2 = init_field.prune(height);
        for operation in operations {
            f2.merge(operation.create_mino_field(height).as_ref());
        }

        assert_eq!(f2.as_ref(), expected);
    }

    fn assert_operations<O: Operation>(
        mino_factory: &MinoFactory,
        init_field: &dyn Field,
        operations: &Operations<O>,
        operations_with_key: &[impl MinoOperationWithKey],
        expected: &dyn Field,
        height: u8,
    ) {
        assert_operations_with_put(mino_factory, init_field, operations, expected, height);
        assert_operations_with_mino_field(init_field, operations_with_key, expected, height);
    }

    fn test_parse_to_operations_with_key_wrapper(
        height: u8,
        field_string: String,
        base: &str,
        expected_line: Option<&str>,
        expected_field: String,
    ) {
        let mino_factory = MinoFactory::new();

        let field = field_factory::create_field_with_marks(field_string);

        let operations = base.parse::<Operations<SimpleOperation>>().unwrap();
        let operations_with_key =
            parse_to_operations_with_key(field.as_ref(), &operations, &mino_factory, height);

        if let Some(expected_line) = expected_line {
            assert_eq!(
                operation_with_key_interpreter::full_operation_with_key_to_string(
                    &operations_with_key
                ),
                expected_line
            );
        }

        let expected = field_factory::create_field_with_marks(expected_field);

        assert_operations(
            &mino_factory,
            field.as_ref(),
            &operations,
            &operations_with_key,
            expected.as_ref(),
            height,
        );
    }

    #[test]
    fn test_parse_to_operations_with_key() {
        #[rustfmt::skip]
        test_parse_to_operations_with_key_wrapper(
            4,
            String::new() +
                "____XXXXXX" +
                "____XXXXXX" +
                "____XXXXXX" +
                "____XXXXXX",
            "L,0,2,0;Z,R,2,2;O,0,0,1;L,2,1,1",
            Some("L,0,2,0,0,3;Z,R,2,2,0,14;O,0,0,1,0,6;L,2,1,1,6,9"),
            String::new() +
                "XXXXXXXXXX" +
                "XXXXXXXXXX" +
                "XXXXXXXXXX" +
                "XXXXXXXXXX",
        );
    }

    #[test]
    fn test_parse_to_operations_with_key_2() {
        #[rustfmt::skip]
        test_parse_to_operations_with_key_wrapper(
            4,
            String::new(),
            "T,2,5,1;I,0,2,0;I,0,7,0;I,L,0,1",
            Some("T,2,5,1,0,3;I,0,2,0,0,1;I,0,7,0,0,1;I,L,0,1,0,15"),
            String::new() +
                "X_________" +
                "X_________" +
                "X___XXX___" +
                "XXXXXXXXXX" 
        );
    }

    #[test]
    fn test_parse_to_operations_with_key_3() {
        #[rustfmt::skip]
        test_parse_to_operations_with_key_wrapper(
            6,
            String::new(),
            "J,0,1,0;T,R,3,1;S,0,5,0;L,0,7,0;I,L,9,1;Z,0,4,1",
            None,
            String::new() +
                "___XX____X" +
                "___XXX___X" +
                "X__XXXX_XX" +
                "XXXXXXXXXX",
        );
    }

    #[test]
    fn test_parse_to_operations_with_key_4() {
        #[rustfmt::skip]
        test_parse_to_operations_with_key_wrapper(
            9,
            // manually fill out the field to force the use of MiddleField
            String::new() +
                "___________" +
                "___________" +
                "___________" +
                "___________" +
                "___________" +
                "___________" +
                "___________" +
                "___________" +
                "___________" +
                "___________",
            "I,0,1,6;J,L,5,7;L,2,7,6;T,L,9,7;S,0,5,3;Z,R,4,1;O,0,3,4",
            None,
            String::new() +
                "_____X___X" +
                "_____X__XX" +
                "XXXXXXXXXX" +
                "___XX_X___" +
                "___XXXX___" +
                "____XX____" +
                "_____X____" +
                "____XX____" +
                "____X_____",
        );
    }

    #[test]
    fn test_parse_to_operations() {
        let mino_factory = MinoFactory::new();

        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(String::new() +
            "______XXXX" +
            "______XXXX" +
            "______XXXX" +
            "______XXXX"
        );

        let base = "J,2,2,1;I,0,1,2;J,R,0,1;S,0,2,0";
        let operations = base.parse::<Operations<SimpleOperation>>().unwrap();
        let operations_with_key =
            parse_to_operations_with_key(field.as_ref(), &operations, &mino_factory, 4);
        let restore_operations = parse_to_operations(field.as_ref(), &operations_with_key, 4);

        assert_eq!(restore_operations, operations);
    }

    #[test]
    #[ignore = "implement MinoShifter, MinoRotation, SizedBit, SeparableMinos, TaskResultHelper, ILockedReachableThreadLocal, OnDemandBasicSolutions, InOutPairField, SolutionFilter, PerfectPackSearcher, BuildUpStream, OperationTransform"]
    fn random_parse() {}

    #[test]
    fn test_min_max_y() {
        let mino_factory = MinoFactory::new();
        let operations_list = [
            SimpleOperation::new(Piece::T, Rotate::Right, 0, 2),
            SimpleOperation::new(Piece::I, Rotate::Left, 9, 2),
        ];

        assert_eq!(get_min_y(&mino_factory, &operations_list), Some(1));
        assert_eq!(get_max_y(&mino_factory, &operations_list), Some(4));
    }

    #[test]
    fn test_min_max_y_with_empty() {
        let mino_factory = MinoFactory::new();
        let operations_list: [SimpleOperation; 0] = [];

        assert_eq!(get_min_y(&mino_factory, &operations_list), None);
        assert_eq!(get_max_y(&mino_factory, &operations_list), None);
    }
}
