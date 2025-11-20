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

/**
 * The main function logs all the routes as routes or "services".
 * A service route takes a route function which has used a macro to declare
 * its path.
 * Other routes ( .route) take a path and then a function to call when the path
 * is requested.
 * 
 * Add middleware at the point in the chain where its changes will become needed.
 * If you add middleware before static it will be called multiple times (bad, don't do).
 * Add it too late and its changes won't be available where needed.
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenvy loads env variables for whole app
    // after this, just call std::env::var(variable_name)
    dotenvy::dotenv().ok();
    db_first_entries().await;

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
                    .service(routes::logout_post)
                    .service(routes::update_names)
                    .service(routes::update_password)
            )
            .wrap(from_fn(middleware::jwt_cookie_middleware))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


/**
 * When the server first starts, make sure admin exists in users.
 * Also make sure auth site (this site) exists in client_sites.
 */
async fn db_first_entries() {
    // add the admin user if they don't exist
    match db::create_primary_admin().await {
        Ok(user_created) => {
            if user_created {
                println!("New admin created.");
            } else {
                println!("Admin already exists.")
            }            
        },
        Err(e) => {
            println!("DB Error: {e}");
        }
    };

    // add this site to the client_sites table if it doesn't exist
    match db::create_self_client().await {
        Ok(user_created) => {
            if user_created {
                println!("New client_site (auth) created.");
            } else {
                println!("Auth site already exists.")
            }            
        },
        Err(e) => {
            println!("DB Error: {e}");
        }
    };
}