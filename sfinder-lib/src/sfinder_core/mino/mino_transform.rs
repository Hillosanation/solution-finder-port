//! Helper struct for MinoShifter

use crate::{
    common::datastore::{action::minimal_action::MinimalAction, coordinate::Coordinate},
    sfinder_core::srs::rotate::Rotate,
};

#[derive(Debug)]
pub struct MinoTransform {
    offsets: Vec<Coordinate>,
    rotates: Vec<Rotate>,
    reverse_map: Vec<Vec<Rotate>>,
}

// Porting note: `mirror` is moved to SimpleMinoOperation
impl MinoTransform {
    pub fn new() -> Self {
        Self {
            offsets: vec![Coordinate::new(0, 0); Rotate::get_size()],
            rotates: Rotate::value_list().to_vec(),
            reverse_map: vec![Vec::new(); Rotate::get_size()],
        }
    }

    pub fn set_with(entries: &[(Rotate, i8, i8, Rotate)]) -> Self {
        let mut offsets = vec![Coordinate::new(0, 0); Rotate::get_size()];
        let mut rotates = Rotate::value_list().to_vec();

        for &(rotate, offset_x, offset_y, new_rotate) in entries {
            let index = rotate as usize;

            offsets[index] = Coordinate::new(offset_x, offset_y);
            rotates[index] = new_rotate;
        }

        let reverse_map = Self::create_reverse_map(&offsets, &rotates);

        Self {
            offsets,
            rotates,
            reverse_map,
        }
    }

    fn create_reverse_map(offsets: &[Coordinate], rotates: &[Rotate]) -> Vec<Vec<Rotate>> {
        let mut reverse_map = vec![Vec::new(); Rotate::get_size()];

        for &rotate in Rotate::value_list() {
            let new_rotate = rotates[rotate as usize];

            if rotate != new_rotate {
                // 変換後の回転が同じになる、他の回転とも関連づける
                for r in reverse_map[new_rotate as usize].clone() {
                    reverse_map[r as usize].push(rotate);
                    reverse_map[rotate as usize].push(r);
                }

                // 変換前と変換後を関連づける
                reverse_map[new_rotate as usize].push(rotate);
                reverse_map[rotate as usize].push(new_rotate);
            }
        }

        reverse_map
    }

    pub fn transform(&self, x: u8, y: u8, rotate: Rotate) -> MinimalAction {
        let index = rotate as usize;
        MinimalAction::new(
            u8::try_from(x as i8 + self.offsets[index].x).unwrap(),
            u8::try_from(y as i8 + self.offsets[index].y).unwrap(),
            self.rotates[index],
        )
    }

    pub fn enumerate_others(&self, x: u8, y: u8, rotate: Rotate) -> Vec<MinimalAction> {
        let index = rotate as usize;
        let new_x = x as i8 + self.offsets[index].x;
        let new_y = y as i8 + self.offsets[index].y;

        self.reverse_map[index]
            .iter()
            .copied()
            .map(|prev_rotate| {
                let prev_index = prev_rotate as usize;
                // Assuming the provided Action is valid, this Action is also valid, and will not panic.
                MinimalAction::new(
                    u8::try_from(new_x - self.offsets[prev_index].x).unwrap(),
                    u8::try_from(new_y - self.offsets[prev_index].y).unwrap(),
                    prev_rotate,
                )
            })
            .collect()
    }

    pub fn transform_rotate(&self, rotate: Rotate) -> Rotate {
        self.rotates[rotate as usize]
    }

    pub fn get_unique_rotates(&self) -> Vec<Rotate> {
        // surely sorting 4 items is less intensive than keeping a hash table?
        let mut unique = self.rotates.clone();
        unique.sort_unstable();
        unique.dedup();

        unique
    }
}
