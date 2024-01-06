use crate::{extras::hash_code::HashCode, sfinder_core::srs::rotate_direction::RotateDirection};
use std::{fmt::Display, str::FromStr};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rotate {
    Spawn = 0,
    Right,
    Reverse,
    Left,
}

const ROTATE_COUNT: usize = 4;
const VALUE_LIST: [Rotate; ROTATE_COUNT] =
    [Rotate::Spawn, Rotate::Right, Rotate::Reverse, Rotate::Left];

/// Porting note: casting replaces getNumber
impl Rotate {
    /// Porting note: replaces getRotate
    /// Panics if number is out of range.
    pub fn new(number: u8) -> Self {
        VALUE_LIST[number as usize]
    }

    // Porting note: replaces StringEnumTransform::toNEWSRotate
    pub fn from_cardinal(c: char) -> Self {
        match c {
            'N' => Rotate::Spawn,
            'E' => Rotate::Right,
            'W' => Rotate::Left,
            'S' => Rotate::Reverse,
            _ => panic!("Invalid cardinal: {c}"),
        }
    }

    pub fn value_list() -> &'static [Rotate] {
        &VALUE_LIST
    }

    pub const fn get_size() -> usize {
        ROTATE_COUNT
    }

    /// Porting note: replaces get
    /// TODO: This seems to only be used once, and is used before RotateDirection was reversed.
    ///       Implement unapply instead?
    pub fn apply(self, rotation: RotateDirection) -> Self {
        match rotation {
            RotateDirection::Clockwise => self.get_cw_rotate(),
            RotateDirection::CounterClockwise => self.get_ccw_rotate(),
            RotateDirection::Rotate180 => self.get_180_rotate(),
        }
    }

    // Porting note follows RotateDirection naming convention
    // TODO: Remove these functions from pub and use apply directly?
    pub fn get_ccw_rotate(self) -> Self {
        match self {
            Rotate::Spawn => Rotate::Left,
            Rotate::Left => Rotate::Reverse,
            Rotate::Reverse => Rotate::Right,
            Rotate::Right => Rotate::Spawn,
        }
    }

    pub fn get_cw_rotate(self) -> Self {
        match self {
            Rotate::Spawn => Rotate::Right,
            Rotate::Right => Rotate::Reverse,
            Rotate::Reverse => Rotate::Left,
            Rotate::Left => Rotate::Spawn,
        }
    }

    pub fn get_180_rotate(self) -> Self {
        match self {
            Rotate::Spawn => Rotate::Reverse,
            Rotate::Right => Rotate::Left,
            Rotate::Reverse => Rotate::Spawn,
            Rotate::Left => Rotate::Right,
        }
    }

    pub fn mirror(self) -> Self {
        match self {
            Rotate::Right => Rotate::Left,
            Rotate::Left => Rotate::Right,
            _ => self,
        }
    }
}

impl HashCode for Rotate {
    type Output = u8;

    fn hash_code(&self) -> Self::Output {
        *self as u8
    }
}

impl std::hash::Hash for Rotate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.hash_code())
    }
}

impl nohash::IsEnabled for Rotate {}

impl FromStr for Rotate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Rotate::Spawn),
            "R" => Ok(Rotate::Right),
            "2" => Ok(Rotate::Reverse),
            "L" => Ok(Rotate::Left),
            _ => Err(format!("Invalid rotate: {s}")),
        }
    }
}

impl Display for Rotate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rotate::Spawn => write!(f, "0"),
            Rotate::Right => write!(f, "R"),
            Rotate::Reverse => write!(f, "2"),
            Rotate::Left => write!(f, "L"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_rotate() {
        assert_eq!(Rotate::new(0), Rotate::Spawn);
        assert_eq!(Rotate::new(1), Rotate::Right);
        assert_eq!(Rotate::new(2), Rotate::Reverse);
        assert_eq!(Rotate::new(3), Rotate::Left);
    }

    #[test]
    fn value_list() {
        assert_eq!(
            Rotate::value_list(),
            &[Rotate::Spawn, Rotate::Right, Rotate::Reverse, Rotate::Left]
        );
    }

    #[test]
    fn get_size() {
        assert_eq!(Rotate::get_size(), 4);
    }

    #[test]
    fn spawn() {
        assert_eq!(Rotate::Spawn as u8, 0);
        assert_eq!(Rotate::Spawn.get_cw_rotate(), Rotate::Right);
        assert_eq!(Rotate::Spawn.get_ccw_rotate(), Rotate::Left);
    }

    #[test]
    fn right() {
        assert_eq!(Rotate::Right as u8, 1);
        assert_eq!(Rotate::Right.get_cw_rotate(), Rotate::Reverse);
        assert_eq!(Rotate::Right.get_ccw_rotate(), Rotate::Spawn);
    }

    #[test]
    fn reverse() {
        assert_eq!(Rotate::Reverse as u8, 2);
        assert_eq!(Rotate::Reverse.get_cw_rotate(), Rotate::Left);
        assert_eq!(Rotate::Reverse.get_ccw_rotate(), Rotate::Right);
    }

    #[test]
    fn left() {
        assert_eq!(Rotate::Left as u8, 3);
        assert_eq!(Rotate::Left.get_cw_rotate(), Rotate::Spawn);
        assert_eq!(Rotate::Left.get_ccw_rotate(), Rotate::Reverse);
    }

    // tests retrieved from StringEnumTransformTest.java
    #[test]
    fn from_cardinal() {
        assert_eq!(Rotate::from_cardinal('N'), Rotate::Spawn);
        assert_eq!(Rotate::from_cardinal('E'), Rotate::Right);
        assert_eq!(Rotate::from_cardinal('W'), Rotate::Left);
        assert_eq!(Rotate::from_cardinal('S'), Rotate::Reverse);
    }

    #[test]
    fn to_string() {
        assert_eq!(Rotate::Spawn.to_string(), "0");
        assert_eq!(Rotate::Right.to_string(), "R");
        assert_eq!(Rotate::Reverse.to_string(), "2");
        assert_eq!(Rotate::Left.to_string(), "L");
    }

    #[test]
    fn from_str() {
        assert_eq!(Rotate::from_str("0").unwrap(), Rotate::Spawn);
        assert_eq!(Rotate::from_str("R").unwrap(), Rotate::Right);
        assert_eq!(Rotate::from_str("2").unwrap(), Rotate::Reverse);
        assert_eq!(Rotate::from_str("L").unwrap(), Rotate::Left);
    }
}
