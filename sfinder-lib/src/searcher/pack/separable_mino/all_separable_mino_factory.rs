//! Helper struct used only by SeparableMinos.
//! TODO(#15): Since this is only used by SeparableMinos, change the output to FullOperationWithKey.
//! In SeparableMinos, the FullOperationWithKey is the only part that is used, and the calculations done in FullOperationSeparableMino are not used.

use super::full_operation_separable_mino::FullOperationSeparableMino;
use crate::{
    common::datastore::full_operation_with_key::FullOperationWithKey,
    sfinder_core::{
        field::key_operators,
        mino::{mino::Mino, mino_factory::MinoFactory, mino_shifter::MinoShifter, piece::Piece},
    },
};

fn insert_pieces_each_mino<'a>(
    field_width: u8,
    field_height: u8,
    delete_key_mask: u64,
    pieces: &mut Vec<FullOperationSeparableMino<'a>>,
    mino: &'a Mino,
    mino_height: i8,
) {
    // println!(
    //     "mino_height {mino_height}, field_height {}, width {}",
    //     field_height, field_width
    // );
    let mut pattern = (1u64 << mino_height) - 1;
    let end = 1 << field_height;

    // Porting note: this used to use CombinationIterable, but I use bit twiddling directly
    // Retrived from http://www.graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation
    fn next_bit_pattern(v: u64) -> u64 {
        let t = (v | (v - 1)) as i64; // t gets v's least significant 0 bits set to 1

        // Next set to 1 the most significant bit to change,
        // set to 0 the least significant ones, and add the necessary 1 bits.
        ((t + 1) | (((!t & -!t) - 1) >> (v.trailing_zeros() + 1))) as _
    }

    while pattern < end {
        // println!("{:b}", pattern);

        // 一番下の行と一番上の行を取得
        let lower_y = pattern.trailing_zeros() as u8;
        let upper_y = pattern.ilog2() as u8;

        // we work with column keys here to save the effort of converting to bit keys to the end
        let range_mask = pattern.next_power_of_two() - (1 << lower_y);

        let delete_key = key_operators::to_bit_key(!pattern & range_mask);
        let using_key = key_operators::to_bit_key(pattern);

        debug_assert_eq!(
            (delete_key.count_ones() + pattern.count_ones()) as u8,
            upper_y - lower_y + 1
        );

        if delete_key_mask & delete_key == delete_key {
            for x in u8::try_from(0 - mino.get_min_x()).unwrap()
                ..u8::try_from(field_width as i8 - mino.get_min_x()).unwrap()
            {
                let y = u8::try_from(lower_y as i8 - mino.get_min_y()).unwrap();
                pieces.push(FullOperationSeparableMino::new(
                    FullOperationWithKey::new(mino, x, y, delete_key, using_key),
                    upper_y,
                    field_height,
                ))
            }
        }

        pattern = next_bit_pattern(pattern);
    }
}

// No need to use a set, since this is wrapped later by SeparableMinos in a BTreeSet anyways
pub fn create<'a>(
    mino_factory: &'a MinoFactory,
    mino_shifter: &'a MinoShifter,
    field_width: u8,
    field_height: u8,
    delete_key_mask: u64,
) -> Vec<FullOperationSeparableMino<'a>> {
    let mut pieces = Vec::new();

    for &piece in Piece::value_list() {
        for rotate in mino_shifter.get_unique_rotates(piece) {
            let mino = mino_factory.get(piece, rotate);

            // ミノの高さを計算
            let mino_height = mino.get_max_y() - mino.get_min_y() + 1;

            // フィールドの高さ以上にミノを使う場合はおけない
            if mino_height > field_height as i8 {
                continue;
            }

            // 追加
            // let prev_len = pieces.len();
            insert_pieces_each_mino(
                field_width,
                field_height,
                delete_key_mask,
                &mut pieces,
                mino,
                mino_height,
            );

            // println!("{piece} {rotate} added: {}", pieces.len() - prev_len);
        }
    }

    pieces
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::searcher::pack::separable_mino::separable_mino::SeparableMino;

    fn test_counts(width: u8, height: u8, counts: [usize; Piece::get_size()]) {
        let mino_factory = MinoFactory::new();
        let mino_shifter = MinoShifter::new();
        let mask = key_operators::get_mask_for_key_below_y(height);

        let pieces = create(&mino_factory, &mino_shifter, width, height, mask);

        let actual_counts: [usize; 7] = std::array::from_fn(|i| {
            let piece = Piece::new(i as u8);
            pieces
                .iter()
                .filter(|p| p.get_mino_operation_with_key().get_piece() == piece)
                .count()
        });

        assert_eq!(actual_counts, counts);

        let total = counts.iter().sum::<usize>();
        assert_eq!(pieces.len(), total, "{pieces:?}");
    }

    #[test]
    fn create2x3() {
        test_counts(2, 3, [16, 6, 16, 16, 8, 8, 6]);
    }

    #[test]
    fn create2x4() {
        test_counts(2, 4, [40, 10, 40, 40, 20, 20, 12]);
    }

    #[test]
    fn create2x5() {
        test_counts(2, 5, [80, 20, 80, 80, 40, 40, 20]);
    }

    #[test]
    fn create3x3() {
        test_counts(3, 3, [24, 9, 24, 24, 12, 12, 9]);
    }

    #[test]
    fn create3x4() {
        test_counts(3, 4, [60, 15, 60, 60, 30, 30, 18]);
    }

    #[test]
    fn create3x5() {
        test_counts(3, 5, [120, 30, 120, 120, 60, 60, 30]);
    }
}
