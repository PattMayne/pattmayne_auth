use jsonwebtoken::{
    encode, Header, EncodingKey, decode,
    DecodingKey, Validation, Algorithm, errors::Error };
use serde::{ Serialize, Deserialize };
use time::{ Duration, OffsetDateTime };
use actix_web::{ HttpRequest, HttpMessage, cookie::{Cookie, SameSite}};


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


/* 
 * This holds that data that gets encoded into a JSON Web Token (JWT).
 * The user is "claiming" to be a certain identity.
 */ 
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: i32,
    role: String,
    username: String,
    exp: usize, // expiration as a timestamp (seconds since epoch)
}

/* 
 * Middleware will insert this struct into every request so the routes
 * know who they're dealing with.
 * The most basic data about a user, info we might casually need
 * on any page.
 */
#[derive(Clone)]
pub struct UserReqData {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub role: String, // guest, player, admin
    pub logged_in: bool,
}

/**
 * UserReqData is built from a Claims object.
 * If we have a claims struct, the user must be logged in, so send it in as Some<claims>.
 * Otherwise, send in None and we will make a generic Guest object.
 */
impl UserReqData {

    /**
     * If a Claims struct is available then build UserReqData from it.
     * Otherwise generate a generic guest struct.
     */
    pub fn new(claims_option: Option<Claims>) -> Self {
        match claims_option {
            Some(claims) => {
                UserReqData {
                    id: Some(claims.get_sub()),
                    username: Some(claims.get_username().to_owned()),
                    role: claims.get_role().to_owned(),
                    logged_in: true,
                }
            },
            None => {
                UserReqData {
                    id: None,
                    username: None,
                    role: String::from("guest"),
                    logged_in: false,
                }
            }
        }
    }
}

/* functions for the Claims struct */
impl Claims {
    pub fn get_sub(&self) -> i32 { self.sub }
    pub fn get_role(&self) -> &String { &self.role }
    pub fn get_username(&self) -> &String { &self.username }
    pub fn get_exp(&self) -> usize { self.exp }
}

/**
 * JSON Web Token generator.
 * Take some info about the user to create a Claims struct,
 * along with the JWT secret,
 * and generate an encoded JWT String
 * to use as an access token for the user.
 */
pub fn generate_jwt(
    user_id: i32,
    username: String,
    role: String,
    secret: &[u8]
) -> Result<String, jsonwebtoken::errors::Error> {
    // Set expiration for 1 hour from now
    let exp: usize = (OffsetDateTime::now_utc() + Duration::hours(1)).unix_timestamp() as usize;

    let claims: Claims = Claims {
        sub: user_id,
        username,
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



/**
 * Decode the jwt string, check it against the Claims struct, and check
 * expiry date. If all is well, return the Claims stuct in case we want to
 * use that data or check it against DB data.
 */
pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // get the jwt secret so we can decode the jwt string
    let secret: String = match get_jwt_secret() {
        Ok(s) => s,
        Err(_e) =>
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidKeyFormat ))
    };

    // HS256 algorithm matches the header default I use to encode
    let validation: Validation = Validation::new(Algorithm::HS256);
    let token_data: jsonwebtoken::TokenData<Claims> = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}


// Get the JWT secret from env variables
pub fn get_jwt_secret() -> Result<String, std::env::VarError> {
    std::env::var("JWT_SECRET")
}


/**
 * Send in the request and we'll extract the UserReqData for you.
 * If it doesn't exist we'll assumed the user is a guest, and we will
 * make a new UserReqData for you.
 */
pub fn get_user_req_data(req: &HttpRequest) -> UserReqData {
    let guest_user: UserReqData = UserReqData::new(None);
    let extensions: std::cell::Ref<'_, actix_web::dev::Extensions> = req.extensions();

    // Get user data from req
    match extensions.get::<UserReqData>() {
        Some(user_data) => user_data,
        None => &guest_user
    }.to_owned()
}