use serde::{ Deserialize, Serialize };
use actix_web::{
    web, HttpResponse, HttpRequest,
    Responder, http::StatusCode, http::header,
    get, post, web::Redirect };

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

    let user_data: UserData = UserData {
        user_id: 1,
        username: "pattmayne".to_string(),
        refresh_token: "fgdgdfgdfgd".to_string(),
    };

    HttpResponse::Ok()
        .json(user_data)

}
