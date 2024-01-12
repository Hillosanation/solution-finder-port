//! Helper function used only by RemainderFieldRunner

pub struct RemainderField {
    pub min_x: u8,
    pub target_block_count: u8,
}

impl RemainderField {
    pub fn new(min_x: u8, target_block_count: u8) -> Self {
        Self {
            min_x,
            target_block_count,
        }
    }
}
