use serde::{ Serialize };
use actix_web::{
    web, HttpResponse, HttpRequest,
    http::StatusCode, post };

    use crate::{ db, auth,
        auth_code_shared::{
            AuthCodeSuccess,
            AuthCodeError,
            AuthCodeRequest,
            AuthCodeResponse,
        },
    };
/* 
 * 
 * 
 * 
 * 
 * =============================
 * =============================
 * =====                   =====
 * =====  EXTERNAL ROUTES  =====
 * =====                   =====
 * =============================
 * =============================
 * 
 * 
 * 
 * Routes to be called by external client apps.
 * 
 * /login should actually have a dropdown of client sites (not in external scope actually)
 * /refresh will verify refresh token, return OK
 * 
 * 
 * FLOW:
 * 
 * For any client app, START from the auth app's login page.
 * User selects which site to login to.
 * User is redirected
 * 
*/



#[derive(Serialize)]
struct UserData {
    user_id: i32,
    username: String,
    refresh_token: String,
}


#[post("/verify_auth_code")]
async fn verify_auth_code(req: HttpRequest, inputs: web::Json<AuthCodeRequest>) -> HttpResponse {

    println!("Code: {}", inputs.code);
    println!("Id: {}", inputs.client_id);
    println!("Secret: {}", inputs.client_secret);

    /* 
     * From DB gather:
     * * THINGS TO CHECK
     * * THINGS TO SEND TO USER
     * 
     * THINGS TO CHECK:
     * * auth_codes.user_id
     * * auth_codes.client_id
     * * auth_codes.expiry_date
     * 
     * THINGS TO SEND:
     * * user.sub (id)
     * * user.username
     * * user.role
     * * refresh_token
     * 
     * We will have to CREATE the refresh_token
     * 
     * We will need a custom error struct
     */

    let auth_code_data: db::AuthCodeData = match db::get_auth_code_data(&inputs.code).await {
        Ok(option) => {
            match option {
                Some(data) => data,
                None => { return ext_not_found_err_json() }
            }
        },
        Err(_e) => { return ext_internal_err_json() }
    };

    // Make sure it's not expired
    if auth_code_data.is_expired() {
        eprint!("Expired auth code");
        println!("auth code id: {}", auth_code_data.id);
        return ext_authentication_err_json();
    }

    // GOT the auth_code_data. Now check it against the input data
    // make sure client_id and client_secret are the right ones.

    let hashed_client_secret: String = match db::get_client_secret(&inputs.client_id).await {
        Ok(option) => {
            match option {
                Some(secret_obj) => secret_obj.hashed_client_secret,
                None => { return ext_not_found_err_json() }
            }
        },
        Err(_e) => { return ext_internal_err_json() }
    };

    let secrets_match: bool = auth::verify_password(&inputs.client_secret, &hashed_client_secret);
    let client_ids_match: bool = inputs.client_id == auth_code_data.client_id;

    // TODO: check auth_code EXPIRY date

    if secrets_match && client_ids_match {

        println!("SUCCESS: ALL MATCH");

        let username = match db::get_username_by_id(auth_code_data.user_id).await {
            Ok(option) => {
                match option {
                    Some(username_obj) => username_obj.username,
                    None => { return ext_not_found_err_json() }
                }
            },
            Err(_e) => { return ext_internal_err_json() }
        };

        // CREATE the refresh token and save to DB
        // create a refresh_token and put it in the DB
        let refresh_token: String = match db::add_refresh_token(
            auth_code_data.user_id,
            auth_code_data.client_id,
            auth::generate_refresh_token()
        ).await {
            Ok(refresh_token) => refresh_token,
            Err(_e) =>  return ext_internal_err_json()
        };


        let user_data: AuthCodeSuccess = AuthCodeSuccess {
            user_id: auth_code_data.user_id,
            username,
            refresh_token
        };

        // now DELETE the auth token

        println!("SUCCESS: SENDING");

        return HttpResponse::Ok()
            .json(user_data);
    }


    println!("FAILURE: NO MATCH");
    // RETURN FAILURE
    ext_authentication_err_json()
}


// If something is not found
pub fn ext_not_found_err_json() -> HttpResponse {
    HttpResponse::Unauthorized().json(AuthCodeError{
        message: String::from("Not Found"),
        error_code: 406
    })
}


/**
 * Sometimes we don't know what went wrong and we need to return a JSON
 * object which says so.
 */
pub fn ext_internal_err_json() -> HttpResponse {
    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
        .json(AuthCodeResponse::Err(
            AuthCodeError{
                message: String::from("Internal server error"),
                error_code: 500
        }))
}



// authentication failed
pub fn ext_authentication_err_json() -> HttpResponse {
    HttpResponse::Unauthorized().json(AuthCodeResponse::Err(
        AuthCodeError{
            message: String::from("Authentication required"),
            error_code: 401
    }))
}
