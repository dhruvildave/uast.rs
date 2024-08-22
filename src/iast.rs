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

            if i + 1 < str.len() {
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
            } else {
                arr.push("a".to_string());
            }
        }

        i += 1;
    }

    arr.join("")
}

#[cfg(test)]
mod tests {
    use crate::iast::devanāgarī_to_iast;

    fn check(s: &str) -> String {
        s.to_string()
            .split_whitespace()
            .map(|x| devanāgarī_to_iast(x.to_string()))
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[test]
    fn test_devanāgarī_to_iast() {
        assert_eq!(
            check("मङ्गलं भगवान्विष्णुर्मङ्गलं गरुडध्वजः। मङ्गलं पुण्डरीकाक्षो मङ्गलायतनं हरिः॥"),
            "maṅgalaṃ bhagavānviṣṇurmaṅgalaṃ garuḍadhvajaḥ. maṅgalaṃ puṇḍarīkākṣo maṅgalāyatanaṃ hariḥ.."
        );

        assert_eq!(
            check("ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥"),
            "ॐ bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt.."
        );

        assert_eq!(
            check("अग्निमीळे पुरोहितं यज्ञस्य देवमृत्विजम्। होतारं रत्नधातमम्॥ अग्निः पूर्वेभिरृषिभिरीड्यो नूतनैरूत। स देवाँ एह वक्षति॥ अग्निना रयिमश्नवत्पोषमेव दिवेदिवे। यशसं वीरवत्तमम्॥ अग्ने यं यज्ञमध्वरं विश्वतः परिभूरसि। स इद्देवेषु गच्छति॥ अग्निर्होता कविक्रतुः सत्यश्चित्रश्रवस्तमः। देवो देवेभिरा गमत्॥ यदङ्ग दाशुषे त्वमग्ने भद्रं करिष्यसि। तवेत्तत्सत्यमङ्गिरः॥ उप त्वाग्ने दिवेदिवे दोषावस्तर्धिया वयम्। नमो भरन्त एमसि॥ राजन्तमध्वराणां गोपामृतस्य दीदिविम्। वर्धमानं स्वे दमे॥ स नः पितेव सूनवेऽग्ने सूपायनो भव। सचस्वा नः स्वस्तये॥"),
            "agnimīḻe purohitaṃ yajñasya devamṛtvijam. hotāraṃ ratnadhātamam.. agniḥ pūrvebhirṛṣibhirīḍyo nūtanairūta. sa devāã eha vakṣati.. agninā rayimaśnavatpoṣameva divedive. yaśasaṃ vīravattamam.. agne yaṃ yajñamadhvaraṃ viśvataḥ paribhūrasi. sa iddeveṣu gacchati.. agnirhotā kavikratuḥ satyaścitraśravastamaḥ. devo devebhirā gamat.. yadaṅga dāśuṣe tvamagne bhadraṃ kariṣyasi. tavettatsatyamaṅgiraḥ.. upa tvāgne divedive doṣāvastardhiyā vayam. namo bharanta emasi.. rājantamadhvarāṇāṃ gopāmṛtasya dīdivim. vardhamānaṃ sve dame.. sa naḥ piteva sūnave'gne sūpāyano bhava. sacasvā naḥ svastaye.."
        );
    }
}
