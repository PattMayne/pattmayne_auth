# Pattmayne Auth

An authentication app I will use to serve a few web apps and games I intend to make.
I'll use JSON webtokens and a postgresql database.

REQUIRED CRATES:
* actix-web = "4"
* jsonwebtoken = { version = "10", features = ["rust_crypto"] }
* sqlx = "0.8.6"
* tokio = "1.47.2"
* askama = "0.14.0"
* actix-web-httpauth = "0.8.2"
* actix-files = "0.6.8"
