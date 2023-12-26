use crate::sfinder_core::srs::rotate::Rotate;

pub trait Action {
    // Porting note: I wasn't sure if this needs to be signed, so leaving it generic for now.
    // It will most likely only be specialized to u8 or i8.
    // TODO: clean up the generic type
    type Coord;

    fn get_x(&self) -> Self::Coord;

    fn get_y(&self) -> Self::Coord;

    fn get_rotate(&self) -> Rotate;
}
