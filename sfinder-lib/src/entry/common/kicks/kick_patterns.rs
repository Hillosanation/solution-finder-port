use super::{kick_pattern::KickPattern, kick_type::KickType};
use crate::sfinder_core::{
    mino::piece::Piece,
    srs::{pattern::Pattern, rotate::Rotate},
};
use std::collections::BTreeMap;

pub struct KickPatterns {
    kick_patterns: BTreeMap<KickType, KickPattern>,
}

impl KickPatterns {
    pub fn new(kick_patterns: Vec<KickPattern>) -> Self {
        Self {
            kick_patterns: kick_patterns
                .into_iter()
                .map(|kick_pattern| (kick_pattern.get_kick_type().clone(), kick_pattern))
                .collect(),
        }
    }

    pub fn get_pattern(&self, piece: Piece, from: Rotate, to: Rotate) -> Option<&Pattern> {
        self.kick_patterns
            .get(&KickType { piece, from, to })?
            .get_pattern(&self.kick_patterns)
    }

    // Porting note: replaces size
    pub fn len(&self) -> usize {
        self.kick_patterns.len()
    }
}
