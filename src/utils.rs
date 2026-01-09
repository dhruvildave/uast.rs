//! Utility function for splitting line and converting

pub(crate) fn split_line_and_convert<F>(f: F, s: &str) -> String
where
    F: Fn(&str) -> String,
{
    let mut res = String::new();
    let mut first = true;

    for word in s.split_whitespace() {
        if !first {
            res.push(' ');
        }
        res.push_str(&f(word));
        first = false;
    }
    res
}

pub(crate) fn binary_search<U, V, F>(arr: &[(char, U)], c: char, f: F) -> Option<V>
where
    U: Copy,
    F: Fn(U) -> V,
{
    if arr.is_empty() {
        return None;
    }

    let mut i = 0;
    let mut j = arr.len() - 1;
    let p = arr.as_ptr();

    while i <= j {
        let m = i + (j - i) / 2;
        let v = unsafe { *p.add(m) };

        if c == v.0 {
            return Some(f(v.1));
        }

        if c > v.0 {
            i = match m.checked_add(1) {
                Some(val) => val,
                None => return None,
            };
        } else {
            j = match m.checked_sub(1) {
                Some(val) => val,
                None => return None,
            };
        }
    }

    None
}
