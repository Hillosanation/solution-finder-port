use super::action::action::Action;
use crate::sfinder_core::mino::piece::Piece;

pub trait Operation<Coord>: Action<Coord = Coord>
where
    u32: From<Coord>,
    u64: From<Coord>,
{
    fn get_piece(&self) -> Piece;

    fn default_hash_code(&self) -> u32 {
        let mut result = u32::from(self.get_y());
        result = 10 * result + u32::from(self.get_x());
        result = 7 * result + self.get_piece() as u32;
        result = 4 * result + self.get_rotate() as u32;

        result
    }

    fn to_unique_key(&self) -> u64 {
        self.get_piece() as u64 * 4 * 24 * 10
            + self.get_rotate() as u64 * 24 * 10
            + u64::from(self.get_y()) * 10
            + u64::from(self.get_x())
    }
}
