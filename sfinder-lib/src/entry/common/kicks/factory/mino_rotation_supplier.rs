use crate::{
    entry::common::kicks::{kick_patterns::KickPatterns, kick_type::KickType},
    sfinder_core::{
        mino::piece::Piece,
        srs::{
            mino_rotation::MinoRotation, mino_rotation_impl::MinoRotationImpl,
            mino_rotation_no_180_impl::MinoRotationNo180Impl, pattern::Pattern, rotate::Rotate,
            rotate_direction::RotateDirection,
        },
    },
};

type MapResult = Result<[Pattern; Piece::get_size() * Rotate::get_size()], KickType>;

const SIZE_90_ONLY: usize = Piece::get_size() * Rotate::get_size() * 2;
const SIZE_WITH_180: usize = Piece::get_size() * Rotate::get_size() * 3;

pub fn create_new_rotation(kick_patterns: KickPatterns) -> Result<Box<dyn MinoRotation>, String> {
    match kick_patterns.len() {
        SIZE_90_ONLY => Ok(Box::new(MinoRotationNo180Impl::new(
            create_map(&kick_patterns, RotateDirection::Clockwise).map_err(fmt_kick_type_error)?,
            create_map(&kick_patterns, RotateDirection::CounterClockwise)
                .map_err(fmt_kick_type_error)?,
        ))),
        SIZE_WITH_180 => Ok(Box::new(MinoRotationImpl::new(
            create_map(&kick_patterns, RotateDirection::Clockwise).map_err(fmt_kick_type_error)?,
            create_map(&kick_patterns, RotateDirection::CounterClockwise)
                .map_err(fmt_kick_type_error)?,
            create_map(&kick_patterns, RotateDirection::Rotate180).map_err(fmt_kick_type_error)?,
        ))),
        len => Err(format!("Invalid kick pattern size: {len}")),
    }
}

fn fmt_kick_type_error(kick_type: KickType) -> String {
    format!(
        "Invalid kick type: piece={:?}, from={:?}, to={:?}",
        kick_type.piece, kick_type.from, kick_type.to
    )
}

fn create_map(kick_patterns: &KickPatterns, direction: RotateDirection) -> MapResult {
    let result = (0..Piece::get_size() * Rotate::get_size())
        .map(|i| {
            let piece = Piece::new((i / Rotate::get_size()) as _);
            let from = Rotate::new((i % Rotate::get_size()) as _);

            let to = from.apply(direction);
            kick_patterns
                .get_pattern(piece, from, to)
                .cloned()
                .ok_or(KickType { piece, from, to })
        })
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        // never fails as size is determined by range
        .unwrap();

    Ok(result)
}
