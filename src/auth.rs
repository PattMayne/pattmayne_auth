use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Result as JWTResult};
use serde::{Serialize, Deserialize};
use std::env;
use time::{ Duration, OffsetDateTime };



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
