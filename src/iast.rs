//! Devanāgarī to IAST

use std::collections::HashMap;
use std::sync::LazyLock;

type CharMap<'a> = HashMap<char, &'a str>;

struct ScriptSpecials {
    om: char,
    halanta: char,
    visarga: char,
    anusvāra: char,
    candrabindu: char,
}

struct Script<'a> {
    vowels: CharMap<'a>,
    vowel_signs: CharMap<'a>,
    consonants: CharMap<'a>,
    misc: CharMap<'a>,
    specials: ScriptSpecials,
}

static CHAR_DICT: LazyLock<Script> = LazyLock::new(|| Script {
    vowels: CharMap::from([
        ('अ', "a"),
        ('आ', "ā"),
        ('इ', "i"),
        ('ई', "ī"),
        ('उ', "u"),
        ('ऊ', "ū"),
        ('ऋ', "ṛ"),
        ('ॠ', "ṝ"),
        ('ऌ', "ḷ"),
        ('ॡ', "ḹ"),
        ('ए', "e"),
        ('ऐ', "ai"),
        ('ओ', "o"),
        ('औ', "au"),
    ]),
    vowel_signs: CharMap::from([
        ('ा', "ā"),
        ('ि', "i"),
        ('ी', "ī"),
        ('ु', "u"),
        ('ू', "ū"),
        ('ृ', "ṛ"),
        ('ॄ', "ṝ"),
        ('ॢ', "ḷ"),
        ('ॣ', "ḹ"),
        ('े', "e"),
        ('ै', "ai"),
        ('ो', "o"),
        ('ौ', "au"),
    ]),
    consonants: CharMap::from([
        ('क', "k"),
        ('ख', "kh"),
        ('ग', "g"),
        ('घ', "gh"),
        ('ङ', "ṅ"),
        ('च', "c"),
        ('छ', "ch"),
        ('ज', "j"),
        ('झ', "jh"),
        ('ञ', "ñ"),
        ('ट', "ṭ"),
        ('ठ', "ṭh"),
        ('ड', "ḍ"),
        ('ढ', "ḍh"),
        ('ण', "ṇ"),
        ('त', "t"),
        ('थ', "th"),
        ('द', "d"),
        ('ध', "dh"),
        ('न', "n"),
        ('प', "p"),
        ('फ', "ph"),
        ('ब', "b"),
        ('भ', "bh"),
        ('म', "m"),
        ('य', "y"),
        ('र', "r"),
        ('ल', "l"),
        ('व', "v"),
        ('श', "ś"),
        ('ष', "ṣ"),
        ('स', "s"),
        ('ह', "h"),
        ('ळ', "ḻ"),
    ]),
    misc: CharMap::from([
        ('ऽ', "'"),
        ('।', "."),
        ('॥', ".."),
        ('०', "0"),
        ('१', "1"),
        ('२', "2"),
        ('३', "3"),
        ('४', "4"),
        ('५', "5"),
        ('६', "6"),
        ('७', "7"),
        ('८', "8"),
        ('९', "9"),
    ]),
    specials: ScriptSpecials {
        om: 'ॐ',
        anusvāra: 'ं',
        visarga: 'ः',
        candrabindu: 'ँ',
        halanta: '्',
    },
});

pub(crate) fn devanāgarī_to_iast(dn: String) -> String {
    let str = dn.to_lowercase().chars().collect::<Vec<char>>();

    let mut arr = Vec::<String>::with_capacity(str.len());

    let mut i = 0;

    // if starts with vowel
    if let Some(v) = CHAR_DICT.vowels.get(&str[i]) {
        arr.push((*v).to_string());
        i += 1;
    }

    while i < str.len() {
        if str[i] == CHAR_DICT.specials.om {
            arr.push(str[i].to_string());
            i += 1;
            continue;
        }

        if let Some(v) = CHAR_DICT.misc.get(&str[i]) {
            arr.push((*v).to_string());
            i += 1;
            continue;
        }

        if str[i] == CHAR_DICT.specials.anusvāra {
            arr.push("ṃ".to_string());
            i += 1;
            continue;
        }

        if str[i] == CHAR_DICT.specials.visarga {
            arr.push("ḥ".to_string());
            i += 1;
            continue;
        }

        if str[i] == CHAR_DICT.specials.candrabindu {
            arr.push("ã".to_string());
            i += 1;
            continue;
        }

        if let Some(c) = CHAR_DICT.consonants.get(&str[i]) {
            arr.push((*c).to_string());

            if i + 1 == str.len() {
                arr.push("a".to_string());
                i += 1;
                continue;
            }

            let v = str[i + 1];
            if v == CHAR_DICT.specials.halanta {
                i += 2;
                continue;
            }

            if let Some(s) = CHAR_DICT.vowel_signs.get(&v) {
                arr.push((*s).to_string());
                i += 2;
                continue;
            }

            if CHAR_DICT.consonants.contains_key(&v)
                || CHAR_DICT.misc.contains_key(&v)
                || v == CHAR_DICT.specials.anusvāra
                || v == CHAR_DICT.specials.visarga
                || v == CHAR_DICT.specials.candrabindu
            {
                arr.push("a".to_string());
                i += 1;
                continue;
            }
        }

        i += 1;
    }

    arr.join("")
}
