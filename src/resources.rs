use phf::phf_map;

use crate::utils::SupportedLangs;

/**
 * Text strings to use on website.
 * Placeholders must be NUMBERED starting with ZERO:
 * ie:  Hello, {0}! I hope you're having a good {1}!
 * --> where {0} and {1} can later be replaced with username and Morning/Afternoon
 */
static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {

    // HOME PAGE
    "home.title.fr" => "Crankade",
    "home.title.en" => "Crankade",
    "home.greeting.en" => "Hello, {0}!",
    "home.greeting.fr" => "Bonjour, {0}!",

    // DASHBOARD PAGE
    "dash.title.fr" => "DASHBOARD",
    "dash.title.en" => "TABLEAU DE BORD",
    "dash.greeting.en" => "Edit your details, {0}!",
    "dash.greeting.fr" => "Modifiez vos informations, {0}!",

    // ADMIN DASHBOARD PAGE
    "admin_dash.title.fr" => "ADMIN HOME",
    "admin_dash.title.en" => "TABLEAU DE BORD",
    "admin_dash.greeting.en" => "Perform admin actions",
    "admin_dash.greeting.fr" => "Effectuer des actions administratives",

    //LOGIN PAGE
    "login.title.en" => "LOGIN",
    "login.title.fr" => "ACCUEIL ADMINISTRATIF",
    "login.message.en" => "Please Log In",
    "login.message.fr" => "Veuillez vous connecter",

    //REGISTER PAGE
    "register.title.en" => "REGISTER",
    "register.title.fr" => "INSCRIPTION",
    "register.message.en" => "Please register",
    "register.message.fr" => "Veuillez vous connecter",

    // NEW CLIENT SITE
    "new_client.title.en" => "NEW CLIENT SITE",
    "new_client.title.fr" => "NOUVEAU SITE CLIENT",
    "new_client.message.en" => "Add a new client site to the network.",
    "new_client.message.fr" => "Ajoutez un nouveau site client au réseau.",

    // NEW CLIENT SITE
    "edit_client.title.en" => "EDIT CLIENT SITE",
    "edit_client.title.fr" => "MODIFIER LE SITE DU CLIENT",
    "edit_client.message.en" => "Update existing client.",
    "edit_client.message.fr" => "Mettez à jour le client existant.",
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
    let translation_option: Option<&&str> = TRANSLATIONS.get(&full_key);
    
    if translation_option.is_none() {
        return missing_trans(lang).to_string();
    }

    let translation: String = translation_option
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