use super::field_constants::FIELD_WIDTH;

/// Porting note: Helper function to writing repeating a bit pattern for all rows on a board. This should be evaluated at compile time.
pub const fn repeat_rows(row_mask: u64) -> u64 {
    // assert cannot be used in const fn
    if row_mask & !0x3ff != 0 {
        panic!("invalid row mask");
    }
    row_mask
        | row_mask << FIELD_WIDTH
        | row_mask << (2 * FIELD_WIDTH)
        | row_mask << (3 * FIELD_WIDTH)
        | row_mask << (4 * FIELD_WIDTH)
        | row_mask << (5 * FIELD_WIDTH)
}

// Note this is the same as the first entries in KEY_MASKS by coincidence
const COLUMN_ONE_LINE_BELOW: [u64; 7] = [
    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000001,
    0b0000000000_0000000000_0000000000_0000000000_0000000001_0000000001,
    0b0000000000_0000000000_0000000000_0000000001_0000000001_0000000001,
    0b0000000000_0000000000_0000000001_0000000001_0000000001_0000000001,
    0b0000000000_0000000001_0000000001_0000000001_0000000001_0000000001,
    0b0000000001_0000000001_0000000001_0000000001_0000000001_0000000001,
];

/// Panics if max_y > BOARD_HEIGHT
// y行より下の1列ブロックマスクを取得する（y行を含まない）
pub const fn get_column_one_row_below_y(max_y: u8) -> u64 {
    // assert!(max_y <= BOARD_HEIGHT);
    // if max_y == 0 {
    //     0
    // } else {
    //     repeat_rows(0b0000000001) & (1 << (max_y - 1) * FIELD_WIDTH - 1)
    // }

    // TODO: try removing precomputation
    COLUMN_ONE_LINE_BELOW[max_y as usize]
}

pub const fn get_column_mask(max_y: u8, x: u8) -> u64 {
    get_column_one_row_below_y(max_y) << x
}

const COLUMN_KEYS_RIGHT: [u64; 11] = [
    repeat_rows(0b1111111111),
    repeat_rows(0b1111111110),
    repeat_rows(0b1111111100),
    repeat_rows(0b1111111000),
    repeat_rows(0b1111110000),
    repeat_rows(0b1111100000),
    repeat_rows(0b1111000000),
    repeat_rows(0b1110000000),
    repeat_rows(0b1100000000),
    repeat_rows(0b1000000000),
    repeat_rows(0b0000000000),
];

// x列より右の列を選択するマスクを作成（x列を含む）
/// Panics if x > FIELD_WIDTH
/// Porting note: replaces getColumnMaskRightX
pub const fn get_column_mask_right_of_row(min_x: u8) -> u64 {
    COLUMN_KEYS_RIGHT[min_x as usize]

    // TODO: try removing precomputation
    // repeat_rows(0b1111111111 - (1 << min_x - 1))
}

/// Panics if max_x > FIELD_WIDTH
/// Porting note: replaces getColumnMaskLeftX
// x列より左の列を選択するマスクを作成（x列を含まない）
pub const fn get_column_mask_left_of_row(min_x: u8) -> u64 {
    // TODO: try removing precomputation
    repeat_rows(0b1111111111) - get_column_mask_right_of_row(min_x)
}

const ROW_MASK: [u64; 7] = [
    0b0000000000_0000000000_0000000000_0000000000_0000000000_0000000000,
    0b0000000000_0000000000_0000000000_0000000000_0000000000_1111111111,
    0b0000000000_0000000000_0000000000_0000000000_1111111111_1111111111,
    0b0000000000_0000000000_0000000000_1111111111_1111111111_1111111111,
    0b0000000000_0000000000_1111111111_1111111111_1111111111_1111111111,
    0b0000000000_1111111111_1111111111_1111111111_1111111111_1111111111,
    0b1111111111_1111111111_1111111111_1111111111_1111111111_1111111111,
];

/// Panics if max_y > BOARD_HEIGHT
// yより下の行を選択するマスクを作成 (y行は含まない)
pub const fn get_row_mask_below_y(y: u8) -> u64 {
    ROW_MASK[y as usize]
}

pub const fn get_row_mask_above_y(y: u8) -> u64 {
    repeat_rows(0b1111111111) - get_row_mask_below_y(y)
}

/// Paincs if board == 0
pub fn get_lowest_y(board: u64) -> u8 {
    try_get_lowest_y(board).unwrap()
}

pub fn try_get_lowest_y(board: u64) -> Option<u8> {
    (board != 0).then(|| board.trailing_zeros() as u8 / FIELD_WIDTH)
}

/// Panics if board == 0
pub fn get_highest_y(board: u64) -> u8 {
    try_get_highest_y(board).unwrap()
}

pub fn try_get_highest_y(board: u64) -> Option<u8> {
    board.checked_ilog2().map(|index| index as u8 / FIELD_WIDTH)
}

pub fn try_get_lowest_x(mut board: u64) -> Option<u8> {
    (board != 0).then(|| {
        // fold the 60 bits into a single row
        board |= board >> (2 * FIELD_WIDTH);
        board |= board >> (2 * FIELD_WIDTH);
        board |= board >> FIELD_WIDTH;

        board.trailing_zeros() as _
    })
}

// x列とその左の列の間が壁（隙間がない）とき true を返却。1 <= xであること
pub fn is_wall_between_left(x: u8, max_y: u8, board: u64) -> bool {
    let reverse_x_board_high = !board;
    let column_high = get_column_mask(max_y, x);
    let right_high = reverse_x_board_high & column_high;
    let left_high = reverse_x_board_high & (column_high >> 1);

    (left_high << 1) & right_high == 0
}

// Common functions manipulating a board.

pub const fn get_x_mask(x: u8, y: u8) -> u64 {
    1 << (x + y * FIELD_WIDTH)
}

pub const fn board_shl(board: u64, shift: u8) -> u64 {
    board << (shift * FIELD_WIDTH)
}

pub const fn board_shr(board: u64, shift: u8) -> u64 {
    board >> (shift * FIELD_WIDTH)
}

pub const fn get_row_mask(y: u8) -> u64 {
    board_shl(0x3ff, y)
}

#[cfg(test)]
mod tests {
    use crate::sfinder_core::field::field_constants::BOARD_HEIGHT;

    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_get_column_one_row_below_y() {
        for y in 0..=BOARD_HEIGHT {
            let mask = get_column_one_row_below_y(y);

            // y行より下の行が含まれることを確認
            for line in 0..y {
                assert_ne!(mask & get_x_mask(0, line), 0);
            }

            // y行を含めた上の行が含まれないことを確認
            for line in y..=BOARD_HEIGHT {
                assert_eq!(mask & get_x_mask(0, line), 0);
            }
        }
    }

    #[test]
    fn test_get_column_mask_right_of_row() {
        for x in 0..=FIELD_WIDTH {
            let mask = get_column_mask_right_of_row(x);

            // x列より左の列が含まれないことを確認
            for column in 0..x {
                for y in 0..BOARD_HEIGHT {
                    assert_eq!(mask & get_x_mask(column, y), 0);
                }
            }

            // x列を含めた右の列が含まれることを確認
            for column in x..FIELD_WIDTH {
                for y in 0..BOARD_HEIGHT {
                    assert_ne!(mask & get_x_mask(column, y), 0);
                }
            }
        }
    }

    #[test]
    fn test_get_column_mask_left_of_row() {
        for x in 0..=FIELD_WIDTH {
            let mask = get_column_mask_left_of_row(x);

            for column in 0..x {
                for y in 0..BOARD_HEIGHT {
                    assert_ne!(mask & get_x_mask(column, y), 0);
                }
            }

            for column in x..FIELD_WIDTH {
                for y in 0..BOARD_HEIGHT {
                    assert_eq!(mask & get_x_mask(column, y), 0);
                }
            }
        }
    }

    #[test]
    fn test_get_row_mask_below_y() {
        for y in 0..=BOARD_HEIGHT {
            let mask = get_row_mask_below_y(y);

            // y行を含めた下の行が含まれることを確認
            for line in 0..y {
                for x in 0..FIELD_WIDTH {
                    assert_ne!(mask & get_x_mask(x, line), 0);
                }
            }

            // y行より上の行が含まれないことを確認
            for line in y..BOARD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    assert_eq!(mask & get_x_mask(x, line), 0);
                }
            }
        }
    }

    #[test]
    fn test_get_row_mask_above_y() {
        for y in 0..=BOARD_HEIGHT {
            let mask = get_row_mask_above_y(y);

            // y行より下の行が含まれないことを確認
            for line in 0..y {
                for x in 0..FIELD_WIDTH {
                    assert_eq!(mask & get_x_mask(x, line), 0);
                }
            }

            // y行を含めた上の行が含まれることを確認
            for line in y..BOARD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    assert_ne!(mask & get_x_mask(x, line), 0);
                }
            }
        }
    }

    #[test]
    fn test_get_lowest_y() {
        let mut rngs = thread_rng();

        for _ in 0..1000 {
            let board = rngs.gen_range(1..1 << 60);
            let y = get_lowest_y(board);
            assert_ne!(board & get_row_mask(y), 0);
            assert!(
                y == 0 || board & get_row_mask(y - 1) == 0,
                "y: {y}, board: {board:060b}",
            )
        }
    }

    #[test]
    fn test_get_highest_y() {
        let mut rngs = thread_rng();

        for _ in 0..1000 {
            let board = rngs.gen_range(1..1 << 60);
            let y = get_highest_y(board);
            assert_ne!(board & get_row_mask(y), 0);
            assert!(
                y == 6 || board & get_row_mask(y + 1) == 0,
                "y: {y}, board: {board:060b}",
            )
        }
    }

    #[test]
    fn test_try_get_lowest_x() {
        let mut rngs = thread_rng();

        for _ in 0..1000 {
            let board = rngs.gen_range(1..1 << 60);
            let min_x = try_get_lowest_x(board).unwrap();

            // println!("{board:060b}, min: {min_x}");
            assert_ne!(board & get_column_mask(BOARD_HEIGHT, min_x), 0);
            for x in 0..min_x {
                assert_eq!(board & get_column_mask(BOARD_HEIGHT, x), 0);
            }
        }
    }

    mod legacy {
        use super::*;
        use crate::sfinder_core::field::field_constants::{BOARD_HEIGHT, VALID_BOARD_RANGE};

        // legacy_bit_to_[xy] seems to be slightly faster from amateur microbenching, but the following are better when there are 4
        // boardのうち1ビットがオンになっているとき、そのビットのy座標を返却
        fn legacy_bit_to_y(bit: u64) -> u8 {
            assert_eq!(bit.count_ones(), 1);
            bit.trailing_zeros() as u8 / FIELD_WIDTH
        }

        fn legacy_bit_to_x(bit: u64) -> u8 {
            assert_eq!(bit.count_ones(), 1);
            bit.trailing_zeros() as u8 % FIELD_WIDTH
        }

        #[test]
        fn bit_to_y_agrees() {
            for y in 0..BOARD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    let bit = get_x_mask(x, y);
                    assert_eq!(legacy_bit_to_y(bit), get_lowest_y(bit));
                    assert_eq!(legacy_bit_to_y(bit), get_highest_y(bit));
                }
            }
        }

        #[test]
        fn bit_to_x_agrees() {
            for y in 0..BOARD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    let bit = get_x_mask(x, y);
                    // println!("{bit:060b}");
                    assert_eq!(legacy_bit_to_x(bit), try_get_lowest_x(bit).unwrap());
                }
            }
        }

        #[test]
        fn test_bit_to_y() {
            for y in 0..BOARD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    let bit = get_x_mask(x, y);
                    let actual_y = legacy_bit_to_y(bit);
                    assert_eq!(actual_y, y);
                }
            }
        }

        // this is a refactor from FieldHelper::create_upper_board
        #[test]
        fn mask_before_shift_redundant() {
            let mut rngs = thread_rng();

            for _ in 0..10000 {
                // unset bits are not guarenteed
                let board: u64 = rngs.gen();

                for left_row in 0..=BOARD_HEIGHT {
                    assert_eq!(
                        board_shr(board & VALID_BOARD_RANGE, left_row),
                        board_shr(board & get_row_mask_above_y(left_row), left_row)
                    );
                }
            }
        }
    }
}
