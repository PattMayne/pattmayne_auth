use phf::phf_map;

use crate::utils::SupportedLangs;

/**
 * Text strings to use on website.
 * Placeholders must be NUMBERED starting with ZERO:
 * ie:  Hello, {0}! I hope you're having a good {1}!
 * --> where {0} and {1} can later be replaced with username and Morning/Afternoon
 */
static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "home.title.fr" => "Crankade",
    "home.title.en" => "Crankade",
    "home.greeting.en" => "Hello, {0}!",
    "home.greeting.fr" => "Bonjour, {0}!",
};


fn missing_trans(lang: &SupportedLangs) -> &'static str {
    match lang {
        SupportedLangs::English => "Missing translation",
        SupportedLangs::French => "Traduction manquante"
    }
}


/**
 * Take the keyword for the translation to call,
 * a language enum so we know the language suffix to add,
 * and an optional set of &str slices for placeholder phrases
 */
pub fn get_translation(
    keyword: &str,
    lang: &SupportedLangs,
    params_option: Option<&[&str]>
) -> String {
    let full_key: String = format!("{}.{}", keyword, lang.suffix());
    let translation: String =
        TRANSLATIONS.get(&full_key)
        .unwrap_or(&missing_trans(lang))
        .to_string();

    match params_option {
        None => translation,
        Some(args) => {
            let mut translation: String = translation;
            for (i, arg) in args.iter().enumerate() {
                let placeholder: String = format!("{{{}}}", i);
                translation = translation.replace(&placeholder, arg);
            }

            translation
        }
    }
}