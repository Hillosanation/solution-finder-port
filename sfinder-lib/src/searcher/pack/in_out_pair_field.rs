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
    // the first `width` columns are always empty in the outer field
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
        for start_x in (width..FIELD_WIDTH).step_by(width as usize) {
            for y in 0..height {
                for x in start_x..(start_x + 3).min(FIELD_WIDTH) {
                    if init_field.is_empty_block(x, y) {
                        let x_off = x - start_x;
                        max_outer_board.remove_block(width + x_off, y, height);
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
            pairs.push(Self::new(
                Self::read_to_column_field(field.as_ref(), width, height, 0, false),
                Self::read_to_column_field(field.as_ref(), width, height, width, false),
            ));
            field.slide_left(width);
        }

        for y in 0..height {
            for x in FIELD_WIDTH - (width * (max - 1))..width + 3 {
                field.set_block(x, y);
            }
        }

        pairs.push(Self::new(
            Self::read_to_column_field(field.as_ref(), width, height, 0, false),
            Self::read_to_column_field(field.as_ref(), 3, height, width, false),
        ));

        pairs
    }

    fn parse(field: &dyn Field, width: u8, height: u8) -> Self {
        Self::new(
            Self::read_to_column_field(field, width, height, 0, false),
            Self::read_to_column_field(field, width, height, width, false),
        )
    }

    fn parse_last(field: &dyn Field, width: u8, height: u8) -> Self {
        Self::new(
            Self::read_to_column_field(field, width, height, 0, false),
            Self::read_to_column_field(field, 3, height, width, false),
        )
    }

    pub fn create_inner_fields(
        sized_bit: &SizedBit,
        init_field: &dyn Field,
    ) -> Vec<ColumnSmallField> {
        let width = sized_bit.width;
        let height = sized_bit.height;
        let max = 9 / width + 1;
        let mut fields = Vec::with_capacity(max as usize);

        let mut field = init_field.prune(height);
        for _ in 0..max {
            fields.push(Self::read_to_column_field(
                init_field, width, height, 0, false,
            ));
            field.slide_left(width);
        }

        fields
    }

    fn read_to_column_field(
        field: &dyn Field,
        width: u8,
        height: u8,
        offset: u8,
        fill_out_of_bounds: bool,
    ) -> ColumnSmallField {
        let mut column_field = column_field_factory::create_small_field();

        for y in 0..height {
            for x in 0..width {
                let actual_x = x + offset;
                // we need to do the bounds checking ourselves
                if actual_x >= FIELD_WIDTH {
                    if fill_out_of_bounds {
                        column_field.set_block(actual_x, y, height);
                    }
                } else if field.exists_block(actual_x, y) {
                    column_field.set_block(actual_x, y, height);
                }
            }
        }

        column_field
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

    #[test]
    fn create_in_out_pair_fields2x5() {
        let field = field_factory::create_field_with_marks(
            String::new()
                + "____X___X_"
                + "___X___XX_"
                + "__X___XX_X"
                + "_X___XX__X"
                + "X____X___X",
        );

        let sized_bit = SizedBit::new(2, 5);

        // Create pairs
        let fields = InOutPairField::create_in_out_pair_fields(&sized_bit, field.as_ref());
        assert_eq!(fields.len(), 4);

        // Check inner
        #[rustfmt::skip]
        let inner_field1 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "__"
                + "__"
                + "__"
                + "_X"
                + "X_",
            5,
        );
        #[rustfmt::skip]
        let inner_field2 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "__"
                + "_X"
                + "X_"
                + "__"
                + "__",
            5,
        );
        #[rustfmt::skip]
        let inner_field3 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "X_"
                + "__"
                + "__"
                + "_X"
                + "_X",
            5,
        );
        #[rustfmt::skip]
        let inner_field4 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "__"
                + "_X"
                + "XX"
                + "X_"
                + "__",
            5,
        );

        assert_eq!(
            fields.iter().map(|f| f.get_inner()).collect::<Vec<_>>(),
            vec![&inner_field1, &inner_field2, &inner_field3, &inner_field4]
        );

        // Check outer
        #[rustfmt::skip]
        let outer_field1 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "_____"
                + "___X_"
                + "__X__"
                + "_____"
                + "_____",
            5,
        );
        #[rustfmt::skip]
        let outer_field2 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "__X__"
                + "_____"
                + "_____"
                + "___X_"
                + "___X_",
            5,
        );
        #[rustfmt::skip]
        let outer_field3 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "_____"
                + "___X_"
                + "__XX_"
                + "__X__"
                + "_____",
            5,
        );
        #[rustfmt::skip]
        let outer_field4 = column_field_factory::create_small_field_with_marks(
            String::new()
                + "__X_X"
                + "__X_X"
                + "___XX"
                + "___XX"
                + "___XX",
            5,
        );

        assert_eq!(
            fields.iter().map(|f| f.get_outer()).collect::<Vec<_>>(),
            vec![&outer_field1, &outer_field2, &outer_field3, &outer_field4]
        );
    }

    #[test]
    fn create_max_outer_board3x4() {
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
            + "___X__X_XX"
            + "__X__XXXX_"
            + "_X__XX_XX_"
            + "X___X__X_X",
        );
        let sized_bit = SizedBit::new(3, 4);

        let max_outer_board = InOutPairField::create_max_outer_board(&sized_bit, field.as_ref());
        #[rustfmt::skip]
        let expects = column_field_factory::create_small_field_with_marks(
            String::new()
                + "___X__"
                + "_____X"
                + "____XX"
                + "____X_",
            4,
        );

        assert_eq!(max_outer_board, expects);
    }

    #[test]
    fn create_max_outer_board3x4_2() {
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "___X__X_X_"
                + "__X__XXXX_"
                + "_X__XX_XX_"
                + "X___X__X_X",
        );
        let sized_bit = SizedBit::new(3, 4);

        let max_outer_board = InOutPairField::create_max_outer_board(&sized_bit, field.as_ref());
        #[rustfmt::skip]
        let expects = column_field_factory::create_small_field_with_marks(
            String::new()
                + "______"
                + "_____X"
                + "____XX"
                + "____X_",
            4,
        );

        assert_eq!(max_outer_board, expects);
    }

    #[test]
    fn create_max_outer_board2x5() {
        #[rustfmt::skip]
        let field = field_factory::create_field_with_marks(
            String::new()
                + "____X___X_"
                + "___X___XX_"
                + "__X___XX_X"
                + "_X___XX__X"
                + "X____X___X",
        );
        let sized_bit = SizedBit::new(2, 5);

        let max_outer_board = InOutPairField::create_max_outer_board(&sized_bit, field.as_ref());
        #[rustfmt::skip]
        let expects = column_field_factory::create_small_field_with_marks(
            String::new()
                + "_____"
                + "_____"
                + "_____"
                + "_____"
                + "_____",
            5,
        );

        assert_eq!(max_outer_board, expects);
    }

    #[test]
    fn create_max_outer_board2x5_2() {
        let field = field_factory::create_field_with_marks(
            String::new()
                + "___XX_XXXX"
                + "XXXXX_XXXX"
                + "_XXX_XXX_X"
                + "__XXXXXXXX"
                + "X_XXXXXXXX",
        );
        let sized_bit = SizedBit::new(2, 5);

        let max_outer_board = InOutPairField::create_max_outer_board(&sized_bit, field.as_ref());
        #[rustfmt::skip]
        let expects = column_field_factory::create_small_field_with_marks(
            String::new()
                + "____X"
                + "__X_X"
                + "___X_"
                + "__XXX"
                + "__XXX",
            5,
        );

        assert_eq!(max_outer_board, expects);
    }
}
