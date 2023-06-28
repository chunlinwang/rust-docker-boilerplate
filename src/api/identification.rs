use actix_web::{post, HttpResponse};
use actix_identity::Identity;
use crate::jwt;
use actix_web::cookie::Cookie;
use serde::{Deserialize, Serialize};
use serde_json;
use crate::constants::AUTHENTICATION_COOKIE_NAME;
use jsonwebtoken::errors::Error;

#[post("/login")]
pub async fn login(id: Identity) -> HttpResponse {
    println!("{:?}", id.identity());
    id.remember("User1".to_owned()); // <- remember identity

    let token: Result<String, Error> = jwt::build_token();

    let response = match token {
        Ok(token) => {
            let authCookie: Cookie = Cookie::new(AUTHENTICATION_COOKIE_NAME, token);
            HttpResponse::Ok().cookie(authCookie).finish()
        },
        Err(e) => {
            HttpResponse::Unauthorized().body(e.to_string())
        },
    };

    response
}

#[post("/logout")]
pub async fn logout(id: Identity) -> HttpResponse {
    println!("{:?}", id.identity());
    id.forget();                      // <- remove identity
    HttpResponse::Ok().finish()
}


