use serde::{ Deserialize, Serialize };
use actix_web::{
    web, HttpResponse, HttpRequest,
    Responder, http::StatusCode, http::header,
    get, post, web::Redirect };

    use crate::{ db, auth, utils,
        resource_mgr::{ error_by_code },
        routes_utils::{
            ErrorResponse,
            return_authentication_err_json,
            return_internal_err_json,
            return_not_found_err_json
        }
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



#[derive(Deserialize)]
struct ClientAuthData {
    code: String,
    client_id: String,
    client_secret: String,
}

#[derive(Serialize)]
struct UserData {
    user_id: i32,
    username: String,
    refresh_token: String,
}


#[post("/verify_auth_code")]
async fn verify_auth_code(req: HttpRequest, inputs: web::Json<ClientAuthData>) -> HttpResponse {

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

    let auth_code_data: db::AuthCodeData = match db::get_auth_code_data(&inputs.client_id).await {
        Ok(option) => {
            match option {
                Some(data) => data,
                None => { return return_not_found_err_json() }
            }
        },
        Err(_e) => { return return_internal_err_json() }
    };

    // GOT the auth_code_data. Now check it against the input data
    // make sure client_id and client_secret are the right ones.

    let hashed_client_secret: String = match db::get_client_secret(&inputs.client_id).await {
        Ok(option) => {
            match option {
                Some(secret_obj) => secret_obj.hashed_client_secret,
                None => { return return_not_found_err_json() }
            }
        },
        Err(_e) => { return return_internal_err_json() }
    };

    let secrets_match: bool = auth::verify_password(&inputs.client_secret, &hashed_client_secret);
    let client_ids_match: bool = inputs.client_id == auth_code_data.id.to_string();

    if secrets_match && client_ids_match {

        let username = match db::get_username_by_id(auth_code_data.id).await {
            Ok(option) => {
                match option {
                    Some(username_obj) => username_obj.username,
                    None => { return return_not_found_err_json() }
                }
            },
            Err(_e) => { return return_internal_err_json() }
        };

        // CREATE the refresh token and save to DB
        // create a refresh_token and put it in the DB
        let refresh_token: String = match db::add_refresh_token(
            auth_code_data.user_id,
            auth_code_data.client_id,
            auth::generate_refresh_token()
        ).await {
            Ok(refresh_token) => refresh_token,
            Err(_e) =>  return return_internal_err_json()
        };


        let user_data: UserData = UserData {
            user_id: auth_code_data.id,
            username,
            refresh_token
        };

        return HttpResponse::Ok()
            .json(user_data);
    }


    // RETURN FAILURE INSTEAD

    let user_data: UserData = UserData {
        user_id: 1,
        username: "pattmayne".to_string(),
        refresh_token: "fgdgdfgdfgd".to_string(),
    };

    HttpResponse::Ok()
        .json(user_data)

}
