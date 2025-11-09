#![allow(dead_code)] // dead code come on I'm just not using the fields yet.

use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, post, web::Redirect};
use askama::Template;
use actix_files::Files;
use serde::{Deserialize, Serialize};

// local modules
mod db;
mod utils;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
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


async fn auth_home() -> impl Responder {
    Redirect::to("/auth/login")
}


/* ROOT DOMAIN */
#[get("/")]
async fn real_home() -> impl Responder {
    let _ = db::load_db(); // TODO: catch potential errors

    let pw_hash = db::hash_password(String::from("mottolax"));
    println!("{}", pw_hash);
    

    // For now we create a static fake user who is not logged in
    let user : User = User { username: String::from("Matt"), is_logged_in: false };

    // create a ternary for a message based on whether fake user is logged in
    let state_string: &str = if user.is_logged_in {"LOGGED IN"} else {"NOT LOGGED IN"};
    let title: &str = "Pattmayne Games";

    let home_template: HomeTemplate<'_> = HomeTemplate { message: state_string, title: title };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(home_template.render().unwrap())
 }


/* LOGIN PAGE ROUTE FUNCTION */
async fn login_page() -> impl Responder {
    
    let user : User = User { username: String::from("Matt"), is_logged_in: false };

    // create a ternary for a message based on whether fake user is logged in
    let state_string: &str = if user.is_logged_in {"ALREADY LOGGED IN"} else {"PLEASE LOG IN"};
    let title: &str = "LOGIN";

    let login_template = LoginTemplate { message: state_string, title: title };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(login_template.render().unwrap())
}



/* REGISTER PAGE ROUTE FUNCTION */
async fn register_page() -> impl Responder {
    
    let user : User = User { username: String::from("Matt"), is_logged_in: false };

    // create a ternary for a message based on whether fake user is logged in
    let state_string: &str = if user.is_logged_in {"ALREADY LOGGED IN"} else {"PLEASE LOG IN"};
    let title: &str = "REGISTER";

    let register_template = RegisterTemplate { message: state_string, title: title };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(register_template.render().unwrap())
}

/* TEST ROUTE FUNCTION (delete later) */
#[get("/home")]
async fn home() -> impl Responder {
    "You are home"
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

    // TO DO: check the database for duplicate email or username (code 409 for failure)

    if !credentials_are_ok {
        // 401 is auth failure (for login failure) 
        // 422 is for regex failure / validation error
        // 409 is for conflict: email or username already taken

        // 401 (not valid here for reg)
        //return HttpResponse::Unauthorized().body("Invalid username or password");

        // 409 Conflict   (specify whether email or password already exists)
        return HttpResponse::Conflict().body("Email or password already in use");

        // BUT send a JSON instead of a .body("message")
        // For ALL of them!
    }

    HttpResponse::Ok().finish()
}



/** LOGIN
 * Get user data, check it against the DB & see if it's right.
*/
#[post("/login")]
async fn login_post(info: web::Json<LoginCredentials>) -> HttpResponse {
    println!("Loggin in");
    println!("Username or Password: {}", info.username_or_email);
    println!("Password: {}", info.password);
    
    let credentials_are_ok: bool = true;

    if info.username_or_email.trim().is_empty() || info.password.trim().is_empty() {
        println!("empty something");
        return HttpResponse::BadRequest().body("Username or password is empty");
    }

    if !credentials_are_ok {
        return HttpResponse::Unauthorized().body("Invalid username or password");
    }


    // TRYING TO GET A USER:
    // for now just assume it's a username

    let user_result = db::get_user_by_username(&info.username_or_email).await;

    // NOW we can do PATTERN MATCHING to return something

    match user_result {
        Ok(Some(user)) => {
            println!("found a user");
            HttpResponse::Ok().json(user)
        },
        Ok(None) => {
            println!("Did NOT find a user");
            HttpResponse::NotFound().body("User not found")
        },
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            HttpResponse::InternalServerError().body("Internal error")
        }
    }


}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}




struct User {
    username: String,
    is_logged_in: bool,
}


/*
 * ROUTES:
 *
 *  GET:
 *  -index (shows whether logged in or not, and links)
 *  -login (login page to take credentials)
 *  -register (register page to take credentials)
 *
 *  POST:
 *  -login (returns JWT (JSON obj with signature))
 *  -register
 *
 * 
 * */
