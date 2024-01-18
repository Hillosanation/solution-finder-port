//! Extra structures that make porting easier, by mimicking builtin methods of Objects in Java as traits.

pub mod callable;
pub mod hash_code;
pub mod test_functions;

// TODO: create Index impls for Piece/Rotate to refactor casts to usize
