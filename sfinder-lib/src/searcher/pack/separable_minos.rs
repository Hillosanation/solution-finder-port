use super::{
    separable_mino::{
        full_operation_separable_mino::FullOperationSeparableMino, separable_mino::SeparableMino,
    },
    sized_bit::SizedBit,
};
use crate::{
    common::datastore::mino_operation_with_key::MinoOperationWithKey,
    searcher::pack::separable_mino::all_separable_mino_factory,
    sfinder_core::{
        field::key_operators,
        mino::{mino_factory::MinoFactory, mino_shifter::MinoShifter},
    },
};
use std::{
    collections::{btree_set::Range, BTreeSet},
    ops::Bound,
};

// Seems like constructors always take in FullOperationSeparableMino, if so just use that
pub struct SeparableMinos<'a> {
    // Porting note: indexes uses MinoOperationWithKey instead of OperationWithKey,
    // as all callers of toIndex provide SeparableMino provide MinoOperationWithKey using toMinoOperationWithKey.
    // Thie circumvents the need to cast from OperationWithKey to MinoOperationWithKey.
    indexes: BTreeSet<Box<dyn MinoOperationWithKey + 'a>>,
}

impl<'a> SeparableMinos<'a> {
    pub fn new(
        mino_factory: &'a MinoFactory,
        mino_shifter: &'a MinoShifter,
        sized_bit: &SizedBit,
    ) -> Self {
        let height = sized_bit.height;
        let mask = key_operators::get_mask_for_key_below_y(height);
        Self::new_with_mask(mino_factory, mino_shifter, sized_bit, mask)
    }

    fn new_with_mask(
        mino_factory: &'a MinoFactory,
        mino_shifter: &'a MinoShifter,
        sized_bit: &SizedBit,
        delete_key_mask: u64,
    ) -> Self {
        Self::from(all_separable_mino_factory::create(
            mino_factory,
            mino_shifter,
            sized_bit.width,
            sized_bit.height,
            delete_key_mask,
        ))
    }

    fn get_minos(&self) -> impl Iterator<Item = &Box<dyn MinoOperationWithKey + 'a>> {
        self.indexes.iter()
    }

    // Porting note: partially replaces getIndex, used when comparing two SeparableMinos.
    // TODO(#14): phase out this and compare the MinoOperationWithKey directly through the Ord trait
    pub fn compare_index(a: &dyn SeparableMino, b: &dyn SeparableMino) -> std::cmp::Ordering {
        let a_op = a.get_mino_operation_with_key();
        let b_op = b.get_mino_operation_with_key();

        Self::compare_index_operations(a_op, b_op)
    }

    fn compare_index_operations(
        a: &dyn MinoOperationWithKey,
        b: &dyn MinoOperationWithKey,
    ) -> std::cmp::Ordering {
        a.cmp(b)
    }

    // Porting note: partially replaces getIndex, used when listing all SeparableMinos that are greater than the given one.
    pub fn all_operations_greater_than(
        &self,
        separable_mino: &'a dyn SeparableMino,
    ) -> Range<'_, Box<dyn MinoOperationWithKey + 'a>> {
        self.indexes.range::<dyn MinoOperationWithKey, _>((
            Bound::Excluded(separable_mino.get_mino_operation_with_key()),
            Bound::Unbounded,
        ))
    }
}

impl<'a> From<Vec<FullOperationSeparableMino<'a>>> for SeparableMinos<'a> {
    fn from(minos: Vec<FullOperationSeparableMino<'a>>) -> Self {
        Self {
            indexes: minos
                .into_iter()
                .map(|mino| mino.to_mino_operation_with_key())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        searcher::pack::{
            separable_mino::full_operation_separable_mino::FullOperationSeparableMino,
            sized_bit::SizedBit,
        },
        sfinder_core::{
            field::field_constants::FIELD_WIDTH,
            mino::{mino_factory::MinoFactory, mino_shifter::MinoShifter},
        },
    };
    use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

    fn create_separable_mino_set<'a>(
        rngs: &mut ThreadRng,
        mino_factory: &'a MinoFactory,
        mino_shifter: &'a MinoShifter,
    ) -> Vec<FullOperationSeparableMino<'a>> {
        let sized_bit = SizedBit::new(rngs.gen_range(1..=FIELD_WIDTH), rngs.gen_range(1..=4));

        all_separable_mino_factory::create(
            mino_factory,
            mino_shifter,
            sized_bit.width,
            sized_bit.height,
            sized_bit.fill_board,
        )
    }

    #[test]
    fn create() {
        let mut rngs = thread_rng();
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();

        let minos = create_separable_mino_set(&mut rngs, &mino_factory, &mino_shifter);
        let minos1 = minos.clone();
        let mut minos2 = minos;
        minos2.shuffle(&mut rngs);

        let separable_minos1 = SeparableMinos::from(minos1);
        let separable_minos2 = SeparableMinos::from(minos2);

        assert_eq!(
            separable_minos1.get_minos().collect::<Vec<_>>(),
            separable_minos2.get_minos().collect::<Vec<_>>()
        );
    }

    #[test]
    fn to_index() {
        // Porting note: checking consistency of index doesn't make sense when the index is elided, so I am checking the consistency of compare_index with Ord
        let mut rngs = thread_rng();
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();

        let minos = create_separable_mino_set(&mut rngs, &mino_factory, &mino_shifter);

        for _ in 0..10000 {
            let mino1 = minos.choose(&mut rngs).unwrap();
            let mino2 = minos.choose(&mut rngs).unwrap();

            let cmp = SeparableMinos::compare_index(mino1, mino2);
            assert_eq!(
                cmp,
                mino1
                    .get_mino_operation_with_key()
                    .cmp(mino2.get_mino_operation_with_key())
            );
        }
    }

    mod full_operation_separable_mino_comparator {
        use super::*;

        // tests retrieved from FullOperationSeparableMinoComparatorTest
        fn test_create_separable_minos<'a>(
            mino_factory: &'a MinoFactory,
            mino_shifter: &'a MinoShifter,
        ) -> SeparableMinos<'a> {
            let sized_bit = SizedBit::new(3, 4);

            SeparableMinos::new(mino_factory, mino_shifter, &sized_bit)
        }

        #[test]
        fn compare_mino_field_equal() {
            let mino_factory = MinoFactory::new();
            let mino_shifter = MinoShifter::new();

            let separable_minos = test_create_separable_minos(&mino_factory, &mino_shifter);
            let minos = separable_minos.get_minos().collect::<Vec<_>>();
            for mino in minos {
                // this should pass regardless, because Eq ensures reflexive equality
                assert_eq!(mino.as_ref(), mino.as_ref());
            }
        }

        #[test]
        fn compare_mino_field_diff() {
            let mino_factory = MinoFactory::new();
            let mino_shifter = MinoShifter::new();
            let mut rngs = thread_rng();

            let separable_minos = test_create_separable_minos(&mino_factory, &mino_shifter);
            let minos = separable_minos.get_minos().collect::<Vec<_>>();
            for _ in 0..10000 {
                let mino1 = minos.choose(&mut rngs).unwrap();
                let mino2 = minos.choose(&mut rngs).unwrap();

                let cmp = SeparableMinos::compare_index_operations(mino1.as_ref(), mino2.as_ref());
                let cmp_rev =
                    SeparableMinos::compare_index_operations(mino2.as_ref(), mino1.as_ref());
                if cmp == std::cmp::Ordering::Equal {
                    continue;
                }

                assert_eq!(cmp.reverse(), cmp_rev);
            }
        }
    }
}
