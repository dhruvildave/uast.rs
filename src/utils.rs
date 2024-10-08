//! Utility function for splitting line and converting

pub(crate) fn split_line_and_convert<F>(f: F, s: &String) -> String
where
    F: Fn(&String) -> String,
{
    s.split_whitespace()
        .map(|i| f(&i.to_string()))
        .collect::<Vec<String>>()
        .join(" ")
}
