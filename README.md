# PATTMAYNE AUTH

An authentication app I will use to serve a few web apps and games I intend to make.
I'll use JSON webtokens and a postgresql database.

### REQUIRED CRATES:
* actix-web = "4"
* jsonwebtoken = { version = "10", features = ["rust_crypto"] }
* sqlx = "0.8.6"
* tokio = "1.47.2"
* askama = "0.14.0"
* actix-web-httpauth = "0.8.2"
* actix-files = "0.6.8"
* serde = "1.0.228"
* serde_json = "1.0.145"
* dotenvy = "0.15.7"dotenvy = "0.15.7"


### TO DO:
 * on login or register page, the form sends a post request and creates a logged in user for main / page
 * nav in header included in template ( HOME | LOGOUT | LOGIN | REGISTER ) (send User obj to header template)
 * incorporate database and make schema
 * suspend IP address if too many failed attempts
 * create actual JWT
 * remove extra routes
 * style nicely (html and css)
 * create endpoints for another app to authenticate
 * * create an enum of apps that can use this
 * * or MAYBE they should be in the DB instead.
 * Containerize with Docker
 
