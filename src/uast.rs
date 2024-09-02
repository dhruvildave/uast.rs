//! Rust implementation of [Unicode Aware Saṃskṛta Transliteration](https://arxiv.org/html/2203.14277)

use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

type CharMap<'a> = HashMap<&'a str, char>;

struct ScriptSpecials {
    om: char,
    halanta: char,
}

struct LangMap<'a> {
    misc: CharMap<'a>,
    numbers: CharMap<'a>,
    vowels: CharMap<'a>,
    vowel_signs: CharMap<'a>,
    consonants: CharMap<'a>,
    specials: ScriptSpecials,
}

static UNICODE_MAP: LazyLock<HashMap<&str, CharMap>> = LazyLock::new(|| {
    HashMap::from([(
        "sa",
        CharMap::from([
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
        ]),
    )])
});

static CHAR_DICT: LazyLock<HashMap<&str, LangMap>> = LazyLock::new(|| {
    HashMap::from([(
        "sa",
        LangMap {
            misc: CharMap::from([(".", '।'), ("..", '॥'), ("'", 'ऽ'), ("ã", 'ँ')]),
            numbers: CharMap::from([
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
            ]),
            vowels: CharMap::from([
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
            ]),
            vowel_signs: CharMap::from([
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
            ]),
            consonants: CharMap::from([
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
            ]),
            specials: ScriptSpecials {
                om: 'ॐ',
                halanta: '्',
            },
        },
    )])
});

static UNASPIRATED_CONSONANTS: LazyLock<HashSet<char>> =
    LazyLock::new(|| HashSet::from(['b', 'c', 'd', 'g', 'j', 'k', 'p', 't', 'ḍ', 'ṭ']));

fn chars_to_string(data: &[char], start: usize, end: usize) -> String {
    data[start..end].iter().collect::<String>()
}

fn handle_unicode(script: &str, uast: String) -> Vec<char> {
    let m = UNICODE_MAP.get(script).unwrap();
    let str = uast.to_lowercase().chars().collect::<Vec<char>>();

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

        if let Some(v) = m.get(c.as_str()) {
            arr.push(*v);
        }

        i += 1;
    }

    arr
}

fn iast_to_devanāgarī(script: &str, data: Vec<char>) -> String {
    let m = CHAR_DICT.get(script).unwrap();
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
    if m.vowels.contains_key(data[0].to_string().as_str()) {
        if i + 1 < data.len() && data[0] == 'a' && (data[1] == 'i' || data[1] == 'u') {
            i = 2;
        } else {
            i = 1;
        }

        // a valid vowel exists here
        arr.push(m.vowels[chars_to_string(&data, 0, i).as_str()].to_string());
    }

    while i < data.len() {
        if data[i] == m.specials.om {
            arr.push(m.specials.om.to_string());
            i += 1;
            continue;
        }

        if let Some(v) = m.misc.get(data[i].to_string().as_str()) {
            if i + 1 < data.len() && data[i] == '.' && data[i + 1] == '.' {
                arr.push(m.misc[".."].to_string());
                i += 2;
            } else {
                arr.push(v.to_string());
                i += 1;
            }
            continue;
        }

        if let Some(v) = m.numbers.get(data[i].to_string().as_str()) {
            arr.push(v.to_string());
            i += 1;
            continue;
        }

        // at this point, if we find any illegal character then we simply ignore it
        let k = data[i].to_string();
        if !m.vowel_signs.contains_key(&k.as_str())
            && !m.vowels.contains_key(&k.as_str())
            && !m.consonants.contains_key(&k.as_str())
        {
            i += 1;
            continue;
        }

        if i + 1 < data.len() && UNASPIRATED_CONSONANTS.contains(&data[i]) && data[i + 1] == 'h' {
            // a valid aspirated consonant exists here
            arr.push(m.consonants[chars_to_string(&data, i, i + 2).as_str()].to_string());
            i += 2;
        } else {
            // if valid consonant then push it else ignore invalid consonants completely
            if let Some(v) = m.consonants.get(chars_to_string(&data, i, i + 1).as_str()) {
                arr.push(v.to_string());
                i += 1;
            }
        }

        // if end of word or anything other than a vowel-sign then we just push a halanta and start the process again
        if i == data.len()
            || (!m.vowel_signs.contains_key(data[i].to_string().as_str()) && data[i] != 'a')
        {
            arr.push(m.specials.halanta.to_string());
            continue;
        }

        if i + 1 < data.len() && data[i] == 'a' && (data[i + 1] == 'i' || data[i + 1] == 'u') {
            arr.push(m.vowel_signs[chars_to_string(&data, i, i + 2).as_str()].to_string());
            i += 2;
        } else {
            if data[i] != 'a' {
                arr.push(m.vowel_signs[chars_to_string(&data, i, i + 1).as_str()].to_string());
            }
            i += 1;
        }
    }

    arr.join("")
}

pub(crate) fn process_uast(script: &str, line: String) -> String {
    iast_to_devanāgarī(script, handle_unicode(script, line))
}
