use super::mask_field::MaskField;
use crate::sfinder_core::field::{
    field::Field, field_constants::FIELD_WIDTH, field_factory, key_operators,
};
use nohash::IntMap;

const DIFF: [i8; 2] = [-1, 1];

pub struct SpinMaskFields {
    mask_fields: IntMap<u8, Vec<MaskField>>,
}

impl SpinMaskFields {
    pub fn new(allow_fill_max_height: u8, field_height: u8) -> Self {
        Self {
            mask_fields: Self::get_t_spin_mask_fields(allow_fill_max_height, field_height),
        }
    }

    // Tスピンとして判定されるのに必要なブロックを取得
    fn get_t_spin_mask_fields(
        allow_fill_max_height: u8,
        field_height: u8,
    ) -> IntMap<u8, Vec<MaskField>> {
        let max_height = allow_fill_max_height + 1;
        assert!(max_height <= field_height);
        assert!(max_height * FIELD_WIDTH <= u8::MAX);

        (0..max_height)
            .flat_map(|y| {
                (0..FIELD_WIDTH).map(move |x| {
                    (
                        Self::to_key(x, y),
                        Self::create_mask_fields(x as i8, y as i8, field_height),
                    )
                })
            })
            .collect()
    }

    fn to_key(x: u8, y: u8) -> u8 {
        x + y * FIELD_WIDTH
    }

    fn create_mask_fields(x: i8, y: i8, field_height: u8) -> Vec<MaskField> {
        let mut field = field_factory::create_field(field_height);

        for dx in DIFF {
            for dy in DIFF {
                let fx = x + dx;
                let fy = y + dy;

                if let Some((fx, fy)) = Self::convert_to_field_coord(fx, fy) {
                    field.set_block(fx, fy);
                }
            }
        }

        [
            Self::create_mask_field(&field, x - 1, y - 1, field_height),
            Self::create_mask_field(&field, x + 1, y - 1, field_height),
            Self::create_mask_field(&field, x - 1, y + 1, field_height),
            Self::create_mask_field(&field, x + 1, y + 1, field_height),
            Self::create_mask_field_empty(&field, field_height),
        ]
        .into_iter()
        .filter_map(|mask_field| mask_field)
        .collect()
    }

    // Porting note: replaces isInField to simultaneously check if values are in bound and convert to u8
    fn convert_to_field_coord(fx: i8, fy: i8) -> Option<(u8, u8)> {
        let (x, y) = (u8::try_from(fx).ok()?, u8::try_from(fy).ok()?);
        (x < FIELD_WIDTH).then_some((x, y))
    }

    fn create_mask_field(
        field: &Box<dyn Field>,
        x: i8,
        y: i8,
        field_height: u8,
    ) -> Option<MaskField> {
        let (x, y) = Self::convert_to_field_coord(x, y)?;

        let mut freeze_need = field.clone();
        freeze_need.remove_block(x, y);

        let mut freeze_not_allowed = field_factory::create_field(field_height);
        freeze_not_allowed.set_block(x, y);

        Some(MaskField::new(freeze_need, freeze_not_allowed))
    }

    fn create_mask_field_empty(field: &Box<dyn Field>, field_height: u8) -> Option<MaskField> {
        Some(MaskField::new(
            field.clone(),
            field_factory::create_field(field_height),
        ))
    }

    #[cfg(test)]
    fn get(&self, x: u8, y: u8) -> impl Iterator<Item = &MaskField> {
        self.mask_fields[&Self::to_key(x, y)].iter()
    }

    pub fn get_with_delete_key(
        &self,
        x: u8,
        y: u8,
        deleted_key: u64,
    ) -> impl Iterator<Item = MaskField> + '_ {
        // slide_y <= y
        let slide_y = (deleted_key & key_operators::get_mask_for_key_below_y(y as u8)).count_ones();
        let new_y = y - slide_y as u8;
        let key = Self::to_key(x, new_y);

        self.mask_fields[&key].iter().map(move |mask_field| {
            let mut freeze_need = mask_field.rest.clone();
            freeze_need.insert_blank_row_with_key(deleted_key);

            let mut freeze_not_allowed = mask_field.not_allowed.clone();
            freeze_not_allowed.insert_blank_row_with_key(deleted_key);

            MaskField::new(freeze_need, freeze_not_allowed)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_HEIGHT: u8 = 8;

    // Coordinate isn't really appropriate as it uses i8
    fn to_field(coords: &[(u8, u8)]) -> Box<dyn Field> {
        let mut field = field_factory::create_field(MAX_HEIGHT);
        for (x, y) in coords {
            field.set_block(*x, *y);
        }
        field
    }

    fn assert_mask_field(mask_field: &MaskField, rest: &[(u8, u8)], not_allowed: &[(u8, u8)]) {
        assert_eq!(&mask_field.rest, &to_field(rest));
        assert_eq!(&mask_field.not_allowed, &to_field(not_allowed));
    }

    fn test_get_wrapper(x: u8, y: u8, result: &[(&[(u8, u8)], &[(u8, u8)])]) {
        let spin_mask_fields = SpinMaskFields::new(MAX_HEIGHT, MAX_HEIGHT + 1);

        let mut mask_fields = spin_mask_fields.get(x, y).collect::<Vec<_>>();
        mask_fields.sort_unstable_by(|a, b| a.rest.partial_cmp(&b.rest).unwrap());

        assert_eq!(mask_fields.len(), result.len());

        for (mask_field, (rest, not_allowed)) in mask_fields.iter().zip(result.iter()) {
            assert_mask_field(mask_field, rest, not_allowed);
        }
    }

    fn test_get_with_delete_key_wrapper(
        x: u8,
        y: u8,
        deleted_key: u64,
        result: &[(&[(u8, u8)], &[(u8, u8)])],
    ) {
        let spin_mask_fields = SpinMaskFields::new(MAX_HEIGHT, MAX_HEIGHT + 1);

        let mut mask_fields = spin_mask_fields
            .get_with_delete_key(x, y, deleted_key)
            .collect::<Vec<_>>();
        mask_fields.sort_unstable_by(|a, b| a.rest.partial_cmp(&b.rest).unwrap());

        assert_eq!(mask_fields.len(), result.len());

        for (mask_field, (rest, not_allowed)) in mask_fields.iter().zip(result.iter()) {
            assert_mask_field(mask_field, rest, not_allowed);
        }
    }

    #[test]
    fn center() {
        // 中央
        test_get_wrapper(
            1,
            1,
            &[
                (&[(0, 0), (2, 0), (0, 2)], &[(2, 2)]),
                (&[(0, 0), (2, 0), (2, 2)], &[(0, 2)]),
                (&[(0, 0), (0, 2), (2, 2)], &[(2, 0)]),
                (&[(2, 0), (0, 2), (2, 2)], &[(0, 0)]),
                (&[(0, 0), (2, 0), (0, 2), (2, 2)], &[]),
            ],
        );
    }

    #[test]
    fn left_side() {
        // 左端
        test_get_wrapper(
            0,
            2,
            &[
                (&[(1, 1)], &[(1, 3)]),
                (&[(1, 3)], &[(1, 1)]),
                (&[(1, 1), (1, 3)], &[]),
            ],
        )
    }

    #[test]
    fn right_side() {
        // 右端
        test_get_wrapper(
            9,
            6,
            &[
                (&[(8, 7)], &[(8, 5)]),
                (&[(8, 5)], &[(8, 7)]),
                (&[(8, 5), (8, 7)], &[]),
            ],
        );
    }

    #[test]
    fn bottom() {
        // 下端
        test_get_wrapper(
            5,
            0,
            &[
                (&[(4, 1)], &[(6, 1)]),
                (&[(6, 1)], &[(4, 1)]),
                (&[(4, 1), (6, 1)], &[]),
            ],
        );
    }

    #[test]
    fn upper() {
        // 上端
        let y = MAX_HEIGHT - 1;

        test_get_wrapper(
            4,
            y,
            &[
                (&[(3, y - 1), (5, y - 1), (3, y + 1)], &[(5, y + 1)]),
                (&[(3, y - 1), (5, y - 1), (5, y + 1)], &[(3, y + 1)]),
                (&[(3, y - 1), (3, y + 1), (5, y + 1)], &[(5, y - 1)]),
                (&[(5, y - 1), (3, y + 1), (5, y + 1)], &[(3, y - 1)]),
                (&[(3, y - 1), (5, y - 1), (3, y + 1), (5, y + 1)], &[]),
            ],
        );
    }

    #[test]
    fn deleted_line_1() {
        test_get_with_delete_key_wrapper(
            4,
            2,
            key_operators::get_bit_keys(&[3, 5]),
            &[
                (&[(3, 1), (5, 1), (3, 4)], &[(5, 4)]),
                (&[(3, 1), (5, 1), (5, 4)], &[(3, 4)]),
                (&[(3, 1), (3, 4), (5, 4)], &[(5, 1)]),
                (&[(5, 1), (3, 4), (5, 4)], &[(3, 1)]),
                (&[(3, 1), (5, 1), (3, 4), (5, 4)], &[]),
            ],
        );
    }

    #[test]
    fn delete_line_2() {
        test_get_with_delete_key_wrapper(
            4,
            4,
            key_operators::get_bit_keys(&[0, 1, 3, 5]),
            &[
                (&[(3, 2), (3, 6), (5, 6)], &[(5, 2)]),
                (&[(5, 2), (3, 6), (5, 6)], &[(3, 2)]),
                (&[(3, 2), (5, 2), (3, 6)], &[(5, 6)]),
                (&[(3, 2), (5, 2), (5, 6)], &[(3, 6)]),
                (&[(3, 2), (5, 2), (3, 6), (5, 6)], &[]),
            ],
        );
    }
}
