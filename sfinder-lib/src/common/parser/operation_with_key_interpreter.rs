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

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::{
        common::datastore::operation_with_key::OperationWithKey,
        entry::common::kicks::factory::srs_mino_rotation_factory,
        sfinder_core::{
            action::reachable::reachable_facade,
            field::{field_constants::FIELD_WIDTH, field_factory},
            mino::mino_shifter::MinoShifter,
        },
        sfinder_lib::randoms,
    };

    #[test]
    #[ignore = "BuildUp::cansBuild is not implemented"]
    fn parse_to_operation_with_key() {
        #[rustfmt::skip]
        let init_field = field_factory::create_field_with_marks(
            String::new()
                + "____XXXXXX"
                + "____XXXXXX"
                + "____XXXXXX"
                + "____XXXXXX",
        );

        let mino_factory = MinoFactory::new();
        let base = "J,0,1,0,0,3;I,0,1,2,0,4;L,L,3,1,4,11;Z,0,1,1,4,10";
        let operation_with_keys = parse_to_vec(base, &mino_factory);

        let mino_shifter = MinoShifter::new();
        let mino_rotation = srs_mino_rotation_factory::create();
        {
            let mut reachable = reachable_facade::create_90_locked(
                &mino_factory,
                &mino_shifter,
                mino_rotation.as_ref(),
                8,
            );
            todo!("BuildUp::cansBuild");
        }

        let line = full_operation_with_key_to_string(&operation_with_keys);
        assert_eq!(line, base);
    }

    #[test]
    fn parse_random() {
        let mut rngs = thread_rng();
        let mino_factory = MinoFactory::new();

        for size in 1..20 {
            let operations = (0..size)
                .map(|_| {
                    // TODO: a similar function to generate random FullOperationWithKey is use in SlideXOperationWithKey
                    let piece = randoms::gen_piece(&mut rngs);
                    let rotate = randoms::gen_rotate(&mut rngs);
                    let x = rngs.gen_range(0..FIELD_WIDTH);
                    let y = rngs.gen_range(0..4);
                    let delete_key = randoms::gen_key(&mut rngs);
                    let using_key = randoms::gen_key(&mut rngs);

                    Box::new(FullOperationWithKey::new(
                        mino_factory.get(piece, rotate),
                        x,
                        y,
                        delete_key,
                        using_key,
                    )) as _
                })
                .collect::<Vec<Box<dyn OperationWithKey>>>();

            let str = operations
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(";");
            let actual = parse_to_stream(&str, &mino_factory)
                .map(|op| Box::new(op) as _)
                .collect::<Vec<Box<dyn OperationWithKey>>>();

            assert_eq!(actual, operations);
        }
    }
}
