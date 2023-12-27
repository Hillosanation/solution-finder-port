use super::operation::Operation;
use crate::extras::hash_code::HashCode;

#[derive(Debug)]
pub struct Operations<O: Operation<u8>> {
    // Porting note: I don't think you need to accomodate for each operation being a different concrete type.
    // In practive, Operations should be getting a vec of operations that are of the same concrete type, we just don't know which one.
    // TODO: check this
    operations: Vec<O>,
}

impl<O: Operation<u8>> Operations<O> {
    pub fn from_vec(operations: Vec<O>) -> Self {
        Self { operations }
    }

    pub fn get_operations(&self) -> &[O] {
        &self.operations
    }
}

impl<O: Operation<u8>> FromIterator<O> for Operations<O> {
    fn from_iter<I: IntoIterator<Item = O>>(iter: I) -> Self {
        Self {
            operations: iter.into_iter().collect(),
        }
    }
}

impl<O: Operation<u8> + std::cmp::PartialEq> PartialEq for Operations<O> {
    fn eq(&self, other: &Self) -> bool {
        self.operations == other.operations
    }
}

impl<T, O: Operation<u8> + HashCode<Output = T>> HashCode for Operations<O>
where
    u32: std::ops::Add<T, Output = u32>,
{
    type Output = u32;

    fn hash_code(&self) -> Self::Output {
        self.operations
            .iter()
            .map(|operation| operation.hash_code())
            // combining hash codes with defined by java
            .fold(1, |acc, hash_code| 31 * acc + hash_code)
    }
}

impl<O: Operation<u8> + PartialOrd> PartialOrd for Operations<O> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.operations.partial_cmp(&other.operations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::datastore::simple_operation::SimpleOperation,
        sfinder_core::{mino::piece::Piece, srs::rotate::Rotate},
    };
    use rand::{rngs::ThreadRng, Rng};

    #[test]
    fn create() {
        let list = vec![
            SimpleOperation::new(Piece::I, Rotate::Left, 0, 1),
            SimpleOperation::new(Piece::L, Rotate::Spawn, 2, 0),
            SimpleOperation::new(Piece::O, Rotate::Spawn, 1, 1),
            SimpleOperation::new(Piece::J, Rotate::Reverse, 2, 3),
        ];

        let operations = Operations::from_vec(list.clone());
        assert_eq!(operations.get_operations(), list.as_slice());
    }

    /// Note that this doesn't check if the operation is valid
    fn create_random_operation(rngs: &mut ThreadRng) -> SimpleOperation {
        SimpleOperation::new(
            Piece::new(rngs.gen_range(0..Piece::get_size()) as u8),
            Rotate::new(rngs.gen_range(0..Rotate::get_size()) as u8),
            rngs.gen_range(0..10),
            rngs.gen_range(0..20),
        )
    }

    #[test]
    fn compare_random() {
        let mut rngs = rand::thread_rng();
        for _ in 0..1000 {
            let list1 = (0..10)
                .map(|_| create_random_operation(&mut rngs))
                .collect::<Vec<_>>();
            let operations1 = Operations::from_vec(list1.clone());
            let list2 = (0..10)
                .map(|_| create_random_operation(&mut rngs))
                .collect::<Vec<_>>();
            let operations2 = Operations::from_vec(list2.clone());

            assert_eq!(operations1.get_operations(), list1.as_slice());
            assert_eq!(operations2.get_operations(), list2.as_slice());

            if list1 == list2 {
                assert_eq!(operations1, Operations::from_vec(list2));
            } else {
                assert!(
                    operations1 > operations2 && operations2 < operations1
                        || operations1 < operations2 && operations2 > operations1
                )
            }
        }
    }
}
