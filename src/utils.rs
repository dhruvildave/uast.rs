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
