#[cfg(test)]
pub fn walk(size: u8) -> impl Iterator<Item = Vec<bool>> {
    (0..(1 << size)).map(move |val| (0..size).map(move |i| val & (1 << i) == 0).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn walk_has_size() {
        for size in 1..=12 {
            assert_eq!(walk(size).count(), 2usize.pow(size as u32));
        }
    }

    #[test]
    fn walk_4() {
        let walk = walk(4).collect::<HashSet<_>>();

        assert_eq!(walk.len(), 16);
        for item in [
            vec![true, true, true, true],
            vec![true, true, true, false],
            vec![true, true, false, true],
            vec![true, true, false, false],
            vec![true, false, true, true],
            vec![true, false, true, false],
            vec![true, false, false, true],
            vec![true, false, false, false],
            vec![false, true, true, true],
            vec![false, true, true, false],
            vec![false, true, false, true],
            vec![false, true, false, false],
            vec![false, false, true, true],
            vec![false, false, true, false],
            vec![false, false, false, true],
            vec![false, false, false, false],
        ] {
            assert!(walk.contains(&item));
        }
    }
}
