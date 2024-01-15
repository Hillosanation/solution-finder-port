use super::kick_type::KickType;
use crate::sfinder_core::srs::pattern::Pattern;
use std::collections::BTreeMap;

pub trait KickPattern {
    fn get_kick_type(&self) -> &KickType;

    fn get_pattern(&self, fallback: BTreeMap<KickType, Box<dyn KickPattern>>) -> Pattern;
}
