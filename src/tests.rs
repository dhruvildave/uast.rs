//! Tests

#[cfg(test)]

fn check<F>(s: &str, f: F) -> String
where
    F: Fn(&String) -> String,
{
    f(&s.to_string())
}

#[test]
fn test_process_uast() {
    use crate::process_uast;

    let arr = vec![
        ("ma/nu/gala/m/ bhagav/a/nvi/sl//nl/urma/nu/gala/m/ garu/d/adhvaja/h/. ma/nu/gala/m/ pu/nl//d/ar/i/k/a/k/sl/o ma/nu/gal/a/yatana/m/ hari/h/..0..", "मङ्गलं भगवान्विष्णुर्मङ्गलं गरुडध्वजः। मङ्गलं पुण्डरीकाक्षो मङ्गलायतनं हरिः॥०॥"),
        ("/om/ bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt..", "ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥"),
        ("agnimīḻe purohitaṃ yajñasya devamṛtvijam. hotāraṃ ratnadhātamam.. agniḥ pūrvebhirṛṣibhirīḍyo nūtanairūta. sa devāã eha vakṣati.. agninā rayimaśnavatpoṣameva divedive. yaśasaṃ vīravattamam.. agne yaṃ yajñamadhvaraṃ viśvataḥ paribhūrasi. sa iddeveṣu gacchati.. agnirhotā kavikratuḥ satyaścitraśravastamaḥ. devo devebhirā gamat.. yadaṅga dāśuṣe tvamagne bhadraṃ kariṣyasi. tavettatsatyamaṅgiraḥ.. upa tvāgne divedive doṣāvastardhiyā vayam. namo bharanta emasi.. rājantamadhvarāṇāṃ gopāmṛtasya dīdivim. vardhamānaṃ sve dame.. sa naḥ piteva sūnave'gne sūpāyano bhava. sacasvā naḥ svastaye..", "अग्निमीळे पुरोहितं यज्ञस्य देवमृत्विजम्। होतारं रत्नधातमम्॥ अग्निः पूर्वेभिरृषिभिरीड्यो नूतनैरूत। स देवाँ एह वक्षति॥ अग्निना रयिमश्नवत्पोषमेव दिवेदिवे। यशसं वीरवत्तमम्॥ अग्ने यं यज्ञमध्वरं विश्वतः परिभूरसि। स इद्देवेषु गच्छति॥ अग्निर्होता कविक्रतुः सत्यश्चित्रश्रवस्तमः। देवो देवेभिरा गमत्॥ यदङ्ग दाशुषे त्वमग्ने भद्रं करिष्यसि। तवेत्तत्सत्यमङ्गिरः॥ उप त्वाग्ने दिवेदिवे दोषावस्तर्धिया वयम्। नमो भरन्त एमसि॥ राजन्तमध्वराणां गोपामृतस्य दीदिविम्। वर्धमानं स्वे दमे॥ स नः पितेव सूनवेऽग्ने सूपायनो भव। सचस्वा नः स्वस्तये॥"),
        ("ai", "ऐ"),
        ("/x/", ""),
        ("x", ""),
        ("k/a", "का")
    ];

    for (k, v) in arr {
        assert_eq!(check(k, process_uast), v);
    }
}

#[test]
fn test_devanāgarī_to_iast() {
    use crate::devanāgarī_to_iast;

    let arr = vec![
        ("मङ्गलं भगवान्विष्णुर्मङ्गलं गरुडध्वजः। मङ्गलं पुण्डरीकाक्षो मङ्गलायतनं हरिः॥", "maṅgalaṃ bhagavānviṣṇurmaṅgalaṃ garuḍadhvajaḥ. maṅgalaṃ puṇḍarīkākṣo maṅgalāyatanaṃ hariḥ.."),
        ("ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥", "ॐ bhūrbhuvaḥ svaḥ tatsaviturvareṇyaṃ bhargo devasya dhīmahi. dhiyo yo naḥ pracodayāt.."),
        ("अग्निमीळे पुरोहितं यज्ञस्य देवमृत्विजम्। होतारं रत्नधातमम्॥ अग्निः पूर्वेभिरृषिभिरीड्यो नूतनैरूत। स देवाँ एह वक्षति॥ अग्निना रयिमश्नवत्पोषमेव दिवेदिवे। यशसं वीरवत्तमम्॥ अग्ने यं यज्ञमध्वरं विश्वतः परिभूरसि। स इद्देवेषु गच्छति॥ अग्निर्होता कविक्रतुः सत्यश्चित्रश्रवस्तमः। देवो देवेभिरा गमत्॥ यदङ्ग दाशुषे त्वमग्ने भद्रं करिष्यसि। तवेत्तत्सत्यमङ्गिरः॥ उप त्वाग्ने दिवेदिवे दोषावस्तर्धिया वयम्। नमो भरन्त एमसि॥ राजन्तमध्वराणां गोपामृतस्य दीदिविम्। वर्धमानं स्वे दमे॥ स नः पितेव सूनवेऽग्ने सूपायनो भव। सचस्वा नः स्वस्तये॥", "agnimīḻe purohitaṃ yajñasya devamṛtvijam. hotāraṃ ratnadhātamam.. agniḥ pūrvebhirṛṣibhirīḍyo nūtanairūta. sa devāã eha vakṣati.. agninā rayimaśnavatpoṣameva divedive. yaśasaṃ vīravattamam.. agne yaṃ yajñamadhvaraṃ viśvataḥ paribhūrasi. sa iddeveṣu gacchati.. agnirhotā kavikratuḥ satyaścitraśravastamaḥ. devo devebhirā gamat.. yadaṅga dāśuṣe tvamagne bhadraṃ kariṣyasi. tavettatsatyamaṅgiraḥ.. upa tvāgne divedive doṣāvastardhiyā vayam. namo bharanta emasi.. rājantamadhvarāṇāṃ gopāmṛtasya dīdivim. vardhamānaṃ sve dame.. sa naḥ piteva sūnave'gne sūpāyano bhava. sacasvā naḥ svastaye.."),
        ("ક્", ""),
        ("कँ", "kaã")
    ];

    for (k, v) in arr {
        assert_eq!(check(k, devanāgarī_to_iast), v);
    }
}

#[test]
fn test_devanāgarī_to_gujarātī() {
    use crate::devanāgarī_to_gujarātī;

    let arr = vec![
        ("मङ्गलं भगवान्विष्णुर्मङ्गलं गरुडध्वजः। मङ्गलं पुण्डरीकाक्षो मङ्गलायतनं हरिः॥", "મઙ્ગલં ભગવાન્વિષ્ણુર્મઙ્ગલં ગરુડધ્વજઃ। મઙ્ગલં પુણ્ડરીકાક્ષો મઙ્ગલાયતનં હરિઃ॥"),
        ("ॐ भूर्भुवः स्वः तत्सवितुर्वरेण्यं भर्गो देवस्य धीमहि। धियो यो नः प्रचोदयात्॥", "ૐ ભૂર્ભુવઃ સ્વઃ તત્સવિતુર્વરેણ્યં ભર્ગો દેવસ્ય ધીમહિ। ધિયો યો નઃ પ્રચોદયાત્॥"),
        ("अग्निमीळे पुरोहितं यज्ञस्य देवमृत्विजम्। होतारं रत्नधातमम्॥ अग्निः पूर्वेभिरृषिभिरीड्यो नूतनैरूत। स देवाँ एह वक्षति॥ अग्निना रयिमश्नवत्पोषमेव दिवेदिवे। यशसं वीरवत्तमम्॥ अग्ने यं यज्ञमध्वरं विश्वतः परिभूरसि। स इद्देवेषु गच्छति॥ अग्निर्होता कविक्रतुः सत्यश्चित्रश्रवस्तमः। देवो देवेभिरा गमत्॥ यदङ्ग दाशुषे त्वमग्ने भद्रं करिष्यसि। तवेत्तत्सत्यमङ्गिरः॥ उप त्वाग्ने दिवेदिवे दोषावस्तर्धिया वयम्। नमो भरन्त एमसि॥ राजन्तमध्वराणां गोपामृतस्य दीदिविम्। वर्धमानं स्वे दमे॥ स नः पितेव सूनवेऽग्ने सूपायनो भव। सचस्वा नः स्वस्तये॥", "અગ્નિમીળે પુરોહિતં યજ્ઞસ્ય દેવમૃત્વિજમ્। હોતારં રત્નધાતમમ્॥ અગ્નિઃ પૂર્વેભિરૃષિભિરીડ્યો નૂતનૈરૂત। સ દેવાઁ એહ વક્ષતિ॥ અગ્નિના રયિમશ્નવત્પોષમેવ દિવેદિવે। યશસં વીરવત્તમમ્॥ અગ્ને યં યજ્ઞમધ્વરં વિશ્વતઃ પરિભૂરસિ। સ ઇદ્દેવેષુ ગચ્છતિ॥ અગ્નિર્હોતા કવિક્રતુઃ સત્યશ્ચિત્રશ્રવસ્તમઃ। દેવો દેવેભિરા ગમત્॥ યદઙ્ગ દાશુષે ત્વમગ્ને ભદ્રં કરિષ્યસિ। તવેત્તત્સત્યમઙ્ગિરઃ॥ ઉપ ત્વાગ્ને દિવેદિવે દોષાવસ્તર્ધિયા વયમ્। નમો ભરન્ત એમસિ॥ રાજન્તમધ્વરાણાં ગોપામૃતસ્ય દીદિવિમ્। વર્ધમાનં સ્વે દમે॥ સ નઃ પિતેવ સૂનવેઽગ્ને સૂપાયનો ભવ। સચસ્વા નઃ સ્વસ્તયે॥"),
        ("મઙ્ગલં", ""),
    ];

    for (k, v) in arr {
        assert_eq!(check(k, devanāgarī_to_gujarātī), v);
    }
}
