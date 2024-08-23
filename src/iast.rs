//! Devanāgarī to IAST

use std::collections::HashMap;
use std::sync::LazyLock;

static VOWELS: LazyLock<HashMap<char, &str>> = LazyLock::new(|| {
    HashMap::from([
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
    ])
});

static VOWEL_SIGNS: LazyLock<HashMap<char, &str>> = LazyLock::new(|| {
    HashMap::from([
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
    ])
});

static CONSONANTS: LazyLock<HashMap<char, &str>> = LazyLock::new(|| {
    HashMap::from([
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
    ])
});

static MISC: LazyLock<HashMap<char, &str>> = LazyLock::new(|| {
    HashMap::from([
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
    ])
});

pub fn devanāgarī_to_iast(dn: String) -> String {
    let str = dn.to_lowercase().chars().collect::<Vec<char>>();

    let mut arr = Vec::<String>::with_capacity(str.len());

    let mut i = 0;

    // if starts with vowel
    if let Some(v) = VOWELS.get(&str[i]) {
        arr.push((*v).to_string());
        i += 1;
    }

    while i < str.len() {
        if str[i] == 'ॐ' {
            arr.push(str[i].to_string());
            i += 1;
            continue;
        }

        if let Some(v) = MISC.get(&str[i]) {
            arr.push((*v).to_string());
            i += 1;
            continue;
        }

        if str[i] == 'ं' {
            arr.push("ṃ".to_string());
            i += 1;
            continue;
        }

        if str[i] == 'ः' {
            arr.push("ḥ".to_string());
            i += 1;
            continue;
        }

        if str[i] == 'ँ' {
            arr.push("ã".to_string());
            i += 1;
            continue;
        }

        if let Some(c) = CONSONANTS.get(&str[i]) {
            arr.push((*c).to_string());

            if i + 1 == str.len() {
                arr.push("a".to_string());
                i += 1;
                continue;
            }

            let v = str[i + 1];
            if v == '्' {
                i += 2;
                continue;
            }

            if let Some(s) = VOWEL_SIGNS.get(&v) {
                arr.push((*s).to_string());
                i += 2;
                continue;
            }

            if CONSONANTS.contains_key(&v)
                || MISC.contains_key(&v)
                || v == 'ं'
                || v == 'ः'
                || v == 'ँ'
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
