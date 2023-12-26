use super::wrapper::wrapper;
use crate::sfinder_core::field::key_operators::get_bit_key;

const IS_FILLED_ON_DELETE_ROW: bool = true;

type BoolRows = [bool; 6];

fn parse_to_key(left_flags: BoolRows) -> u64 {
    let bitkey = left_flags
        .iter()
        .enumerate()
        .filter(|(_, flag)| !*flag)
        // .inspect(|(i, _)| println!("inspect {i}"))
        .map(|(i, _)| get_bit_key(i as u8))
        .fold(0, std::ops::BitOr::bitor);

    wrapper(bitkey)
}

fn create_left_flags(pattern: u8) -> BoolRows {
    // std::array::from_fn(|i| (pattern & (1 << i)) != 0)
    let mut booleans = [false; 6];
    let mut value = pattern;
    for i in 0..6 {
        booleans[i] = (value & 1) != 0;
        value >>= 1;
    }

    booleans
}

fn create_operation_insert(
    left_start: &[u8],
    left_rows: &[u8],
    left_flags: BoolRows,
) -> Vec<String> {
    create_operation(left_start, left_rows, left_flags, true)
}

fn create_operation(
    left_start: &[u8],
    left_rows: &[u8],
    left_flags: BoolRows,
    insert: bool, // insert, else delete
) -> Vec<String> {
    let mut operations = Vec::new();

    // 残ったブロックを移動させる
    for (&new_start, row) in left_start.iter().zip(left_rows.iter()) {
        let src_start = row;
        let slide = (new_start - src_start) * 10;
        let mask = ((1u64 << (row * 10)) - 1) << (src_start * 10);

        if new_start + row == 6 && new_start == 0 {
            operations.push("x".to_owned());
        } else {
            if insert {
                if slide == 0 {
                    operations.push(format!("x & {mask:#x}"));
                } else {
                    operations.push(format!("(x & {mask:#x}) << {slide}"));
                }
            } else {
                if new_start == 0 {
                    // equivalent to slide?
                    operations.push(format!("x & {mask:#x}"));
                } else {
                    operations.push(format!("(x & {mask:#x}) >> {slide}"));
                }
            }
        }
    }

    // 消えたブロックを復元させる
    if insert && IS_FILLED_ON_DELETE_ROW {
        operations.push(format!(
            "{:#x}",
            left_flags
                .iter()
                .enumerate()
                .filter_map(|(i, flag)| (!flag).then_some(0x3ffu64 << (10 * i)))
                .fold(0, std::ops::BitOr::bitor)
        ));
    }

    if operations.is_empty() {
        operations.push("0".to_owned());
    }

    return operations;
}

// no need to use map, the keys wont collide
fn create_bit_operation_map() -> Vec<(u64, String)> {
    (0..1 << 6)
        .map(|pattern| {
            let left_flags = create_left_flags(pattern);

            // ブロックで残し始めるインデックスと行数
            let mut left_start = Vec::new();
            let mut left_rows = vec![0]; // prefix sum
            let mut count = 0;

            for (i, &flag) in left_flags.iter().enumerate() {
                if flag {
                    if count == 0 {
                        left_start.push(i as u8);
                    }
                    count += 1;
                } else {
                    if count != 0 {
                        // always push prefix sum here, instead of just getting the count
                        left_rows.push(left_rows.last().unwrap() + count);
                    }
                    count = 0;
                }
            }
            if count != 0 {
                left_rows.push(count);
            }

            // println!("pattern: {:#06b}", pattern);
            // println!("start: {left_start:?}\nrow: {left_rows:?}");

            // ビット操作に変換する
            let operation = create_operation_insert(&left_start, &left_rows, left_flags);

            // flagsからkeyに変換
            let key = parse_to_key(left_flags);

            (key, operation.join(" | "))
        })
        .collect()
}

fn run() {
    println!("match x {{");
    for (key, operation) in create_bit_operation_map() {
        println!("    {:#024b} => {operation},", key);
    }
    println!("    _ => unreachable!(),");
    println!("}}");
}

fn insert_empty(x: u64) -> u64 {
    match x {
        0b1100000000110000000011 => 0,
        0b1100000000110000000010 => x & 0x3ff,
        0b1100000000100000000011 => (x & 0x3ff) << 10,
        0b1100000000100000000010 => x & 0xfffff,
        0b1000000000110000000011 => (x & 0x3ff) << 20,
        0b1000000000110000000010 => x & 0x3ff | (x & 0xffc00) << 10,
        0b1000000000100000000011 => (x & 0xfffff) << 10,
        0b1000000000100000000010 => x & 0x3fffffff,
        0b1100000000110000000001 => (x & 0x3ff) << 30,
        0b1100000000110000000000 => x & 0x3ff | (x & 0xffc00) << 20,
        0b1100000000100000000001 => (x & 0x3ff) << 10 | (x & 0xffc00) << 20,
        0b1100000000100000000000 => x & 0xfffff | (x & 0x3ff00000) << 10,
        0b1000000000110000000001 => (x & 0xfffff) << 20,
        0b1000000000110000000000 => x & 0x3ff | (x & 0x3ffffc00) << 10,
        0b1000000000100000000001 => (x & 0x3fffffff) << 10,
        0b1000000000100000000000 => x & 0xffffffffff,
        0b1100000000010000000011 => (x & 0x3ff) << 40,
        0b1100000000010000000010 => x & 0x3ff | (x & 0xffc00) << 30,
        0b1100000000000000000011 => (x & 0x3ff) << 10 | (x & 0xffc00) << 30,
        0b1100000000000000000010 => x & 0xfffff | (x & 0x3ff00000) << 20,
        0b1000000000010000000011 => (x & 0x3ff) << 20 | (x & 0xffc00) << 30,
        0b1000000000010000000010 => x & 0x3ff | (x & 0xffc00) << 10 | (x & 0x3ff00000) << 20,
        0b1000000000000000000011 => (x & 0xfffff) << 10 | (x & 0x3ff00000) << 20,
        0b1000000000000000000010 => x & 0x3fffffff | (x & 0xffc0000000) << 10,
        0b1100000000010000000001 => (x & 0xfffff) << 30,
        0b1100000000010000000000 => x & 0x3ff | (x & 0x3ffffc00) << 20,
        0b1100000000000000000001 => (x & 0x3ff) << 10 | (x & 0x3ffffc00) << 20,
        0b1100000000000000000000 => x & 0xfffff | (x & 0xfffff00000) << 10,
        0b1000000000010000000001 => (x & 0x3fffffff) << 20,
        0b1000000000010000000000 => x & 0x3ff | (x & 0xfffffffc00) << 10,
        0b1000000000000000000001 => (x & 0xffffffffff) << 10,
        0b1000000000000000000000 => x & 0x3ffffffffffff,
        0b0100000000110000000011 => (x & 0x3ff) << 50,
        0b0100000000110000000010 => x & 0x3ff | (x & 0xffc00) << 40,
        0b0100000000100000000011 => (x & 0x3ff) << 10 | (x & 0xffc00) << 40,
        0b0100000000100000000010 => x & 0xfffff | (x & 0x3ff00000) << 30,
        0b0000000000110000000011 => (x & 0x3ff) << 20 | (x & 0xffc00) << 40,
        0b0000000000110000000010 => x & 0x3ff | (x & 0xffc00) << 10 | (x & 0x3ff00000) << 30,
        0b0000000000100000000011 => (x & 0xfffff) << 10 | (x & 0x3ff00000) << 30,
        0b0000000000100000000010 => x & 0x3fffffff | (x & 0xffc0000000) << 20,
        0b0100000000110000000001 => (x & 0x3ff) << 30 | (x & 0xffc00) << 40,
        0b0100000000110000000000 => x & 0x3ff | (x & 0xffc00) << 20 | (x & 0x3ff00000) << 30,
        0b0100000000100000000001 => {
            (x & 0x3ff) << 10 | (x & 0xffc00) << 20 | (x & 0x3ff00000) << 30
        }
        0b0100000000100000000000 => x & 0xfffff | (x & 0x3ff00000) << 10 | (x & 0xffc0000000) << 20,
        0b0000000000110000000001 => (x & 0xfffff) << 20 | (x & 0x3ff00000) << 30,
        0b0000000000110000000000 => x & 0x3ff | (x & 0x3ffffc00) << 10 | (x & 0xffc0000000) << 20,
        0b0000000000100000000001 => (x & 0x3fffffff) << 10 | (x & 0xffc0000000) << 20,
        0b0000000000100000000000 => x & 0xffffffffff | (x & 0x3ff0000000000) << 10,
        0b0100000000010000000011 => (x & 0xfffff) << 40,
        0b0100000000010000000010 => x & 0x3ff | (x & 0x3ffffc00) << 30,
        0b0100000000000000000011 => (x & 0x3ff) << 10 | (x & 0x3ffffc00) << 30,
        0b0100000000000000000010 => x & 0xfffff | (x & 0xfffff00000) << 20,
        0b0000000000010000000011 => (x & 0x3ff) << 20 | (x & 0x3ffffc00) << 30,
        0b0000000000010000000010 => x & 0x3ff | (x & 0xffc00) << 10 | (x & 0xfffff00000) << 20,
        0b0000000000000000000011 => (x & 0xfffff) << 10 | (x & 0xfffff00000) << 20,
        0b0000000000000000000010 => x & 0x3fffffff | (x & 0x3ffffc0000000) << 10,
        0b0100000000010000000001 => (x & 0x3fffffff) << 30,
        0b0100000000010000000000 => x & 0x3ff | (x & 0xfffffffc00) << 20,
        0b0100000000000000000001 => (x & 0x3ff) << 10 | (x & 0xfffffffc00) << 20,
        0b0100000000000000000000 => x & 0xfffff | (x & 0x3fffffff00000) << 10,
        0b0000000000010000000001 => (x & 0xffffffffff) << 20,
        0b0000000000010000000000 => x & 0x3ff | (x & 0x3fffffffffc00) << 10,
        0b0000000000000000000001 => (x & 0x3ffffffffffff) << 10,
        0b0000000000000000000000 => x,
        _ => unreachable!(),
    }
}

fn insert_filled(x: u64) -> u64 {
    match x {
        0b1100000000110000000011 => (1152921504606846975),
        0b1100000000110000000010 => x & 0x3ff | (1152921504606845952),
        0b1100000000100000000011 => (x & 0x3ff) << 10 | (1152921504605799423),
        0b1100000000100000000010 => x & 0xfffff | (1152921504605798400),
        0b1000000000110000000011 => (x & 0x3ff) << 20 | (1152921503534153727),
        0b1000000000110000000010 => x & 0x3ff | (x & 0xffc00) << 10 | (1152921503534152704),
        0b1000000000100000000011 => (x & 0xfffff) << 10 | (1152921503533106175),
        0b1000000000100000000010 => x & 0x3fffffff | (1152921503533105152),
        0b1100000000110000000001 => (x & 0x3ff) << 30 | (1152920406168961023),
        0b1100000000110000000000 => x & 0x3ff | (x & 0xffc00) << 20 | (1152920406168960000),
        0b1100000000100000000001 => (x & 0x3ff) << 10 | (x & 0xffc00) << 20 | (1152920406167913471),
        0b1100000000100000000000 => x & 0xfffff | (x & 0x3ff00000) << 10 | (1152920406167912448),
        0b1000000000110000000001 => (x & 0xfffff) << 20 | (1152920405096267775),
        0b1000000000110000000000 => x & 0x3ff | (x & 0x3ffffc00) << 10 | (1152920405096266752),
        0b1000000000100000000001 => (x & 0x3fffffff) << 10 | (1152920405095220223),
        0b1000000000100000000000 => x & 0xffffffffff | (1152920405095219200),
        0b1100000000010000000011 => (x & 0x3ff) << 40 | (1151796704211632127),
        0b1100000000010000000010 => x & 0x3ff | (x & 0xffc00) << 30 | (1151796704211631104),
        0b1100000000000000000011 => (x & 0x3ff) << 10 | (x & 0xffc00) << 30 | (1151796704210584575),
        0b1100000000000000000010 => x & 0xfffff | (x & 0x3ff00000) << 20 | (1151796704210583552),
        0b1000000000010000000011 => (x & 0x3ff) << 20 | (x & 0xffc00) << 30 | (1151796703138938879),
        0b1000000000010000000010 => {
            x & 0x3ff | (x & 0xffc00) << 10 | (x & 0x3ff00000) << 20 | (1151796703138937856)
        }
        0b1000000000000000000011 => {
            (x & 0xfffff) << 10 | (x & 0x3ff00000) << 20 | (1151796703137891327)
        }
        0b1000000000000000000010 => {
            x & 0x3fffffff | (x & 0xffc0000000) << 10 | (1151796703137890304)
        }
        0b1100000000010000000001 => (x & 0xfffff) << 30 | (1151795605773746175),
        0b1100000000010000000000 => x & 0x3ff | (x & 0x3ffffc00) << 20 | (1151795605773745152),
        0b1100000000000000000001 => {
            (x & 0x3ff) << 10 | (x & 0x3ffffc00) << 20 | (1151795605772698623)
        }
        0b1100000000000000000000 => x & 0xfffff | (x & 0xfffff00000) << 10 | (1151795605772697600),
        0b1000000000010000000001 => (x & 0x3fffffff) << 20 | (1151795604701052927),
        0b1000000000010000000000 => x & 0x3ff | (x & 0xfffffffc00) << 10 | (1151795604701051904),
        0b1000000000000000000001 => (x & 0xffffffffff) << 10 | (1151795604700005375),
        0b1000000000000000000000 => x & 0x3ffffffffffff | (1151795604700004352),
        0b0100000000110000000011 => (x & 0x3ff) << 50 | (1125899906842623),
        0b0100000000110000000010 => x & 0x3ff | (x & 0xffc00) << 40 | (1125899906841600),
        0b0100000000100000000011 => (x & 0x3ff) << 10 | (x & 0xffc00) << 40 | (1125899905795071),
        0b0100000000100000000010 => x & 0xfffff | (x & 0x3ff00000) << 30 | (1125899905794048),
        0b0000000000110000000011 => (x & 0x3ff) << 20 | (x & 0xffc00) << 40 | (1125898834149375),
        0b0000000000110000000010 => {
            x & 0x3ff | (x & 0xffc00) << 10 | (x & 0x3ff00000) << 30 | (1125898834148352)
        }
        0b0000000000100000000011 => {
            (x & 0xfffff) << 10 | (x & 0x3ff00000) << 30 | (1125898833101823)
        }
        0b0000000000100000000010 => x & 0x3fffffff | (x & 0xffc0000000) << 20 | (1125898833100800),
        0b0100000000110000000001 => (x & 0x3ff) << 30 | (x & 0xffc00) << 40 | (1124801468956671),
        0b0100000000110000000000 => {
            x & 0x3ff | (x & 0xffc00) << 20 | (x & 0x3ff00000) << 30 | (1124801468955648)
        }
        0b0100000000100000000001 => {
            (x & 0x3ff) << 10 | (x & 0xffc00) << 20 | (x & 0x3ff00000) << 30 | (1124801467909119)
        }
        0b0100000000100000000000 => {
            x & 0xfffff | (x & 0x3ff00000) << 10 | (x & 0xffc0000000) << 20 | (1124801467908096)
        }
        0b0000000000110000000001 => {
            (x & 0xfffff) << 20 | (x & 0x3ff00000) << 30 | (1124800396263423)
        }
        0b0000000000110000000000 => {
            x & 0x3ff | (x & 0x3ffffc00) << 10 | (x & 0xffc0000000) << 20 | (1124800396262400)
        }
        0b0000000000100000000001 => {
            (x & 0x3fffffff) << 10 | (x & 0xffc0000000) << 20 | (1124800395215871)
        }
        0b0000000000100000000000 => {
            x & 0xffffffffff | (x & 0x3ff0000000000) << 10 | (1124800395214848)
        }
        0b0100000000010000000011 => (x & 0xfffff) << 40 | (1099511627775),
        0b0100000000010000000010 => x & 0x3ff | (x & 0x3ffffc00) << 30 | (1099511626752),
        0b0100000000000000000011 => (x & 0x3ff) << 10 | (x & 0x3ffffc00) << 30 | (1099510580223),
        0b0100000000000000000010 => x & 0xfffff | (x & 0xfffff00000) << 20 | (1099510579200),
        0b0000000000010000000011 => (x & 0x3ff) << 20 | (x & 0x3ffffc00) << 30 | (1098438934527),
        0b0000000000010000000010 => {
            x & 0x3ff | (x & 0xffc00) << 10 | (x & 0xfffff00000) << 20 | (1098438933504)
        }
        0b0000000000000000000011 => {
            (x & 0xfffff) << 10 | (x & 0xfffff00000) << 20 | (1098437886975)
        }
        0b0000000000000000000010 => x & 0x3fffffff | (x & 0x3ffffc0000000) << 10 | (1098437885952),
        0b0100000000010000000001 => (x & 0x3fffffff) << 30 | (1073741823),
        0b0100000000010000000000 => x & 0x3ff | (x & 0xfffffffc00) << 20 | (1073740800),
        0b0100000000000000000001 => (x & 0x3ff) << 10 | (x & 0xfffffffc00) << 20 | (1072694271),
        0b0100000000000000000000 => x & 0xfffff | (x & 0x3fffffff00000) << 10 | (1072693248),
        0b0000000000010000000001 => (x & 0xffffffffff) << 20 | (1048575),
        0b0000000000010000000000 => x & 0x3ff | (x & 0x3fffffffffc00) << 10 | (1047552),
        0b0000000000000000000001 => (x & 0x3ffffffffffff) << 10 | (1023),
        0b0000000000000000000000 => x,
        _ => unreachable!(),
    }
}

#[test]
fn a() {
    run()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn legacy_parse_to_key(left_flags: BoolRows) -> u64 {
        let mut key = 0;

        for i in (0..left_flags.len()).rev() {
            key <<= 10;

            if !left_flags[i] {
                key |= 1;
            }
        }

        let lower_mask = (1 << 30) - 1;

        (key >> (30 - 1)) | (key & lower_mask)
    }

    // test function to check intuition
    fn running() {
        println!("{:0b}", legacy_parse_to_key(create_left_flags(0b011111)));
        println!("{:0b}", legacy_parse_to_key(create_left_flags(0b101111)));
        println!("{:0b}", legacy_parse_to_key(create_left_flags(0b110111)));
        println!("{:0b}", legacy_parse_to_key(create_left_flags(0b111011)));
        println!("{:0b}", legacy_parse_to_key(create_left_flags(0b111101)));
        println!("{:0b}", legacy_parse_to_key(create_left_flags(0b111110)));
    }

    // Should be equivalent to column keys, except for it folding the 6 columns into 3.
    #[test]
    fn parse_to_key_agrees() {
        for i in 0..64 {
            let left_flags = create_left_flags(i);
            let key = legacy_parse_to_key(left_flags);
            let key2 = parse_to_key(left_flags);

            assert_eq!(key, key2, "{key:0b}, {key2:0b}");
        }
    }
}
