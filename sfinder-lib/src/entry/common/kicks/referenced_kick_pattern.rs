use super::{kick_pattern::KickPattern, kick_type::KickType};
use crate::sfinder_core::srs::pattern::Pattern;
use std::collections::BTreeMap;

pub struct ReferencedKickPattern {
    kick_type: KickType,
    reference_kick_type: KickType,
}

impl ReferencedKickPattern {
    pub fn new(kick_type: KickType, reference_kick_type: KickType) -> Self {
        assert_ne!(kick_type, reference_kick_type, "Cannot refer to itself");
        Self {
            kick_type,
            reference_kick_type,
        }
    }
}

impl KickPattern for ReferencedKickPattern {
    fn get_kick_type(&self) -> &KickType {
        &self.kick_type
    }

    fn get_pattern<'a>(
        &'a self,
        fallback: &'a BTreeMap<KickType, Box<dyn KickPattern>>,
    ) -> Option<&Pattern> {
        fallback
            .get(&self.reference_kick_type)
            .and_then(move |kick_pattern| kick_pattern.get_pattern(fallback))
    }
}
