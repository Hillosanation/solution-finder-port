use crate::sfinder_core::{field::field_constants::FIELD_WIDTH, mino::mino::Mino};

// TODO: This does not use Coordinate because it uses i8
#[cfg(test)]
pub fn walk(mino: &'static Mino, max_y: u8) -> impl Iterator<Item = (u8, u8)> {
    let (x_range, y_range) = get_ranges(mino, max_y);
    y_range.flat_map(move |y| x_range.clone().map(move |x| (x, y)))
}

// (x_range, y_range)
#[inline]
pub fn get_ranges(mino: &'static Mino, max_y: u8) -> (std::ops::Range<u8>, std::ops::Range<u8>) {
    (
        u8::try_from(-mino.get_min_x()).unwrap()
            ..u8::try_from(FIELD_WIDTH as i8 - mino.get_max_x()).unwrap(),
        u8::try_from(-mino.get_min_y()).unwrap()
            ..u8::try_from(max_y as i8 - mino.get_max_y()).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sfinder_core::{
        mino::{mino_factory::MinoFactory, piece::Piece},
        srs::rotate::Rotate,
    };
    use std::collections::HashSet;

    // Spawn, Right, Reverse, Left
    fn test_wrapper(piece: Piece, sizes: [usize; Rotate::get_size()]) {
        let mino_factory = MinoFactory::new();

        for (&size, &rotate) in sizes.iter().zip(Rotate::value_list()) {
            let mino = mino_factory.get(piece, rotate);
            assert_eq!(walk(mino, 4).count(), size);
        }
    }

    #[test]
    fn walk_size_with_i() {
        test_wrapper(Piece::I, [28, 10, 28, 10]);
    }

    #[test]
    fn walk_size_with_j() {
        test_wrapper(Piece::J, [24, 18, 24, 18]);
    }

    #[test]
    fn walk_size_with_l() {
        test_wrapper(Piece::L, [24, 18, 24, 18]);
    }

    #[test]
    fn walk_size_with_o() {
        test_wrapper(Piece::O, [27, 27, 27, 27]);
    }

    #[test]
    fn walk_size_with_s() {
        test_wrapper(Piece::S, [24, 18, 24, 18]);
    }

    #[test]
    fn walk_size_with_z() {
        test_wrapper(Piece::Z, [24, 18, 24, 18]);
    }

    #[test]
    fn walk_size_with_t() {
        test_wrapper(Piece::T, [24, 18, 24, 18]);
    }

    #[test]
    fn walk_contains_with_i() {
        let mino_factory = MinoFactory::new();
        let coordinates =
            walk(mino_factory.get(Piece::I, Rotate::Spawn), 4).collect::<HashSet<_>>();

        for coord in &[(1, 0), (7, 0), (1, 3), (7, 3)] {
            assert!(coordinates.contains(coord));
        }

        for coord in &[
            (0, 0),
            (8, 0),
            (0, 3),
            (8, 3),
            // impossible by type
            // (1, -1),
            // (7, -1),
            (1, 4),
            (7, 4),
        ] {
            assert!(!coordinates.contains(coord));
        }
    }
}
