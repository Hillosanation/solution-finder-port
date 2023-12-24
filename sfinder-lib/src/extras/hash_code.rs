pub trait HashCode {
    type Output;
    // Useful for composing hash codes for the guarentee in NoHashHasher.
    fn hash_code(&self) -> Self::Output;
}
