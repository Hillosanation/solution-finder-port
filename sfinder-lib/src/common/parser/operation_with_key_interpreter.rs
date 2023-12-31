//! These functions are almost always used in tests, to make creating operations easier.
#[cfg(test)]
use crate::{
    common::datastore::full_operation_with_key::FullOperationWithKey,
    sfinder_core::{
        field::key_operators,
        mino::{mino_factory::MinoFactory, piece::Piece},
        srs::rotate::Rotate,
    },
};

// Porting note: parseToString and parseToStringSimple are moved to the OperationWithKey trait.

// Porting note: Replaces parseToList
#[cfg(test)]
pub fn parse_to_vec<'m>(
    operations: &'m str,
    mino_factory: &'m MinoFactory,
) -> Vec<FullOperationWithKey<'m>> {
    parse_to_stream(operations, mino_factory).collect()
}

#[cfg(test)]
pub fn parse_to_stream<'m>(
    operations: &'m str,
    mino_factory: &'m MinoFactory,
) -> impl Iterator<Item = FullOperationWithKey<'m>> {
    operations.split(';').map(|s| {
        let operation_str = s.split(',').collect::<Vec<_>>();
        assert_eq!(operation_str.len(), 6);

        let piece = operation_str[0].parse::<Piece>().unwrap();
        let rotate = operation_str[1].parse::<Rotate>().unwrap();

        FullOperationWithKey::new(
            mino_factory.get(piece, rotate),
            operation_str[2].parse::<u8>().unwrap(),
            operation_str[3].parse::<u8>().unwrap(),
            key_operators::to_bit_key(operation_str[4].parse::<u64>().unwrap()),
            key_operators::to_bit_key(operation_str[5].parse::<u64>().unwrap()),
        )
    })
}
