use phf::phf_map;

use crate::utils::SupportedLangs;

static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "home.title.fr" => "Crankade",
    "home.title.en" => "Crankade",
};


fn missing_trans(lang: &SupportedLangs) -> &'static str {
    match lang {
        SupportedLangs::English => "Missing translation",
        SupportedLangs::French => "Traduction manquante"
    }
}


pub fn get_translation(keyword: &str, lang: &SupportedLangs) -> &'static str {
    let full_key: String = format!("{}.{}", keyword, lang.suffix());
    TRANSLATIONS.get(&full_key).unwrap_or(&missing_trans(lang))
}