//! Helper struct used only by FullOperationSeparableMino

use crate::sfinder_core::field::field::Field;

// TODO(#12): it seems like MinoMask is only ever instantiated to immediately call get_mino_mask, then is never used again. Maybe we can just make this a function?
pub trait MinoMask {
    fn get_mino_mask(&self, x: u8) -> Box<dyn Field>;
}
