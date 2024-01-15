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
