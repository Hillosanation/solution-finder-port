use super::mino_field_memento::MinoFieldMemento;
use crate::searcher::pack::mino_field::mino_field::MinoField;

// マルチスレッドに対応していなければならない
pub trait SolutionFilter {
    // memento が有効な場合は true を返却する
    fn test(&self, memento: &dyn MinoFieldMemento) -> bool;

    fn test_last(&self, memento: &dyn MinoFieldMemento) -> bool;

    fn test_mino_field(&self, mino_field: &dyn MinoField) -> bool;
}
