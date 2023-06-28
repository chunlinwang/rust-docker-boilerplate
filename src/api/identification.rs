use actix_web::{get, post, HttpResponse, Responder, HttpRequest, HttpMessage};
use actix_identity::Identity;
use crate::jwt;
use actix_web::cookie::Cookie;
use serde::{Deserialize, Serialize};
use serde_json;
use crate::constants::AUTHENTICATION_COOKIE_NAME;
use jsonwebtoken::errors::Error;

#[post("/login")]
pub async fn login(request: HttpRequest) -> HttpResponse {

    Identity::login(&request.extensions(), "User1".into()).unwrap();

    let token: Result<String, Error> = jwt::build_token();

    let response = match token {
        Ok(token) => {
            let auth_cookie: Cookie = Cookie::new(AUTHENTICATION_COOKIE_NAME, token);
            HttpResponse::Ok().cookie(auth_cookie).finish()
        },
        Err(e) => {
            HttpResponse::Unauthorized().body(e.to_string())
        },
    };

    response
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}


