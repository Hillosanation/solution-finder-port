use super::kick_type::KickType;
use crate::sfinder_core::srs::pattern::Pattern;
use std::collections::BTreeMap;

pub trait KickPattern {
    fn get_kick_type(&self) -> &KickType;

    fn get_pattern<'a>(
        &'a self,
        // TODO: can we just use FixedKickPattern? I don't think you need multiple layers of indirection
        fallback: &'a BTreeMap<KickType, Box<dyn KickPattern>>,
    ) -> Option<&Pattern>;
}

pub enum KickPatternType {
    Fixed { pattern: Pattern },
    Referenced { reference_kick_type: KickType },
}

struct _KickPattern {
    kick_type: KickType,
    kick_pattern_type: KickPatternType,
}

impl _KickPattern {
    pub fn new(kick_type: KickType, kick_pattern_type: KickPatternType) -> Self {
        if let KickPatternType::Referenced {
            reference_kick_type,
        } = kick_pattern_type
        {
            assert_ne!(kick_type, reference_kick_type, "Cannot refer to itself");
        }
        Self {
            kick_type,
            kick_pattern_type,
        }
    }
}

impl KickPattern for _KickPattern {
    fn get_kick_type(&self) -> &KickType {
        &self.kick_type
    }

    fn get_pattern<'a>(
        &'a self,

        fallback: &'a BTreeMap<KickType, Box<dyn KickPattern>>,
    ) -> Option<&Pattern> {
        match &self.kick_pattern_type {
            KickPatternType::Fixed { pattern } => Some(pattern),
            KickPatternType::Referenced {
                reference_kick_type,
            } => fallback
                .get(reference_kick_type)
                .and_then(move |kick_pattern| kick_pattern.get_pattern(fallback)),
        }
    }
}
