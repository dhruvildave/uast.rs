//! Rust implementation of [Unicode Aware Saṃskṛta Transliteration](https://arxiv.org/html/2203.14277)

use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

type CharMap<'a> = HashMap<&'a str, char>;

struct LangMap<'a> {
    misc: CharMap<'a>,
    numbers: CharMap<'a>,
    vowels: CharMap<'a>,
    vowel_signs: CharMap<'a>,
    consonants: CharMap<'a>,
}

static UNICODE_MAP: LazyLock<CharMap> = LazyLock::new(|| {
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
    ])
});

static CHAR_DICT: LazyLock<LangMap> = LazyLock::new(|| LangMap {
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
});

static UNASPIRATED_CONSONANTS: LazyLock<HashSet<char>> =
    LazyLock::new(|| HashSet::from(['b', 'c', 'd', 'g', 'j', 'k', 'p', 't', 'ḍ', 'ṭ']));

fn chars_to_string(data: &[char], start: usize, end: usize) -> String {
    data[start..end].iter().collect::<String>()
}

fn handle_unicode(uast: String) -> Vec<char> {
    let str = uast.to_lowercase().chars().collect::<Vec<char>>();

    let mut arr = Vec::<char>::new();

    let mut i = 0;
    while i < str.len() {
        let curr = str[i];

        if curr == '/' {
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

            if let Some(v) = UNICODE_MAP.get(c.as_str()) {
                arr.push(*v);
            }

            i += 1;
            continue;
        }

        arr.push(curr);
        i += 1;
    }

    arr
}

fn iast_to_devanāgarī(data: Vec<char>) -> String {
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
    if CHAR_DICT.vowels.contains_key(data[0].to_string().as_str()) {
        if i + 1 < data.len() && data[0] == 'a' && (data[1] == 'i' || data[1] == 'u') {
            i = 2;
        } else {
            i = 1;
        }

        // a valid vowel exists here
        arr.push(CHAR_DICT.vowels[chars_to_string(&data, 0, i).as_str()].to_string());
    }

    while i < data.len() {
        if data[i] == 'ॐ' {
            arr.push("ॐ".to_string());
            i += 1;
            continue;
        }

        if let Some(v) = CHAR_DICT.misc.get(data[i].to_string().as_str()) {
            if i + 1 < data.len() && data[i] == '.' && data[i + 1] == '.' {
                arr.push(CHAR_DICT.misc[".."].to_string());
                i += 2;
            } else {
                arr.push(v.to_string());
                i += 1;
            }
            continue;
        }

        if let Some(v) = CHAR_DICT.numbers.get(data[i].to_string().as_str()) {
            arr.push(v.to_string());
            i += 1;
            continue;
        }

        // at this point, if we find any illegal character then we simply ignore it
        let k = data[i].to_string();
        if !CHAR_DICT.vowel_signs.contains_key(&k.as_str())
            && !CHAR_DICT.vowels.contains_key(&k.as_str())
            && !CHAR_DICT.consonants.contains_key(&k.as_str())
        {
            i += 1;
            continue;
        }

        if i + 1 < data.len() && UNASPIRATED_CONSONANTS.contains(&data[i]) && data[i + 1] == 'h' {
            // a valid aspirated consonant exists here
            arr.push(CHAR_DICT.consonants[chars_to_string(&data, i, i + 2).as_str()].to_string());
            i += 2;
        } else {
            // if valid consonant then push it else ignore invalid consonants completely
            if let Some(v) = CHAR_DICT
                .consonants
                .get(chars_to_string(&data, i, i + 1).as_str())
            {
                arr.push(v.to_string());
                i += 1;
            }
        }

        // if end of word or anything other than a vowel-sign then we just push a halanta and start the process again
        if i == data.len()
            || (!CHAR_DICT
                .vowel_signs
                .contains_key(data[i].to_string().as_str())
                && data[i] != 'a')
        {
            arr.push("्".to_string());
            continue;
        }

        if i + 1 < data.len() && data[i] == 'a' && (data[i + 1] == 'i' || data[i + 1] == 'u') {
            arr.push(CHAR_DICT.vowel_signs[chars_to_string(&data, i, i + 2).as_str()].to_string());
            i += 2;
        } else {
            if data[i] != 'a' {
                arr.push(
                    CHAR_DICT.vowel_signs[chars_to_string(&data, i, i + 1).as_str()].to_string(),
                );
            }
            i += 1;
        }
    }

    arr.join("")
}

pub fn process_uast(line: String) -> String {
    iast_to_devanāgarī(handle_unicode(line))
}

#[cfg(test)]
mod tests {
    use crate::uast::process_uast;

    fn check(s: &str) -> String {
        s.to_string()
            .split_whitespace()
            .map(|x| process_uast(x.to_string()))
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[test]
    fn test_process_uast() {
        assert_eq!(
            check("ma/nu/gala/m/ bhagav/a/nvi/sl//nl/urma/nu/gala/m/ garu/d/adhvaja/h/. ma/nu/gala/m/ pu/nl//d/ar/i/k/a/k/sl/o ma/nu/gal/a/yatana/m/ hari/h/.."),
            "मङ्गलं भगवान्विष्णुर्मङ्गलं गरुडध्वजः। मङ्गलं पुण्डरीकाक्षो मङ्गलायतनं हरिः॥"
        );

        assert_eq!(
            check("/om/ bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt.."),
            "ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥"
        );

        assert_eq!(
            check("agnimīḻe purohitaṃ yajñasya devamṛtvijam. hotāraṃ ratnadhātamam.. agniḥ pūrvebhirṛṣibhirīḍyo nūtanairūta. sa devāã eha vakṣati.. agninā rayimaśnavatpoṣameva divedive. yaśasaṃ vīravattamam.. agne yaṃ yajñamadhvaraṃ viśvataḥ paribhūrasi. sa iddeveṣu gacchati.. agnirhotā kavikratuḥ satyaścitraśravastamaḥ. devo devebhirā gamat.. yadaṅga dāśuṣe tvamagne bhadraṃ kariṣyasi. tavettatsatyamaṅgiraḥ.. upa tvāgne divedive doṣāvastardhiyā vayam. namo bharanta emasi.. rājantamadhvarāṇāṃ gopāmṛtasya dīdivim. vardhamānaṃ sve dame.. sa naḥ piteva sūnave'gne sūpāyano bhava. sacasvā naḥ svastaye.."),
            "अग्निमीळे पुरोहितं यज्ञस्य देवमृत्विजम्। होतारं रत्नधातमम्॥ अग्निः पूर्वेभिरृषिभिरीड्यो नूतनैरूत। स देवाँ एह वक्षति॥ अग्निना रयिमश्नवत्पोषमेव दिवेदिवे। यशसं वीरवत्तमम्॥ अग्ने यं यज्ञमध्वरं विश्वतः परिभूरसि। स इद्देवेषु गच्छति॥ अग्निर्होता कविक्रतुः सत्यश्चित्रश्रवस्तमः। देवो देवेभिरा गमत्॥ यदङ्ग दाशुषे त्वमग्ने भद्रं करिष्यसि। तवेत्तत्सत्यमङ्गिरः॥ उप त्वाग्ने दिवेदिवे दोषावस्तर्धिया वयम्। नमो भरन्त एमसि॥ राजन्तमध्वराणां गोपामृतस्य दीदिविम्। वर्धमानं स्वे दमे॥ स नः पितेव सूनवेऽग्ने सूपायनो भव। सचस्वा नः स्वस्तये॥"
        );
    }
}
