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

pub struct MinoRotationSupplier {
    kick_patterns: KickPatterns,
}

impl MinoRotationSupplier {
    pub fn new(kick_patterns: KickPatterns) -> Self {
        Self::validate(&kick_patterns).unwrap();
        Self { kick_patterns }
    }

    fn validate(kick_patterns: &KickPatterns) -> Result<(), String> {
        let len = kick_patterns.len();
        if !matches!(len, SIZE_90_ONLY | SIZE_WITH_180) {
            return Err(format!("Unexpected number of kicks: size={len}"));
        }

        let cw_map = Self::create_map(kick_patterns, RotateDirection::Clockwise)
            .map_err(|kick| Self::fmt_kick_type_error(&kick))?;

        let ccw_map = Self::create_map(kick_patterns, RotateDirection::CounterClockwise)
            .map_err(|kick| Self::fmt_kick_type_error(&kick))?;

        if Self::supports_180(len) {
            let rotate_180_map = Self::create_map(kick_patterns, RotateDirection::Rotate180)
                .map_err(|kick| Self::fmt_kick_type_error(&kick))?;
        }

        Ok(())
    }

    pub fn create_new_rotation(&self) -> Box<dyn MinoRotation> {
        if Self::supports_180(self.kick_patterns.len()) {
            Box::new(MinoRotationImpl::new(
                Self::create_map(&self.kick_patterns, RotateDirection::Clockwise).unwrap(),
                Self::create_map(&self.kick_patterns, RotateDirection::CounterClockwise).unwrap(),
                Self::create_map(&self.kick_patterns, RotateDirection::Rotate180).unwrap(),
            ))
        } else {
            Box::new(MinoRotationNo180Impl::new(
                Self::create_map(&self.kick_patterns, RotateDirection::Clockwise).unwrap(),
                Self::create_map(&self.kick_patterns, RotateDirection::CounterClockwise).unwrap(),
            ))
        }
    }

    fn fmt_kick_type_error(kick_type: &KickType) -> String {
        format!(
            "Invalid kick type: piece={:?}, from={:?}, to={:?}",
            kick_type.piece, kick_type.from, kick_type.to
        )
    }

    fn supports_180(len: usize) -> bool {
        len == SIZE_WITH_180
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
}
