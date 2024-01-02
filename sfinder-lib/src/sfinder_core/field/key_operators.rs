use crate::sfinder_core::field::bit_operators;

use super::bit_operators::repeat_rows;

/// Folds each row in the board to signify which rows are completely filled.
pub const fn get_delete_key(board: u64) -> u64 {
    let b0101010101 = (board & repeat_rows(0b1010101010)) >> 1 & board;
    let b0000010101 = (b0101010101 & repeat_rows(0b0101010000)) >> 4 & b0101010101;
    let b0000000101 = (b0000010101 & repeat_rows(0b0000010100)) >> 2 & b0000010101;
    (b0000000101 & repeat_rows(0b0000000100)) >> 2 & b0000000101
}

const KEY_MASKS: [u64; 25] = [
    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000001,
    0b0000000000_0000000000_0000000000_0000000000_0000000001_0000000001,
    0b0000000000_0000000000_0000000000_0000000001_0000000001_0000000001,
    0b0000000000_0000000000_0000000001_0000000001_0000000001_0000000001,
    0b0000000000_0000000001_0000000001_0000000001_0000000001_0000000001,
    0b0000000001_0000000001_0000000001_0000000001_0000000001_0000000001,
    0b0000000001_0000000001_0000000001_0000000001_0000000001_0000000011,
    0b0000000001_0000000001_0000000001_0000000001_0000000011_0000000011,
    0b0000000001_0000000001_0000000001_0000000011_0000000011_0000000011,
    0b0000000001_0000000001_0000000011_0000000011_0000000011_0000000011,
    0b0000000001_0000000011_0000000011_0000000011_0000000011_0000000011,
    0b0000000011_0000000011_0000000011_0000000011_0000000011_0000000011,
    0b0000000011_0000000011_0000000011_0000000011_0000000011_0000000111,
    0b0000000011_0000000011_0000000011_0000000011_0000000111_0000000111,
    0b0000000011_0000000011_0000000011_0000000111_0000000111_0000000111,
    0b0000000011_0000000011_0000000111_0000000111_0000000111_0000000111,
    0b0000000011_0000000111_0000000111_0000000111_0000000111_0000000111,
    0b0000000111_0000000111_0000000111_0000000111_0000000111_0000000111,
    0b0000000111_0000000111_0000000111_0000000111_0000000111_0000001111,
    0b0000000111_0000000111_0000000111_0000000111_0000001111_0000001111,
    0b0000000111_0000000111_0000000111_0000001111_0000001111_0000001111,
    0b0000000111_0000000111_0000001111_0000001111_0000001111_0000001111,
    0b0000000111_0000001111_0000001111_0000001111_0000001111_0000001111,
    0b0000001111_0000001111_0000001111_0000001111_0000001111_0000001111,
];

// y行上のブロックは対象に含まない
// TODO: Make a better type wrapper for Keys
/// Crashes if y > 24
pub const fn get_mask_for_key_below_y(y: u8) -> u64 {
    KEY_MASKS[y as usize]
}

// y行上のブロックは対象に含む
pub const fn get_mask_for_key_above_y(y: u8) -> u64 {
    repeat_rows(0b0000001111) - get_mask_for_key_below_y(y)
}

#[cfg(test)]
pub fn get_bit_keys(ys: &[u8]) -> u64 {
    ys.iter()
        .map(|&y| get_bit_key(y))
        .fold(0, std::ops::BitOr::bitor)
}

// TODO (#3): inline this function
pub const fn get_delete_bit_key(y: u8) -> u64 {
    get_bit_key(y)
}

const BIT_KEY_MASKS: [u64; 24] = [
    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000001,
    0b0000000000_0000000000_0000000000_0000000000_0000000001_0000000000,
    0b0000000000_0000000000_0000000000_0000000001_0000000000_0000000000,
    0b0000000000_0000000000_0000000001_0000000000_0000000000_0000000000,
    0b0000000000_0000000001_0000000000_0000000000_0000000000_0000000000,
    0b0000000001_0000000000_0000000000_0000000000_0000000000_0000000000,
    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000010,
    0b0000000000_0000000000_0000000000_0000000000_0000000010_0000000000,
    0b0000000000_0000000000_0000000000_0000000010_0000000000_0000000000,
    0b0000000000_0000000000_0000000010_0000000000_0000000000_0000000000,
    0b0000000000_0000000010_0000000000_0000000000_0000000000_0000000000,
    0b0000000010_0000000000_0000000000_0000000000_0000000000_0000000000,
    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000100,
    0b0000000000_0000000000_0000000000_0000000000_0000000100_0000000000,
    0b0000000000_0000000000_0000000000_0000000100_0000000000_0000000000,
    0b0000000000_0000000000_0000000100_0000000000_0000000000_0000000000,
    0b0000000000_0000000100_0000000000_0000000000_0000000000_0000000000,
    0b0000000100_0000000000_0000000000_0000000000_0000000000_0000000000,
    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000001000,
    0b0000000000_0000000000_0000000000_0000000000_0000001000_0000000000,
    0b0000000000_0000000000_0000000000_0000001000_0000000000_0000000000,
    0b0000000000_0000000000_0000001000_0000000000_0000000000_0000000000,
    0b0000000000_0000001000_0000000000_0000000000_0000000000_0000000000,
    0b0000001000_0000000000_0000000000_0000000000_0000000000_0000000000,
];

// TODO (#4): check if using 1 << (y % 6 * 10 + y / 6) instead is good enough
/// Panics if y > 23
pub const fn get_bit_key(y: u8) -> u64 {
    BIT_KEY_MASKS[y as usize]
}

pub const fn mirror(mut field: u64) -> u64 {
    field = (field & repeat_rows(0b1111100000)) >> 5 | (field & repeat_rows(0b0000011111)) << 5;

    let fixed = field & repeat_rows(0b0010000100);

    field &= repeat_rows(0b1101111011);

    field = (field & repeat_rows(0b1100011000)) >> 3 | (field & repeat_rows(0b0001100011)) << 3;

    field = (field & repeat_rows(0b1001010010)) >> 1 | (field & repeat_rows(0b0100101001)) << 1;

    field | fixed
}

// The last step is necessary because we are ORing the cells unlike delete key,
pub const fn get_using_key(board: u64) -> u64 {
    let b0101010101 = (board & repeat_rows(0b1010101010)) >> 1 | board;
    let b0000010101 = (b0101010101 & repeat_rows(0b0101010000)) >> 4 | b0101010101;
    let b0000000101 = (b0000010101 & repeat_rows(0b0000010100)) >> 2 | b0000010101;
    let b0000000001 = (b0000000101 & repeat_rows(0b0000000100)) >> 2 | b0000000101;

    b0000000001 & repeat_rows(0b0000000001)
}

// keyのうち1ビットがオンになっているとき、そのビットのy座標を返却
pub fn bit_to_y_from_key(key: u64) -> u8 {
    assert_eq!(
        (key & repeat_rows(0b0000001111)).count_ones(),
        1,
        "{key:0b}"
    );

    if let low @ 1.. = key & repeat_rows(0b0000000001) {
        bit_operators::get_lowest_y(low)
    } else if let mid_low @ 1.. = key & repeat_rows(0b0000000010) {
        bit_operators::get_lowest_y(mid_low >> 1) + 6
    } else if let mid_high @ 1.. = key & repeat_rows(0b0000000100) {
        bit_operators::get_lowest_y(mid_high >> 2) + 6 * 2
    } else {
        let high = key & repeat_rows(0b0000001000);
        bit_operators::get_lowest_y(high >> 3) + 6 * 3
    }
}

// reused in Field implementations
/// If you just want to get the most/least set bits, use `ilog2` and `trailing_zeros` instead.
pub(super) const fn get_lowest_bit(x: u64) -> u64 {
    // compiles down to the same thing as x & -x, getting the least significant bit, in -O
    (x as i64 & -(x as i64)) as u64
}

// keyのうち、最も低い行のbitを取り出す
pub fn extract_lower_bit(key: u64) -> u64 {
    assert!(
        (key & repeat_rows(0b0000001111)).count_ones() >= 1,
        "{key:0b}"
    );

    // although the return signature looks similar, the different branches give different results because each masks a different range of bits.
    if let low @ 1.. = key & repeat_rows(0b0000000001) {
        get_lowest_bit(low)
    } else if let mid_low @ 1.. = key & repeat_rows(0b0000000010) {
        get_lowest_bit(mid_low)
    } else if let mid_high @ 1.. = key & repeat_rows(0b0000000100) {
        get_lowest_bit(mid_high)
    } else {
        let high = key & repeat_rows(0b0000001000);
        get_lowest_bit(high)
    }
}

pub fn to_column_key(bit_key: u64) -> u64 {
    (0..24)
        .filter(|&y| bit_key & get_bit_key(y) != 0)
        .map(get_column_key)
        .fold(0, std::ops::BitOr::bitor)
}

pub const fn get_column_key(y: u8) -> u64 {
    1 << y
}

pub fn to_bit_key(column_key: u64) -> u64 {
    (0..24)
        .filter(|&y| column_key & get_column_key(y) != 0)
        .map(get_bit_key)
        .fold(0, std::ops::BitOr::bitor)
}

#[cfg(test)]
mod tests {
    use crate::{
        sfinder_core::field::{field::Field, small_field::SmallField},
        sfinder_lib::boolean_walker,
    };
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_get_delete_key() {
        let mut rngs = thread_rng();

        for booleans in boolean_walker::walk(6) {
            let mut field = SmallField::new();
            let mut expect_delete_key = 0;

            for y in 0..booleans.len() as u8 {
                if booleans[y as usize] {
                    // ラインを全て埋める
                    for x in 0..10 {
                        field.set_block(x, y);
                    }
                    expect_delete_key |= get_delete_bit_key(y);
                } else {
                    // ラインを全て埋めない
                    for x in 0..10 {
                        if rngs.gen_bool(0.8) {
                            field.set_block(x, y);
                        }
                    }
                    field.remove_block(rngs.gen_range(0..10), y);
                }

                let board = field.get_x_board();
                let delete_key = get_delete_key(board);
                assert_eq!(delete_key, expect_delete_key)
            }
        }
    }

    #[test]
    fn test_get_mask_for_key_below_y() {
        for y in 0..=24 {
            let mask = get_mask_for_key_below_y(y);

            // y行より下の行が含まれることを確認
            for line in 0..y {
                assert_ne!(mask & 1 << ((line % 6) * 10 + (line / 6)), 0);
            }

            // y行を含めた上の行が含まれないことを確認
            for line in y..24 {
                assert_eq!(mask & 1 << ((line % 6) * 10 + (line / 6)), 0);
            }
        }
    }

    #[test]
    fn test_get_mask_for_key_above_y() {
        for y in 0..=24 {
            let mask = get_mask_for_key_above_y(y);
            // println!("{mask:0b}");

            // y行より下の行が含まれないことを確認
            for line in 0..y {
                assert_eq!(mask & 1 << ((line % 6) * 10 + (line / 6)), 0);
            }

            // y行を含めた上の行が含まれることを確認
            for line in y..24 {
                assert_ne!(mask & 1 << ((line % 6) * 10 + (line / 6)), 0);
            }
        }
    }

    #[test]
    fn test_get_delete_bit_key() {
        for y in 0..24 {
            assert_eq!(get_delete_bit_key(y), 1 << ((y % 6) * 10 + (y / 6)));
        }
    }

    #[test]
    fn mirror_sample_case() {
        assert_eq!(
            mirror(0b001111100011111000001101010001),
            0b000111110000000111111000101011
        );
    }

    #[test]
    fn test_bit_to_y_from_key() {
        for y in 0..24 {
            let key = get_bit_key(y);
            assert_eq!(bit_to_y_from_key(key), y);
        }
    }

    #[test]
    fn test_extract_lower_bit() {
        let mut rngs = thread_rng();

        for y in 0..24 {
            let key = get_bit_key(y);

            let mut current = key;
            for dy in y + 1..24 {
                if rngs.gen_bool(0.5) {
                    current |= get_bit_key(dy);
                }
            }

            assert_eq!(extract_lower_bit(current), key);
        }
    }

    #[test]
    fn test_get_column_key() {
        assert_eq!(get_column_key(0), 0b1);
        assert_eq!(get_column_key(1), 0b10);
        assert_eq!(get_column_key(2), 0b100);
        assert_eq!(get_column_key(10), 0b10000000000);
    }

    #[test]
    fn test_to_column_key() {
        for y in 0..24 {
            let bit_key = get_bit_key(y);
            assert_eq!(to_column_key(bit_key), get_column_key(y));
        }
    }

    #[test]
    fn test_to_bit_key() {
        for y in 0..24 {
            let column_key = get_column_key(y);
            assert_eq!(to_bit_key(column_key), get_bit_key(y));
        }
    }

    #[test]
    fn masks_agree() {
        // Equivalence retrieved from common/generator/DeleteBitKeyGenerator.java
        for y in 0..24 {
            let delete_key = get_delete_bit_key(y);
            assert_eq!(delete_key.count_ones(), 1);
            assert_eq!(
                delete_key,
                get_mask_for_key_above_y(y) & get_mask_for_key_below_y(y + 1)
            );
        }
    }
}
