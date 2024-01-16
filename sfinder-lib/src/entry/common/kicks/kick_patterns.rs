use super::{kick_pattern::KickPattern, kick_type::KickType};
use std::collections::BTreeMap;

pub struct KickPatterns {
    kick_patterns: BTreeMap<KickType, KickPattern>,
}

impl KickPatterns {
    pub fn new(kick_patterns: Vec<KickPattern>) -> Self {
        Self {
            kick_patterns: kick_patterns
                .into_iter()
                // TODO: just sort by overriding Ord of KickPattern and collect into BTreeSet
                .map(|kick_pattern| (kick_pattern.get_kick_type().clone(), kick_pattern))
                .collect(),
        }
    }
}
