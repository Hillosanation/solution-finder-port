//! These functions are almost always used in tests, to make creating operations easier.
use crate::{
    common::datastore::full_operation_with_key::FullOperationWithKey,
    sfinder_core::{field::key_operators, mino::mino_factory::MinoFactory},
};

// Porting note: parseToString and parseToStringSimple are moved to the OperationWithKey trait.

// Porting note: Replaces parseToList
pub fn parse_to_vec<'m>(
    operations: &'m str,
    mino_factory: &'m MinoFactory,
) -> Vec<FullOperationWithKey> {
    parse_to_stream(operations, mino_factory).collect()
}

pub fn parse_to_stream<'m>(
    operations: &'m str,
    mino_factory: &'m MinoFactory,
) -> impl Iterator<Item = FullOperationWithKey> + 'm {
    operations.split(';').map(|s| {
        let operation_str = s.split(',').collect::<Vec<_>>();
        assert_eq!(operation_str.len(), 6);

        FullOperationWithKey::new(
            mino_factory.get(
                operation_str[0].parse().unwrap(),
                operation_str[1].parse().unwrap(),
            ),
            operation_str[2].parse().unwrap(),
            operation_str[3].parse().unwrap(),
            key_operators::to_bit_key(operation_str[4].parse().unwrap()),
            key_operators::to_bit_key(operation_str[5].parse().unwrap()),
        )
    })
}

pub fn full_operation_with_key_to_string(operations: &[FullOperationWithKey]) -> String {
    operations
        .iter()
        .map(|o| format!("{o}"))
        // .inspect(|s| println!("{s}"))
        .collect::<Vec<_>>()
        .join(";")
}
