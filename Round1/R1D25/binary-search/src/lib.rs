pub fn find<T, A>(array: A, key: T) -> Option<usize>
where
    T: PartialOrd,
    A: AsRef<[T]>,
{
    let list = array.as_ref();
    if list.is_empty() {
        return None;
    }

    let mut begin = 0;
    let mut end = list.len() - 1;

    while begin <= end {
        let middle = (begin + end) / 2;
        if key < list[middle] {
            end = match middle.checked_sub(1) {
                Some(v) => v,
                None => return None,
            };
        } else if key > list[middle] {
            begin = middle + 1;
        } else {
            return Some(middle);
        }
    }
    None
}