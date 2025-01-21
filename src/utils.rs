//! Utility function for splitting line and converting

pub(crate) fn split_line_and_convert<F>(f: F, s: &str) -> String
where
    F: Fn(&str) -> String,
{
    s.split_whitespace().map(f).collect::<Vec<_>>().join(" ")
}

pub(crate) fn binary_search<U, V, F>(arr: &[(char, U)], c: char, f: F) -> Option<V>
where
    U: Copy,
    F: Fn(U) -> V,
{
    let mut i = 0_isize;
    let mut j = (arr.len() - 1) as isize;

    while i <= j {
        let m = (i + j) / 2;
        let v = arr[m as usize];

        if c == v.0 {
            return Some(f(v.1));
        }

        if c > v.0 {
            i = m + 1;
        } else {
            j = m - 1;
        }
    }

    None
}
