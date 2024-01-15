use super::{kick_pattern::KickPattern, kick_type::KickType};
use crate::sfinder_core::srs::pattern::Pattern;
use std::collections::BTreeMap;

pub struct FixedKickPattern {
    kick_type: KickType,
    pattern: Pattern,
}

impl FixedKickPattern {
    pub fn new(kick_type: KickType, pattern: Pattern) -> Self {
        Self { kick_type, pattern }
    }
}

impl KickPattern for FixedKickPattern {
    fn get_kick_type(&self) -> &KickType {
        &self.kick_type
    }

    fn get_pattern<'a>(
        &'a self,
        _fallback: &'a BTreeMap<KickType, Box<dyn KickPattern>>,
    ) -> Option<&Pattern> {
        Some(&self.pattern)
    }
}
