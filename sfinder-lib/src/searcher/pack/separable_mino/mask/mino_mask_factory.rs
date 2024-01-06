use super::{mino_mask::MinoMask, small_mino_mask::SmallMinoMask};
use crate::{
    searcher::pack::separable_mino::mask::middle_mino_mask::MiddleMinoMask,
    sfinder_core::{field::field_constants::BOARD_HEIGHT, mino::mino::Mino},
};

pub fn create(max_height: u8, mino: &Mino, y: u8, delete_key: u64) -> Box<dyn MinoMask> {
    const MAX_MIDDLE_HEIGHT: u8 = BOARD_HEIGHT * 2;

    match max_height {
        ..=BOARD_HEIGHT => Box::new(SmallMinoMask::new(mino, y, delete_key)),
        ..=MAX_MIDDLE_HEIGHT => Box::new(MiddleMinoMask::new(mino, y, delete_key)),
        _ => panic!("max_height is too large (> {MAX_MIDDLE_HEIGHT})"),
    }
}
