use whatlang::detect;
use whatlang::Lang::*;


pub fn detect_language(text: &str) -> Result<String, String>  {

    if let Some(info) = detect(text) {
        let lang = info.lang();

        let iso_code = match lang {
            Epo => "eo",  // Esperanto
            Eng => "en",  // English
            Rus => "ru",  // Russian
            Cmn => "zh",  // Mandarin Chinese
            Spa => "es",  // Spanish
            Por => "pt",  // Portuguese
            Ita => "it",  // Italian
            Ben => "bn",  // Bengali
            Fra => "fr",  // French
            Deu => "de",  // German
            Ukr => "uk",  // Ukrainian
            Kat => "ka",  // Georgian
            Ara => "ar",  // Arabic
            Hin => "hi",  // Hindi
            Jpn => "ja",  // Japanese
            Heb => "he",  // Hebrew
            Yid => "yi",  // Yiddish
            Pol => "pl",  // Polish
            Amh => "am",  // Amharic
            Jav => "jv",  // Javanese
            Kor => "ko",  // Korean
            Nob => "nb",  // Norwegian Bokmål
            Dan => "da",  // Danish
            Swe => "sv",  // Swedish
            Fin => "fi",  // Finnish
            Tur => "tr",  // Turkish
            Nld => "nl",  // Dutch
            Hun => "hu",  // Hungarian
            Ces => "cs",  // Czech
            Ell => "el",  // Greek
            Bul => "bg",  // Bulgarian
            Bel => "be",  // Belarusian
            Mar => "mr",  // Marathi
            Kan => "kn",  // Kannada
            Ron => "ro",  // Romanian
            Slv => "sl",  // Slovenian
            Hrv => "hr",  // Croatian
            Srp => "sr",  // Serbian
            Mkd => "mk",  // Macedonian
            Lit => "lt",  // Lithuanian
            Lav => "lv",  // Latvian
            Est => "et",  // Estonian
            Tam => "ta",  // Tamil
            Vie => "vi",  // Vietnamese
            Urd => "ur",  // Urdu
            Tha => "th",  // Thai
            Guj => "gu",  // Gujarati
            Uzb => "uz",  // Uzbek
            Pan => "pa",  // Punjabi
            Aze => "az",  // Azerbaijani
            Ind => "id",  // Indonesian
            Tel => "te",  // Telugu
            Pes => "fa",  // Persian
            Mal => "ml",  // Malayalam
            Ori => "or",  // Odia
            Mya => "my",  // Burmese
            Nep => "ne",  // Nepali
            Sin => "si",  // Sinhala
            Khm => "km",  // Khmer
            Tuk => "tk",  // Turkmen
            Aka => "ak",  // Akan
            Zul => "zu",  // Zulu
            Sna => "sn",  // Shona
            Afr => "af",  // Afrikaans
            Lat => "la",  // Latin
            Slk => "sk",  // Slovak
            Cat => "ca",  // Catalan
            Tgl => "tl",  // Tagalog
            Hye => "hy",  // Armenian
            _ => return Err(format!("Unsupported language: {:?}", lang)),
        };
        Ok(iso_code.to_string())

    } else {
        Err("Language detection failed".to_string())   
    }
}