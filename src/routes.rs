use actix_web::{
    web, HttpResponse, HttpRequest,
    Responder, http::StatusCode, http::header,
    get, post, web::Redirect };
use actix_web::cookie::{ Cookie };
use askama::Template;
use serde::{ Deserialize, Serialize };
use time::{ OffsetDateTime };

// local modules, loaded as crates (declared as mods in main.rs)
use crate::db;
use crate::utils;
use crate::auth;

/*   STRUCTS for JSON SERIALIZATION (outgoing data) */

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
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

/*   STRUCTS for JSON DE-SERIALIZATION (incoming data) */


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


/*  ASKAMA HTML TEMPLATES   */


#[derive(Template)]
#[template(path ="index.html")]
struct HomeTemplate<'a> {
    title: &'a str,
    message: &'a str,
    logged_in: bool,
}


#[derive(Template)]
#[template(path ="login.html")]
struct LoginTemplate<'a> {
    title: &'a str,
    message: &'a str,
    logged_in: bool,
}


#[derive(Template)]
#[template(path ="register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
    message: &'a str,
    logged_in: bool,
}


#[derive(Template)]
#[template(path ="error.html")]
struct ErrorTemplate<'a> {
    title: &'a str,
    message: &'a str,
    code: &'a str,
    logged_in: bool,
}


#[derive(Template)]
#[template(path ="dashboard.html")]
struct DashboardTemplate<'a> {
    title: &'a str,
    user_data: &'a db::User,
    logged_in: bool,
}



/*  POST ROUTES  */


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
            let user_result: Result<Option<db::User>, anyhow::Error> =
                db::get_user_by_id(user_id).await;
            
            match user_result {
                Ok(Some(user)) => {
                    println!("Retrieved the user that we just saved");
                    // User may now receive JWT and refresh token.
                    return give_user_auth_cookies(user);
                },
                Ok(None) => {
                    let lookup_failure_data: ErrorResponse = ErrorResponse {
                        error: String::from("User not found."),
                        code: 404
                    };

                    return HttpResponse::NotFound().json(lookup_failure_data);
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

            let err_data: ErrorResponse = ErrorResponse {
                error: e.to_string(),
                code: 500
            };

            HttpResponse::InternalServerError().json(err_data)
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
        println!("empty field(s)");
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
            println!("found a user");

            // Now check the password
            if db::verify_password(&info.password, user.get_password_hash()) {
                println!("password match");

                // User may now receive JWT and refresh token.
                return give_user_auth_cookies(user);
            }

            // Auth clearly failed
            println!("password NOT match");

            HttpResponse::Unauthorized().json(
                ErrorResponse {
                error: String::from("Invalid Credentials"),
                code: 401
            })
        },
        Ok(None) => {
            HttpResponse::NotFound().json(
                ErrorResponse {
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
 * We only do this once the user has been authenticated.
 * Calls functions to generate JWT and refresh token,
 * puts them in cookies and sends the response,
 * along with some user info in a JSON.
 */
fn give_user_auth_cookies(user: db::User) -> HttpResponse {

    // generate JWT. Don't send user obj (with password) back
    let jwt_secret_result: Result<String, std::env::VarError> = auth::get_jwt_secret();
    let jwt_err_string: &str = "JSON Web Token Error: ";

    // Checking that the secret exists
    match jwt_secret_result {
        Ok(jwt_secret) => {

            // Secret exists. Now let's generate the actual token
            let jwt_result: Result<String, jsonwebtoken::errors::Error> = auth::generate_jwt(
                user.get_id(),
                user.get_username().to_owned(),
                user.get_role().to_owned(),
                jwt_secret.as_bytes()
            );

            // Make sure we really got a token
            match jwt_result {
                Ok(jwt) => {
                    // TOTAL SUCCESS: Returning auth data in a json
                    let refresh_token: String = String::from("PLACEHOLDER_REFRESH_TOKEN");

                    let jwt_cookie: Cookie<'_> = auth::build_token_cookie(
                        jwt,
                        String::from("jwt"));
                    let refresh_token_cookie: Cookie<'_> = auth::build_token_cookie(
                        refresh_token,
                        String::from("refresh_token"));

                    return HttpResponse::Ok()
                        .cookie(jwt_cookie)
                        .cookie(refresh_token_cookie)
                        .json(
                            FreshLoginData {
                                user_id: user.get_id(),
                                username: user.get_username().to_owned()
                    });
                },

                // No token. Show error
                Err(e) => {
                    // Returning error data in a json
                    return HttpResponse::InternalServerError().json(
                        ErrorResponse {
                            error: format!("{}{}", jwt_err_string, e),
                            code: 404
                    });
                }
            }          
        },

        // No JWT secret. Show error
        Err(e) => {
            // Returning error data in a json
            return HttpResponse::InternalServerError().json(
                ErrorResponse {
                    error: format!("{}{}", jwt_err_string, e),
                    code: 404
            });
        },
    }
}




#[post("/update_password")]
pub async fn update_password(req: HttpRequest, password_obj: web::Json<NewPassword>) -> HttpResponse {
    println!("called update_password API");
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    // make sure user is logged in
    match user_req_data.id {
        Some(id) => {
            let user_result: Result<Option<db::User>, anyhow::Error> = db::get_user_by_id(id).await;
            match user_result {
                Ok(Some(user)) =>{
                    // User is real user
                    // get password from password_obj
                    // check password for length. Send back if too long or short

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
                        db::update_password(
                            &password_obj.password,
                            id
                    ).await;
                    
                    match update_password_result {
                        Ok(rows_affected) => {
                            return HttpResponse::Ok()
                                .json(UpdateData::new(rows_affected > 0))
                        },
                        Err(e) => {
                            // return negative message in json
                            return HttpResponse::Found()
                                .append_header((header::LOCATION, "/auth/login"))
                                .finish();
                        }
                    }
                },
                Ok(None) => {
                    // redirect to LOGIN
                    return HttpResponse::Found()
                        .append_header((header::LOCATION, "/auth/login"))
                        .finish();
                },
                Err(e) => {
                    // redirect to ERROR PAGE
                    return HttpResponse::Found()
                        .append_header((header::LOCATION, "/error"))
                        .finish();
                }
            };
        },
        None => {
            // redirect to LOGIN
            return HttpResponse::Found()
                .append_header((header::LOCATION, "/auth/login"))
                .finish();
        }
    }
}





#[post("/update_names")]
pub async fn update_names(req: HttpRequest, names: web::Json<RealNames>) -> HttpResponse {
    println!("called update_names API");
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    // make sure user is logged in
    match user_req_data.id {
        Some(id) => {
            let user_result: Result<Option<db::User>, anyhow::Error> = db::get_user_by_id(id).await;
            match user_result {
                Ok(Some(user)) =>{
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
                                .json(UpdateData::new(rows_affected > 0))
                        },
                        Err(e) => {
                            // return negative message in json
                            return HttpResponse::Found()
                                .append_header((header::LOCATION, "/auth/login"))
                                .finish();
                        }
                    }
                },
                Ok(None) => {
                    // redirect to LOGIN
                    return HttpResponse::Found()
                        .append_header((header::LOCATION, "/auth/login"))
                        .finish();
                },
                Err(e) => {
                    // redirect to ERROR PAGE
                    return HttpResponse::Found()
                        .append_header((header::LOCATION, "/error"))
                        .finish();
                }
            };
        },
        None => {
            // redirect to LOGIN
            return HttpResponse::Found()
                .append_header((header::LOCATION, "/auth/login"))
                .finish();
        }
    }
}




#[post("/logout")]
pub async fn logout_post() -> HttpResponse {

    println!("kenny loggin out");

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

    HttpResponse::Ok()
        .cookie(jwt_cookie)
        .cookie(refresh_cookie)
        .json(LogoutData::new())
}



/*   GET ROUTES    */



/* ROOT DOMAIN */
#[get("/")]
async fn home(req: HttpRequest) -> impl Responder {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    // TEST PRINTING NOW but we will use this to display username later
    println!("role: {}", user_req_data.role);

    // ORIGINAL CODE FOLLOWS

    let state_string: &str = if user_req_data.logged_in { "YOU ARE LOGGED IN" } else { "NOT LOGGED IN" };
    let title: &str = "Pattmayne Games";

    let home_template: HomeTemplate<'_> = HomeTemplate {
        message: state_string,
        title: title,
        logged_in: user_req_data.logged_in
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(home_template.render().unwrap())
 }


// if user just goes to /auth
pub async fn auth_home() -> impl Responder {
    Redirect::to("/auth/login")
}


/* LOGIN PAGE ROUTE FUNCTION */
pub async fn login_page(req: HttpRequest) -> impl Responder {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    let state_string: &str = "PLEASE LOG IN";
    let title: &str = "LOGIN";

    let login_template = LoginTemplate {
        message: state_string,
        title: title,
        logged_in: user_req_data.logged_in
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
        logged_in: user_req_data.logged_in
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(register_template.render().unwrap())
}


#[get("/dashboard")]
pub async fn dashboard_page(req: HttpRequest) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);
    
    match user_req_data.id {
        Some(id) => {
            let title: &str = "DASHBOARD";
            let user_result: Result<Option<db::User>, anyhow::Error> = db::get_user_by_id(id).await;
            match user_result {
                Ok(Some(user)) =>{
                    let dashboard_template: DashboardTemplate<'_> = DashboardTemplate {
                        user_data: &user,
                        title,
                        logged_in: user_req_data.logged_in
                    };

                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(dashboard_template.render().unwrap());
                },
                Ok(None) => {
                    // redirect to LOGIN
                    return HttpResponse::Found()
                        .append_header((header::LOCATION, "/auth/login"))
                        .finish();
                },
                Err(e) => {
                    // redirect to ERROR PAGE
                    return HttpResponse::Found()
                        .append_header((header::LOCATION, "/error"))
                        .finish();
                }
            };
        },
        None => {
            // redirect to LOGIN
            HttpResponse::Found()
                .append_header((header::LOCATION, "/auth/login"))
                .finish()
        }
    }

}


#[get("/error")]
async fn error_page(req: HttpRequest) -> HttpResponse {
    let user_req_data: auth::UserReqData = auth::get_user_req_data(&req);

    let message: &str = "Welcome to your error";
    let title: &str = "ERROR TITLE";
    let code: &str = "500?";

    let error_template: ErrorTemplate<'_> = ErrorTemplate {
        message,
        title,
        code,
        logged_in: user_req_data.logged_in
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(error_template.render().unwrap())
}


