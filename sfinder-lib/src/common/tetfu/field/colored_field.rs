use crate::{common::tetfu::common::color_type::ColorType, sfinder_core::mino::mino::Mino};

// Blockの番号とColorの番号
pub trait ColoredField {
    // Porting note: replaces freeze
    fn prune(&self) -> Box<dyn ColoredField>;

    // Porting note: getBlockNumber is dropped in favor of converting yourself
    fn get_color(&self, x: u8, y: u8) -> ColorType;

    fn put_mino(&mut self, mino: Mino, x: u8, y: u8);

    // Porting note: putBlockNumber is dropped in favor of converting yourself
    fn set_color(&mut self, x: u8, y: u8, color: ColorType);

    fn clear_row(&mut self);

    fn block_up(&mut self);

    fn mirror(&mut self);

    fn get_max_height(&self) -> u8;

    // Porting note: replaces getUsingHeight
    fn get_max_y(&self) -> u8;

    fn is_filled_row(&self, y: u8) -> bool;

    // Porting note: replaces isPerfect
    fn is_empty(&self) -> bool;
}
