use actix_web::{web, App, HttpServer, HttpResponse, Responder, get};
use askama::Template;
use actix_files::Files;

// Askama template macros

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


async fn hello() -> impl Responder {
    "Hello world"
}

async fn auth_home() -> impl Responder {
    "Auth Home"
}


/* ROOT DOMAIN */
#[get("/")]
async fn real_home() -> impl Responder {
    // For now we create a static fake user who is not logged in
    let user : User = User { username: String::from("Matt"), is_logged_in: false };

    // create a ternary for a message based on whether fake user is logged in
    let state_string: &str = if user.is_logged_in {"LOGGED IN"} else {"NOT LOGGED IN"};
    let title: &str = "Pattmayne Games";

    let home_template = HomeTemplate { message: state_string, title: title };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(home_template.render().unwrap())
 }


/* LOGIN ROUTE FUNCTION */
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



/* EGISTER ROUT FUNCTION */
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


#[get("/home")]
async fn home() -> impl Responder {
    "You are home"
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
                    .route("/", web::get().to(auth_home)))
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
 *  TO DO:
 * -- incorporate askama templates
 * -- clicking a link to login or register opens a page with a form
 * -- on login or register page, the form sends a post request and creates a logged in user for main / page
 * -- incorporate database and make schema\
 * -- create actual JWT
 * -- remove extra routes
 * -- style nicely (html and css)
 * -- create endpoints for another app to authenticate
 * --------- create an enum of apps that can use this
 * 
 * 
 * */
