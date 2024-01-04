use super::colored_field::ColoredField;
use crate::{
    common::tetfu::common::color_type::ColorType,
    sfinder_core::{field::field_constants::FIELD_WIDTH, mino::mino::Mino},
};

#[derive(Clone)]
pub struct ArrayColoredField {
    field: Vec<[ColorType; FIELD_WIDTH as usize]>,
}

impl ArrayColoredField {
    pub fn new(max_height: u8) -> Self {
        Self {
            field: vec![[ColorType::Empty; FIELD_WIDTH as usize]; max_height as usize],
        }
    }
}

impl ColoredField for ArrayColoredField {
    fn create_new(&self, max_height: u8) -> Box<dyn ColoredField> {
        let field = if max_height <= self.field.len() as u8 {
            self.field[..max_height as usize].to_vec()
        } else {
            let mut new_field = vec![[ColorType::Empty; FIELD_WIDTH as usize]; max_height as usize];
            for i in 0..max_height as usize {
                new_field[i] = self.field[i];
            }
            new_field
        };
        Box::new(Self { field })
    }

    fn get_color(&self, x: u8, y: u8) -> ColorType {
        self.field[y as usize][x as usize]
    }

    fn put_mino(&mut self, mino: Mino, x: u8, y: u8) {
        let color: ColorType = mino.get_piece().into();
        for positions in mino.get_positions() {
            self.set_color(
                u8::try_from(x as i8 + positions.x).unwrap(),
                u8::try_from(y as i8 + positions.y).unwrap(),
                color,
            );
        }
    }

    fn set_color(&mut self, x: u8, y: u8, color: ColorType) {
        self.field[y as usize][x as usize] = color;
    }

    fn clear_filled_rows(&mut self) {
        let length = self.field.len();
        self.field
            .retain(|row| row.iter().any(|color| *color == ColorType::Empty));
        for _ in 0..length - self.field.len() {
            self.field.push([ColorType::Empty; FIELD_WIDTH as usize]);
        }
    }

    fn block_up(&mut self) {
        self.field.rotate_right(1);
        self.field[0] = [ColorType::Empty; FIELD_WIDTH as usize];
    }

    fn mirror(&mut self) {
        for row in &mut self.field {
            row.reverse();
        }
    }

    fn get_max_height(&self) -> usize {
        self.field.len()
    }

    fn get_max_y(&self) -> u8 {
        self.field
            .iter()
            .enumerate()
            // start searching from the top
            .rev()
            .find_map(|(i, row)| {
                row.iter()
                    .any(|color| *color != ColorType::Empty)
                    .then_some(i)
            })
            .map_or(0, |i| i as u8 + 1)
    }

    fn is_filled_row(&self, y: u8) -> bool {
        self.field[y as usize]
            .iter()
            .all(|color| *color != ColorType::Empty)
    }

    fn is_empty(&self) -> bool {
        self.field
            .iter()
            .all(|row| row.iter().all(|color| *color == ColorType::Empty))
    }
}
