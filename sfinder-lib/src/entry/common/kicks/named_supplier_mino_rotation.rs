//! Wrapper to bundle a name of the MinoRotation
//! Mainly used in command line parsing

use super::factory::srs_mino_rotation_factory;
use crate::sfinder_core::srs::mino_rotation::MinoRotation;

pub struct NamedSupplierMinoRotation {
    name: String,
    rotation: Box<dyn MinoRotation>,
}

impl NamedSupplierMinoRotation {
    pub fn new(name: String, rotation: Box<dyn MinoRotation>) -> Self {
        Self { name, rotation }
    }
}

impl Default for NamedSupplierMinoRotation {
    fn default() -> Self {
        Self {
            name: "srs".to_string(),
            rotation: srs_mino_rotation_factory::create(),
        }
    }
}
