//! देवनागरी to ગુજરાતી

use crate::utils::split_line_and_convert;

static CHAR_DICT: [(char, char); 79] = [
    ('।', '।'),
    ('॥', '॥'),
    ('ॐ', 'ૐ'),
    ('ऽ', 'ઽ'),
    ('०', '૦'),
    ('१', '૧'),
    ('२', '૨'),
    ('३', '૩'),
    ('४', '૪'),
    ('५', '૫'),
    ('६', '૬'),
    ('७', '૭'),
    ('८', '૮'),
    ('९', '૯'),
    ('अ', 'અ'),
    ('आ', 'આ'),
    ('इ', 'ઇ'),
    ('ई', 'ઈ'),
    ('उ', 'ઉ'),
    ('ऊ', 'ઊ'),
    ('ऋ', 'ઋ'),
    ('ॠ', 'ૠ'),
    ('ऌ', 'ઌ'),
    ('ॡ', 'ૡ'),
    ('ए', 'એ'),
    ('ऐ', 'ઐ'),
    ('ओ', 'ઓ'),
    ('औ', 'ઔ'),
    ('ा', 'ા'),
    ('ि', 'િ'),
    ('ी', 'ી'),
    ('ु', 'ુ'),
    ('ू', 'ૂ'),
    ('ृ', 'ૃ'),
    ('ॄ', 'ૄ'),
    ('ॢ', 'ૢ'),
    ('ॣ', 'ૣ'),
    ('े', 'ે'),
    ('ै', 'ૈ'),
    ('ो', 'ો'),
    ('ौ', 'ૌ'),
    ('ं', 'ં'),
    ('ः', 'ઃ'),
    ('ँ', 'ઁ'),
    ('्', '્'),
    ('क', 'ક'),
    ('ख', 'ખ'),
    ('ग', 'ગ'),
    ('घ', 'ઘ'),
    ('ङ', 'ઙ'),
    ('च', 'ચ'),
    ('छ', 'છ'),
    ('ज', 'જ'),
    ('झ', 'ઝ'),
    ('ञ', 'ઞ'),
    ('ट', 'ટ'),
    ('ठ', 'ઠ'),
    ('ड', 'ડ'),
    ('ढ', 'ઢ'),
    ('ण', 'ણ'),
    ('त', 'ત'),
    ('थ', 'થ'),
    ('द', 'દ'),
    ('ध', 'ધ'),
    ('न', 'ન'),
    ('प', 'પ'),
    ('फ', 'ફ'),
    ('ब', 'બ'),
    ('भ', 'ભ'),
    ('म', 'મ'),
    ('य', 'ય'),
    ('र', 'ર'),
    ('ल', 'લ'),
    ('व', 'વ'),
    ('श', 'શ'),
    ('ष', 'ષ'),
    ('स', 'સ'),
    ('ह', 'હ'),
    ('ळ', 'ળ'),
];

fn get_char(c: char) -> char {
    if let Some(v) = CHAR_DICT.iter().find(|x| x.0 == c) {
        v.1
    } else {
        '\0'
    }
}

fn convertor(dn: &str) -> String {
    dn.chars().map(get_char).filter(|c| *c != '\0').collect()
}

/// This function converts देवनागरी to ગુજરાતી.
///
/// ```
/// use uast::devanāgarī_to_gujarātī;
///
/// let s = "ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥".to_string();
/// assert_eq!(
///     "ૐ ભૂર્ભુવઃ સ્વઃ તત્સવિતુર્વરેણ્યં ભર્ગો દેવસ્ય ધીમહિ। ધિયો યો નઃ પ્રચોદયાત્॥".to_string(),
///     devanāgarī_to_gujarātī(&s)
/// );
/// ```
pub fn devanāgarī_to_gujarātī(dn: &str) -> String {
    split_line_and_convert(convertor, dn)
}
