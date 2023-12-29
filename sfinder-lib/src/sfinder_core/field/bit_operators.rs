/// Porting note: Helper function to writing repeating a bit pattern for all rows on a board. This should be evaluated at compile time.
pub const fn repeat_rows(row_mask: u64) -> u64 {
    // assert cannot be used in const fn
    if row_mask & !0x3ff != 0 {
        panic!("invalid row mask");
    }
    row_mask | row_mask << 10 | row_mask << 20 | row_mask << 30 | row_mask << 40 | row_mask << 50
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

/// Panics if max_y > 6
// y行より下の1列ブロックマスクを取得する（y行を含まない）
pub const fn get_column_one_row_below_y(max_y: u8) -> u64 {
    // // replace 6 with FIELD_HEIGHT?
    // assert!(max_y <= 6);
    // if max_y == 0 {
    //     0
    // } else {
    //     repeat_rows(0b0000000001) & (1 << (max_y - 1) * 10 - 1)
    // }

    // TODO: try removing precomputation
    COLUMN_ONE_LINE_BELOW[max_y as usize]
}

/// TODO: refactor using this function, usually you want to use this, but it can block refactoring of bitshifts cancelling out
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
/// Panics if x > 10
/// Porting note: replaces getColumnMaskRightX
pub const fn get_column_mask_right_of_row(min_x: u8) -> u64 {
    COLUMN_KEYS_RIGHT[min_x as usize]

    // TODO: try removing precomputation
    // repeat_rows(0b1111111111 - (1 << min_x - 1))
}

/// Panics if max_x > 10
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

/// Panics if max_y > 6
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
    (board != 0).then(|| (board.trailing_zeros() / 10) as _)
}

/// Panics if board == 0
pub fn get_highest_y(board: u64) -> u8 {
    try_get_highest_y(board).unwrap()
}

pub fn try_get_highest_y(board: u64) -> Option<u8> {
    board.checked_ilog2().map(|index| (index / 10) as _)
}

pub fn try_get_lowest_x(mut board: u64) -> Option<u8> {
    (board != 0).then(|| {
        // fold the 60 bits into a single row
        board |= board >> 20;
        board |= board >> 20;
        board |= board >> 10;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_core::field::field::{Field, FieldHelper};
    use rand::{thread_rng, Rng};

    #[test]
    fn test_get_column_one_row_below_y() {
        for y in 0..=6 {
            let mask = get_column_one_row_below_y(y);

            // y行より下の行が含まれることを確認
            for line in 0..y {
                assert_ne!(mask & 1 << (line * 10), 0);
            }

            // y行を含めた上の行が含まれないことを確認
            for line in y..=6 {
                assert_eq!(mask & 1 << (line * 10), 0);
            }
        }
    }

    #[test]
    fn test_get_column_mask_right_of_row() {
        for x in 0..=10 {
            let mask = get_column_mask_right_of_row(x);

            // x列より左の列が含まれないことを確認
            for column in 0..x {
                for y in 0..6 {
                    assert_eq!(mask & 1 << (y * 10 + column), 0);
                }
            }

            // x列を含めた右の列が含まれることを確認
            for column in x..10 {
                for y in 0..6 {
                    assert_ne!(mask & 1 << (y * 10 + column), 0);
                }
            }
        }
    }

    #[test]
    fn test_get_column_mask_left_of_row() {
        for x in 0..=10 {
            let mask = get_column_mask_left_of_row(x);

            for column in 0..x {
                for y in 0..6 {
                    assert_ne!(mask & 1 << (y * 10 + column), 0);
                }
            }

            for column in x..10 {
                for y in 0..6 {
                    assert_eq!(mask & 1 << (y * 10 + column), 0);
                }
            }
        }
    }

    #[test]
    fn test_get_row_mask_below_y() {
        for y in 0..=6 {
            let mask = get_row_mask_below_y(y);

            // y行を含めた下の行が含まれることを確認
            for line in 0..y {
                for x in 0..10 {
                    assert_ne!(mask & 1 << (line * 10 + x), 0);
                }
            }

            // y行より上の行が含まれないことを確認
            for line in y..6 {
                for x in 0..10 {
                    assert_eq!(mask & 1 << (line * 10 + x), 0);
                }
            }
        }
    }

    #[test]
    fn test_get_row_mask_above_y() {
        for y in 0..=6 {
            let mask = get_row_mask_above_y(y);

            // y行より下の行が含まれないことを確認
            for line in 0..y {
                for x in 0..10 {
                    assert_eq!(mask & 1 << (line * 10 + x), 0);
                }
            }

            // y行を含めた上の行が含まれることを確認
            for line in y..6 {
                for x in 0..10 {
                    assert_ne!(mask & 1 << (line * 10 + x), 0);
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
            assert_ne!(board & <dyn Field>::get_row_mask(y), 0);
            assert!(
                y == 0 || board & <dyn Field>::get_row_mask(y - 1) == 0,
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
            assert_ne!(board & <dyn Field>::get_row_mask(y), 0);
            assert!(
                y == 6 || board & <dyn Field>::get_row_mask(y + 1) == 0,
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
            assert_ne!(board & get_column_mask(6, min_x), 0);
            for x in 0..min_x {
                assert_eq!(board & get_column_mask(6, x), 0);
            }
        }
    }

    mod legacy {
        use super::*;

        // legacy_bit_to_[xy] seems to be slightly faster from microbenching, but the following are better when there are 4
        // boardのうち1ビットがオンになっているとき、そのビットのy座標を返却
        fn legacy_bit_to_y(bit: u64) -> u8 {
            assert_eq!(bit.count_ones(), 1);
            (bit.trailing_zeros() / 10) as _
        }

        fn legacy_bit_to_x(bit: u64) -> u8 {
            assert_eq!(bit.count_ones(), 1);
            (bit.trailing_zeros() % 10) as _
        }

        #[test]
        fn bit_to_y_agrees() {
            for y in 0..6 {
                for x in 0..10 {
                    let bit = <dyn Field>::get_x_mask(x, y);
                    assert_eq!(legacy_bit_to_y(bit), get_lowest_y(bit));
                    assert_eq!(legacy_bit_to_y(bit), get_highest_y(bit));
                }
            }
        }

        #[test]
        fn bit_to_x_agrees() {
            for y in 0..6 {
                for x in 0..10 {
                    let bit = <dyn Field>::get_x_mask(x, y);
                    // println!("{bit:060b}");
                    assert_eq!(legacy_bit_to_x(bit), try_get_lowest_x(bit).unwrap());
                }
            }
        }

        #[test]
        fn test_bit_to_y() {
            for y in 0..6 {
                for x in 0..10 {
                    let bit = <dyn Field>::get_x_mask(x, y);
                    let actual_y = legacy_bit_to_y(bit);
                    assert_eq!(actual_y, y);
                }
            }
        }
    }
}
