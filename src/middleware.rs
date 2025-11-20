/* 
 * ========================
 * ========================
 * =====              =====
 * =====  MIDDLEWARE  =====
 * =====              =====
 * ========================
 * ========================
 * 
 * 
 * 
 * If multiple middleware functions are chained in the App::new() chain (inside a wrap() function)
 * they are each called in sequence, and they can each act upon the request and change the request.
 * In any function, post-processing can happen after the next.call(req).await call.
 * That post-processing happens AFTER all the later calls
 */


use std::default;

use actix_web::{
    Error, HttpMessage,
    body::MessageBody, dev::{ServiceRequest, ServiceResponse},
    middleware::{ Next } };

use crate::{auth, db, utils};


pub struct NewJwtObj {
    token: String
}

impl NewJwtObj {
    pub fn new(token: String) -> Self {
        NewJwtObj { token }
    }

    pub fn get_token(&self) -> &String { &self.token }
}


/* MIDDLEWARE FUNCTIONS */

/**
 * Check for JSON web token in req's cookies, and validate the token.
 * Create a UserReqData object indicating whether user is logged in,
 * or a guest (based on whether JWT is valid).
 * Put that UserReqData object into the response for later functions.
 * 
 * TODO: If JWT is valid EXCEPT that it's expired,
 * we must check for a valid refresh token in the cookie.
 * If valid refresh token exists, generate and deliver a new JWT.
 * Cookie will be added in post-processing based on previously explained logic.
 * 
 * TODO: This is deeply nested. Rewrite it to be more readable.
 */
pub async fn login_status_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {

    let guest_data: auth::UserReqData = auth::UserReqData::new(None);
    let user_req_data_opt: Option<actix_web::cookie::Cookie<'_>> = req.cookie("jwt");
    let user_req_data: auth::UserReqData = get_user_req_data_from_opt(
        user_req_data_opt,
        &req,
        guest_data
    ).await?;

    // Put UserReqData into the request object to identify user to all routes.
    req.extensions_mut().insert(user_req_data);

    next.call(req).await
}


pub async fn jwt_cookie_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, Error> where B: MessageBody, {

    let mut res: ServiceResponse<B> = next.call(req).await?;

    let new_jwt: Option<String> = res
        .request()
        .extensions()
        .get::<NewJwtObj>()
        .map(|obj| obj.get_token().to_owned());

    // After handler, check for the NewJwt flag and add cookie if present
    if let Some(token) = new_jwt {
        println!("INSERTING NEW JWT COOKIE");
        let cookie: actix_web::cookie::Cookie<'_> =
            auth::build_token_cookie(
                token,
                String::from("jwt")
            );

        res.response_mut().add_cookie(&cookie).ok();
    }
    Ok(res)
}

// NOTE FOR LATER: HOW TO ADD A COOKIE WITH NEW JWT WHEN NEEDED (it will inevitable be needed!)

/*

    // Call the next service (route handler)
    let mut res = next.call(req).await?;

    // POST-PROCESSING!!!

    // Create your cookie
    let cookie = actix_web::cookie::Cookie::build("my_random_cookie", "cookie_value")
        .http_only(true)
        .secure(true)
        .path("/")
        .finish();

    // Add the cookie to the response
    res.response_mut().add_cookie(&cookie)?;

    Ok(res)


*/

// This should return a result so we can return errors (now that it's getting simpler)
async fn get_user_req_data_from_opt(
    option: Option<actix_web::cookie::Cookie<'_>>,
    req: &ServiceRequest,
    guest_data: auth::UserReqData
) -> Result<auth::UserReqData, Error> {
    // This was deeply nested match expressions, so we're checking for none instead

    if option.is_none() { return Ok(guest_data); }
    let jwt_cookie: actix_web::cookie::Cookie<'_> = option.unwrap();

    // Must use match here because of multiple enums
    match auth::verify_jwt(jwt_cookie.value()).await {
        auth::JwtVerification::Valid(claims) => {
            Ok(auth::UserReqData::new(Some(claims)))
        },
        auth::JwtVerification::Expired(claims) => {
            // JWT is expired but otherwise valid.
            // set an object in the req to send a new cookie
            /* 
                * PROCESS:
                * => check REFRESH TOKEN
                * => if that is valid (and non-expired):
                * ====> set FLAG for setting the new JWT
                * => ELSE (TO DO)
                * ====> set FLAG to make user log in again
                */
            
            // Check the cookies for a refresh_token
            let r_token_optn = req.cookie("refresh_token");
            if r_token_optn.is_none() { return Ok(guest_data); }
            let r_tkn_ckie: actix_web::cookie::Cookie<'_> = r_token_optn.unwrap();

            // check DB for refresh_token to compare
            let r_db_token_result: Result<Option<db::RefreshToken>, anyhow::Error> = db::get_refresh_token(
                claims.get_sub(),
                utils::auth_client_id()
            ).await;

            if r_db_token_result.is_err() {
                // actully return an error when we switch to Result return type
                return Ok(guest_data);
            }

            let r_db_token_option: Option<db::RefreshToken> = r_db_token_result.unwrap();
            if r_db_token_option.is_none() { return Ok(guest_data); }
            let r_db_token = r_db_token_option.unwrap();

            let r_tkn_valid: bool = 
                r_tkn_ckie.value() == r_db_token.get_token() &&
                !r_db_token.is_expired();

            if r_tkn_valid {
                // CREATE and GIVE NEW JWT
                let new_jwt_rslt: Result<String, auth::AuthError> =
                    auth::generate_jwt(
                        claims.get_sub(),
                        claims.get_username().to_owned(),
                        claims.get_role().to_owned()
                    );
                
                if new_jwt_rslt.is_err() {
                    // actully return an error when we switch to Result return type
                    return Ok(guest_data);
                }

                let new_jwt = new_jwt_rslt.unwrap();
                req.extensions_mut().insert(NewJwtObj::new(new_jwt));
                println!("getting new JWT");
                return Ok(auth::UserReqData::new(Some(claims)));
            } else {
                Ok(guest_data)
            }                   
        },
        auth::JwtVerification::Invalid => Ok(guest_data)
    }
}