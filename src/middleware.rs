use actix_web::{
    Error, HttpMessage,
    body::MessageBody, dev::{ServiceRequest, ServiceResponse},
    middleware::{ Next } };

use crate::auth;

/* MIDDLEWARE FUNCTIONS */

pub async fn login_status_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    
    // pre-processing

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
    // post-processing (what can go here?)
}