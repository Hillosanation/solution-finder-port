use super::column_field::ColumnField;
use crate::extras::hash_code::HashCode;

/**
 * heightの値はメモリ節約のためインスタンス内で保持しない (ただしheight <= 10を想定)
 * 必要な場合はインスタンス外で記録しておくこと
 * 幅は最大6
 */
// Porting note: to save space, the actual height of the ColumnField is not stored, thus indexing requires you to pass in the height.
#[derive(Clone)]
pub struct ColumnSmallField(u64);

impl ColumnSmallField {
    pub const fn new() -> Self {
        Self(0)
    }
}

impl From<u64> for ColumnSmallField {
    fn from(board: u64) -> Self {
        Self(board)
    }
}

// Porting note: follows the original Java behaviour, but you shouldn't need to rely on the wrapping behaviour in normal usage.
const fn get_y_mask(x: u8, y: u8, height: u8) -> u64 {
    1u64.overflowing_shl((y + height * x) as u32).0
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

impl PartialOrd for ColumnSmallField {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        <dyn ColumnField>::partial_cmp(self, other)
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

#[cfg(test)]
mod tests {
    use crate::{
        extras::test_functions::assert_partialord_symmetric,
        sfinder_core::column_field::column_field_factory,
    };

    use super::*;

    #[test]
    fn set_block() {
        let mut field = ColumnSmallField::new();
        let height = 4;

        assert!(field.is_empty(0, 0, height));
        field.set_block(0, 0, height);
        assert!(!field.is_empty(0, 0, height));
        field.remove_block(0, 0, height);
        assert!(field.is_empty(0, 0, height));
    }

    #[test]
    fn get_board() {
        let mut field = ColumnSmallField::new();
        let height = 4;

        assert_eq!(field.get_board(0), 0);
        field.set_block(0, 0, height);
        assert_eq!(field.get_board(0), 1);
    }

    #[test]
    fn get_board_count() {
        let height = 4;
        #[rustfmt::skip]
        let field = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_",
            height,
        );
        assert_eq!(field.get_board_count(), 1);
    }

    #[test]
    fn get_num_of_all_blocks() {
        let height = 4;
        #[rustfmt::skip]
        let field = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_",
            height,
        );
        assert_eq!(field.get_num_of_all_blocks(), 4);
    }

    #[test]
    fn merge() {
        let height = 4;
        #[rustfmt::skip]
        let mut field1 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_",
            height,
        );
        #[rustfmt::skip]
        let field2 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "___"
            + "__X"
            + "X__"
            + "___",
            height,
        );
        #[rustfmt::skip]
        let expect = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "__X"
            + "X__"
            + "_X_",
            height,
        );

        field1.merge(&field2);

        assert_eq!(field1, expect);
    }

    #[test]
    fn reduce() {
        let height = 4;
        #[rustfmt::skip]
        let mut field1 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "_XX"
            + "XX_"
            + "_XX",
            height,
        );
        #[rustfmt::skip]
        let field2 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "_X_"
            + "_X_"
            + "_X_"
            + "_X_",
            height,
        );
        #[rustfmt::skip]
        let expect = column_field_factory::create_small_field_with_marks(
            String::new()
            + "X__"
            + "__X"
            + "X__"
            + "__X",
            height,
        );

        field1.reduce(&field2);

        assert_eq!(field1, expect);
    }

    #[test]
    fn can_merge_1() {
        let height = 4;
        #[rustfmt::skip]
        let field1 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_",
            height,
        );
        #[rustfmt::skip]
        let field2 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "___"
            + "__X"
            + "X__"
            + "___",
            height,
        );

        assert!(!field1.can_merge(&field2));
    }

    #[test]
    fn can_merge_2() {
        let height = 4;
        #[rustfmt::skip]
        let field1 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_",
            height,
        );
        #[rustfmt::skip]
        let field2 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "___"
            + "__X"
            + "__X"
            + "__X",
            height,
        );

        assert!(field1.can_merge(&field2));
    }

    #[test]
    fn freeze() {
        let height = 4;
        #[rustfmt::skip]
        let field = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_",
            height,
        );
        let mut freeze = field.prune();

        freeze.set_block(0, 2, height);
        freeze.set_block(0, 3, height);

        #[rustfmt::skip]
        let expect = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_",
            height,
        );

        assert_ne!(freeze.as_ref() as &dyn ColumnField, &expect as &_);
        assert_eq!(field, expect);
    }

    #[test]
    fn equals() {
        let height = 4;
        #[rustfmt::skip]
        let field1 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_",
            height,
        );
        #[rustfmt::skip]
        let field2 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_",
            height,
        );
        #[rustfmt::skip]
        let field3 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "__X"
            + "__X"
            + "XX_",
            height,
        );

        assert_eq!(field1, field2);
        assert_eq!(field2, field1);
        assert_ne!(field1, field3);
        assert_ne!(field3, field1);
    }

    #[test]
    fn hash_code_1() {
        let height = 4;
        #[rustfmt::skip]
        let field1 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_",
            height,
        );
        #[rustfmt::skip]
        let field2 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_", height,
        );
        #[rustfmt::skip]
        let field3 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "__X"
            + "__X"
            + "XX_", height,
        );

        assert_eq!(field1.hash_code(), field2.hash_code());
        assert_ne!(field1.hash_code(), field3.hash_code());
    }

    #[test]
    fn compare_to() {
        let height = 4;
        #[rustfmt::skip]
        let field1 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_", height,
        );
        #[rustfmt::skip]
        let field2 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "___"
            + "X__"
            + "_X_", height,
        );
        #[rustfmt::skip]
        let field3 = column_field_factory::create_small_field_with_marks(
            String::new()
            + "XX_"
            + "__X"
            + "__X"
            + "XX_", height,
        );

        assert_eq!(field1.partial_cmp(&field2), Some(std::cmp::Ordering::Equal));
        assert_eq!(field2.partial_cmp(&field1), Some(std::cmp::Ordering::Equal));
        assert_partialord_symmetric(field3, field1);
    }
}
