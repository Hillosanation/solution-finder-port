use crate::sfinder_core::{
    mino::piece::Piece,
    srs::{
        mino_rotation::MinoRotation,
        mino_rotation_no_180_impl::MinoRotationNo180Impl,
        offset_define::{self, OffsetType},
        pattern::_Pattern,
        rotate::Rotate,
        rotate_direction::RotateDirection,
    },
};

// Porting note: moved functions to module level, since MinoRotation is only generated once at the start, I am changing this to generate lazily

fn create_map(direction: RotateDirection) -> [_Pattern; Piece::get_size() * Rotate::get_size()] {
    std::array::from_fn(|i| {
        let piece = Piece::new((i / Rotate::get_size()) as _);
        let rotate = Rotate::new((i % Rotate::get_size()) as _);

        get_pattern(piece, rotate, rotate.apply(direction))
    })
}

fn get_pattern(piece: Piece, current: Rotate, next: Rotate) -> _Pattern {
    offset_define::create_pattern(OffsetType::from(piece), current, next)
}

pub fn create() -> Box<dyn MinoRotation> {
    Box::new(MinoRotationNo180Impl::new(
        create_map(RotateDirection::Clockwise),
        create_map(RotateDirection::CounterClockwise),
    ))
}
