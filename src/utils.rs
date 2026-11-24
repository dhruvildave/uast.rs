//! Utility function for splitting line and converting

pub(crate) fn split_line_and_convert<F>(f: F, s: &str) -> String
where
    F: Fn(&str) -> String,
{
    let mut res = String::with_capacity(s.len());

    for (i, word) in s.split_whitespace().enumerate() {
        if i > 0 {
            res.push(' ');
        }
        res.push_str(&f(word));
    }
    res
}

pub(crate) fn binary_search<U, V, F>(arr: &[(char, U)], c: char, f: F) -> Option<V>
where
    U: Copy,
    F: Fn(U) -> V,
{
    let i = arr.binary_search_by_key(&c, |&(key, _)| key).ok()?;
    Some(f(arr[i].1))
}

/// Whether `items` are strictly increasing: sorted, with no duplicates.
/// Every binary search over a lookup table in this crate relies on that.
#[cfg(test)]
pub(crate) fn strictly_increasing<K: PartialOrd>(items: impl IntoIterator<Item = K>) -> bool {
    items.into_iter().is_sorted_by(|a, b| a < b)
}

/// Whether the keys of a `(key, value)` lookup table are strictly increasing.
#[cfg(test)]
pub(crate) fn keys_strictly_increasing<K: PartialOrd, V>(table: &[(K, V)]) -> bool {
    strictly_increasing(table.iter().map(|(k, _)| k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strictly_increasing_rejects_unsorted_and_duplicate_keys() {
        assert!(strictly_increasing([1, 2, 3]));
        assert!(strictly_increasing(Vec::<i32>::new()));
        assert!(!strictly_increasing([1, 3, 2]));
        assert!(!strictly_increasing([1, 1]));
        assert!(keys_strictly_increasing(&[('a', 0), ('b', 0)]));
        assert!(!keys_strictly_increasing(&[('a', 0), ('a', 1)]));
    }
}
