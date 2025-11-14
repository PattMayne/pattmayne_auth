#![allow(dead_code)] // dead code come on I'm just not using the fields yet.

use actix_web::{ App, HttpServer, middleware::{from_fn}, test, web };
use actix_files::Files;
use dotenvy;

// Local mods (they can use each other as crates instead of mods)
mod routes;
mod db;
mod utils;
mod auth;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenvy loads env variables for whole app
    // after this, just call std::env::var(variable_name)
    dotenvy::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static"))
            .wrap(from_fn(middleware::login_status_middleware))
            .service(routes::home)
            .service(routes::dashboard_page)
            .service(routes::error_page)
            .service(
                web::scope("/auth")
                    .route("/login", web::get().to(routes::login_page))
                    .route("/register", web::get().to(routes::register_page))
                    .route("/", web::get().to(routes::auth_home))
                    .service(routes::login_post)
                    .service(routes::register_post)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
