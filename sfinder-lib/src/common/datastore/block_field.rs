use crate::extras::hash_code::HashCode;
use crate::sfinder_core::{
    field::{field::Field, field_factory},
    mino::piece::Piece,
};

#[derive(Debug)]
pub struct BlockField {
    /// map is indexed by Piece
    map: [Option<Box<dyn Field>>; Piece::get_size()],
    height: u8,
}

impl BlockField {
    pub fn new(height: u8) -> Self {
        Self {
            // call from closure instead of array initializer because Box is not Copy
            map: std::array::from_fn(|_| None),
            height,
        }
    }

    pub fn set_block(&mut self, piece: Piece, x: u8, y: u8) {
        self.map[piece as usize]
            .get_or_insert_with(|| field_factory::create_field(self.height))
            .set_block(x, y);
    }

    pub fn merge(&mut self, field: &dyn Field, piece: Piece) {
        self.map[piece as usize]
            .get_or_insert_with(|| field_factory::create_field(self.height))
            .merge(field);
    }

    pub fn get(&self, piece: Piece) -> Option<&dyn Field> {
        self.map[piece as usize]
            .as_ref()
            .map(|field| field.as_ref())
    }

    pub fn contains_all(&self, target: BlockField) -> bool {
        target
            .map
            .iter()
            .zip(self.map.iter())
            .all(|(my_field, target_field)| match (my_field, target_field) {
                (Some(my_field), Some(target_field)) => my_field.contains(target_field.as_ref()),
                (None, Some(t)) => t.is_empty(),
                _ => true,
            })
    }

    pub fn get_piece_of_block(&self, x: u8, y: u8) -> Option<Piece> {
        self.map.iter().enumerate().find_map(|(i, field)| {
            field
                .as_ref()?
                .exists_block(x, y)
                .then(|| Piece::new(i as u8))
        })
    }

    pub fn get_height(&self) -> u8 {
        self.height
    }
}

impl PartialEq for BlockField {
    fn eq(&self, other: &Self) -> bool {
        // height is not relevant, similar to the equality of Field
        self.map == other.map
    }
}

impl HashCode for BlockField {
    type Output = u32;

    fn hash_code(&self) -> Self::Output {
        self.map
            .iter()
            .map(|opt_field| opt_field.as_ref().map_or(0, |field| field.hash_code()))
            // combining hash codes with defined by java
            .fold(1, |acc, hash_code| 31 * acc + hash_code)
    }
}

impl PartialOrd for BlockField {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // height is not relevant, similar to the equality of Field
        self.map.partial_cmp(&other.map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extras::test_functions::assert_partialord_symmetric;

    // tests retrieved from PieceFieldTest.java

    #[test]
    fn merge_and_get() {
        let height = 4;
        let mut block_field = BlockField::new(height);

        #[rustfmt::skip]
        let field1 = field_factory::create_field_with_marks(String::new() +
            "__X_______" +
            "_XXX______"
        );
        block_field.merge(field1.as_ref(), Piece::T);

        #[rustfmt::skip]
        let field2 = field_factory::create_field_with_marks(String::new() +
            "X_________" +
            "XX________" +
            "X_________"
        );
        block_field.merge(field2.as_ref(), Piece::T);

        #[rustfmt::skip]
        let merged = field_factory::create_field_with_marks(String::new() +
            "X_________" +
            "XXX_______" +
            "XXXX______"
        );
        assert_eq!(block_field.get(Piece::T).unwrap(), merged.as_ref());

        for &piece in Piece::value_list() {
            assert_eq!(block_field.get(piece).is_some(), piece == Piece::T);
        }
    }

    #[test]
    fn compare_1() {
        let height = 4;
        let mut block_field_1 = BlockField::new(height);

        #[rustfmt::skip]
        let field1 = field_factory::create_field_with_marks(String::new() +
            "__X_______" +
            "_XXX______"
        );
        block_field_1.merge(field1.as_ref(), Piece::T);

        #[rustfmt::skip]
        let field2 = field_factory::create_field_with_marks(String::new() +
            "X_________" +
            "XX________" +
            "X_________"
        );
        block_field_1.merge(field2.as_ref(), Piece::T);

        let mut block_field_2 = BlockField::new(height);
        #[rustfmt::skip]
        let merged = field_factory::create_field_with_marks(String::new() +
            "X_________" +
            "XXX_______" +
            "XXXX______"
        );
        block_field_2.merge(merged.as_ref(), Piece::T);

        assert_eq!(block_field_1, block_field_2);
        assert_eq!(
            block_field_1.partial_cmp(&block_field_2),
            Some(std::cmp::Ordering::Equal)
        );
    }

    #[test]
    fn compare_2() {
        let height = 4;
        let mut block_field_1 = BlockField::new(height);

        #[rustfmt::skip]
        let field1 = field_factory::create_field_with_marks(String::new() +
            "__X_______" +
            "_XXX______"
        );
        block_field_1.merge(field1.as_ref(), Piece::T);

        #[rustfmt::skip]
        let field2 = field_factory::create_field_with_marks(String::new() +
            "X_________" +
            "XX________" +
            "X_________"
        );
        block_field_1.merge(field2.as_ref(), Piece::T);

        let mut block_field_2 = BlockField::new(height);
        #[rustfmt::skip]
        let merged = field_factory::create_field_with_marks(String::new() +
            "X_________" +
            "XXX_______" +
            "XXXX______"
        );
        block_field_2.merge(merged.as_ref(), Piece::I);

        assert_ne!(block_field_1, block_field_2);
        assert_partialord_symmetric(&block_field_1, &block_field_2);
    }

    #[test]
    fn compare_3() {
        let height = 4;
        let mut block_field_1 = BlockField::new(height);

        #[rustfmt::skip]
        let field1 = field_factory::create_field_with_marks(String::new() +
            "___X______" +
            "__XX______" +
            "___X______"
        );
        block_field_1.merge(field1.as_ref(), Piece::T);

        #[rustfmt::skip]
        let field2 = field_factory::create_field_with_marks(String::new() +
            "X_________" +
            "XX________" +
            "X_________"
        );
        block_field_1.merge(field2.as_ref(), Piece::T);

        let mut block_field_2 = BlockField::new(height);
        #[rustfmt::skip]
        let merged = field_factory::create_field_with_marks(String::new() +
            "X_________" +
            "XXX_______" +
            "XXXX______"
        );
        block_field_2.merge(merged.as_ref(), Piece::T);

        assert_ne!(block_field_1, block_field_2);
        assert_partialord_symmetric(&block_field_1, &block_field_2);
    }
}
