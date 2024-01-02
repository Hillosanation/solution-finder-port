//! Many functions can just use rand functions directly
//! nextBoolean -> rand::Rng::gen_bool
//! nextIntOpen/nextIntClosed -> rand::Rng::gen_range
//! pick -> rand::seq::SliceRandom::choose
//! sample -> rand::seq::SliceRandom::choose_multiple
//! nextDouble -> rand::Rng::gen_range (unused)
//! block10InCycle is unused

use crate::sfinder_core::{
    field::{
        field::{Field, FIELD_WIDTH},
        field_factory, key_operators,
    },
    mino::piece::Piece,
    srs::rotate::Rotate,
};
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

const STRINGS: [char; 153] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '?', '/', '_', '-',
    '^', '¥', '=', '~', '|', '[', ']', '@', ':', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
    '!', '\\', '"', '#', '$', '%', '&', '\'', '(', ')', '<', '>', 'あ', 'い', 'う', 'え', 'お',
    'か', 'き', 'く', 'け', 'こ', 'さ', 'し', 'す', 'せ', 'そ', 'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ',
    'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', '朝', '午', '前', '後', '電', '話', '時',
    '計', '机', '携', '帯', '光', '空', '雨', '青', '赤', '車', '動', '力', '鉄', '　', '＿', '？',
    '：', '；', '：', '。', '＾', 'ー', '￥', '：', '＄', '”', '（', '）',
];

pub fn gen_piece(rngs: &mut ThreadRng) -> Piece {
    Piece::new(rngs.gen_range(0..Piece::get_size() as u8))
}

pub fn gen_pieces(rngs: &mut ThreadRng, size: usize) -> Vec<Piece> {
    (0..size).map(|_| gen_piece(rngs)).collect()
}

pub fn gen_rotate(rngs: &mut ThreadRng) -> Rotate {
    Rotate::new(rngs.gen_range(0..Rotate::get_size() as u8))
}

pub fn gen_field(rngs: &mut ThreadRng, height: u8, num_of_empty_minos: u8) -> Box<dyn Field> {
    assert!(num_of_empty_minos <= 10 * height / 4);

    let total_num_of_empty = num_of_empty_minos * 4;

    let mut num_of_empty_in_rows = vec![0; height as usize];
    let num_of_blocks = 10 * height - total_num_of_empty;
    if total_num_of_empty < num_of_blocks {
        // 空白のほうが少ないとき
        let mut count = 0;
        while count < total_num_of_empty {
            let index = rngs.gen_range(0..height) as usize;
            if num_of_empty_in_rows[index] < 10 {
                num_of_empty_in_rows[index] += 1;
                count += 1;
            }
        }
    } else {
        // ブロックのほうが少ないとき
        num_of_empty_in_rows = vec![10; height as usize];

        let mut count = 0;
        while count < num_of_blocks {
            let index = rngs.gen_range(0..height) as usize;
            if num_of_empty_in_rows[index] > 0 {
                num_of_empty_in_rows[index] -= 1;
                count += 1;
            }
        }
    }

    assert_eq!(num_of_empty_in_rows.iter().sum::<u8>(), total_num_of_empty);

    let mut field = field_factory::create_field(height);
    let mut prev_start = 0;
    let mut prev_end = 10;
    for y in (0..height).rev() {
        match num_of_empty_in_rows[y as usize] {
            // すべてのブロックを埋める
            0 => field.fill_row(y),
            // leave blank, do nothing
            FIELD_WIDTH => {}
            // 一部に空白をつくる
            count => {
                let min = (1u8 + prev_start).checked_sub(count).unwrap_or(0);
                let max = (FIELD_WIDTH - count).min(prev_end);

                let start = rngs.gen_range(min..max);
                assert!(start < FIELD_WIDTH);
                let end = start + count;
                assert!(end < FIELD_WIDTH);

                field.fill_row(y);
                for x in start..end {
                    field.remove_block(x, y);
                }

                prev_start = start;
                prev_end = end;
            }
        }
    }

    assert_eq!(
        height * FIELD_WIDTH - field.get_num_of_all_blocks() as u8,
        total_num_of_empty,
        "{field:?}"
    );

    field
}

pub fn gen_key(rngs: &mut ThreadRng) -> u64 {
    let value = rngs.gen_range(0..1 << 6);
    (0..6)
        .filter(|i| value & 1 << i != 0)
        .fold(0, |acc, i| acc | key_operators::get_delete_bit_key(i))
}

pub fn gen_char(rngs: &mut ThreadRng, size: usize) -> char {
    STRINGS[rngs.gen_range(0..STRINGS.len())]
}

pub fn block_11_in_cycle(rngs: &mut ThreadRng, cycle: u8) -> Vec<Piece> {
    fn get_11_cycle_counts(mut cycle: u8) -> Vec<u8> {
        if cycle == 0 {
            return vec![7, 4];
        }

        cycle %= 7;
        let prev_last_used = (10 * cycle + 1) % 7;
        let first_loop = 7 - prev_last_used;

        if first_loop <= 2 {
            vec![1, first_loop, 7, 3 - first_loop]
        } else {
            vec![1, first_loop, 10 - first_loop]
        }
    }

    get_11_cycle_counts(cycle)
        .into_iter()
        .flat_map(|count| Piece::value_list().choose_multiple(rngs, count as usize))
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::sfinder_core::field::bit_operators;

    use super::*;
    use rand::thread_rng;

    #[test]
    fn gen_piece_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let piece = gen_piece(&mut rngs);
            assert!(Piece::value_list().contains(&piece), "{piece}");
        }
    }

    #[test]
    fn gen_pieces_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let size = rngs.gen_range(3..19);
            let pieces = gen_pieces(&mut rngs, size);
            assert_eq!(pieces.len(), size);
        }
    }

    #[test]
    fn gen_rotate_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let rotate = gen_rotate(&mut rngs);
            assert!(Rotate::value_list().contains(&rotate), "{rotate}");
        }
    }

    #[test]
    fn gen_field_random() {
        let mut rngs = thread_rng();
        for _ in 0..10000 {
            let height = rngs.gen_range(1..=12);
            let num_of_minos = rngs.gen_range(1..height * 10 / 4);
            let random_field = gen_field(&mut rngs, height, num_of_minos);
            assert_eq!(
                random_field.get_num_of_all_blocks(),
                (10 * height - 4 * num_of_minos) as u32,
                "{random_field:?}"
            );
        }
    }

    #[test]
    fn gen_key_random() {
        let mut rngs = thread_rng();
        let mask = bit_operators::repeat_rows(0b1111111110);
        for _ in 0..10000 {
            let key = gen_key(&mut rngs);
            assert_eq!(key & mask, 0);
        }
    }

    mod pieces_11 {
        use super::*;
        use std::collections::HashSet;

        fn assert_unique(slices: &[&[Piece]]) {
            for &slice in slices {
                assert_eq!(
                    HashSet::<_, nohash::BuildNoHashHasher<Piece>>::from_iter(slice).len(),
                    slice.len()
                );
            }
        }

        #[test]
        fn first() {
            let mut rngs = thread_rng();
            for _ in 0..100000 {
                let pieces = block_11_in_cycle(&mut rngs, 0);
                assert_eq!(pieces.len(), 11);
                assert_unique(&[&pieces[0..7], &pieces[7..11]]);
            }
        }

        #[test]
        fn second() {
            let mut rngs = thread_rng();
            for _ in 0..100000 {
                let pieces = block_11_in_cycle(&mut rngs, 1);
                assert_eq!(pieces.len(), 11);
                assert_unique(&[&pieces[0..1], &pieces[1..4], &pieces[4..11]]);
            }
        }

        #[test]
        fn third() {
            let mut rngs = thread_rng();
            for _ in 0..100000 {
                let pieces = block_11_in_cycle(&mut rngs, 2);
                assert_eq!(pieces.len(), 11);
                assert_unique(&[&pieces[0..1], &pieces[1..8], &pieces[8..11]]);
            }
        }

        #[test]
        fn fourth() {
            let mut rngs = thread_rng();
            for _ in 0..100000 {
                let pieces = block_11_in_cycle(&mut rngs, 3);
                assert_eq!(pieces.len(), 11);
                assert_unique(&[&pieces[0..1], &pieces[1..5], &pieces[5..11]]);
            }
        }

        #[test]
        fn fifth() {
            let mut rngs = thread_rng();
            for _ in 0..100000 {
                let pieces = block_11_in_cycle(&mut rngs, 4);
                assert_eq!(pieces.len(), 11);
                assert_unique(&[&pieces[0..1], &pieces[1..2], &pieces[2..9], &pieces[9..11]]);
            }
        }

        #[test]
        fn sixth() {
            let mut rngs = thread_rng();
            for _ in 0..100000 {
                let pieces = block_11_in_cycle(&mut rngs, 5);
                assert_eq!(pieces.len(), 11);
                assert_unique(&[&pieces[0..1], &pieces[1..6], &pieces[6..11]]);
            }
        }

        #[test]
        fn seventh() {
            let mut rngs = thread_rng();
            for _ in 0..100000 {
                let pieces = block_11_in_cycle(&mut rngs, 6);
                assert_eq!(pieces.len(), 11);
                assert_unique(&[
                    &pieces[0..1],
                    &pieces[1..3],
                    &pieces[3..10],
                    &pieces[10..11],
                ]);
            }
        }

        #[test]
        fn eighth() {
            let mut rngs = thread_rng();
            for _ in 0..100000 {
                let pieces = block_11_in_cycle(&mut rngs, 7);
                assert_eq!(pieces.len(), 11);
                assert_unique(&[&pieces[0..1], &pieces[1..7], &pieces[7..11]]);
            }
        }
    }
}
