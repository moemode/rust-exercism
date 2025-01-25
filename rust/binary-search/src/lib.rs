pub fn find<U: AsRef<[T]>, T: Ord>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();
    let (mut l, mut r) = (0, array.len());
    while l < r {
        let mid = (l + r) / 2;
        match array[mid].cmp(&key) {
            std::cmp::Ordering::Less => l = mid + 1,
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Greater => r = mid,
        }
    }
    None
}
