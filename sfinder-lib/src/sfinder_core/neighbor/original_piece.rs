use crate::sfinder_core::field::field::Field;

// Porting note: EMPTY_COLLIDER_PIECE was used as a null check and is removed.
struct OriginalPiece {
    operation_with_key: FullOperationWithKey,
    harddrop_collider: Box<dyn Field>,
    mino_field: Box<dyn Field>,
}

struct FullOperationWithKey {}
