use crate::{
    common::datastore::{
        action::action::Action, mino_operation::MinoOperation, operation_with_key::OperationWithKey,
    },
    sfinder_core::{
        field::{field::Field, field_constants::FIELD_WIDTH, key_operators},
        neighbor::simple_original_piece::SimpleOriginalPiece,
        srs::{rotate::Rotate, rotate_direction::RotateDirection, spin_result::SpinResult},
    },
};

fn exists_on_ground(
    init_field: &dyn Field,
    all_merged_field: &dyn Field,
    all_merged_filled_rows: u64,
    one_piece_filled_rows: u64,
    operation: SimpleOriginalPiece,
) -> bool {
    let using_key = operation.get_using_key();

    // operationで使われているラインは揃わない
    let filled_rows = all_merged_filled_rows & !using_key;

    // operationを置くのに消えている必要があるライン
    let need_deleted_key = operation.get_need_deleted_key();
    if filled_rows & need_deleted_key != need_deleted_key {
        return false;
    }

    // operationより下で消えるラインで、1ミノで即消えるライン上にはおけないので消去する
    // mask all rows below least significant bit in using_key
    let one_piece_filled_below_operation = (!using_key & (using_key - 1)) & one_piece_filled_rows;
    let mino = operation.get_mino();
    let x = operation.get_x();
    let y = operation.get_y();

    // 最初から置くことができる
    {
        let mut freeze = dyn_clone::clone_box(init_field);
        freeze.delete_rows_with_key(need_deleted_key);
        freeze.remove(mino, x, y);
        if freeze.is_on_ground(mino, x, y) {
            return true;
        }
    }

    // operationが地面の上なのか
    {
        let mut freeze = dyn_clone::clone_box(all_merged_field);
        freeze.delete_rows_with_key(need_deleted_key | one_piece_filled_below_operation);
        let ny = y - one_piece_filled_below_operation.count_ones() as u8;
        debug_assert!(ny as i8 + mino.get_min_y() >= 0);
        freeze.remove(mino, x, ny);
        if freeze.is_on_ground(mino, x, y) {
            return true;
        }
    }

    return false;
}

// Tスピンか判定
fn can_t_spin_with_filled_rows(
    field_without_t: &dyn Field,
    operation_t: SimpleOriginalPiece,
) -> bool {
    let mut freeze = dyn_clone::clone_box(field_without_t);

    // ラインを消去する
    let filled_line_without_t = freeze.clear_filled_rows_return_key();

    // 消去されたラインに合わせてyを移動
    let mino = operation_t.get_mino();
    let y = operation_t.get_y();
    let slide_y = (filled_line_without_t
        & key_operators::get_mask_for_key_below_y(
            u8::try_from(y as i8 + mino.get_min_y()).unwrap(),
        ))
    .count_ones() as u8;

    can_t_spin(freeze.as_ref(), operation_t.get_x(), y - slide_y)
}

fn can_t_spin(field: &dyn Field, x: u8, y: u8) -> bool {
    [
        (x as i8 - 1, y as i8 - 1),
        (x as i8 - 1, y as i8 + 1),
        (x as i8 + 1, y as i8 - 1),
        (x as i8 + 1, y as i8 + 1),
    ]
    .into_iter()
    .filter(|(x, y)| is_block(field, *x, *y))
    .count()
        >= 3
}

// out of bounds or exists block
fn is_block(field: &dyn Field, x: i8, y: i8) -> bool {
    x < 0 || x >= FIELD_WIDTH as i8 || y < 0 || field.exists_block(x as u8, y as u8)
}

fn get_spins(before: &dyn Field, spin_result: &SpinResult, cleared_rows: u8) -> Spin {
    let to_rotate = spin_result.get_to_rotate();
    let to_x = spin_result.x;
    let to_y = spin_result.y;

    let filled_t_front = is_filled_t_front(before, to_rotate, to_x, to_y);

    let direction = spin_result.direction;
    let name = get_t_spin_name(spin_result, to_rotate, filled_t_front, direction);

    let spin = get_t_spin(spin_result, filled_t_front);

    Spin::new(spin, name, cleared_rows)
}

// Tの凸側のブロックが両方とも埋まっているか
// `true`のとき、T-SpinはRegularになる。
// `false`のとき、MiniかRegularか判別するにはさらに条件が必要
fn is_filled_t_front(before: &dyn Field, rotate: Rotate, to_x: u8, to_y: u8) -> bool {
    match rotate {
        Rotate::Spawn => {
            is_block(before, to_x as i8 - 1, to_y as i8 + 1)
                && is_block(before, to_x as i8 + 1, to_y as i8 + 1)
        }
        Rotate::Right => {
            is_block(before, to_x as i8 + 1, to_y as i8 - 1)
                && is_block(before, to_x as i8 + 1, to_y as i8 + 1)
        }
        Rotate::Reverse => {
            is_block(before, to_x as i8 - 1, to_y as i8 - 1)
                && is_block(before, to_x as i8 + 1, to_y as i8 - 1)
        }
        Rotate::Left => {
            is_block(before, to_x as i8 - 1, to_y as i8 - 1)
                && is_block(before, to_x as i8 - 1, to_y as i8 + 1)
        }
    }
}

fn get_t_spin(spin_result: &SpinResult, filled_t_front: bool) -> TSpins {
    // 前提: Tスピンとなる条件（Tの隅に3つ以上ブロックが存在している）はこの時点で満たしている

    // Tの凸側のブロックが両方揃っている
    // or
    // TSTフォームのような特権がある場合はRegularと判定する
    // e.g. SRSでは「接着時にTが横向き and 回転テストパターンが最後のケース」の場合はRegular
    if filled_t_front || spin_result.is_privilege_spins {
        TSpins::Regular
    } else {
        TSpins::Mini
    }
}

fn get_t_spin_name(
    spin_result: &SpinResult,
    to_rotate: Rotate,
    filled_t_front: bool,
    direction: RotateDirection,
) -> TSpinNames {
    if (direction == RotateDirection::CounterClockwise && to_rotate == Rotate::Right)
        || (direction == RotateDirection::Clockwise && to_rotate == Rotate::Left)
    {
        match spin_result.test_pattern_index {
            // 正面側に2つブロックがある
            3 if filled_t_front => TSpinNames::Iso,
            3 => TSpinNames::Neo,
            4 => TSpinNames::Fin,
            _ => TSpinNames::NoName,
        }
    } else {
        TSpinNames::NoName
    }
}

struct Spin {
    spin: TSpins,
    name: TSpinNames,
    cleared_rows: ClearedRows,
}

impl Spin {
    fn new(spin: TSpins, name: TSpinNames, cleared_rows: u8) -> Self {
        debug_assert!(
            !((spin == TSpins::Regular && name == TSpinNames::Neo)
                || (spin == TSpins::Mini && matches!(name, TSpinNames::Iso | TSpinNames::Fin))),
            "invalid spin: spin={spin:?}, name={name:?}"
        );
        Self {
            spin,
            name,
            cleared_rows: ClearedRows::from(cleared_rows),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TSpins {
    Regular,
    Mini,
}

#[derive(Debug, PartialEq)]
pub enum TSpinNames {
    Iso,
    Neo,
    Fin,
    NoName,
}

pub enum ClearedRows {
    Zero = 0,
    Single,
    Double,
    Triple,
}

impl From<u8> for ClearedRows {
    fn from(value: u8) -> Self {
        match value {
            0 => ClearedRows::Zero,
            1 => ClearedRows::Single,
            2 => ClearedRows::Double,
            3 => ClearedRows::Triple,
            _ => panic!("invalid number of cleared rows: {value}"),
        }
    }
}
