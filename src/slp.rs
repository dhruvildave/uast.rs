//! SLP1 to IAST

use crate::utils::split_line_and_convert;

static CHAR_DICT: [(char, &str); 63] = [
    ('a', "a"),
    ('A', "ā"),
    ('i', "i"),
    ('I', "ī"),
    ('u', "u"),
    ('U', "ū"),
    ('f', "ṛ"),
    ('F', "ṝ"),
    ('x', "ḷ"),
    ('X', "ḹ"),
    ('e', "e"),
    ('E', "ai"),
    ('o', "o"),
    ('O', "au"),
    ('M', "ṃ"),
    ('H', "ḥ"),
    ('~', "ã"),
    ('.', "."),
    ('\'', "'"),
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
    ('k', "k"),
    ('K', "kh"),
    ('g', "g"),
    ('G', "gh"),
    ('N', "ṅ"),
    ('c', "c"),
    ('C', "ch"),
    ('j', "j"),
    ('J', "jh"),
    ('Y', "ñ"),
    ('w', "ṭ"),
    ('W', "ṭh"),
    ('q', "ḍ"),
    ('Q', "ḍh"),
    ('R', "ṇ"),
    ('t', "t"),
    ('T', "th"),
    ('d', "d"),
    ('D', "dh"),
    ('n', "n"),
    ('p', "p"),
    ('P', "ph"),
    ('b', "b"),
    ('B', "bh"),
    ('m', "m"),
    ('y', "y"),
    ('r', "r"),
    ('l', "l"),
    ('v', "v"),
    ('S', "ś"),
    ('z', "ṣ"),
    ('s', "s"),
    ('h', "h"),
    ('L', "ḻ"),
];

fn get_char(c: char) -> &'static str {
    if let Some(v) = CHAR_DICT.iter().find(|x| x.0 == c) {
        v.1
    } else {
        ""
    }
}

fn convertor(dn: &str) -> String {
    dn.chars().map(get_char).collect()
}

/// This function converts SLP to IAST
///
/// ```
/// use uast::slp_to_iast;
///
/// let s = "om BUrBuvaH svaH tatsaviturvareRyaM Bargo devasya DImahi. Diyo yo naH pracodayAt..".to_string();
/// assert_eq!(
///     "om bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt..".to_string(),
///     slp_to_iast(&s)
/// );
/// ```
pub fn slp_to_iast(dn: &str) -> String {
    split_line_and_convert(convertor, dn)
}
