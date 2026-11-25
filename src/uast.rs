//! This module implements the functionality of UAST-IO and IAST to देवनागरी

use crate::utils::split_line_and_convert;

type T = (&'static str, char);

struct ScriptSpecials {
    om: char,
    halanta: char,
    saṃkṣipta: char,
}

struct LangMap<'a> {
    misc: &'a [T; 7],
    numbers: &'a [T; 10],
    vowels: &'a [T; 14],
    vowel_signs: &'a [T; 13],
    consonants: &'a [T; 34],
    specials: &'a ScriptSpecials,
}

impl LangMap<'_> {
    fn binary_search(arr: &[T], c: &[char]) -> Option<char> {
        arr.binary_search_by(|v| v.0.chars().cmp(c.iter().copied()))
            .ok()
            .map(|i| arr[i].1)
    }

    fn get_vowel(&self, c: &[char]) -> Option<char> {
        Self::binary_search(self.vowels, c)
    }

    fn get_vowelsign(&self, c: &[char]) -> Option<char> {
        Self::binary_search(self.vowel_signs, c)
    }

    fn get_number(&self, c: &[char]) -> Option<char> {
        Self::binary_search(self.numbers, c)
    }

    fn get_misc(&self, c: &[char]) -> Option<char> {
        Self::binary_search(self.misc, c)
    }

    fn get_consonant(&self, c: &[char]) -> Option<char> {
        Self::binary_search(self.consonants, c)
    }

    fn contains(arr: &[T], c: char) -> bool {
        let mut buf = [0u8; 4];
        let c: &str = c.encode_utf8(&mut buf);
        arr.binary_search_by(|v| v.0.cmp(&c)).is_ok()
    }

    fn contains_vowel(&self, c: char) -> bool {
        Self::contains(self.vowels, c)
    }

    fn contains_vowelsign(&self, c: char) -> bool {
        Self::contains(self.vowel_signs, c)
    }

    fn contains_consonant(&self, c: char) -> bool {
        Self::contains(self.consonants, c)
    }
}

static VEDA_ACCENTS: &[char; 14] = &['॑', '॒', '᳚', '꣠', '꣡', '꣢', '꣣', '꣤', '꣥', '꣦', '꣧', '꣨', '꣩', 'ꣳ'];

static UNICODE_MAP: &[T; 34] = &[
    ("'", '॑'),
    ("''", '᳚'),
    ("-", '॒'),
    (".", '॰'),
    ("0", '꣠'),
    ("1", '꣡'),
    ("2", '꣢'),
    ("3", '꣣'),
    ("4", '꣤'),
    ("5", '꣥'),
    ("6", '꣦'),
    ("7", '꣧'),
    ("8", '꣨'),
    ("9", '꣩'),
    ("a", 'ā'),
    ("au", 'ã'),
    ("cv", 'ꣳ'),
    ("d", 'ḍ'),
    ("h", 'ḥ'),
    ("i", 'ī'),
    ("l", 'ḷ'),
    ("ll", 'ḻ'),
    ("lu", 'ḹ'),
    ("m", 'ṃ'),
    ("n", 'ñ'),
    ("nl", 'ṇ'),
    ("nu", 'ṅ'),
    ("om", 'ॐ'),
    ("r", 'ṛ'),
    ("ru", 'ṝ'),
    ("sl", 'ṣ'),
    ("su", 'ś'),
    ("t", 'ṭ'),
    ("u", 'ū'),
];

fn unicode_map_binary_search(c: &str) -> Option<char> {
    let mut i = 0_isize;
    let mut j = (UNICODE_MAP.len() - 1) as isize;

    while i <= j {
        let m = (i + j) / 2;

        let v = UNICODE_MAP[m as usize];

        if c == v.0 {
            return Some(v.1);
        }

        if c > v.0 {
            i = m + 1;
        } else {
            j = m - 1;
        }
    }

    None
}

static CHAR_DICT: &LangMap = &LangMap {
    misc: &[
        ("'", 'ऽ'),
        ("-", 'ॱ'),
        (".", '।'),
        ("..", '॥'),
        ("ã", 'ँ'),
        ("ḥ", 'ः'),
        ("ṃ", 'ं'),
    ],
    numbers: &[
        ("0", '०'),
        ("1", '१'),
        ("2", '२'),
        ("3", '३'),
        ("4", '४'),
        ("5", '५'),
        ("6", '६'),
        ("7", '७'),
        ("8", '८'),
        ("9", '९'),
    ],
    vowels: &[
        ("a", 'अ'),
        ("ai", 'ऐ'),
        ("au", 'औ'),
        ("e", 'ए'),
        ("i", 'इ'),
        ("o", 'ओ'),
        ("u", 'उ'),
        ("ā", 'आ'),
        ("ī", 'ई'),
        ("ū", 'ऊ'),
        ("ḷ", 'ऌ'),
        ("ḹ", 'ॡ'),
        ("ṛ", 'ऋ'),
        ("ṝ", 'ॠ'),
    ],
    vowel_signs: &[
        ("ai", 'ै'),
        ("au", 'ौ'),
        ("e", 'े'),
        ("i", 'ि'),
        ("o", 'ो'),
        ("u", 'ु'),
        ("ā", 'ा'),
        ("ī", 'ी'),
        ("ū", 'ू'),
        ("ḷ", 'ॢ'),
        ("ḹ", 'ॣ'),
        ("ṛ", 'ृ'),
        ("ṝ", 'ॄ'),
    ],
    consonants: &[
        ("b", 'ब'),
        ("bh", 'भ'),
        ("c", 'च'),
        ("ch", 'छ'),
        ("d", 'द'),
        ("dh", 'ध'),
        ("g", 'ग'),
        ("gh", 'घ'),
        ("h", 'ह'),
        ("j", 'ज'),
        ("jh", 'झ'),
        ("k", 'क'),
        ("kh", 'ख'),
        ("l", 'ल'),
        ("m", 'म'),
        ("n", 'न'),
        ("p", 'प'),
        ("ph", 'फ'),
        ("r", 'र'),
        ("s", 'स'),
        ("t", 'त'),
        ("th", 'थ'),
        ("v", 'व'),
        ("y", 'य'),
        ("ñ", 'ञ'),
        ("ś", 'श'),
        ("ḍ", 'ड'),
        ("ḍh", 'ढ'),
        ("ḻ", 'ळ'),
        ("ṅ", 'ङ'),
        ("ṇ", 'ण'),
        ("ṣ", 'ष'),
        ("ṭ", 'ट'),
        ("ṭh", 'ठ'),
    ],
    specials: &ScriptSpecials {
        om: 'ॐ',
        halanta: '्',
        saṃkṣipta: '॰',
    },
};

static UNASPIRATED_CONSONANTS: &[char; 10] = &['b', 'c', 'd', 'g', 'j', 'k', 'p', 't', 'ḍ', 'ṭ'];

fn char_slice_contains(slice: &[char], c: char) -> bool {
    let mut i = 0_isize;
    let mut j = (slice.len() - 1) as isize;

    while i <= j {
        let m = (i + j) / 2;

        let v = slice[m as usize];

        if c == v {
            return true;
        }

        if c > v {
            i = m + 1;
        } else {
            j = m - 1;
        }
    }

    false
}

fn handle_unicode(uast: &str) -> Vec<char> {
    let str = uast.to_lowercase().chars().collect::<Vec<char>>();

    let mut arr = Vec::<char>::with_capacity(str.len());

    let mut i = 0;
    while i < str.len() {
        let curr = str[i];

        if curr != '/' {
            arr.push(curr);
            i += 1;
            continue;
        }

        let mut c = String::with_capacity(2);

        for j in (i + 1)..str.len() {
            let curr = str[j];
            if curr == '/' {
                i = j;
                break;
            }

            if j == str.len() - 1 {
                i = j;
            }

            c.push(curr);
        }

        if let Some(v) = unicode_map_binary_search(&c) {
            arr.push(v);
        }

        i += 1;
    }

    arr
}

fn iast_to_devanāgarī(data: Vec<char>) -> String {
    if data.is_empty() {
        return "".to_string();
    }

    // Build the result in one buffer. Most output chars are 3-byte UTF-8, so this is a capacity hint.
    let mut out = String::with_capacity(data.len() * 3);
    let mut i = 0;

    // here's a little thing about how saṃskṛta and devanāgarī work:
    // the general formation of a syllable in saṃskṛta is `consonant + vowel_sign`.
    // there is a designated symbol for each consonant and vowel_sign in devanāgarī.
    // in the case where the word may start with a vowel, we put a special symbol instead of its sign.
    // if we encounter a miscellaneous symbol, we put its designated symbol and start finding the consonant again.
    // in the case that the `misc` symbol occurs in middle of a word like an avagraha, we first have to finish the
    // preceding syllable. if that syllable was a running consonant, we add a halanta. we also add halanta
    // at end of word to represent a consonant without a corresponding vowel with it.

    // starts with vowel
    if LangMap::contains_vowel(CHAR_DICT, data[0]) {
        if i + 1 < data.len() && data[0] == 'a' && (data[1] == 'i' || data[1] == 'u') {
            i = 2;
        } else {
            i = 1;
        }

        // a valid vowel exists here
        out.push(CHAR_DICT.get_vowel(&data[0..i]).unwrap());
    }

    while i < data.len() {
        if char_slice_contains(VEDA_ACCENTS, data[i]) {
            out.push(data[i]);
            i += 1;
            continue;
        }

        if data[i] == CHAR_DICT.specials.om {
            out.push(CHAR_DICT.specials.om);
            i += 1;
            continue;
        }

        if data[i] == CHAR_DICT.specials.saṃkṣipta {
            out.push(CHAR_DICT.specials.saṃkṣipta);
            i += 1;
            continue;
        }

        if let Some(v) = CHAR_DICT.get_misc(&data[i..i + 1]) {
            if i + 1 < data.len() && data[i] == '.' && data[i + 1] == '.' {
                out.push('॥');
                i += 2;
            } else {
                out.push(v);
                i += 1;
            }
            continue;
        }

        if let Some(v) = CHAR_DICT.get_number(&data[i..i + 1]) {
            out.push(v);
            i += 1;
            continue;
        }

        // at this point, if we find any illegal character then we simply ignore it
        if !LangMap::contains_vowelsign(CHAR_DICT, data[i])
            && !LangMap::contains_vowel(CHAR_DICT, data[i])
            && !LangMap::contains_consonant(CHAR_DICT, data[i])
        {
            i += 1;
            continue;
        }

        if i + 1 < data.len()
            && char_slice_contains(UNASPIRATED_CONSONANTS, data[i])
            && data[i + 1] == 'h'
        {
            // a valid aspirated consonant exists here
            out.push(CHAR_DICT.get_consonant(&data[i..i + 2]).unwrap());
            i += 2;
        } else {
            // if valid consonant then push it else ignore invalid consonants completely
            if let Some(v) = CHAR_DICT.get_consonant(&data[i..i + 1]) {
                out.push(v);
                i += 1;
            }
        }

        // if end of word or anything other than a vowel-sign then we just push a halanta and start the process again
        if i == data.len() || (!LangMap::contains_vowelsign(CHAR_DICT, data[i]) && data[i] != 'a') {
            out.push(CHAR_DICT.specials.halanta);
            continue;
        }

        if i + 1 < data.len() && data[i] == 'a' && (data[i + 1] == 'i' || data[i + 1] == 'u') {
            out.push(CHAR_DICT.get_vowelsign(&data[i..i + 2]).unwrap());
            i += 2;
        } else {
            if data[i] != 'a' {
                out.push(CHAR_DICT.get_vowelsign(&data[i..i + 1]).unwrap());
            }
            i += 1;
        }
    }

    out
}

fn convertor(line: &str) -> String {
    iast_to_devanāgarī(handle_unicode(line))
}

/// This function can accept both UAST-IO and IAST and returns देवनागरी.
///
/// ```
/// use uast::uast_to_devanāgarī;
///
/// let s =
///     "/om/ bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt..";
/// assert_eq!(
///     "ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥",
///     uast_to_devanāgarī(&s)
/// );
/// ```
pub fn uast_to_devanāgarī(line: &str) -> String {
    split_line_and_convert(convertor, line)
}

#[cfg(test)]
mod invariants {
    //! The lookups in this module use binary search, and the `unwrap()` calls in
    //! `iast_to_devanāgarī` are safe only because of the table facts checked here.
    //! These tests stop a future table edit from becoming a wrong lookup or a panic.

    use super::*;
    use crate::utils::{keys_strictly_increasing, strictly_increasing};

    #[test]
    fn tables_are_strictly_sorted() {
        // `get_*` compares the `&str` keys by char and `contains` compares them by byte.
        // UTF-8 byte order matches code point order, so one check covers both lookups.
        assert!(keys_strictly_increasing(UNICODE_MAP));
        assert!(keys_strictly_increasing(CHAR_DICT.misc));
        assert!(keys_strictly_increasing(CHAR_DICT.numbers));
        assert!(keys_strictly_increasing(CHAR_DICT.vowels));
        assert!(keys_strictly_increasing(CHAR_DICT.vowel_signs));
        assert!(keys_strictly_increasing(CHAR_DICT.consonants));
        assert!(strictly_increasing(VEDA_ACCENTS));
        assert!(strictly_increasing(UNASPIRATED_CONSONANTS));
    }

    #[test]
    fn hand_written_searches_find_every_entry() {
        for &(k, v) in UNICODE_MAP {
            assert_eq!(
                unicode_map_binary_search(k),
                Some(v),
                "UNICODE_MAP key {k:?}"
            );
        }
        assert_eq!(unicode_map_binary_search(""), None);
        assert_eq!(unicode_map_binary_search("zz"), None);

        for &c in VEDA_ACCENTS {
            assert!(char_slice_contains(VEDA_ACCENTS, c));
        }
        for &c in UNASPIRATED_CONSONANTS {
            assert!(char_slice_contains(UNASPIRATED_CONSONANTS, c));
        }
        assert!(!char_slice_contains(UNASPIRATED_CONSONANTS, 'a'));
        assert!(!char_slice_contains(UNASPIRATED_CONSONANTS, 'z'));
    }

    #[test]
    fn aspirated_consonants_exist() {
        // Unwrapped: the lookup of an unaspirated consonant followed by 'h'.
        for &c in UNASPIRATED_CONSONANTS {
            assert!(CHAR_DICT.get_consonant(&[c, 'h']).is_some(), "missing {c}h");
        }
    }

    #[test]
    fn diphthongs_are_vowels_and_vowel_signs() {
        // Unwrapped: the lookup of "ai" and "au" as a vowel and as a vowel sign.
        for d in [['a', 'i'], ['a', 'u']] {
            assert!(CHAR_DICT.get_vowel(&d).is_some());
            assert!(CHAR_DICT.get_vowelsign(&d).is_some());
        }
    }

    #[test]
    fn single_char_vowels_other_than_a_are_vowel_signs() {
        // Unwrapped: a one-char vowel that reaches the vowel-sign branch is looked up as a sign.
        // Only 'a' (the inherent vowel) has no sign, and the converter handles it separately.
        fn singles(arr: &[T]) -> Vec<&'static str> {
            arr.iter()
                .map(|v| v.0)
                .filter(|k| k.chars().count() == 1)
                .collect()
        }

        let vowels: Vec<&'static str> = singles(CHAR_DICT.vowels)
            .into_iter()
            .filter(|&k| k != "a")
            .collect();
        assert_eq!(vowels, singles(CHAR_DICT.vowel_signs));
    }
}
