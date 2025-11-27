/* 
 * 
 * 
 * 
 * 
 * ==============================
 * ==============================
 * =====                    =====
 * =====  RESOURCE MANAGER  =====
 * =====                    =====
 * ==============================
 * ==============================
 * 
 * 
 * Gather translations into structs in this script
 * to keep that login out of the routes script.
 * 
 * 
 * 
*/


use crate::{
    auth::UserReqData,
    resources::{ get_translation, raw_trans_or_missing },
    utils::SupportedLangs
};


/* 
 * 
 * 
 * 
 * 
 * ==================================
 * ==================================
 * =====                        =====
 * =====  TRANSLATIONS STRUCTS  =====
 * =====                        =====
 * ==================================
 * ==================================
 * 
 * 
 * Each askama template will have a struct
 * designed to hold all necessary text.
 * 
 * 
 * 
*/



/**
 * route: get "/"
 */
pub struct HomeTexts {
    pub title: String,
    pub message: String,
    pub nav: NavTexts
}

impl HomeTexts {
    pub fn new(user_req_data: &UserReqData) -> HomeTexts {
        let title: String = get_translation("home.title", &user_req_data.lang, None);
        let message: String = get_translation(
            "home.greeting",
            &user_req_data.lang,
            Some(&[&user_req_data.get_role()])
        );
        let nav = NavTexts::new(&user_req_data.lang);

        HomeTexts {
            title,
            message,
            nav
        }
    }
}


/**
 * route: get "/login"
 */
pub struct LoginTexts {
    pub title: String,
    pub message: String,
    pub nav: NavTexts
}

impl LoginTexts {
    pub fn new(user_req_data: &UserReqData) -> LoginTexts {
        let title: String = get_translation("login.title", &user_req_data.lang, None);
        let message: String = get_translation(
            "login.message",
            &user_req_data.lang,
            Some(&[&user_req_data.get_role()])
        );
        let nav = NavTexts::new(&user_req_data.lang);

        LoginTexts {
            title,
            message,
            nav
        }
    }
}


/**
 * route: get "/admin"
 */
pub struct AdminTexts {
    pub title: String,
    pub message: String,
    pub nav: NavTexts
}

impl AdminTexts {
    pub fn new(user_req_data: &UserReqData) -> AdminTexts {
        let title: String = get_translation("admin.title", &user_req_data.lang, None);
        let message: String = get_translation(
            "admin.message",
            &user_req_data.lang,
            Some(&[&user_req_data.get_role()])
        );
        let nav = NavTexts::new(&user_req_data.lang);

        AdminTexts {
            title,
            message,
            nav
        }
    }
}


/**
 * route: get "/admin/new_client"
 */
pub struct NewClientTexts {
    pub title: String,
    pub message: String,
    pub nav: NavTexts
}

impl NewClientTexts {
    pub fn new(user_req_data: &UserReqData) -> NewClientTexts {
        let title: String = get_translation("new_client.title", &user_req_data.lang, None);
        let message: String = get_translation(
            "new_client.message",
            &user_req_data.lang,
            Some(&[&user_req_data.get_role()])
        );
        let nav = NavTexts::new(&user_req_data.lang);

        NewClientTexts {
            title,
            message,
            nav
        }
    }
}


/**
 * route: get "/admin/edit_client"
 */
pub struct EditClientTexts {
    pub title: String,
    pub message: String,
    pub nav: NavTexts
}

impl EditClientTexts {
    pub fn new(user_req_data: &UserReqData) -> EditClientTexts {
        let title: String = get_translation("edit_client.title", &user_req_data.lang, None);
        let message: String = get_translation(
            "edit_client.message",
            &user_req_data.lang,
            Some(&[&user_req_data.get_role()])
        );
        let nav = NavTexts::new(&user_req_data.lang);

        EditClientTexts {
            title,
            message,
            nav
        }
    }
}


/**
 * route: get "/register"
 */
pub struct RegisterTexts {
    pub title: String,
    pub message: String,
    pub nav: NavTexts
}

impl RegisterTexts {
    pub fn new(user_req_data: &UserReqData) -> RegisterTexts {
        let title: String = get_translation("register.title", &user_req_data.lang, None);
        let message: String = get_translation(
            "register.message",
            &user_req_data.lang,
            Some(&[&user_req_data.get_role()])
        );
        let nav = NavTexts::new(&user_req_data.lang);

        RegisterTexts {
            title,
            message,
            nav
        }
    }
}


/**
 * route: get "/error"
 */
pub struct ErrorTexts {
    pub nav: NavTexts
}

impl ErrorTexts {
    pub fn new(user_req_data: &UserReqData) -> ErrorTexts {
        let nav = NavTexts::new(&user_req_data.lang);

        ErrorTexts {
            nav
        }
    }
}


/**
 * route: get "/auth/dashboard"
 */
pub struct DashboardTexts {
    pub title: String,
    pub message: String,
    pub nav: NavTexts
}

impl DashboardTexts {
    pub fn new(user_req_data: &UserReqData) -> DashboardTexts {
        let title: String = get_translation("dash.title", &user_req_data.lang, None);
        let message: String = get_translation(
            "dash.greeting",
            &user_req_data.lang,
            Some(&[&user_req_data.get_role()])
        );
        let nav = NavTexts::new(&user_req_data.lang);

        DashboardTexts {
            title,
            message,
            nav
        }
    }
}





/* 
 * 
 * 
 * 
 * 
 * =====================
 * =====================
 * =====           =====
 * =====  TOP NAV  =====
 * =====           =====
 * =====================
 * =====================
 * 
 * 
 * 
 * 
*/




/**
 * The top-nav bar is loaded on every page, so here is a struct to gather
 * all of its button translations together.
 * They can be static references because they will never build by replacing
 * placeholders. Simple strings.
 */
pub struct NavTexts {
    pub home: &'static str,
    pub admin: &'static str,
    pub dashboard: &'static str,
    pub login: &'static str,
    pub register: &'static str,
    pub logout: &'static str,
}



impl NavTexts {

    /**
     * Just pass in a language to this constructor and get the right language version
     * of all the strings for the top-nav buttons.
     */
    pub fn new(lang: &SupportedLangs) -> NavTexts {
        let lang_suffix: &str = lang.suffix();

        let home_key: String = format!("{}.{}", "nav.home", lang_suffix);
        let admin_key: String = format!("{}.{}", "nav.admin", lang_suffix);
        let dash_key: String = format!("{}.{}", "nav.dashboard", lang_suffix);
        let login_key: String = format!("{}.{}", "nav.login", lang_suffix);
        let register_key: String = format!("{}.{}", "nav.register", lang_suffix);
        let logout_key: String = format!("{}.{}", "nav.logout", lang_suffix);

        let home: &'static str = raw_trans_or_missing(home_key.as_str(), lang);
        let admin: &'static str = raw_trans_or_missing(admin_key.as_str(), lang);
        let dashboard: &'static str = raw_trans_or_missing(dash_key.as_str(), lang);
        let login: &'static str = raw_trans_or_missing(login_key.as_str(), lang);
        let register: &'static str = raw_trans_or_missing(register_key.as_str(), lang);
        let logout: &'static str = raw_trans_or_missing(logout_key.as_str(), lang);

        NavTexts {
            home,
            admin,
            dashboard,
            login,
            register,
            logout,
        }
    }
}