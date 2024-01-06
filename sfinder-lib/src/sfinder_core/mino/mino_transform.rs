//! Helper struct for MinoShifter

use super::mino_factory::MinoFactory;
use crate::{
    common::datastore::{
        action::minimal_action::MinimalAction, coordinate::Coordinate,
        mino_operation::MinoOperation, simple_mino_operation::SimpleMinoOperation,
    },
    sfinder_core::{field::field_constants::FIELD_WIDTH, srs::rotate::Rotate},
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
        let mut this = Self {
            offsets: vec![Coordinate::new(0, 0); Rotate::get_size()],
            rotates: Rotate::value_list().to_vec(),
            reverse_map: vec![Vec::new(); Rotate::get_size()],
        };

        this.refresh();

        this
    }

    pub fn set(&mut self, rotate: Rotate, offset_x: i8, offset_y: i8, new_rotate: Rotate) {
        let index = rotate as usize;
        assert_eq!(self.rotates.len(), index);

        self.offsets[index] = Coordinate::new(offset_x, offset_y);
        self.rotates[index] = new_rotate;
        self.refresh();
    }

    fn refresh(&mut self) {
        for reverse in self.reverse_map.iter_mut() {
            reverse.clear();
        }

        for &rotate in Rotate::value_list() {
            let index = rotate as usize;
            if let Some(&new_rotate) = self.rotates.get(index) {
                if rotate != new_rotate {
                    // 変換後の回転が同じになる、他の回転とも関連づける
                    for r in self.reverse_map[new_rotate as usize].clone() {
                        self.reverse_map[r as usize].push(rotate);
                        self.reverse_map[rotate as usize].push(r);
                    }

                    // 変換前と変換後を関連づける
                    self.reverse_map[new_rotate as usize].push(rotate);
                    self.reverse_map[rotate as usize].push(new_rotate);
                }
            }
        }
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
}
