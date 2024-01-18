pub trait Memory {
    fn get(&self, x: u8, y: u8) -> bool;

    // Porting note: replaces setTrue
    fn set(&mut self, x: u8, y: u8);

    fn clear(&mut self);
}
