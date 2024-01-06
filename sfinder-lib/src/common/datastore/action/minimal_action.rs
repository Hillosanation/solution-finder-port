use super::action::Action;
use crate::{extras::hash_code::HashCode, sfinder_core::srs::rotate::Rotate};

/*
 * y < 24であること
 */
pub struct MinimalAction {
    x: u8,
    y: u8,
    rotate: Rotate,
}

impl MinimalAction {
    pub fn new(x: u8, y: u8, rotate: Rotate) -> Self {
        Self { x, y, rotate }
    }
}

impl Action for MinimalAction {
    fn get_x(&self) -> u8 {
        self.x
    }

    fn get_y(&self) -> u8 {
        self.y
    }

    fn get_rotate(&self) -> Rotate {
        self.rotate
    }
}

impl PartialEq for MinimalAction {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.rotate == other.rotate
    }
}

impl HashCode for MinimalAction {
    type Output = u32;

    fn hash_code(&self) -> Self::Output {
        let mut result = self.x as u32;
        result = 24 * result + self.y as u32;
        result = 4 * result + self.rotate as u32;
        result
    }
}

impl PartialOrd for MinimalAction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.rotate.cmp(&other.rotate) {
            std::cmp::Ordering::Equal => {}
            ordering => return Some(ordering),
        }

        match self.x.cmp(&other.x) {
            std::cmp::Ordering::Equal => {}
            ordering => return Some(ordering),
        }

        Some(self.y.cmp(&other.y))
    }
}
