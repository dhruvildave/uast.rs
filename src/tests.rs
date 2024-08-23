//! Tests

#[cfg(test)]

fn check<F>(s: &str, f: F) -> String
where
    F: Fn(String) -> String,
{
    s.to_string()
        .split_whitespace()
        .map(|x| f(x.to_string()))
        .collect::<Vec<String>>()
        .join(" ")
}

#[test]
fn test_process_uast() {
    use crate::uast::process_uast;

    let arr = vec![
        ("ma/nu/gala/m/ bhagav/a/nvi/sl//nl/urma/nu/gala/m/ garu/d/adhvaja/h/. ma/nu/gala/m/ pu/nl//d/ar/i/k/a/k/sl/o ma/nu/gal/a/yatana/m/ hari/h/..", "मङ्गलं भगवान्विष्णुर्मङ्गलं गरुडध्वजः। मङ्गलं पुण्डरीकाक्षो मङ्गलायतनं हरिः॥"),
        ("/om/ bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt..", "ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥"),
        ("agnimīḻe purohitaṃ yajñasya devamṛtvijam. hotāraṃ ratnadhātamam.. agniḥ pūrvebhirṛṣibhirīḍyo nūtanairūta. sa devāã eha vakṣati.. agninā rayimaśnavatpoṣameva divedive. yaśasaṃ vīravattamam.. agne yaṃ yajñamadhvaraṃ viśvataḥ paribhūrasi. sa iddeveṣu gacchati.. agnirhotā kavikratuḥ satyaścitraśravastamaḥ. devo devebhirā gamat.. yadaṅga dāśuṣe tvamagne bhadraṃ kariṣyasi. tavettatsatyamaṅgiraḥ.. upa tvāgne divedive doṣāvastardhiyā vayam. namo bharanta emasi.. rājantamadhvarāṇāṃ gopāmṛtasya dīdivim. vardhamānaṃ sve dame.. sa naḥ piteva sūnave'gne sūpāyano bhava. sacasvā naḥ svastaye..", "अग्निमीळे पुरोहितं यज्ञस्य देवमृत्विजम्। होतारं रत्नधातमम्॥ अग्निः पूर्वेभिरृषिभिरीड्यो नूतनैरूत। स देवाँ एह वक्षति॥ अग्निना रयिमश्नवत्पोषमेव दिवेदिवे। यशसं वीरवत्तमम्॥ अग्ने यं यज्ञमध्वरं विश्वतः परिभूरसि। स इद्देवेषु गच्छति॥ अग्निर्होता कविक्रतुः सत्यश्चित्रश्रवस्तमः। देवो देवेभिरा गमत्॥ यदङ्ग दाशुषे त्वमग्ने भद्रं करिष्यसि। तवेत्तत्सत्यमङ्गिरः॥ उप त्वाग्ने दिवेदिवे दोषावस्तर्धिया वयम्। नमो भरन्त एमसि॥ राजन्तमध्वराणां गोपामृतस्य दीदिविम्। वर्धमानं स्वे दमे॥ स नः पितेव सूनवेऽग्ने सूपायनो भव। सचस्वा नः स्वस्तये॥")
    ];

    for (k, v) in arr {
        assert_eq!(check(k, process_uast), v);
    }
}

#[test]
fn test_devanāgarī_to_iast() {
    use crate::iast::devanāgarī_to_iast;

    let arr = vec![
        ("मङ्गलं भगवान्विष्णुर्मङ्गलं गरुडध्वजः। मङ्गलं पुण्डरीकाक्षो मङ्गलायतनं हरिः॥", "maṅgalaṃ bhagavānviṣṇurmaṅgalaṃ garuḍadhvajaḥ. maṅgalaṃ puṇḍarīkākṣo maṅgalāyatanaṃ hariḥ.."),
        ("ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥", "ॐ bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt.."),
        ("अग्निमीळे पुरोहितं यज्ञस्य देवमृत्विजम्। होतारं रत्नधातमम्॥ अग्निः पूर्वेभिरृषिभिरीड्यो नूतनैरूत। स देवाँ एह वक्षति॥ अग्निना रयिमश्नवत्पोषमेव दिवेदिवे। यशसं वीरवत्तमम्॥ अग्ने यं यज्ञमध्वरं विश्वतः परिभूरसि। स इद्देवेषु गच्छति॥ अग्निर्होता कविक्रतुः सत्यश्चित्रश्रवस्तमः। देवो देवेभिरा गमत्॥ यदङ्ग दाशुषे त्वमग्ने भद्रं करिष्यसि। तवेत्तत्सत्यमङ्गिरः॥ उप त्वाग्ने दिवेदिवे दोषावस्तर्धिया वयम्। नमो भरन्त एमसि॥ राजन्तमध्वराणां गोपामृतस्य दीदिविम्। वर्धमानं स्वे दमे॥ स नः पितेव सूनवेऽग्ने सूपायनो भव। सचस्वा नः स्वस्तये॥", "agnimīḻe purohitaṃ yajñasya devamṛtvijam. hotāraṃ ratnadhātamam.. agniḥ pūrvebhirṛṣibhirīḍyo nūtanairūta. sa devāã eha vakṣati.. agninā rayimaśnavatpoṣameva divedive. yaśasaṃ vīravattamam.. agne yaṃ yajñamadhvaraṃ viśvataḥ paribhūrasi. sa iddeveṣu gacchati.. agnirhotā kavikratuḥ satyaścitraśravastamaḥ. devo devebhirā gamat.. yadaṅga dāśuṣe tvamagne bhadraṃ kariṣyasi. tavettatsatyamaṅgiraḥ.. upa tvāgne divedive doṣāvastardhiyā vayam. namo bharanta emasi.. rājantamadhvarāṇāṃ gopāmṛtasya dīdivim. vardhamānaṃ sve dame.. sa naḥ piteva sūnave'gne sūpāyano bhava. sacasvā naḥ svastaye..")
    ];

    for (k, v) in arr {
        assert_eq!(check(k, devanāgarī_to_iast), v);
    }
}
