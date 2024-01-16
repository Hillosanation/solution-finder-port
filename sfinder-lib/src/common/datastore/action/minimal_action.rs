use super::action::Action;
use crate::{extras::hash_code::HashCode, sfinder_core::srs::rotate::Rotate};
use std::hash::Hash;

/*
 * y < 24であること
 */
#[derive(Debug, PartialEq, Eq)]
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

impl Hash for MinimalAction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.hash_code());
    }
}

impl nohash::IsEnabled for MinimalAction {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getter() {
        let action = MinimalAction::new(4, 5, Rotate::Spawn);
        assert_eq!(action.get_rotate(), Rotate::Spawn);
        assert_eq!(action.get_x(), 4);
        assert_eq!(action.get_y(), 5);
    }

    #[test]
    fn test_equal() {
        let action = MinimalAction::new(4, 5, Rotate::Spawn);
        assert_eq!(action, MinimalAction::new(4, 5, Rotate::Spawn));
        assert_ne!(action, MinimalAction::new(7, 5, Rotate::Spawn));
        assert_ne!(action, MinimalAction::new(4, 21, Rotate::Spawn));
        assert_ne!(action, MinimalAction::new(4, 5, Rotate::Right));
    }

    #[test]
    fn test_hash_code() {
        let action = MinimalAction::new(4, 5, Rotate::Spawn);
        assert_eq!(
            action.hash_code(),
            MinimalAction::new(4, 5, Rotate::Spawn).hash_code()
        );
        assert_ne!(
            action.hash_code(),
            MinimalAction::new(2, 5, Rotate::Spawn).hash_code()
        );
        assert_ne!(
            action.hash_code(),
            MinimalAction::new(4, 12, Rotate::Spawn).hash_code()
        );
        assert_ne!(
            action.hash_code(),
            MinimalAction::new(4, 5, Rotate::Right).hash_code()
        );
    }

    // void testCompareTo() throws Exception {
    //         MinimalAction action1 = MinimalAction.create(4, 5, Rotate.Spawn);
    //         MinimalAction action2 = MinimalAction.create(4, 5, Rotate.Spawn);
    //         MinimalAction action3 = MinimalAction.create(4, 13, Rotate.Spawn);
    //         MinimalAction action4 = MinimalAction.create(4, 13, Rotate.Reverse);

    //         assertThat(action1.compareTo(action2)).isEqualTo(0);

    //         assertThat(action1.compareTo(action3)).isNotEqualTo(0);
    //         assertThat(action1.compareTo(action4)).isNotEqualTo(0);
    //         assertThat(action3.compareTo(action4)).isNotEqualTo(0);

    //         assert action1.compareTo(action3) < 0 && action3.compareTo(action4) < 0;
    //         assertThat(action1.compareTo(action4)).isLessThan(0);
    //     }
    #[test]
    fn test_compare_to() {
        let action1 = MinimalAction::new(4, 5, Rotate::Spawn);
        let action2 = MinimalAction::new(4, 5, Rotate::Spawn);
        let action3 = MinimalAction::new(4, 13, Rotate::Spawn);
        let action4 = MinimalAction::new(4, 13, Rotate::Reverse);

        assert_eq!(
            action1.partial_cmp(&action2),
            Some(std::cmp::Ordering::Equal)
        );

        assert_ne!(
            action1.partial_cmp(&action3),
            Some(std::cmp::Ordering::Equal)
        );
        assert_ne!(
            action1.partial_cmp(&action4),
            Some(std::cmp::Ordering::Equal)
        );
        assert_ne!(
            action3.partial_cmp(&action4),
            Some(std::cmp::Ordering::Equal)
        );

        assert!(action1 < action3);
        assert!(action3 < action4);
        assert!(action1 < action4);
    }
}
