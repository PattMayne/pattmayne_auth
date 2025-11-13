#![allow(dead_code)] // dead code come on I'm just not using the fields yet.

use actix_web::{ web, App, HttpServer };
use actix_files::Files;
use dotenvy;

// Local mods (they can use each other as crates instead of mods)
mod routes;
mod db;
mod utils;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenvy loads env variables for whole app
    // after this, just call std::env::var(variable_name)
    dotenvy::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static"))
            .service(
                web::scope("/auth")
                    .route("/login", web::get().to(routes::login_page))
                    .route("/register", web::get().to(routes::register_page))
                    .route("/", web::get().to(routes::auth_home))
            .service(routes::login_post)
            .service(routes::register_post))
            .service(routes::home)
            .service(routes::dashboard_page)
            .service(routes::error_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
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