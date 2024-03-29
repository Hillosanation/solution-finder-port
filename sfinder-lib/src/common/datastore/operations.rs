use super::{operation::Operation, simple_operation::SimpleOperation};
use crate::extras::hash_code::HashCode;
use std::{convert::Infallible, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Operations<O: Operation> {
    // Porting note: I don't think you need to accomodate for each operation being a different concrete type.
    // In practice, Operations should be getting a vec of operations that are of the same concrete type, we just don't know which one.
    // TODO: check this
    operations: Vec<O>,
}

impl<O: Operation> Operations<O> {
    pub fn from_vec(operations: Vec<O>) -> Self {
        Self { operations }
    }

    pub fn get_operations(&self) -> &[O] {
        &self.operations
    }
}

impl<O: Operation> FromIterator<O> for Operations<O> {
    fn from_iter<I: IntoIterator<Item = O>>(iter: I) -> Self {
        Self {
            operations: iter.into_iter().collect(),
        }
    }
}

impl<T, O: Operation + HashCode<Output = T>> HashCode for Operations<O>
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

// Porting note: moved from OperationInterpreter
impl<O: Operation + Display> Display for Operations<O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.operations
                .iter()
                .map(|operation| format!("{operation}"))
                .collect::<Vec<_>>()
                .join(";")
        )
    }
}

impl FromStr for Operations<SimpleOperation> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_vec(
            s.split(';').map(|s| s.parse().unwrap()).collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::datastore::simple_operation::SimpleOperation,
        sfinder_core::{
            field::field_constants::FIELD_WIDTH, mino::piece::Piece, srs::rotate::Rotate,
        },
    };
    use rand::{rngs::ThreadRng, thread_rng, Rng};

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
    fn create_random_operation(rngs: &mut ThreadRng, y: u8) -> SimpleOperation {
        SimpleOperation::new(
            Piece::new(rngs.gen_range(0..Piece::get_size()) as u8),
            Rotate::new(rngs.gen_range(0..Rotate::get_size()) as u8),
            rngs.gen_range(0..FIELD_WIDTH),
            rngs.gen_range(0..y),
        )
    }

    #[test]
    fn compare_random() {
        let mut rngs = rand::thread_rng();
        for _ in 0..1000 {
            let list1 = (0..10)
                .map(|_| create_random_operation(&mut rngs, 20))
                .collect::<Vec<_>>();
            let operations1 = Operations::from_vec(list1.clone());
            let list2 = (0..10)
                .map(|_| create_random_operation(&mut rngs, 20))
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

    // tests retrieved from OperationInterpreterTest
    #[test]
    fn parse_to_operations() {
        let base = "T,0,1,0;L,2,1,2;I,L,3,1;J,2,1,3";
        let operations = base.parse::<Operations<SimpleOperation>>().unwrap();
        let str = format!("{}", operations);
        assert_eq!(str, base);
    }

    #[test]
    fn parse_random() {
        let mut rngs = thread_rng();
        for size in 1..20 {
            let operations = (0..size)
                .map(|_| create_random_operation(&mut rngs, 4))
                .collect::<Vec<_>>();

            let operations = Operations::from_vec(operations);
            let str = format!("{}", operations);
            let actual = str.parse::<Operations<SimpleOperation>>().unwrap();

            assert_eq!(operations, actual);
        }
    }
}
