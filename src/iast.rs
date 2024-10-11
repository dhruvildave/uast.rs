//! देवनागरी to IAST

use crate::utils::split_line_and_convert;

type T = (char, &'static str);

struct ScriptSpecials {
    om: char,
    halanta: char,
    visarga: char,
    anusvāra: char,
    candrabindu: char,
}

struct Script {
    vowels: [T; 14],
    vowel_signs: [T; 13],
    consonants: [T; 34],
    misc: [T; 13],
    specials: ScriptSpecials,
}

static CHAR_DICT: Script = Script {
    vowels: [
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
    ],
    vowel_signs: [
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
    ],
    consonants: [
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
    ],
    misc: [
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
    ],
    specials: ScriptSpecials {
        om: 'ॐ',
        anusvāra: 'ं',
        visarga: 'ः',
        candrabindu: 'ँ',
        halanta: '्',
    },
};

impl Script {
    fn get_vowel(&self, c: char) -> Option<String> {
        self.vowels
            .iter()
            .find(|x| x.0 == c)
            .map(|i| i.1.to_string())
    }

    fn get_misc(&self, c: char) -> Option<String> {
        self.misc.iter().find(|x| x.0 == c).map(|i| i.1.to_string())
    }

    fn get_vowelsign(&self, c: char) -> Option<String> {
        self.vowel_signs
            .iter()
            .find(|x| x.0 == c)
            .map(|i| i.1.to_string())
    }

    fn get_consonant(&self, c: char) -> Option<String> {
        self.consonants
            .iter()
            .find(|x| x.0 == c)
            .map(|i| i.1.to_string())
    }
}

fn convertor(dn: &String) -> String {
    let str = dn.trim().to_lowercase().chars().collect::<Vec<char>>();

    let mut arr = Vec::<String>::with_capacity(str.len());

    let mut i = 0;

    // if starts with vowel
    if let Some(v) = CHAR_DICT.get_vowel(str[i]) {
        arr.push(v);
        i += 1;
    }

    while i < str.len() {
        if str[i] == CHAR_DICT.specials.om {
            arr.push(str[i].to_string());
            i += 1;
            continue;
        }

        if let Some(v) = CHAR_DICT.get_misc(str[i]) {
            arr.push(v);
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

        if let Some(c) = CHAR_DICT.get_consonant(str[i]) {
            arr.push(c);

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

            if let Some(s) = CHAR_DICT.get_vowelsign(v) {
                arr.push(s);
                i += 2;
                continue;
            }

            if CHAR_DICT.consonants.iter().any(|c| c.0 == v)
                || CHAR_DICT.misc.iter().any(|c| c.0 == v)
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

/// This function converts देवनागरी to IAST.
///
/// ```
/// use uast::devanāgarī_to_iast;
///
/// let s = "ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥".to_string();
/// assert_eq!(
///     "ॐ bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt.."
///         .to_string(),
///     devanāgarī_to_iast(&s)
/// );
/// ```
pub fn devanāgarī_to_iast(dn: &String) -> String {
    split_line_and_convert(convertor, dn)
}
