use crate::{
    common::datastore::{operation_with_key::OperationWithKey, piece_counter::PieceCounter},
    searcher::pack::{
        mino_field::mino_field::MinoField, separable_mino::separable_mino::SeparableMino,
    },
};

// TODO: investigate if the boxing is necessary
pub trait MinoFieldMemento {
    fn concat(&self, mino_field: &dyn MinoField) -> Box<dyn MinoFieldMemento>;

    fn skip(&self) -> Box<dyn MinoFieldMemento>;

    fn get_sum_block_counter(&self) -> PieceCounter;

    fn is_concat(&self) -> bool;

    fn get_raw_opertion_stream(&self) -> Box<dyn Iterator<Item = Box<dyn OperationWithKey>>>;

    fn get_operation_stream(
        &self,
        width: u8,
    ) -> Box<dyn Iterator<Item = Box<dyn OperationWithKey>>>;

    fn get_separable_mino_stream(&self) -> Box<dyn Iterator<Item = Box<dyn SeparableMino>>>;
}
