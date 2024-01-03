use crate::sfinder_core::srs::rotate::Rotate;

// Porting note: I wasn't sure if this needs to be signed, so leaving it generic for now.
// It will most likely only be specialized to u8 or i8.
// TODO: clean up the generic type
pub trait Action<Coord> {
    fn get_x(&self) -> Coord;

    fn get_y(&self) -> Coord;

    fn get_rotate(&self) -> Rotate;
}
