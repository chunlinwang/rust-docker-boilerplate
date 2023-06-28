use actix_web::{web, get, put, HttpResponse, Responder};
use crate::orm::address::*;

#[get("/address/{id}")]
pub async fn getAddress(web::Path(id): web::Path<String>) -> impl Responder {
    let mut resposne = String::from("Hello world! address");
    resposne.push_str(_hash.as_str());

    HttpResponse::Ok().body(resposne)
}

#[put("/addresses/{id}")]
pub async fn updateAddress(web::Path(id): web::Path<String>, web::Json(data): web::Json<Address>) -> impl Responder {
    let mut resposne = String::from("Hello world! address");
    resposne.push_str(_hash.as_str());

    HttpResponse::Ok().body(resposne)
}

#[get("/addresses")]
pub async fn getAddressesForUser(web::Json(userId): web::Json<String>) -> impl Responder {
    let mut resposne = String::from("Hello world! address");
    resposne.push_str(_hash.as_str());

    HttpResponse::Ok().body(resposne)
}




