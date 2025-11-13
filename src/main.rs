#![allow(dead_code)] // dead code come on I'm just not using the fields yet.

use actix_web::{web, App, HttpServer, HttpResponse, Responder, http::StatusCode, get, post, web::Redirect};
use actix_web::cookie::{Cookie, SameSite};
use askama::Template;
use actix_files::Files;
use serde::{Deserialize, Serialize};
use time::{ OffsetDateTime, Duration };
use dotenvy;


// local modules
mod db;
mod utils;
mod auth;

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


// DEPRECATED: Upon successful Registration or login, send back auth token (JWT token)
#[derive(Serialize)]
struct AuthData {
    user_id: i32,
    jwt: String,
    refresh_token: String,
    refresh_token_expires_at: OffsetDateTime,
    jwt_expires_at: OffsetDateTime,
}


// Upon successful Registration or login, send back auth token (JWT token)
#[derive(Serialize)]
struct FreshLoginData {
    user_id: i32,
    username: String,
}

// Askama template macros (to load HTML templates for route functions to use)

#[derive(Template)]
#[template(path ="index.html")]
struct HomeTemplate<'a> {
    title: &'a str,
    message: &'a str,
}


#[derive(Template)]
#[template(path ="login.html")]
struct LoginTemplate<'a> {
    title: &'a str,
    message: &'a str,
}

#[derive(Template)]
#[template(path ="register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
    message: &'a str,
}


#[derive(Template)]
#[template(path ="error.html")]
struct ErrorTemplate<'a> {
    title: &'a str,
    message: &'a str,
    code: &'a str,
}


#[derive(Template)]
#[template(path ="dashboard.html")]
struct DashboardTemplate<'a> {
    title: &'a str,
    user_data: &'a db::User,
}




async fn auth_home() -> impl Responder {
    Redirect::to("/auth/login")
}


/* ROOT DOMAIN */
#[get("/")]
async fn real_home() -> impl Responder {
    let state_string: &str = "NOT LOGGED IN";
    let title: &str = "Pattmayne Games";

    let home_template: HomeTemplate<'_> = HomeTemplate { message: state_string, title: title };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(home_template.render().unwrap())
 }


/* LOGIN PAGE ROUTE FUNCTION */
async fn login_page() -> impl Responder {
    let state_string: &str = "PLEASE LOG IN";
    let title: &str = "LOGIN";

    let login_template = LoginTemplate { message: state_string, title: title };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(login_template.render().unwrap())
}



/* REGISTER PAGE ROUTE FUNCTION */
async fn register_page() -> impl Responder {
    let state_string: &str = "Please Register";
    let title: &str = "REGISTER";

    let register_template = RegisterTemplate { message: state_string, title: title };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(register_template.render().unwrap())
}

/* TEST ROUTE FUNCTION (delete later) */
#[get("/home")]
async fn home() -> impl Responder { "You are home" }


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
        println!("empty something");
        return HttpResponse::Unauthorized().json(
            ErrorResponse {
            error: String::from("Invalid Credentials: Empty Field."),
            code: 401
        });
    }

    // TRYING TO GET A USER:

    // Find out if pattern matches email (and retrieve use by email), else treat as username (and
    // retrieve by username)
    let user_result: Result<Option<db::User>, anyhow::Error> = if utils::validate_email(&info.username_or_email) {
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

            let auth_failure_data: ErrorResponse = ErrorResponse {
                error: String::from("Invalid Credentials"),
                code: 401
            };

            HttpResponse::Unauthorized().json(auth_failure_data)
        },
        Ok(None) => {
            let lookup_failure_data: ErrorResponse = ErrorResponse {
                error: String::from("User not found."),
                code: 404
            };

            HttpResponse::NotFound().json(lookup_failure_data)
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
    let jwt_secret_result: Result<String, std::env::VarError> = std::env::var("JWT_SECRET");
    let jwt_err_string: &str = "JSON Web Token Error: ";

    // Checking that the secret exists
    match jwt_secret_result {
        Ok(jwt_secret) => {

            // Secret exists. Now let's generate the actual token
            let jwt_result: Result<String, jsonwebtoken::errors::Error> = auth::generate_jwt(
                user.get_id(),
                user.get_role().to_owned(),
                jwt_secret.as_bytes()
            );

            // Make sure we really got a token
            match jwt_result {
                Ok(jwt) => {
                    // TOTAL SUCCESS: Returning auth data in a json
                    let refresh_token: String = String::from("PLACEHOLDER_REFRESH_TOKEN");

                    let jwt_cookie: Cookie<'_> = build_token_cookie(
                        jwt,
                        String::from("jwt"));
                    let refresh_token_cookie: Cookie<'_> = build_token_cookie(
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



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // loads env variables for whole app
    // after this, just call std::env::var(variable_name)
    dotenvy::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static"))
            .service(
                web::scope("/auth")
                    .route("/login", web::get().to(login_page))
                    .route("/register", web::get().to(register_page))
                    .route("/", web::get().to(auth_home))
            .service(login_post)
            .service(register_post))
            .service(home)
            .service(real_home)
            .service(dashboard_page)
            .service(error_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


/**
 * Setting a cookie only works for browsing within the auth site
 * For external app authentication we will implement OAuth2
 */
fn build_token_cookie(token: String, name: String) -> Cookie<'static> {

    // WARNING: THIS MUST BE TRUE IN PROD. Change env variable
    let secure: bool = std::env::var("COOKIE_SECURE")
        .map(|value: String| value == "true")
        .unwrap_or(false);

    Cookie::build(name, token)
        .http_only(true)
        .secure(secure) 
        .same_site(SameSite::Lax)
        .path("/")
        .finish()
}



// OTHER ROUTES



#[get("/dashboard")]
async fn dashboard_page() -> HttpResponse {

    // must do auth check here

    let message: &str = "Welcome to your error";
    let title: &str = "ERROR TITLE";
    let code: &str = "500?";

    let user: db::User = db::User::new(
        1,
        String::from("fake username"),
        String::from("email@address"),
        Some(String::from("fake first name")),
        Some(String::from("fake last name")),
        String::from("player"),
        String::from("fake password hash"),
        OffsetDateTime::now_utc(),
        true);

    let dashboard_template = DashboardTemplate { user_data: &user, title };


    HttpResponse::Ok()
        .content_type("text/html")
        .body(dashboard_template.render().unwrap())

}




#[get("/error")]
async fn error_page() -> HttpResponse {

    let message: &str = "Welcome to your error";
    let title: &str = "ERROR TITLE";
    let code: &str = "500?";

    let error_template = ErrorTemplate { message, title, code };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(error_template.render().unwrap())

}



/*
 * ROUTES:
 *
 *  GET:
 *  -index (shows whether logged in or not, and links)
 *  -login (login page to take credentials)
 *  -register (register page to take credentials)
 *  -error
 *  -dashboard
 *
 *  POST:
 *  -login (returns JWT (JSON obj with signature))
 *  -register
 *
 * 
 * */
