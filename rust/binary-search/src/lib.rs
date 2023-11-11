use std::cmp::Ordering;

pub fn find<T: AsRef<[U]>, U: Ord>(array: T, key: U) -> Option<usize> {
    let a = array.as_ref();
    let mut range = 0..a.len();
    while range.start < range.end {
        let mid = range.start + (range.end - range.start) / 2;
        match a[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => range.start = mid + 1,
            _ => range.end = mid,
        }
    }
    None
}
