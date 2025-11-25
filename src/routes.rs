/* 
 * ====================
 * ====================
 * =====          =====
 * =====  ROUTES  =====
 * =====          =====
 * ====================
 * ====================
 * 
 * 
 * 
 * Functions to be called when user request hits endpoints listed
 * in the main function.
 * 
 * 
 */

use actix_web::{
    web, HttpResponse, HttpRequest,
    Responder, http::StatusCode, http::header,
    get, post, web::Redirect };
use actix_web::cookie::{ Cookie };
use askama::Template;
use serde::{ Deserialize, Serialize };

// local modules, loaded as crates (declared as mods in main.rs)
use crate::db;
use crate::utils;
use crate::auth::{self, UserReqData};

/* 
 * 
 * 
 * 
 * 
 * 
 * =======================================
 * =======================================
 * =====                             =====
 * =====  STRUCTS for SERIALIZATION  =====
 * =====                             =====
 * =======================================
 * =======================================
 * 
 * 
 * (outgoing data)
 * 
 * 
 * 
 * 
 */

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,
}

#[derive(Serialize)]
struct SendToError {
    send_to_error_page: bool,
    code: u16,
}

#[derive(Serialize)]
struct BadRegistrationInputs {
    email_valid: bool,
    username_valid: bool,
    password_valid: bool,
    code: u16,
}


#[derive(Serialize)]
struct BadNames {
    names_valid: bool,
    code: u16,
}


#[derive(Serialize)]
struct RawClientSecret {
    raw_client_secret: String,
}

#[derive(Serialize)]
struct BadPassword {
    password_valid: bool,
    code: u16,
}


// Upon successful Registration or login, send back auth token (JWT token)
#[derive(Serialize)]
struct FreshLoginData {
    user_id: i32,
    username: String,
}


#[derive(Serialize)]
struct LogoutData {
    logout: bool,
}


#[derive(Serialize)]
struct UpdateData {
    success: bool,
}


impl LogoutData {
    pub fn new() -> Self {
        LogoutData { logout: true } }
}


impl UpdateData {
    pub fn new(success: bool) -> Self {
        UpdateData { success } }
}


impl BadNames {
    pub fn new(code: u16) -> Self {
        BadNames {
            code,
            names_valid: false,
        }
    }
}


impl BadPassword {
    pub fn new(code: u16) -> Self {
        BadPassword {
            code,
            password_valid: false,
        }
    }
}

impl SendToError {
    pub fn new(code: u16) -> Self {
        SendToError {
            code,
            send_to_error_page: true,
        }
    }
}


/* 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * ==========================================
 * ==========================================
 * =====                                =====
 * =====  STRUCTS for DE-SERIALIZATION  =====
 * =====                                =====
 * ==========================================
 * ==========================================
 * 
 * for JSON DE-SERIALIZATION (incoming data)
 * 
 * 
 * 
 * 
 * 
 */


// Store credentials when user tries to register
#[derive(Deserialize)]
struct RegisterCredentials {
    username: String,
    email: String,
    password: String,
}


// Store credentials when a user tries to login
#[derive(Deserialize)]
struct LoginCredentials{
    username_or_email: String,
    password: String
}

#[derive(Deserialize)]
struct RealNames {
    pub first_name: String,
    pub last_name: String,
}


#[derive(Deserialize)]
struct NewPassword {
    pub password: String,
}


#[derive(Deserialize)]
pub struct ClientDataReq {
    pub client_id: String,
}


#[derive(Deserialize)]
pub struct ClientInputs {
    pub site_domain: String,
    pub site_name: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub logo_url: String,
    pub description: String,
    pub category: String,
    pub client_type: String,
    pub is_active: bool,
}

impl ClientInputs {
    pub fn trim_all_strings(&mut self) {
        self.client_id = self.client_id.trim().to_string();
        self.site_domain = self.site_domain.trim().to_string();
        self.site_name = self.site_name.trim().to_string();
        self.redirect_uri = self.redirect_uri.trim().to_string();
        self.logo_url = self.logo_url.trim().to_string();
        self.client_type = self.client_type.trim().to_string();
        self.description = self.description.trim().to_string();
        self.category = self.category.trim().to_string();
    }

    pub fn print_all_strings(&self) {
        println!("client_id: {}", self.client_id);
        println!("domain: {}", self.site_domain);
        println!("site_name: {}", self.site_name);
        println!("r_uri: {}", self.redirect_uri);
        println!("logo_url: {}", self.logo_url);
        println!("client_type: {}", self.client_type);
        println!("desc: {}", self.description);
        println!("category: {}", self.category);
    }
}


/* 
 * 
 * 
 * 
 * 
 * ===================================
 * ===================================
 * =====                         =====
 * =====  ASKAMA HTML TEMPLATES  =====
 * =====                         =====
 * ===================================
 * ===================================
 * 
 * 
 * 
 * 
 */


#[derive(Template)]
#[template(path ="index.html")]
struct HomeTemplate<'a> {
    title: &'a str,
    message: &'a str,
    user: auth::UserReqData,
}


#[derive(Template)]
#[template(path ="login.html")]
struct LoginTemplate<'a> {
    title: &'a str,
    message: &'a str,
    user: auth::UserReqData,
}

#[derive(Template)]
#[template(path ="admin_page.html")]
struct AdminTemplate<'a> {
    title: &'a str,
    message: &'a str,
    user: auth::UserReqData,
    client_refs: Vec<db::ClientRef>,
}


#[derive(Template)]
#[template(path ="new_client_form_page.html")]
struct NewClientTemplate<'a> {
    user: auth::UserReqData,
    title: &'a str,
    message: &'a str,
}


#[derive(Template)]
#[template(path ="edit_client_form_page.html")]
struct EditClientTemplate<'a> {
    user: auth::UserReqData,
    title: &'a str,
    message: &'a str,
    client_data: db::ClientData
}


#[derive(Template)]
#[template(path ="register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
    message: &'a str,
    user: auth::UserReqData,
}


#[derive(Template)]
#[template(path ="error.html")]
struct ErrorTemplate<> {
    error_data: utils::ErrorData,
    user: auth::UserReqData,
}


#[derive(Template)]
#[template(path ="dashboard.html")]
struct DashboardTemplate<'a> {
    title: &'a str,
    user_data: &'a db::User,
    user: auth::UserReqData,
}



/*
 * 
 * 
 * 
 * 
 * 
 * 
 * =========================
 * =========================
 * =====               =====
 * =====  POST ROUTES  =====
 * =====               =====
 * =========================
 * =========================
 * 
 * 
 * 
 * 
 * 
 * 
 */


/** REGISTER
 * The user/client calls this API to register.
 * We get user data, check it against regex for formatting,
 * and against the DB & see if it already exists.
*/
#[post("/register")]
async fn register_post(info: web::Json<RegisterCredentials>) -> HttpResponse {    

    // check credentials against regex and size ranges
    let username_valid: bool = utils::validate_username(&info.username);
    let email_valid: bool = utils::validate_email(&info.email);
    let password_valid: bool = utils::validate_password(&info.password);
    let credentials_are_ok: bool = username_valid && email_valid && password_valid;

    if !credentials_are_ok {
        // One of the fields doesn't match the regex
        let bad_creds_data: BadRegistrationInputs = BadRegistrationInputs {
            email_valid,
            username_valid,
            password_valid,
            code: 422,
        };

        return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(bad_creds_data);
    }

    /* Input credentials are acceptable format.
     * Try to enter them in the database.
     * if username or email already exists, send a 409
     * Do a pre-check first.
    */

    let username_exists = db::username_taken(&info.username).await;
    let email_exists = db::email_taken(&info.email).await;
    let username_or_email_already_exists: bool = username_exists || email_exists;

    if username_or_email_already_exists {
        let bad_creds_data: BadRegistrationInputs = BadRegistrationInputs {
            email_valid: !email_exists,
            username_valid: !username_exists,
            password_valid,
            code: 409,
        };

        return HttpResponse::Conflict().json(bad_creds_data);
    }

    // NOW we've done our pre-checks. Time to add User to DATABASE
    // We can still send errors if there's a duplicate or a problem
    
    let user_id_result: Result<i32, anyhow::Error> = db::add_user(
        &info.username,
        &info.email,
        info.password.clone()
    ).await;

    match user_id_result {
        Ok(user_id) => {

            // get user object from DB
            match db::get_user_by_id(user_id).await {
                Ok(Some(user)) => {
                    // User may now receive JWT and refresh token.
                    return give_user_auth_cookies(user).await;
                },
                Ok(None) => {
                    return HttpResponse::NotFound().json(ErrorResponse {
                        error: String::from("User not found."),
                        code: 404
                    });
                },
                Err(e) => {
                    // Worse than not finding a user. Something broke.
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        error: e.to_string(),
                        code: 500
                    });
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to save user to DB: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
                code: 500
            })
        }
    }
}


/** LOGIN
 * Get user data, check it against the DB & see if it's right.
*/
#[post("/login")]
async fn login_post(info: web::Json<LoginCredentials>) -> HttpResponse {

    // Check for empty fields
    if info.username_or_email.trim().is_empty() || info.password.trim().is_empty() {
        return HttpResponse::Unauthorized().json(
            ErrorResponse {
            error: String::from("Invalid Credentials: Empty Field."),
            code: 401
        });
    }

    // TRYING TO GET A USER:

    // Find out if pattern matches email (and retrieve use by email), else treat as username (and
    // retrieve by username)
    let user_result: Result<Option<db::User>, anyhow::Error> =
        if utils::validate_email(&info.username_or_email) {
            db::get_user_by_email(&info.username_or_email).await
        } else {
            db::get_user_by_username(&info.username_or_email).await
    };

    // NOW we can do PATTERN MATCHING to return something
    match user_result {
        Ok(Some(user)) => {

            // Now check the password
            if db::verify_password(&info.password, user.get_password_hash()) {
                // User may now receive JWT and refresh token.
                return give_user_auth_cookies(user).await;
            }

            // Auth clearly failed
            HttpResponse::Unauthorized().json(ErrorResponse {
                error: String::from("Invalid Credentials"),
                code: 401
            })
        },
        Ok(None) => {
            HttpResponse::NotFound().json(ErrorResponse {
                error: String::from("User not found."),
                code: 404
            })
        },
        Err(e) => {
            // Worse than not finding a user. Something broke.
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: e.to_string(),
                code: 500
            })
        }
    }
}


/**
 * The post route for adding new CLIENT SITE to the database.
 * Checks that the user is truly an admin, checks that all the 
 * data is legit, then adds it to the database.
 * Lots of opportunities to send errors.
 */
#[post("/add_client")]
async fn new_client_post(
    req: HttpRequest,
    mut inputs: web::Json<ClientInputs>
) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    // check if they're admin
    if let Some(redirect_resp) = redirect_non_admin(&user_req_data, &req) {
        return redirect_resp;
    }

    // Trim every string
    inputs.trim_all_strings();

    // Make sure the required fields are not empty
    let domains_are_valid: bool =
        utils::validate_url(&inputs.redirect_uri) &&
        utils::validate_url(&inputs.site_domain);
    
    if !domains_are_valid {
        println!("domains are not valid");
        return HttpResponse::build(StatusCode::NOT_ACCEPTABLE)
            .json(ErrorResponse{
                error: String::from("Invalid domain format"),
                code: 406
            })
    }

    // Check all the fields.

    let client_id_is_valid: bool = utils::string_length_valid(
        utils::StringRange{ min: 2, max: 20 },
        &inputs.client_id
    ) && utils::has_no_whitespace(
        &inputs.client_id
    );

    if !client_id_is_valid {
        println!("CLIENT ID is not valid");
        return HttpResponse::build(StatusCode::NOT_ACCEPTABLE)
            .json(ErrorResponse{
                error: String::from("Client ID must be 2 to 20 characters with no spaces"),
                code: 406
            });
    }

    let name_is_valid: bool = utils::string_length_valid(
        utils::StringRange{ min: 2, max: 20 },
        &inputs.site_name
    );

    if !name_is_valid {
        println!("NAME is not valid");
        return HttpResponse::build(StatusCode::NOT_ACCEPTABLE)
            .json(ErrorResponse{
                error: String::from("Site name must be 2 to 20 characters."),
                code: 406
            });
    }

    // If string checks passed, enter into DB, generate secret, show admin secret
    if domains_are_valid && client_id_is_valid && name_is_valid {
        let raw_client_secret: String = utils::generate_client_secret();
        let hashed_secret: String = db::hash_password(raw_client_secret.to_owned());        

        let client_data: db::NewClientData = db::NewClientData {
            site_domain: inputs.site_domain.to_owned(),
            site_name: inputs.site_name.to_owned(),
            client_id: inputs.client_id.to_owned(),
            redirect_uri: inputs.redirect_uri.to_owned(),
            hashed_client_secret: hashed_secret.to_owned(),
            logo_url: inputs.logo_url.to_owned(),
            description: inputs.description.to_owned(),
            category: inputs.category.to_owned(),
            client_type: inputs.client_type.to_owned(),
            is_active: inputs.is_active,
        };

        let update_client_result: Result<u64, anyhow::Error> =
            db::add_external_client(client_data).await;
        
        match update_client_result {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    // We added it to the DB. Send the admin their raw secret.
                    let raw_client_secret_json: RawClientSecret = RawClientSecret {
                        raw_client_secret
                    };
                    return HttpResponse::Ok()
                        .json(raw_client_secret_json);
                } else {
                    return return_internal_err_json();
                }

            },
            Err(e) => {
                // Database error
                eprintln!("Error: {e}");
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(ErrorResponse{
                        error: format!("Error: {e}"),
                        code: 500
                    })
            }
        }

        
    } else {
        HttpResponse::build(StatusCode::NOT_ACCEPTABLE)
            .json(ErrorResponse{
                error: String::from("Bad Inputs"),
                code: 406
            })
    }

}

/**
 * The post route for adding new CLIENT SITE to the database.
 * Checks that the user is truly an admin, checks that all the 
 * data is legit, then adds it to the database.
 * Lots of opportunities to send errors.
 */
#[post("/update_client")]
async fn update_client_post(
    req: HttpRequest,
    mut inputs: web::Json<ClientInputs>
) -> HttpResponse {
    println!("UPDATING CLIENT");

    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    // check if they're admin
    if let Some(redirect_resp) = redirect_non_admin(&user_req_data, &req) {
        return redirect_resp;
    }

    // Trim every string
    inputs.trim_all_strings();

    // Make sure the required fields are not empty
    let domains_are_valid: bool =
        utils::validate_url(&inputs.redirect_uri) &&
        utils::validate_url(&inputs.site_domain);
    
    if !domains_are_valid {
        println!("domains are not valid");
        return HttpResponse::build(StatusCode::NOT_ACCEPTABLE)
            .json(ErrorResponse{
                error: String::from("Invalid domain format"),
                code: 406
            })
    }

    // Check all the fields.

    let client_id_is_valid: bool = utils::string_length_valid(
        utils::StringRange{ min: 2, max: 20 },
        &inputs.client_id
    ) && utils::has_no_whitespace(
        &inputs.client_id
    );

    if !client_id_is_valid {
        println!("CLIENT ID is not valid");
        return HttpResponse::build(StatusCode::NOT_ACCEPTABLE)
            .json(ErrorResponse{
                error: String::from("Client ID must be 2 to 20 characters with no spaces"),
                code: 406
            });
    }

    let name_is_valid: bool = utils::string_length_valid(
        utils::StringRange{ min: 2, max: 20 },
        &inputs.site_name
    );

    if !name_is_valid {
        println!("NAME is not valid");
        return HttpResponse::build(StatusCode::NOT_ACCEPTABLE)
            .json(ErrorResponse{
                error: String::from("Site name must be 2 to 20 characters."),
                code: 406
            });
    }

    // If string checks passed, enter into DB, generate secret, show admin secret
    if domains_are_valid && client_id_is_valid && name_is_valid {
        let raw_client_secret: String = utils::generate_client_secret();
        let hashed_secret: String = db::hash_password(raw_client_secret.to_owned());        

        let client_data: db::UpdateClientData = db::UpdateClientData {
            site_domain: inputs.site_domain.to_owned(),
            site_name: inputs.site_name.to_owned(),
            client_id: inputs.client_id.to_owned(),
            redirect_uri: inputs.redirect_uri.to_owned(),
            logo_url: inputs.logo_url.to_owned(),
            description: inputs.description.to_owned(),
            category: inputs.category.to_owned(),
            client_type: inputs.client_type.to_owned(),
            is_active: inputs.is_active,
        };
    
        let update_client_result: Result<i32, anyhow::Error> =
            db::update_external_client(client_data).await;
        
        match update_client_result {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    let update_data: UpdateData = UpdateData::new(true);

                    HttpResponse::Ok()
                        .json(update_data)
                } else {
                    return_internal_err_json()
                }

            },
            Err(e) => {
                // Database error
                eprintln!("Error: {e}");
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(ErrorResponse{
                        error: format!("Error: {e}"),
                        code: 500
                    })
            }
        }
    } else {
        HttpResponse::build(StatusCode::NOT_ACCEPTABLE)
            .json(ErrorResponse{
                error: String::from("Bad Inputs"),
                code: 406
            })
    }
}



/**
 * We only do this once the user has been authenticated.
 * Calls functions to generate JWT and refresh token,
 * puts them in cookies and sends the response,
 * along with some user info in a JSON.
 */
async fn give_user_auth_cookies(user: db::User) -> HttpResponse {
    // generate JWT. Don't send user obj (with password) back
    let jwt_err_500 = HttpResponse::InternalServerError().json(
        ErrorResponse {
            error: String::from("Error generating access token."),
            code: 500
    });

    // Checking that the secret exists


    // Secret exists. Now let's generate the actual token
    let jwt_result: Result<String, auth::AuthError> = auth::generate_jwt(
        user.get_id(),
        user.get_username().to_owned(),
        user.get_role().to_owned()
    );

    // Make sure we really got a token
    match jwt_result {
        Ok(jwt) => {
            // create a refresh_token and put it in the DB
            let refresh_token: String = auth::generate_refresh_token();
            match db::add_refresh_token(
                user.get_id(),
                utils::auth_client_id(),
                &refresh_token
            ).await {
                Ok(_rows_affected) => {
                    // Refresh token successfully inserted into DB
                    // Now make the cookies and set them in the req
                    let jwt_cookie: Cookie<'_> = auth::build_token_cookie(
                        jwt,
                        String::from("jwt"));
                    let refresh_token_cookie: Cookie<'_> = auth::build_token_cookie(
                        refresh_token,
                        String::from("refresh_token"));

                    return HttpResponse::Ok()
                        .cookie(jwt_cookie)
                        .cookie(refresh_token_cookie)
                        .json(FreshLoginData {
                            user_id: user.get_id(),
                            username: user.get_username().to_owned()
                    });
                },
                Err(e) => {
                    eprint!("Internal Server Error: {e}");
                    jwt_err_500
                }
            }
        },

        // No token. Show error
        Err(_e) => {
            eprint!("Internal Server Error: JWT AuthError");
            jwt_err_500
        }
    }
}


#[post("/update_password")]
pub async fn update_password(req: HttpRequest, password_obj: web::Json<NewPassword>) -> HttpResponse {

    // make sure user is logged in
    match auth::get_user_req_data(&req).id {
        Some(id) => {
            match db::get_user_by_id(id).await {
                Ok(Some(_user)) =>{
                    // User is real user

                    // check credentials against regex and size ranges
                    let password_valid: bool = utils::validate_password(&password_obj.password);

                    if !password_valid {
                        // One of the fields doesn't match the regex
                        let bad_password_data: BadPassword = BadPassword::new(422);
                        return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY)
                            .json(bad_password_data);
                    }

                    // Names are valid. Update the DB
                    let update_password_result: Result<i32, anyhow::Error> =
                        db::update_password(&password_obj.password, id).await;
                    
                    match update_password_result {
                        Ok(rows_affected) => {
                            return HttpResponse::Ok()
                                .json(UpdateData::new(rows_affected > 0))
                        },
                        Err(_e) => {
                            return return_internal_err_json();
                        }
                    }
                },
                Ok(None) => { return return_authentication_err_json(); },
                Err(_e) => {
                    return return_internal_err_json();
                }
            };
        },
        None => return return_authentication_err_json()
    }
}


#[post("/update_names")]
pub async fn update_names(req: HttpRequest, names: web::Json<RealNames>) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    // make sure user is logged in
    match user_req_data.id {
        Some(id) => {
            match db::get_user_by_id(id).await {
                Ok(Some(_user)) =>{
                    // User is real user
                    // get json from the req, and names from json
                    // check names for length. Send back if too long or short

                    // check credentials against regex and size ranges
                    let names_valid: bool = utils::validate_real_name(&names.first_name) &&
                        utils::validate_real_name(&names.last_name);

                    if !names_valid {
                        // One of the fields doesn't match the regex
                        let bad_names_data: BadNames = BadNames::new(422);
                        return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY)
                            .json(bad_names_data);
                    }

                    // Names are valid. Update the DB
                    let update_names_result: Result<i32, anyhow::Error> =
                        db::update_real_names(
                            &names.first_name,
                            &names.last_name,
                            id
                    ).await;
                    
                    match update_names_result {
                        Ok(rows_affected) => {
                            return HttpResponse::Ok()
                                .json(UpdateData::new(rows_affected > 0));
                        },
                        Err(_e) => {
                            return return_internal_err_json();
                        }
                    }
                },
                Ok(None) => { return return_authentication_err_json(); },
                Err(_e) => {
                    return return_authentication_err_json();
                }
            };
        },
        None => return_authentication_err_json()
    }
}


#[post("/logout")]
pub async fn logout_post(req: HttpRequest) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    let user_id = match user_req_data.id {
        Some(id) => id,
        None => 0
    };

    let jwt_cookie: Cookie<'_> = Cookie::build("jwt", "")
        .path("/")
        .max_age(time::Duration::seconds(0))
        .http_only(true)
        .finish();

    // Must also delete refresh_token from DB (currently doesn't exist anyway)
    let refresh_cookie: Cookie<'_> = Cookie::build("refresh_token", "")
        .path("/")
        .max_age(time::Duration::seconds(0))
        .http_only(true)
        .finish();

    match db::delete_refresh_token(user_id).await {
        Ok(_rows_deleted) => {},
        Err(e) => {eprint!("Database error: {e}")}
    }

    HttpResponse::Ok()
        .cookie(jwt_cookie)
        .cookie(refresh_cookie)
        .json(LogoutData::new())
}




/* 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * ========================
 * ========================
 * =====              =====
 * =====  GET ROUTES  =====
 * =====              =====
 * ========================
 * ========================
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 */




/* ROOT DOMAIN */
#[get("/")]
async fn home(req: HttpRequest) -> impl Responder {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    // ORIGINAL CODE FOLLOWS

    let state_string: &str =
        if user_req_data.logged_in { "YOU ARE LOGGED IN" }
        else { "NOT LOGGED IN" };
    let title: &str = "Pattmayne Games";

    let home_template: HomeTemplate<'_> = HomeTemplate {
        message: state_string,
        title: title,
        user: user_req_data
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(home_template.render().unwrap())
 }


// if user just goes to /auth or /auth/
pub async fn auth_home() -> impl Responder {
    Redirect::to("/auth/login")
}


// if user just goes to /auth
pub fn redirect_to_err(err_code: String) -> impl Responder {
    Redirect::to(format!("/error/{}", err_code))
}


/* LOGIN PAGE ROUTE FUNCTION */
pub async fn login_page(req: HttpRequest) -> impl Responder {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    let state_string: &str = "PLEASE LOG IN";
    let title: &str = "LOGIN";

    let login_template = LoginTemplate {
        message: state_string,
        title: title,
        user: user_req_data
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(login_template.render().unwrap())
}



/* REGISTER PAGE ROUTE FUNCTION */
pub async fn register_page(req: HttpRequest) -> impl Responder {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    let state_string: &str = "Please Register";
    let title: &str = "REGISTER";

    let register_template: RegisterTemplate<'_> = RegisterTemplate {
        message: state_string,title: title,
        user: user_req_data
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(register_template.render().unwrap())
}


/**
 * Main admin dashboard
 * if user just goes to /auth
 */
pub async fn admin_home(req: HttpRequest) -> impl Responder {
    println!("ADMIN HOME");
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    // check if they're admin
    if let Some(redirect_resp) = redirect_non_admin(&user_req_data, &req) {
        return redirect_resp;
    }

    let title: &str = "ADMIN DASHBOARD";

    // Get client site references to list on admin site
    let client_refs: Vec<db::ClientRef> = match db::get_client_refs().await {
        Ok(refs) => refs,
        Err(e) => {
            eprintln!("Error retrieving client references: {e}");
            Vec::new()
        }
    };

    let admin_template: AdminTemplate = AdminTemplate {
        title,
        message: "Perform admin actions.",
        user: user_req_data,
        client_refs
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(admin_template.render().unwrap())  
}


pub async fn admin_redirect() -> impl Responder {
    Redirect::to("/admin/dashboard")
}


/**
 * The page where an admin can enter information for a NEW client site.
 * This is just the form. Another (post) function will receive the data
 * submitted from this form and process it.
 */
pub async fn new_client_site_form_page(req: HttpRequest) -> impl Responder {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    // check if they're admin
    if let Some(redirect_resp) = redirect_non_admin(&user_req_data, &req) {
        return redirect_resp;
    }

    let new_client_template: NewClientTemplate = NewClientTemplate {
        title: "Add New Client Site",
        message: "Add a new client site to the network.",
        user: user_req_data
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(new_client_template.render().unwrap())
}


/**
 * The page where an admin can EDIT information for an EXISTING client site.
 * This is just the form. Another (post) function will receive the data
 * submitted from this form and process it.
 */
#[get("/edit_client/{auth_id}")]
pub async fn edit_client_site_form_page(
    req: HttpRequest,
    auth_id: web::Path<String>
) -> impl Responder {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    // check if they're admin
    if let Some(redirect_resp) = redirect_non_admin(&user_req_data, &req) {
        return redirect_resp;
    }

    // Get the requested client site data
    let client_data_result: Result<Option<db::ClientData>, anyhow::Error> =
        db::get_client_by_client_id(&auth_id).await;

    if client_data_result.is_err() {
        return return_error_page(&req, 404);
    }

    match client_data_result.unwrap() {
        Some(client_data) => {
            let new_client_template: EditClientTemplate = EditClientTemplate {
                title: "Add New Client Site",
                message: "Add a new client site to the network.",
                user: user_req_data,
                client_data
            };
            
            HttpResponse::Ok()
                .content_type("text/html")
                .body(new_client_template.render().unwrap())
        },
        None => return_error_page(&req, 404)
    }


}


/**
 * Main page for user account info.
 * */
#[get("/dashboard")]
pub async fn dashboard_page(req: HttpRequest) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    if user_req_data.id.is_none() {
        return send_to_login();
    }

    let id: i32 = user_req_data.id.unwrap();
    let title: &str = "DASHBOARD";
    match db::get_user_by_id(id).await {
        Ok(Some(user)) =>{
            let dashboard_template: DashboardTemplate<'_> = DashboardTemplate {
                user_data: &user,
                title,
                user: user_req_data
            };

            return HttpResponse::Ok()
                .content_type("text/html")
                .body(dashboard_template.render().unwrap());
        },
        Ok(None) => {
            return send_to_login();
        },
        Err(_e) => {
            // redirect to ERROR PAGE
            return HttpResponse::Found()
                .append_header((header::LOCATION, "/error"))
                .finish();
        }
    };
}


// Function for the catch-all "not found" route
pub async fn not_found() -> impl Responder {
    Redirect::to("/error/404")
}


#[get("/error/{code}")]
async fn error_page(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    let code_u16 = match path.into_inner().parse::<u16>() {
        Ok(code) => code,
        Err(_) => 400
    };

    let error_data: utils::ErrorData = utils::ErrorData::new(code_u16);

    let error_template: ErrorTemplate<> = ErrorTemplate {
        error_data,
        user: user_req_data
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(error_template.render().unwrap())
}


#[get("/error")]
async fn error_root() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "/error/500"))
        .finish()
}


#[get("/error/")]
async fn error_root_2() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "/error"))
        .finish()
}

/*
 * 
 * 
 * 
 * 
 * 
 * 
 * ==============================
 * ==============================
 * =====                    =====
 * =====  HELPER FUNCTIONS  =====
 * =====                    =====
 * ==============================
 * ==============================
 * 
 * 
 * 
 * 
 * 
 * 
 */


/**
 * Rather than rewrite this over and over, for situations where a guest tries to access
 * restricted pages, this function returns the login route in an HttpResponse.
 */
fn send_to_login() -> HttpResponse {
    HttpResponse::Found()
        .append_header((header::LOCATION, "/auth/login"))
        .finish()
}

fn non_admin_rejection(req: &HttpRequest) -> HttpResponse {
    // send to unauthorized error page
    return redirect_to_err(String::from("403"))
        .respond_to(req)
        .map_into_boxed_body();
}

/**
 * Sometimes we don't know what went wrong and we need to return a JSON
 * object which says so.
 */
fn return_internal_err_json() -> HttpResponse {
    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
        .json(ErrorResponse{
            error: String::from("Internal server error"),
            code: 500
        })
}

// If authentication failed and user must log back in
fn return_authentication_err_json() -> HttpResponse {
    HttpResponse::Unauthorized().json(ErrorResponse{
        error: String::from("Authentication required"),
        code: 401
    })
}

/**
 * Redirect to error page with a simple and easy function
 */
fn return_error_page(req: &HttpRequest, code: u16) -> HttpResponse {
    return redirect_to_err(code.to_string())
        .respond_to(req)
        .map_into_boxed_body();
}


/**
 * We often check if the user is admin.
 * This returns the appropriate redirect depending on
 * which kind of non-admin the user is.
 */
fn redirect_non_admin(
    user_req_data: &UserReqData,
    req: &HttpRequest
) -> Option<HttpResponse> {
    // Send guest to login
    if user_req_data.id.is_none() {
        return Some(send_to_login());
    }

    // If they're not admin send them to error page
    if !user_req_data.is_admin() {
        return Some(non_admin_rejection(&req));
    }

    // The user is an admin
    None
}
