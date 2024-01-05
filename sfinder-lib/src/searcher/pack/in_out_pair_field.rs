use super::sized_bit::SizedBit;
use crate::sfinder_core::{
    column_field::{
        column_field::ColumnField, column_field_factory, column_small_field::ColumnSmallField,
    },
    field::{field::Field, field_constants::FIELD_WIDTH},
};

pub struct InOutPairField {
    // Porting note: All constructors use ColumnSmallField, so just store them directly
    inner: ColumnSmallField,
    outer: ColumnSmallField,
}

impl InOutPairField {
    pub fn new(inner: ColumnSmallField, outer: ColumnSmallField) -> Self {
        Self { inner, outer }
    }

    pub fn get_inner(&self) -> &ColumnSmallField {
        &self.inner
    }

    pub fn get_outer(&self) -> &ColumnSmallField {
        &self.outer
    }

    // Porting: all calls use SizedBit, so only provide this function

    pub fn create_max_outer_board(
        sized_bit: &SizedBit,
        init_field: &dyn Field,
    ) -> ColumnSmallField {
        let width = sized_bit.width;
        let height = sized_bit.height;

        let mut max_outer_board = column_field_factory::create_small_field();

        // Outerをブロックで埋める
        for y in 0..height {
            for x in 0..3 {
                max_outer_board.set_block(width + x, y, height);
            }
        }

        // 対応部分にブロックがひとつでもないときは、Outerからブロックを削除
        for start_x in width..FIELD_WIDTH {
            for y in 0..height {
                for x in 0..3 {
                    let x1 = start_x + x;
                    if x1 < FIELD_WIDTH && init_field.is_empty_block(x1, y) {
                        max_outer_board.remove_block(width + x, y, height);
                    }
                }
            }
        }

        max_outer_board
    }

    pub fn create_in_out_pair_fields(sized_bit: &SizedBit, init_field: &dyn Field) -> Vec<Self> {
        let width = sized_bit.width;
        let height = sized_bit.height;
        let max = 9 / width;

        let mut pairs = Vec::with_capacity(max as usize);

        let mut field = init_field.prune(height);
        for _ in 0..max - 1 {
            pairs.push(Self::parse(field.as_ref(), width, height));
            field.slide_left(width);
        }

        for y in 0..height {
            for x in FIELD_WIDTH - (width * (max - 1))..width + 3 {
                field.set_block(x, y);
            }
        }

        pairs.push(Self::parse_last(field.as_ref(), width, height));

        pairs
    }

    fn parse(field: &dyn Field, width: u8, height: u8) -> Self {
        let mut inner = column_field_factory::create_small_field();
        let mut outer = column_field_factory::create_small_field();

        for y in 0..height {
            for x in 0..width {
                if field.exists_block(x, y) {
                    inner.set_block(x, y, height);
                }
            }
            for x in width..width * 2 {
                if field.exists_block(x, y) {
                    outer.set_block(x, y, height);
                }
            }
        }

        Self::new(inner, outer)
    }

    fn parse_last(field: &dyn Field, width: u8, height: u8) -> Self {
        let mut inner = column_field_factory::create_small_field();
        let mut outer = column_field_factory::create_small_field();

        for y in 0..height {
            for x in 0..width {
                if field.exists_block(x, y) {
                    inner.set_block(x, y, height);
                }
            }
            for x in width..width + 3 {
                if field.exists_block(x, y) {
                    outer.set_block(x, y, height);
                }
            }
        }

        Self::new(inner, outer)
    }

    pub fn create_inner_fields(
        sized_bit: &SizedBit,
        init_field: &dyn Field,
    ) -> Vec<ColumnSmallField> {
        let width = sized_bit.width;
        let height = sized_bit.height;
        let max = 9 / width + 1;
        let mut fields = Vec::new();

        let mut field = init_field.prune(height);
        for _ in 0..max {
            let mut inner_field = column_field_factory::create_small_field();
            for y in 0..height {
                for x in 0..width {
                    if field.exists_block(x, y) {
                        inner_field.set_block(x, y, height);
                    }
                }
            }

            fields.push(inner_field);
            field.slide_left(width);
        }

        fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_core::field::field_factory;

    #[test]
    fn create_in_out_pair_fields3x4() {
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "___X__X_XX"
                + "__X__XXXX_"
                + "_X__XX_XX_"
                + "X___X__X_X",
        );

        let sized_bit = SizedBit::new(3, 4);

        // Create pairs
        let fields = InOutPairField::create_in_out_pair_fields(&sized_bit, field.as_ref());
        assert_eq!(fields.len(), 3);

        // Check inner
        #[rustfmt::skip]
        let inner_field1 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "___"
                + "__X"
                + "_X_"
                + "X__",
            4,
        );
        #[rustfmt::skip]
        let inner_field2 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "X__"
                + "__X"
                + "_XX"
                + "_X_",
            4,
        );
        #[rustfmt::skip]
        let inner_field3 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "X_X"
                + "XXX"
                + "_XX"
                + "_X_",
            4,
        );

        assert_eq!(
            fields.iter().map(|f| f.get_inner()).collect::<Vec<_>>(),
            vec![&inner_field1, &inner_field2, &inner_field3]
        );

        // Check outer
        #[rustfmt::skip]
        let outer_field1 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "___X__"
                + "_____X"
                + "____XX"
                + "____X_",
            4,
        );
        #[rustfmt::skip]
        let outer_field2 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "___X_X"
                + "___XXX"
                + "____XX"
                + "____X_",
            4,
        );
        #[rustfmt::skip]
        let outer_field3 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "___XXX"
                + "____XX"
                + "____XX"
                + "___XXX",
            4,
        );

        assert_eq!(
            fields.iter().map(|f| f.get_outer()).collect::<Vec<_>>(),
            vec![&outer_field1, &outer_field2, &outer_field3]
        );
    }
}
