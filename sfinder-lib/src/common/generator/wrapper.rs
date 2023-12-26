/// Porting note: Helper function to fold the mask in half.
/// 100000000020000000003000000000400000000050000000006 -> 1400000000250000000036
// TODO: remove wrapper and use the mask directly? This is also just pregenerated.
pub fn wrapper(mask: u64) -> u64 {
    mask >> (30 - 1) | (mask & ((1 << 30) - 1))
}
