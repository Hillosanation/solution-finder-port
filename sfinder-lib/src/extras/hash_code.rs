pub trait HashCode {
    // Useful for composing hash codes for the guarentee in NoHashHasher.
    fn hash_code(&self) -> u64;
}
