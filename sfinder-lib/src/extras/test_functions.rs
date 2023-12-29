#[cfg(test)]
pub fn assert_partialord_symmetric<T: PartialOrd>(a: T, b: T) {
    assert_eq!(a.partial_cmp(&b), b.partial_cmp(&a).map(|o| o.reverse()));
}
