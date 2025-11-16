/*
 * NOTES ABOUT MIDDLEWARE:
 * If multiple middleware functions are chained in the App::new() chain (inside a wrap() function)
 * they are each called in sequence, and they can each act upon the request and change the request.
 * In any function, post-processing can happen after the next.call(req).await call.
 * That post-processing happens AFTER all the later calls
 */


use actix_web::{
    Error, HttpMessage,
    body::MessageBody, dev::{ServiceRequest, ServiceResponse},
    middleware::{ Next } };

use crate::auth;

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
 */
pub async fn login_status_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> { 
    let user_req_data: auth::UserReqData = match req.cookie("jwt") {
        Some(cookie) => {
            match auth::verify_jwt(cookie.value()) {
                Ok(claims) => auth::UserReqData::new(Some(claims)),
                Err(_e) => auth::UserReqData::new(None)
            }
        },
        None => auth::UserReqData::new(None)
    };

    // Put UserReqData into the request object to identify user to all routes.
    req.extensions_mut().insert(user_req_data);

    next.call(req).await
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