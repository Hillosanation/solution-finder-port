use crate::sfinder_core::{field::field_constants::FIELD_WIDTH, mino::mino::Mino};

// TODO: This does not use Coordinate because it uses i8
pub fn walk(mino: &'static Mino, max_y: u8) -> impl Iterator<Item = (u8, u8)> {
    (u8::try_from(-mino.get_min_y()).unwrap()
        ..u8::try_from(max_y as i8 - mino.get_max_y()).unwrap())
        .flat_map(move |y| {
            (u8::try_from(-mino.get_min_x()).unwrap()
                ..u8::try_from(FIELD_WIDTH as i8 - mino.get_max_x()).unwrap())
                .map(move |x| (x, y))
        })
}
