use jsonwebtoken::{encode, Header, EncodingKey };
use serde::{ Serialize, Deserialize };
use time::{ Duration, OffsetDateTime };
use actix_web::cookie::{Cookie, SameSite};


/*
 * TOKEN USAGE AND STORAGE
 *
 * ACCESS TOKENS: JWTs (JSON Web Tokens)
 * -------------
 * -- short-lived (few minutes to an hour) (probably 20 minutes)
 * -- FRONT END STORAGE has TWO SCHEMES:
 * -- -- SPA: For single-page applications (game page), store in JavaScript memory in the front-end
 * -- -- -- in this scenario we send the JWT back for each API call which updates live game state
 * -- -- -- This is the scenario we will always use within the auth app
 * -- -- MPA: For clicking between pages (everywhere else), store in an HTTP-only cookie
 * -- -- -- backend sets this in the response. front-end JS cannot touch it or read it.
 * -- -- -- access token is then sent safely within headers
 * -- -- SWITCHING b/w schemes requires checking refresh token & sending new JWT
 * -- not stored in the backend at all
 * -- algorithmically verified in the backend
 * -- sent back for each request that requires being logged in
 * -- no need for sessions, as this is your ticket for each request
 * -- when expired, user must send refresh token (different token) to get a new access token
 * 
 * 
 * REFRESH TOKENS: Just a long random string
 * -------------
 * -- long-lived (several days to several weeks)
 * -- stored in HttpOnly Cookie in the front-end (protected against XXS)
 * -- stored in database (Users table) in the backend
 * -- Only sent to the backend when user needs a new JWT access token
 * -- backend verifies by checking received token against the one in the DB
 * -- when expired, user has to log in
 * -- logging in and registering generate refresh token
 *
 *
 * TO DO:
 * -- Create refresh_token table
 * -- -- This allows multiple refresh tokens, one for each device, to stay logged in
 * -- -- store extra data cleanly like expiry date, scope, device info
 * -- -- delete after expiry
 * -- Implement OAuth2 or OpenID Connect (OIDC) to authenticate external sites
 * -- -- This app can't set cookies for apps running on other domains.
 * -- -- Therefore we need to research and implement this other protocol
 */



// A Claim is the token-bearer's claims on being a certain identity (subject or sub) and other data
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    role: String,
    exp: usize, // expiration as a timestamp (seconds since epoch)
}


pub fn generate_jwt(user_id: i32, role: String, secret: &[u8])
    -> Result<String, jsonwebtoken::errors::Error> 
{
    // Set expiration for 1 hour from now
    let exp: usize = (OffsetDateTime::now_utc() + Duration::hours(1)).unix_timestamp() as usize;

    let claims: Claims = Claims {
        sub: user_id,
        role,
        exp,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}




/**
 * Setting a cookie only works for browsing within the auth site
 * For external app authentication we will implement OAuth2
 */
pub fn build_token_cookie(token: String, name: String) -> Cookie<'static> {

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