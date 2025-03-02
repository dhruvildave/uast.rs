//! This module implements the functionality of UAST-IO and IAST to देवनागरी

use crate::utils::split_line_and_convert;

type T = (&'static str, char);

struct ScriptSpecials {
    om: char,
    halanta: char,
}

struct LangMap {
    misc: [T; 6],
    numbers: [T; 10],
    vowels: [T; 14],
    vowel_signs: [T; 13],
    consonants: [T; 34],
    specials: ScriptSpecials,
}

impl LangMap {
    fn binary_search(arr: &[T], c: &[char]) -> Option<String> {
        let mut i = 0_isize;
        let mut j = (arr.len() - 1) as isize;
        let txt = c.iter().collect::<String>();

        while i <= j {
            let m = (i + j) / 2;

            let v = arr[m as usize];

            if txt == v.0 {
                return Some(v.1.to_string());
            }

            if txt > v.0.parse().unwrap() {
                i = m + 1;
            } else {
                j = m - 1;
            }
        }

        None
    }

    fn get_vowel(&self, c: &[char]) -> Option<String> {
        Self::binary_search(&self.vowels, c)
    }

    fn get_vowelsign(&self, c: &[char]) -> Option<String> {
        Self::binary_search(&self.vowel_signs, c)
    }

    fn get_number(&self, c: &[char]) -> Option<String> {
        Self::binary_search(&self.numbers, c)
    }

    fn get_misc(&self, c: &[char]) -> Option<String> {
        Self::binary_search(&self.misc, c)
    }

    fn get_consonant(&self, c: &[char]) -> Option<String> {
        Self::binary_search(&self.consonants, c)
    }

    fn contains(arr: &[T], c: &str) -> bool {
        let mut i = 0_isize;
        let mut j = (arr.len() - 1) as isize;

        while i <= j {
            let m = (i + j) / 2;

            let v = arr[m as usize];

            if c == v.0 {
                return true;
            }

            if c > v.0 {
                i = m + 1;
            } else {
                j = m - 1;
            }
        }

        false
    }

    fn contains_vowel(&self, c: &str) -> bool {
        Self::contains(&self.vowels, c)
    }

    fn contains_vowelsign(&self, c: &str) -> bool {
        Self::contains(&self.vowel_signs, c)
    }

    fn contains_consonant(&self, c: &str) -> bool {
        Self::contains(&self.consonants, c)
    }
}

static UNICODE_MAP: [T; 19] = [
    ("a", 'ā'),
    ("au", 'ã'),
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

static CHAR_DICT: LangMap = LangMap {
    misc: [
        ("'", 'ऽ'),
        (".", '।'),
        ("..", '॥'),
        ("ã", 'ँ'),
        ("ḥ", 'ः'),
        ("ṃ", 'ं'),
    ],
    numbers: [
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
    vowels: [
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
    vowel_signs: [
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
    consonants: [
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
    specials: ScriptSpecials {
        om: 'ॐ',
        halanta: '्',
    },
};

static UNASPIRATED_CONSONANTS: [char; 10] = ['b', 'c', 'd', 'g', 'j', 'k', 'p', 't', 'ḍ', 'ṭ'];

fn unaspirated_consonants_contains(c: char) -> bool {
    let mut i = 0_isize;
    let mut j = (UNASPIRATED_CONSONANTS.len() - 1) as isize;

    while i <= j {
        let m = (i + j) / 2;

        let v = UNASPIRATED_CONSONANTS[m as usize];

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

    let mut arr = Vec::<String>::with_capacity(data.len());
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
    if LangMap::contains_vowel(&CHAR_DICT, data[0].to_string().as_str()) {
        if i + 1 < data.len() && data[0] == 'a' && (data[1] == 'i' || data[1] == 'u') {
            i = 2;
        } else {
            i = 1;
        }

        // a valid vowel exists here
        arr.push(CHAR_DICT.get_vowel(&data[0..i]).unwrap());
    }

    while i < data.len() {
        if data[i] == CHAR_DICT.specials.om {
            arr.push(CHAR_DICT.specials.om.to_string());
            i += 1;
            continue;
        }

        let c = data[i].to_string();
        if let Some(v) = CHAR_DICT.get_misc(&data[i..i + 1]) {
            if i + 1 < data.len() && data[i] == '.' && data[i + 1] == '.' {
                arr.push('॥'.to_string());
                i += 2;
            } else {
                arr.push(v.to_string());
                i += 1;
            }
            continue;
        }

        if let Some(v) = CHAR_DICT.get_number(&data[i..i + 1]) {
            arr.push(v.to_string());
            i += 1;
            continue;
        }

        // at this point, if we find any illegal character then we simply ignore it
        if !LangMap::contains_vowelsign(&CHAR_DICT, c.as_str())
            && !LangMap::contains_vowel(&CHAR_DICT, c.as_str())
            && !LangMap::contains_consonant(&CHAR_DICT, c.as_str())
        {
            i += 1;
            continue;
        }

        if i + 1 < data.len() && unaspirated_consonants_contains(data[i]) && data[i + 1] == 'h' {
            // a valid aspirated consonant exists here
            arr.push(CHAR_DICT.get_consonant(&data[i..i + 2]).unwrap());
            i += 2;
        } else {
            // if valid consonant then push it else ignore invalid consonants completely
            if let Some(v) = CHAR_DICT.get_consonant(&data[i..i + 1]) {
                arr.push(v.to_string());
                i += 1;
            }
        }

        // if end of word or anything other than a vowel-sign then we just push a halanta and start the process again
        if i == data.len()
            || (!LangMap::contains_vowelsign(&CHAR_DICT, data[i].to_string().as_str())
                && data[i] != 'a')
        {
            arr.push(CHAR_DICT.specials.halanta.to_string());
            continue;
        }

        if i + 1 < data.len() && data[i] == 'a' && (data[i + 1] == 'i' || data[i + 1] == 'u') {
            arr.push(CHAR_DICT.get_vowelsign(&data[i..i + 2]).unwrap());
            i += 2;
        } else {
            if data[i] != 'a' {
                arr.push(CHAR_DICT.get_vowelsign(&data[i..i + 1]).unwrap());
            }
            i += 1;
        }
    }

    arr.join("")
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
