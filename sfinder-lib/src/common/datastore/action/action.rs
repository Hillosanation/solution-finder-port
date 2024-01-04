use crate::sfinder_core::srs::rotate::Rotate;

// Porting note: I wasn't sure if this needs to be signed, so leaving it generic for now.
// It will most likely only be specialized to u8 or i8.
pub trait Action {
    fn get_x(&self) -> u8;

    fn get_y(&self) -> u8;

    fn get_rotate(&self) -> Rotate;
}
