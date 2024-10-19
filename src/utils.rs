//! Utility function for splitting line and converting

pub(crate) fn split_line_and_convert<F>(f: F, s: &str) -> String
where
    F: Fn(&str) -> String,
{
    s.split_whitespace().map(f).collect::<Vec<_>>().join(" ")
}
