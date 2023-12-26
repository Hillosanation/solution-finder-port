use super::wrapper::wrapper;
use crate::sfinder_core::field::key_operators::get_bit_key;

const IS_FILLED_ON_DELETE_ROW: bool = true;

type BoolRows = [bool; 6];

fn parse_to_key(left_flags: BoolRows) -> u64 {
    let bitkey = left_flags
        .iter()
        .enumerate()
        .filter(|(_, &flag)| flag)
        // .inspect(|(i, _)| println!("inspect {i}"))
        .map(|(i, _)| get_bit_key(i as u8))
        .fold(0, std::ops::BitOr::bitor);

    wrapper(bitkey)
}

fn create_left_flags(pattern: u8) -> BoolRows {
    std::array::from_fn(|i| (pattern & (1 << i)) == 0)
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
    for (&new_start, src_start) in left_start.iter().zip(left_rows.iter()) {
        let mask = ((1u64 << (src_start * 10)) - 1) << (src_start * 10);

        if new_start == 0 {
            if new_start + src_start == 6 {
                operations.push("x".to_owned());
            } else {
                operations.push(format!("x & {mask:#x}"));
            }
        } else {
            let slide = (new_start - src_start) * 10;

            if insert {
                operations.push(format!("(x & {mask:#x}) << {slide}"));
            } else {
                operations.push(format!("(x >> {slide:#x}) & {mask}"));
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
                .filter(|(_, &flag)| flag)
                .map(|(i, _)| 0x3ffu64 << (10 * i))
                .fold(0, std::ops::BitOr::bitor),
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
                    if count != 0 {
                        // always push prefix sum here, instead of just getting the count
                        left_rows.push(left_rows.last().unwrap() + count);
                    }
                    count = 0;
                } else {
                    if count == 0 {
                        left_start.push(i as u8);
                    }
                    count += 1;
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
        for result in [0b011111, 0b101111, 0b110111, 0b111011, 0b111101, 0b111110]
            .iter()
            .map(|flags| legacy_parse_to_key(legacy_create_left_flags(*flags)))
        {
            println!("{result:0b}");
        }
    }

    // Should be equivalent to column keys, except for it folding the 6 columns into 3.
    #[test]
    fn parse_to_key_agrees() {
        for i in 0..64 {
            let key = legacy_parse_to_key(legacy_create_left_flags(i));
            let key2 = parse_to_key(create_left_flags(i));

            assert_eq!(key, key2, "{key:0b}, {key2:0b}");
        }
    }

    fn legacy_create_left_flags(pattern: u8) -> BoolRows {
        let mut booleans = [false; 6];
        let mut value = pattern;
        for i in 0..6 {
            booleans[i] = (value & 1) != 0;
            value >>= 1;
        }

        booleans
    }

    #[test]
    fn create_left_flags_agrees() {
        for pattern in 0..1 << 6 {
            for i in 0..6 {
                assert_ne!(
                    create_left_flags(pattern)[i],
                    legacy_create_left_flags(pattern)[i]
                );
            }
        }
    }
}
