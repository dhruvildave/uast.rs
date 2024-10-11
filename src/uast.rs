//! This module implements the functionality of UAST-IO and IAST to देवनागरी

use crate::utils::split_line_and_convert;

type T = (&'static str, char);

struct ScriptSpecials {
    om: char,
    halanta: char,
}

struct LangMap {
    misc: [T; 4],
    numbers: [T; 10],
    vowels: [T; 14],
    vowel_signs: [T; 15],
    consonants: [T; 34],
    specials: ScriptSpecials,
}

impl LangMap {
    fn get_vowel(&self, c: &[char]) -> Option<String> {
        self.vowels
            .iter()
            .find(|x| x.0 == c.iter().collect::<String>())
            .map(|i| i.1.to_string())
    }

    fn get_vowelsign(&self, c: &[char]) -> Option<String> {
        self.vowel_signs
            .iter()
            .find(|x| x.0 == c.iter().collect::<String>())
            .map(|i| i.1.to_string())
    }

    fn get_number(&self, c: &[char]) -> Option<String> {
        self.numbers
            .iter()
            .find(|x| x.0 == c.iter().collect::<String>())
            .map(|i| i.1.to_string())
    }

    fn get_misc(&self, c: &[char]) -> Option<String> {
        self.misc
            .iter()
            .find(|x| x.0 == c.iter().collect::<String>())
            .map(|i| i.1.to_string())
    }

    fn get_consonant(&self, c: &[char]) -> Option<String> {
        self.consonants
            .iter()
            .find(|x| x.0 == c.iter().collect::<String>())
            .map(|i| i.1.to_string())
    }
}

static UNICODE_MAP: [T; 19] = [
    ("a", 'ā'),
    ("i", 'ī'),
    ("u", 'ū'),
    ("r", 'ṛ'),
    ("ru", 'ṝ'),
    ("l", 'ḷ'),
    ("lu", 'ḹ'),
    ("ll", 'ḻ'),
    ("t", 'ṭ'),
    ("d", 'ḍ'),
    ("m", 'ṃ'),
    ("h", 'ḥ'),
    ("n", 'ñ'),
    ("nu", 'ṅ'),
    ("nl", 'ṇ'),
    ("su", 'ś'),
    ("sl", 'ṣ'),
    ("au", 'ã'),
    ("om", 'ॐ'),
];

static CHAR_DICT: LangMap = LangMap {
    misc: [(".", '।'), ("..", '॥'), ("'", 'ऽ'), ("ã", 'ँ')],
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
        ("ā", 'आ'),
        ("i", 'इ'),
        ("ī", 'ई'),
        ("u", 'उ'),
        ("ū", 'ऊ'),
        ("ṛ", 'ऋ'),
        ("ṝ", 'ॠ'),
        ("ḷ", 'ऌ'),
        ("ḹ", 'ॡ'),
        ("e", 'ए'),
        ("ai", 'ऐ'),
        ("o", 'ओ'),
        ("au", 'औ'),
    ],
    vowel_signs: [
        ("ā", 'ा'),
        ("i", 'ि'),
        ("ī", 'ी'),
        ("u", 'ु'),
        ("ū", 'ू'),
        ("ṛ", 'ृ'),
        ("ṝ", 'ॄ'),
        ("ḷ", 'ॢ'),
        ("ḹ", 'ॣ'),
        ("e", 'े'),
        ("ai", 'ै'),
        ("o", 'ो'),
        ("au", 'ौ'),
        ("ṃ", 'ं'),
        ("ḥ", 'ः'),
    ],
    consonants: [
        ("k", 'क'),
        ("kh", 'ख'),
        ("g", 'ग'),
        ("gh", 'घ'),
        ("ṅ", 'ङ'),
        ("c", 'च'),
        ("ch", 'छ'),
        ("j", 'ज'),
        ("jh", 'झ'),
        ("ñ", 'ञ'),
        ("ṭ", 'ट'),
        ("ṭh", 'ठ'),
        ("ḍ", 'ड'),
        ("ḍh", 'ढ'),
        ("ṇ", 'ण'),
        ("t", 'त'),
        ("th", 'थ'),
        ("d", 'द'),
        ("dh", 'ध'),
        ("n", 'न'),
        ("p", 'प'),
        ("ph", 'फ'),
        ("b", 'ब'),
        ("bh", 'भ'),
        ("m", 'म'),
        ("y", 'य'),
        ("r", 'र'),
        ("l", 'ल'),
        ("v", 'व'),
        ("ś", 'श'),
        ("ṣ", 'ष'),
        ("s", 'स'),
        ("h", 'ह'),
        ("ḻ", 'ळ'),
    ],
    specials: ScriptSpecials {
        om: 'ॐ',
        halanta: '्',
    },
};

static UNASPIRATED_CONSONANTS: [char; 10] = ['b', 'c', 'd', 'g', 'j', 'k', 'p', 't', 'ḍ', 'ṭ'];

fn handle_unicode(uast: &String) -> Vec<char> {
    let str = uast.trim().to_lowercase().chars().collect::<Vec<char>>();

    let mut arr = Vec::<char>::new();

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

        if let Some(v) = UNICODE_MAP.iter().find(|i| i.0 == c) {
            arr.push(v.1);
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
    if CHAR_DICT.vowels.iter().any(|c| c.0 == data[0].to_string()) {
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
        if !CHAR_DICT.vowel_signs.iter().any(|i| i.0 == c)
            && !CHAR_DICT.vowels.iter().any(|i| i.0 == c)
            && !CHAR_DICT.consonants.iter().any(|i| i.0 == c)
        {
            i += 1;
            continue;
        }

        if i + 1 < data.len() && UNASPIRATED_CONSONANTS.contains(&data[i]) && data[i + 1] == 'h' {
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
            || (!CHAR_DICT
                .vowel_signs
                .iter()
                .any(|c| c.0 == data[i].to_string())
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

fn convertor(line: &String) -> String {
    iast_to_devanāgarī(handle_unicode(line))
}

/// This function can accept both UAST-IO and IAST and returns देवनागरी.
///
/// ```
/// use uast::process_uast;
///
/// fn main() {
///     let s =
///         "/om/ bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt.."
///             .to_string();
///     assert_eq!(
///         "ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥".to_string(),
///         process_uast(&s)
///     );
/// }
/// ```
pub fn process_uast(line: &String) -> String {
    split_line_and_convert(convertor, line)
}
