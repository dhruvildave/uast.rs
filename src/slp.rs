//! SLP1 to IAST

use crate::utils::{binary_search, split_line_and_convert};

static CHAR_DICT: [(char, &str); 63] = [
    ('\'', "'"),
    ('.', "."),
    ('0', "0"),
    ('1', "1"),
    ('2', "2"),
    ('3', "3"),
    ('4', "4"),
    ('5', "5"),
    ('6', "6"),
    ('7', "7"),
    ('8', "8"),
    ('9', "9"),
    ('A', "ā"),
    ('B', "bh"),
    ('C', "ch"),
    ('D', "dh"),
    ('E', "ai"),
    ('F', "ṝ"),
    ('G', "gh"),
    ('H', "ḥ"),
    ('I', "ī"),
    ('J', "jh"),
    ('K', "kh"),
    ('L', "ḻ"),
    ('M', "ṃ"),
    ('N', "ṅ"),
    ('O', "au"),
    ('P', "ph"),
    ('Q', "ḍh"),
    ('R', "ṇ"),
    ('S', "ś"),
    ('T', "th"),
    ('U', "ū"),
    ('W', "ṭh"),
    ('X', "ḹ"),
    ('Y', "ñ"),
    ('a', "a"),
    ('b', "b"),
    ('c', "c"),
    ('d', "d"),
    ('e', "e"),
    ('f', "ṛ"),
    ('g', "g"),
    ('h', "h"),
    ('i', "i"),
    ('j', "j"),
    ('k', "k"),
    ('l', "l"),
    ('m', "m"),
    ('n', "n"),
    ('o', "o"),
    ('p', "p"),
    ('q', "ḍ"),
    ('r', "r"),
    ('s', "s"),
    ('t', "t"),
    ('u', "u"),
    ('v', "v"),
    ('w', "ṭ"),
    ('x', "ḷ"),
    ('y', "y"),
    ('z', "ṣ"),
    ('~', "ã"),
];

fn get_char(c: char) -> Option<&'static str> {
    binary_search(&CHAR_DICT, c, |i| i)
}

fn convertor(dn: &str) -> String {
    dn.chars().filter_map(get_char).collect()
}

/// This function converts SLP to IAST
///
/// ```
/// use uast::slp_to_iast;
///
/// let s = "om BUrBuvaH svaH tatsaviturvareRyaM Bargo devasya DImahi. Diyo yo naH pracodayAt..";
/// assert_eq!(
///     "om bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt..",
///     slp_to_iast(&s)
/// );
/// ```
pub fn slp_to_iast(dn: &str) -> String {
    split_line_and_convert(convertor, dn)
}
