//! देवनागरी to IAST

use crate::utils::{binary_search, split_line_and_convert};

type T = (char, &'static str);

struct ScriptSpecials {
    om: char,
    halanta: char,
    visarga: char,
    anusvāra: char,
    candrabindu: char,
    saṃkṣipta: char,
}

struct Script<'a> {
    vowels: &'a [T; 14],
    vowel_signs: &'a [T; 13],
    consonants: &'a [T; 34],
    misc: &'a [T; 14],
    specials: &'a ScriptSpecials,
}

static CHAR_DICT: &Script = &Script {
    vowels: &[
        ('अ', "a"),
        ('आ', "ā"),
        ('इ', "i"),
        ('ई', "ī"),
        ('उ', "u"),
        ('ऊ', "ū"),
        ('ऋ', "ṛ"),
        ('ऌ', "ḷ"),
        ('ए', "e"),
        ('ऐ', "ai"),
        ('ओ', "o"),
        ('औ', "au"),
        ('ॠ', "ṝ"),
        ('ॡ', "ḹ"),
    ],
    vowel_signs: &[
        ('ा', "ā"),
        ('ि', "i"),
        ('ी', "ī"),
        ('ु', "u"),
        ('ू', "ū"),
        ('ृ', "ṛ"),
        ('ॄ', "ṝ"),
        ('े', "e"),
        ('ै', "ai"),
        ('ो', "o"),
        ('ौ', "au"),
        ('ॢ', "ḷ"),
        ('ॣ', "ḹ"),
    ],
    consonants: &[
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
        ('ळ', "ḻ"),
        ('व', "v"),
        ('श', "ś"),
        ('ष', "ṣ"),
        ('स', "s"),
        ('ह', "h"),
    ],
    misc: &[
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
        ('ॱ', "-"),
    ],
    specials: &ScriptSpecials {
        om: 'ॐ',
        anusvāra: 'ं',
        visarga: 'ः',
        candrabindu: 'ँ',
        halanta: '्',
        saṃkṣipta: '॰',
    },
};

impl Script<'_> {
    fn get_vowel(&self, c: char) -> Option<&'static str> {
        binary_search(self.vowels, c, |i| i)
    }

    fn get_misc(&self, c: char) -> Option<&'static str> {
        binary_search(self.misc, c, |i| i)
    }

    fn get_vowelsign(&self, c: char) -> Option<&'static str> {
        binary_search(self.vowel_signs, c, |i| i)
    }

    fn get_consonant(&self, c: char) -> Option<&'static str> {
        binary_search(self.consonants, c, |i| i)
    }

    fn contains_consonant(&self, c: char) -> bool {
        binary_search(self.consonants, c, |i| i).is_some()
    }

    fn contains_misc(&self, c: char) -> bool {
        binary_search(self.misc, c, |i| i).is_some()
    }
}

fn convertor(dn: &str) -> String {
    let str = dn.to_lowercase().chars().collect::<Vec<char>>();

    // Build the result in one buffer instead of one String per output glyph.
    let mut out = String::with_capacity(dn.len());

    let mut i = 0;

    // if starts with vowel
    if let Some(v) = CHAR_DICT.get_vowel(str[i]) {
        out.push_str(v);
        i += 1;
    }

    while i < str.len() {
        if str[i] == CHAR_DICT.specials.om || str[i] == CHAR_DICT.specials.saṃkṣipta {
            out.push(str[i]);
            i += 1;
            continue;
        }

        if let Some(v) = CHAR_DICT.get_misc(str[i]) {
            out.push_str(v);
            i += 1;
            continue;
        }

        if str[i] == CHAR_DICT.specials.anusvāra {
            out.push_str("ṃ");
            i += 1;
            continue;
        }

        if str[i] == CHAR_DICT.specials.visarga {
            out.push_str("ḥ");
            i += 1;
            continue;
        }

        if str[i] == CHAR_DICT.specials.candrabindu {
            out.push_str("ã");
            i += 1;
            continue;
        }

        if let Some(c) = CHAR_DICT.get_consonant(str[i]) {
            out.push_str(c);

            if i + 1 == str.len() {
                out.push('a');
                i += 1;
                continue;
            }

            let v = str[i + 1];
            if v == CHAR_DICT.specials.halanta {
                i += 2;
                continue;
            }

            if let Some(s) = CHAR_DICT.get_vowelsign(v) {
                out.push_str(s);
                i += 2;
                continue;
            }

            if CHAR_DICT.contains_consonant(v)
                || CHAR_DICT.contains_misc(v)
                || v == CHAR_DICT.specials.anusvāra
                || v == CHAR_DICT.specials.visarga
                || v == CHAR_DICT.specials.candrabindu
            {
                out.push('a');
                i += 1;
                continue;
            }
        }

        i += 1;
    }

    out
}

/// This function converts देवनागरी to IAST.
///
/// ```
/// use uast::devanāgarī_to_iast;
///
/// let s = "ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥";
/// assert_eq!(
///     "ॐ bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt..",
///     devanāgarī_to_iast(&s)
/// );
/// ```
pub fn devanāgarī_to_iast(dn: &str) -> String {
    split_line_and_convert(convertor, dn)
}

#[cfg(test)]
mod invariants {
    use super::*;
    use crate::utils::keys_strictly_increasing;

    #[test]
    fn tables_are_strictly_sorted() {
        // `utils::binary_search` needs every table sorted by its char key.
        assert!(keys_strictly_increasing(CHAR_DICT.vowels));
        assert!(keys_strictly_increasing(CHAR_DICT.vowel_signs));
        assert!(keys_strictly_increasing(CHAR_DICT.consonants));
        assert!(keys_strictly_increasing(CHAR_DICT.misc));
    }
}
