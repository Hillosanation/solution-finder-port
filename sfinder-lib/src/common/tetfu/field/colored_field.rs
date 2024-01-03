use crate::{common::tetfu::common::color_type::ColorType, sfinder_core::mino::mino::Mino};

// Blockの番号とColorの番号
pub trait ColoredField {
    // Porting note: replaces freeze
    fn create_new(&self, max_height: u8) -> Box<dyn ColoredField>;

    // Porting note: getBlockNumber is dropped in favor of converting yourself
    fn get_color(&self, x: u8, y: u8) -> ColorType;

    fn put_mino(&mut self, mino: Mino, x: u8, y: u8);

    // Porting note: putBlockNumber is dropped in favor of converting yourself
    fn set_color(&mut self, x: u8, y: u8, color: ColorType);

    fn clear_filled_rows(&mut self);

    fn block_up(&mut self);

    fn mirror(&mut self);

    fn get_max_height(&self) -> usize;

    // Porting note: replaces getUsingHeight
    // Result is one-indexed. Returns 0 iff there are no blocks in the field.
    fn get_max_y(&self) -> u8;

    fn is_filled_row(&self, y: u8) -> bool;

    // Porting note: replaces isPerfect
    fn is_empty(&self) -> bool;
}
