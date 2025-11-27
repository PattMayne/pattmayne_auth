use phf::phf_map;

use crate::utils::SupportedLangs;

/* 
 * 
 * 
 * 
 * 
 * =======================
 * =======================
 * =====             =====
 * =====  RESOURCES  =====
 * =====             =====
 * =======================
 * =======================
 * 
 * 
 * Text strings and functions to retrieve them.
 * Text is stored in static string references in a phf_map constant.
 * They can be retrieved by calling their keys.
 * 
 * We will have French and English versions of everything.
 * 
 * 
 * 
*/


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
    "dash.title.fr" => "TABLEAU DE BORD",
    "dash.title.en" => "DASHBOARD",
    "dash.greeting.en" => "Edit your details, {0}!",
    "dash.greeting.fr" => "Modifiez vos informations, {0}!",
    // DASHBOARD LABELS
    "dash.firstname.en" => "First Name:",
    "dash.firstname.fr" => "Prénom:",
    "dash.lastname.en" => "Last Name:",
    "dash.lastname.fr" => "Nom:",
    "dash.password1.en" => "New Password:",
    "dash.password1.fr" => "Nouveau mot de passe :",
    "dash.password2.en" => "Confirm Password:",
    "dash.password2.fr" => "Confirmer le mot de passe:",
    // DASHBOARD BUTTONS
    "dash.updatenames.btn.en" => "UPDATE NAMES",
    "dash.updatenames.btn.fr" => "MAJ NOMS",
    "dash.updatepass.btn.en" => "UPDATE PASSWORD",
    "dash.updatepass.btn.fr" => "MAJ MOT DE PASSE",

    // ADMIN DASHBOARD PAGE
    "admin.title.en" => "ADMIN HOME",
    "admin.title.fr" => "TABLEAU DE BORD ADMIN",
    "admin.message.en" => "Perform admin actions",
    "admin.message.fr" => "Effectuer des actions administratives",
    "admin.actions.label.en" => "ADMIN ACTIONS",
    "admin.actions.label.fr" => "ACTIONS ADMIN",
    "admin.editclients.label.en" => "EDIT CLIENT SITES",
    "admin.editclients.label.fr" => "MODIFIER LES SITES CLIENTS",
    "admin.newclient.btn.en" => "ADD NEW CLIENT",
    "admin.newclient.btn.fr" => "AJOUTEZ SITE CLIENT",


    // LOGIN PAGE
    "login.title.en" => "LOGIN",
    "login.title.fr" => "ACCUEIL",
    "login.message.en" => "Please Log In",
    "login.message.fr" => "Veuillez vous connecter",
    // LOGIN LABELS
    "login.username.email.label.en" => "Username or Email:",
    "login.username.email.label.fr" => "Nom d'utilisateur ou e-mail:",
    "login.password.label.en" => "Password:",
    "login.password.label.fr" => "Mot de passe:",
    // LOGIN BUTTONS
    "login.btn.en" => "LOGIN",
    "login.btn.fr" => "ACCUEIL",

    // REGISTER PAGE
    "register.title.en" => "REGISTER",
    "register.title.fr" => "INSCRIPTION",
    "register.message.en" => "Please register",
    "register.message.fr" => "Veuillez vous connecter",
    // REGISTER LABELS
    "register.username.label.en" => "Username:",
    "register.username.label.fr" => "Nom d'utilisateur:",
    "register.email.label.en" => " Email:",
    "register.email.label.fr" => "E-mail:",
    "register.password.label.en" => "Password:",
    "register.password.label.fr" => "Mot de passe:",
    // REGISTER BUTTONS
    "register.btn.en" => "REGISTER",
    "register.btn.fr" => "INSCRIPTION",

    // NEW CLIENT SITE
    "new_client.title.en" => "NEW CLIENT SITE",
    "new_client.title.fr" => "NOUVEAU SITE CLIENT",
    "new_client.message.en" => "Add a new client site to the network.",
    "new_client.message.fr" => "Ajoutez un nouveau site client au réseau.",

    // EDIT CLIENT SITE
    "edit_client.title.en" => "EDIT CLIENT SITE",
    "edit_client.title.fr" => "MODIFIER LE SITE DU CLIENT",
    "edit_client.message.en" => "Update existing client.",
    "edit_client.message.fr" => "Mettez à jour le client existant.",

    // CLIENT FORM LABELS (for both NEW and EDIT)
    "clientform.domain.en" => "Site Domain:",
    "clientform.domain.fr" => "Domaine du site:",
    "clientform.name.en" => "Site Name (public-facing title):",
    "clientform.name.fr" => "Nom du site (titre public) :",
    "clientform.id.en" => "Client ID (random unique identifier):",
    "clientform.id.fr" => "Client ID (unique identifier aléatoire):",
    "clientform.red_uri.en" => "Redirect URI:",
    "clientform.red_uri.fr" => "Redirection URI :",
    "clientform.logo_url.en" => "Logo URL:",
    "clientform.logo_url.fr" => "URL du logo :",
    "clientform.type.en" => "Type:",
    "clientform.type.fr" => "Type:",
    "clientform.cat.en" => "Category:",
    "clientform.cat.fr" => "Catégorie:",
    "clientform.desc.en" => "Description:",
    "clientform.desc.fr" => "Description:",
    "clientform.isactive.en" => "Is Active:",
    "clientform.isactive.fr" => "Est actif:",
    // CLIENT FORM BUTTONS
    "clientform.submit.en" => "SUBMIT",
    "clientform.submit.fr" => "SUBMIT",
    "clientform.save_changes.en" => "SUBMIT",
    "clientform.save_changes.fr" => "ENVOYER",
    "clientform.gen_secret.en" => "GENERATE NEW SECRET",
    "clientform.gen_secret.fr" => "GÉNÉRER UN NOUVEAU SECRET",

    // NAV BUTTONS
    "nav.home.en" => "HOME",
    "nav.home.fr" => "ACCUEIL",
    "nav.admin.en" => "ADMIN",
    "nav.admin.fr" => "ADMIN",
    "nav.login.en" => "LOGIN",
    "nav.login.fr" => "CONNEXION",
    "nav.register.en" => "REGISTER",
    "nav.register.fr" => "INSCRIPTION",
    "nav.logout.en" => "LOGOUT",
    "nav.logout.fr" => "DÉCONNEXION",
    "nav.dashboard.en" => "DASHBOARD",
    "nav.dashboard.fr" => "TABLEAU DE BORD",
};


/**
 * For missing translations, or mis-typed keys.
 */
fn missing_trans(lang: &SupportedLangs) -> &'static str {
    match lang {
        SupportedLangs::English => "[ translation missing ]",
        SupportedLangs::French => "[ traduction manquante ]"
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
    
    match TRANSLATIONS.get(full_key.as_str()) {
        Some(translation) => {
            let translation: String = translation.to_string();

            // replace placeholders with text from args
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
        },
        None => missing_trans(lang).to_string()
    }
}


/**
 * A quick and dirty retrieval of translations
 * which do NOT have placeholders.
 * Primarily for the nav translations.
 */
pub fn raw_trans_or_missing(
    key: &str,
    lang: &SupportedLangs
) -> &'static str {
    match TRANSLATIONS.get(key) {
        Some(translation) => translation,
        None => missing_trans(lang)
    }
}