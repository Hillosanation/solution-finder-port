use crate::{extras::hash_code::HashCode, sfinder_core::mino::piece::Piece};

#[derive(Debug, Clone, PartialEq)]
pub struct PieceCounter(u64);

const SLIDE_MASK: [u64; Piece::get_size()] = [
    1 >> (8 * 0),
    1 << (8 * 1),
    1 << (8 * 2),
    1 << (8 * 3),
    1 << (8 * 4),
    1 << (8 * 5),
    1 << (8 * 6),
];

impl PieceCounter {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    // Porting note: replaces getSinglePieceCounter
    pub const fn with_single_piece(piece: Piece) -> Self {
        Self(SLIDE_MASK[piece as usize])
    }

    fn get_count(&self, piece: Piece) -> u64 {
        self.0 >> 8 * (piece as usize) & 0xff
    }

    pub const fn contains_all(&self, other: &Self) -> bool {
        let difference = self.0 as i64 - other.0 as i64;
        // 各ブロックの最上位ビットが1のとき（繰り下がり）が発生していない時true
        difference & 0x80808080808080 == 0
    }

    // Porting note: replaces addAndReturnNew
    pub fn add_pieces<I>(&self, pieces: I) -> Self
    where
        I: IntoIterator<Item = Piece>,
    {
        Self(
            pieces
                .into_iter()
                .fold(self.0, |acc, piece| acc + SLIDE_MASK[piece as usize]),
        )
    }

    pub fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0)
    }

    // 引く側のブロックをすべて引かれる側に含まれていること
    // この関数を呼ぶ前にそのことを確認して置くこと
    // Porting note: replaces removeAndReturnNew
    pub fn remove(&self, other: &Self) -> Self {
        assert!(self.contains_all(other));
        Self(self.0 - other.0)
    }

    // Porting note: replaces getBlocks/getBlockStream, as
    pub fn to_blocks(&self) -> Vec<Piece> {
        Piece::value_list()
            .iter()
            .flat_map(|piece| vec![*piece; self.get_count(*piece) as usize])
            .collect()
    }

    // Porting note: replaces getEnumMap
    pub fn to_counts(&self) -> [u64; Piece::get_size()] {
        std::array::from_fn(|i| self.get_count(Piece::new(i as u8)))
    }
}

// Porting note: replaces getCounter
impl From<PieceCounter> for u64 {
    fn from(counter: PieceCounter) -> Self {
        counter.0
    }
}

impl<I> From<I> for PieceCounter
where
    I: IntoIterator<Item = Piece>,
{
    fn from(iter: I) -> Self {
        Self(
            iter.into_iter()
                .map(|piece| SLIDE_MASK[piece as usize])
                .sum(),
        )
    }
}

impl HashCode for PieceCounter {
    type Output = u64;

    fn hash_code(&self) -> Self::Output {
        self.0 ^ (self.0 >> 32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_lib::randoms;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_empty() {
        let counter = PieceCounter::from([]);
        assert_eq!(u64::from(counter), 0);
    }

    #[test]
    fn test_add() {
        let counter = PieceCounter::from([Piece::I, Piece::J]);
        let actual = counter.add_pieces([Piece::T]);

        assert_eq!(counter, PieceCounter::from([Piece::I, Piece::J]));
        assert_eq!(actual, PieceCounter::from([Piece::I, Piece::T, Piece::J]));
    }

    #[test]
    fn test_add_2() {
        let counter1 = PieceCounter::from([Piece::I, Piece::J, Piece::T]);
        let counter2 = PieceCounter::from([Piece::I, Piece::J, Piece::O]);
        let actual = counter1.add(&counter2);

        assert_eq!(
            actual,
            PieceCounter::from([Piece::I, Piece::I, Piece::J, Piece::J, Piece::T, Piece::O])
        );
    }

    #[test]
    fn test_remove() {
        let counter = PieceCounter::from([Piece::I, Piece::J, Piece::I, Piece::L]);
        let actual = counter.remove(&PieceCounter::from([Piece::I, Piece::J]));

        assert_eq!(
            counter,
            PieceCounter::from([Piece::I, Piece::J, Piece::I, Piece::L])
        );
        assert_eq!(actual, PieceCounter::from([Piece::I, Piece::L]));
    }

    // testGet is omitted since getBlockStream is not implemented

    #[test]
    fn test_get_map() {
        let counter = PieceCounter::from([
            Piece::I,
            Piece::J,
            Piece::T,
            Piece::I,
            Piece::I,
            Piece::T,
            Piece::S,
        ]);
        let map = counter.to_counts();

        assert_eq!(map[Piece::I as usize], 3);
        assert_eq!(map[Piece::T as usize], 2);
        assert_eq!(map[Piece::S as usize], 1);
        assert_eq!(map[Piece::J as usize], 1);
        assert_eq!(map[Piece::L as usize], 0);
        assert_eq!(map[Piece::Z as usize], 0);
        assert_eq!(map[Piece::O as usize], 0);
    }

    #[test]
    fn test_random() {
        let mut rngs = thread_rng();

        for _ in 0..10000 {
            let count = rngs.gen_range(0..500);
            let pieces = randoms::gen_pieces(&mut rngs, count);

            let mut actual = [0; Piece::get_size()];
            for piece in &pieces {
                actual[*piece as usize] += 1;
            }

            if actual.iter().any(|&count| count >= 128) {
                // PieceCounter doesn't support more than 128 pieces of the same type
                continue;
            }

            let counter = PieceCounter::from(pieces);
            let expected = counter.to_counts();

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_contains_all_1() {
        let counter1 = PieceCounter::from([Piece::T]);
        let counter2 = PieceCounter::from([Piece::T, Piece::I]);

        assert!(!counter1.contains_all(&counter2));
        assert!(counter2.contains_all(&counter1));
    }

    #[test]
    fn test_contain_all_2() {
        let counter1 = PieceCounter::from([Piece::S, Piece::Z, Piece::T]);
        let counter2 = PieceCounter::from([Piece::Z, Piece::T, Piece::S]);

        assert!(counter1.contains_all(&counter2));
        assert!(counter2.contains_all(&counter1));
    }

    #[test]
    fn test_contains_all_random() {
        let mut rngs = thread_rng();

        for _ in 0..10000 {
            let count = rngs.gen_range(0..500);
            let pieces1 = randoms::gen_pieces(&mut rngs, count);

            let mut actual1 = [0; Piece::get_size()];
            for piece in &pieces1 {
                actual1[*piece as usize] += 1;
            }

            if actual1.iter().any(|&count| count >= 128) {
                // PieceCounter doesn't support more than 128 pieces of the same type
                continue;
            }

            let count = rngs.gen_range(0..500);
            let pieces2 = randoms::gen_pieces(&mut rngs, count);

            let mut actual2 = [0; Piece::get_size()];
            for piece in &pieces2 {
                actual2[*piece as usize] += 1;
            }

            if actual2.iter().any(|&count| count >= 128) {
                // PieceCounter doesn't support more than 128 pieces of the same type
                continue;
            }

            let mut is_child_1 = true;
            let mut is_child_2 = true;
            for piece in Piece::value_list() {
                is_child_1 &= actual1[*piece as usize] <= actual2[*piece as usize];
                is_child_2 &= actual2[*piece as usize] <= actual1[*piece as usize];
            }

            let counter1 = PieceCounter::from(pieces1);
            let counter2 = PieceCounter::from(pieces2);
            assert_eq!(counter1.contains_all(&counter2), is_child_2);
            assert_eq!(counter2.contains_all(&counter1), is_child_1);
        }
    }
}
